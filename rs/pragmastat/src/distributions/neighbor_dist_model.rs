//! Local step-density distribution reconstructed from a sample.

use crate::Rng;
use crate::assumptions::{
    AssumptionError, EstimatorError, Subject, check_cumulance, check_query_validity, check_validity,
};
use crate::dist_numerics::{
    ceil_sqrt, lerp, lower_bound, midpoint, next_down, unit_ratio, upper_bound,
};
use crate::dist_sample_guard::prepare_dist_sample;
use crate::estimators::normalize_zero;
use crate::measurement_unit::MeasurementUnit;
use crate::sample::{Sample, check_non_weighted};

use super::Distribution;
use super::measured_dist::{MeasuredDist, MeasuredModel};
use super::neighbor_landscape::NeighborLandscape;

/// Normative algorithm identifier. Changing the window rule, the boundaries or the
/// normalization requires a new identifier.
pub const NEIGHBOR_DIST_MODEL_ALGORITHM: &str = "dist-neighbor-v1";

/// Local step-density distribution reconstructed from a sample (algorithm `dist-neighbor-v1`).
///
/// Each observation contributes a cell whose height reflects how tightly its nearest neighbors
/// are packed, so the reconstruction follows local structure instead of imposing a single global
/// shape. The window is indexed by `k0 = ceil(sqrt(n))` with radius `r = floor(k0/2)`; windows
/// near the ends are truncated and the numerator is always the actual neighbor count.
///
/// The construction is kNN-inspired but is not the Loftsgaarden-Quesenberry estimator: support
/// is bounded by the observed range and the cell masses are explicitly normalized, so the
/// density integrates to one.
///
/// Only quantile carries a unit -- the sample's own, attached through
/// [`NeighborDistModel::measured`]. Cumulative answers in probabilities and density is a
/// terminal value: both the numeric and the measured path return plain numbers.
///
/// A built model is immutable, so concurrent `quantile`/`cumulative`/`density` reads are safe.
#[derive(Debug)]
pub struct NeighborDistModel {
    boundaries: Vec<f64>,
    cumulative: Vec<f64>,
    masses: Vec<f64>,
    densities: Vec<f64>,
    sources: Vec<usize>,
    scale_index: usize,
    radius: usize,
    unit: MeasurementUnit,
}

impl NeighborDistModel {
    /// Builds the model from a native numeric vector.
    ///
    /// `values` must be non-empty, finite, of size at least two, with a finite representable
    /// range and pairwise distinct elements.
    ///
    /// If `assume_sorted` is true, `values` is assumed already sorted ascending and the internal
    /// sort is skipped. Passing an unsorted slice under this flag is a contract violation with an
    /// unspecified but terminating result.
    ///
    /// The model built this way is dimensionless. A caller holding a measured [`Sample`] uses
    /// [`NeighborDistModel::from_sample`] instead.
    pub fn new(values: &[f64], assume_sorted: bool) -> Result<Self, EstimatorError> {
        check_validity(values, Subject::X)?;
        Ok(Self::trusted(values, assume_sorted)?)
    }

    /// Builds the model from a measured sample and remembers its unit, so the measured queries
    /// reached through [`NeighborDistModel::measured`] carry a dimension.
    ///
    /// The sample already stores its values in ascending order, so the model does not sort
    /// again. Weighted samples are refused: this reconstruction has no weighted form, and
    /// silently ignoring the weights would answer a different question than the caller asked.
    pub fn from_sample(x: &Sample) -> Result<Self, EstimatorError> {
        check_non_weighted("x", x)?;
        let mut model = Self::trusted(x.sorted_values(), true)?;
        model.unit = x.unit().clone();
        Ok(model)
    }

    /// Builds the model from a vector whose validity the caller has already checked.
    pub(crate) fn trusted(values: &[f64], assume_sorted: bool) -> Result<Self, AssumptionError> {
        let x = prepare_dist_sample(values, assume_sorted)?;
        Ok(Self::build(x))
    }

    /// # Panics
    /// Panics if the reconstruction's normalizer is not finite and strictly positive. `x` has
    /// already satisfied `domain(x)` and `distinctness(x)` by the time this runs, so this is not
    /// an assumption violation: it is a numerical-conformance failure, reachable only through a
    /// broken `assume_sorted` promise (undefined behavior).
    fn build(x: Vec<f64>) -> Self {
        let n = x.len();
        let scale_index = ceil_sqrt(n);
        let radius = scale_index / 2;

        let mut counts = vec![0usize; n];
        let mut spans = vec![0.0; n];
        for i in 0..n {
            let left = i.saturating_sub(radius);
            let right = (i + radius).min(n - 1);
            counts[i] = right - left + 1;
            spans[i] = x[right] - x[left];
        }

        let mut e = vec![0.0; n + 1];
        e[0] = x[0];
        for i in 1..n {
            e[i] = midpoint(x[i - 1], x[i]);
        }
        e[n] = x[n - 1];

        // Rounding can collapse the midpoint of two adjacent binary64 observations onto a
        // neighboring boundary. Such a cell carries no mass and is dropped; the surviving cells
        // keep the index of the observation they came from.
        let mut boundaries = Vec::with_capacity(n + 1);
        boundaries.push(e[0]);
        let mut surviving = Vec::with_capacity(n);
        for i in 0..n {
            if e[i + 1] == e[i] {
                continue;
            }
            surviving.push(i);
            boundaries.push(e[i + 1]);
        }

        let s = surviving.len();
        let mut a = vec![0.0; s];
        for (j, &i) in surviving.iter().enumerate() {
            let width = e[i + 1] - e[i];
            a[j] = counts[i] as f64 / n as f64 * (width / spans[i]);
        }

        let total: f64 = a.iter().sum();

        // A broken assume_sorted promise can make a cell wider than the window it normalizes
        // against, so its unnormalized mass overflows, or it can collapse every cell and leave a
        // zero total. Either way the divisor is unusable and NaN would reach the query kernels.
        // For a genuinely ascending sample the total is finite and strictly positive -- it is
        // not bounded by 1, since it is a normalizer rather than a probability; it sits above 1
        // and decays toward it as n grows.
        assert!(
            total.is_finite() && total > 0.0,
            "numerical conformance failure: the NeighborDistModel normalizer is not usable; \
             this indicates an assume_sorted promise that the input does not satisfy"
        );

        // Reserve one representable step for every cell that has not been placed yet, so a mass
        // too small to register at this point in the prefix still advances the cumulative value
        // instead of flattening it into a plateau.
        let mut cap = vec![0.0; s + 1];
        cap[s] = 1.0;
        for j in (0..s).rev() {
            cap[j] = next_down(cap[j + 1]);
        }

        let mut cumulative = vec![0.0; s + 1];
        cumulative[0] = 0.0;
        for j in 0..s.saturating_sub(1) {
            let raw = cumulative[j] + a[j] / total;
            cumulative[j + 1] = raw
                .max(crate::dist_numerics::next_up(cumulative[j]))
                .min(cap[j + 1]);
        }
        cumulative[s] = 1.0;

        let mut masses = vec![0.0; s];
        let mut densities = vec![0.0; s];
        for j in 0..s {
            masses[j] = cumulative[j + 1] - cumulative[j];
            densities[j] = masses[j] / (boundaries[j + 1] - boundaries[j]);
        }

        Self {
            boundaries,
            cumulative,
            masses,
            densities,
            sources: surviving,
            scale_index,
            radius,
            unit: MeasurementUnit::number(),
        }
    }

    /// Number of surviving cells in the reconstruction.
    pub fn cell_count(&self) -> usize {
        self.masses.len()
    }

    /// Lower end of the support.
    pub fn support_min(&self) -> f64 {
        self.boundaries[0]
    }

    /// Upper end of the support.
    pub fn support_max(&self) -> f64 {
        self.boundaries[self.boundaries.len() - 1]
    }

    /// Unit of the observations this model was built from. A model built from a native numeric
    /// vector is dimensionless.
    pub fn unit(&self) -> &MeasurementUnit {
        &self.unit
    }

    /// The measured view of this model: the same three queries, each carrying the dimension its
    /// result actually has.
    pub fn measured(&self) -> MeasuredDist<'_, Self> {
        MeasuredDist::new(self)
    }

    /// Value at each requested cumulance, in the order the cumulances were given.
    pub fn quantile(&self, cumulances: &[f64]) -> Result<Vec<f64>, EstimatorError> {
        check_query_validity(cumulances, Subject::Y)?;
        check_cumulance(cumulances, Subject::Y)?;
        Ok(self.quantile_trusted(cumulances))
    }

    /// Probability of not exceeding each requested value, in the order given.
    pub fn cumulative(&self, values: &[f64]) -> Result<Vec<f64>, EstimatorError> {
        check_query_validity(values, Subject::Y)?;
        Ok(self.cumulative_trusted(values))
    }

    /// Density at each requested value, in the order the values were given.
    pub fn density(&self, values: &[f64]) -> Result<Vec<f64>, EstimatorError> {
        check_query_validity(values, Subject::Y)?;
        Ok(self.density_trusted(values))
    }

    fn quantile_trusted(&self, cumulances: &[f64]) -> Vec<f64> {
        cumulances.iter().map(|&c| self.quantile_at(c)).collect()
    }

    fn cumulative_trusted(&self, values: &[f64]) -> Vec<f64> {
        values.iter().map(|&v| self.cumulative_at(v)).collect()
    }

    fn density_trusted(&self, values: &[f64]) -> Vec<f64> {
        values.iter().map(|&v| self.density_at(v)).collect()
    }

    /// The staircase this model is: cell boundaries, cumulative values, masses and heights.
    ///
    /// Point evaluation answers what the density is at a value a caller picked; this answers
    /// what the density *is*, on the boundaries the model itself uses. Every height is the
    /// expression [`NeighborDistModel::density_at`] evaluates inside the same cell, so the two
    /// agree bit for bit rather than approximately.
    pub fn get_landscape(&self) -> NeighborLandscape {
        NeighborLandscape::new(
            self.scale_index,
            self.radius,
            self.boundaries.clone(),
            self.cumulative.clone(),
            self.masses.clone(),
            self.densities.clone(),
            self.sources.clone(),
        )
    }
}

impl Distribution for NeighborDistModel {
    fn sample(&self, rng: &mut Rng) -> f64 {
        self.quantile_at(rng.uniform_f64())
    }

    /// NaN first, for the same reason `LinearDistModel::quantile_at` checks it: every branch
    /// below compares false against it, and the fallback would otherwise depend on how the
    /// platform converts a NaN cumulance to an index.
    fn quantile_at(&self, p: f64) -> f64 {
        if p.is_nan() {
            return f64::NAN;
        }

        let s = self.masses.len();
        let value = if p <= 0.0 {
            self.boundaries[0]
        } else if p >= 1.0 {
            self.boundaries[s]
        } else {
            let j = lower_bound(&self.cumulative, 1, s + 1, p) - 1;
            let t = unit_ratio(p - self.cumulative[j], self.masses[j]);
            lerp(self.boundaries[j], self.boundaries[j + 1], t)
        };

        // Every branch hands back a value built from the sample's own boundaries -- the clamped
        // ones return one verbatim, and lerp reproduces its endpoints exactly -- so a -0.0 among
        // the observations would otherwise reach the caller unnormalized.
        normalize_zero(value)
    }

    fn cumulative_at(&self, x: f64) -> f64 {
        if x.is_nan() {
            return f64::NAN;
        }

        let s = self.masses.len();
        if x <= self.boundaries[0] {
            return 0.0;
        }
        if x >= self.boundaries[s] {
            return 1.0;
        }

        let j = upper_bound(&self.boundaries, x) - 1;
        let t = unit_ratio(
            x - self.boundaries[j],
            self.boundaries[j + 1] - self.boundaries[j],
        );
        lerp(self.cumulative[j], self.cumulative[j + 1], t)
    }

    fn density_at(&self, x: f64) -> f64 {
        if x.is_nan() {
            return f64::NAN;
        }

        let s = self.masses.len();
        if x < self.boundaries[0] || x > self.boundaries[s] {
            return 0.0;
        }

        let j = if x == self.boundaries[s] {
            s - 1
        } else {
            upper_bound(&self.boundaries, x) - 1
        };
        self.masses[j] / (self.boundaries[j + 1] - self.boundaries[j])
    }
}

impl MeasuredModel for NeighborDistModel {
    fn quantile(&self, cumulances: &[f64]) -> Result<Vec<f64>, EstimatorError> {
        NeighborDistModel::quantile(self, cumulances)
    }

    fn cumulative(&self, values: &[f64]) -> Result<Vec<f64>, EstimatorError> {
        NeighborDistModel::cumulative(self, values)
    }

    fn density(&self, values: &[f64]) -> Result<Vec<f64>, EstimatorError> {
        NeighborDistModel::density(self, values)
    }

    fn unit(&self) -> &MeasurementUnit {
        NeighborDistModel::unit(self)
    }
}
