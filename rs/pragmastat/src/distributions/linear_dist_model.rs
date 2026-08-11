//! Piecewise-linear distribution reconstructed from a sample.

use crate::Rng;
use crate::assumptions::{
    AssumptionError, EstimatorError, Subject, check_cumulance, check_query_validity, check_validity,
};
use crate::dist_numerics::{lerp, upper_bound};
use crate::dist_sample_guard::prepare_dist_sample;
use crate::estimators::normalize_zero;
use crate::measurement_unit::MeasurementUnit;
use crate::sample::{Sample, check_non_weighted};

use super::Distribution;
use super::linear_landscape::LinearLandscape;
use super::measured_dist::{MeasuredDist, MeasuredModel};

/// Normative algorithm identifier. Changing the anchors or the interpolation rule requires a new
/// identifier, not a silent edit.
pub const LINEAR_DIST_MODEL_ALGORITHM: &str = "dist-linear-v1";

/// Piecewise-linear distribution reconstructed from a sample (algorithm `dist-linear-v1`).
///
/// The model interpolates the order statistics at the anchors `(i-1)/(n-1)`, the Hyndman-Fan
/// type 7 rule. Support is `[min(x), max(x)]`.
///
/// Only quantile carries a unit -- the sample's own, attached through
/// [`LinearDistModel::measured`]. Cumulative answers in probabilities and density is a terminal
/// value: both the numeric and the measured path return plain numbers.
///
/// A built model is immutable, so concurrent `quantile`/`cumulative`/`density` reads are safe.
#[derive(Debug)]
pub struct LinearDistModel {
    x: Vec<f64>,
    unit: MeasurementUnit,
}

impl LinearDistModel {
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
    /// [`LinearDistModel::from_sample`] instead.
    pub fn new(values: &[f64], assume_sorted: bool) -> Result<Self, EstimatorError> {
        check_validity(values, Subject::X)?;
        Ok(Self::trusted(values, assume_sorted)?)
    }

    /// Builds the model from a measured sample and remembers its unit, so the measured queries
    /// reached through [`LinearDistModel::measured`] carry a dimension.
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
        Ok(Self {
            x,
            unit: MeasurementUnit::number(),
        })
    }

    /// Number of observations backing the model.
    pub fn size(&self) -> usize {
        self.x.len()
    }

    /// Lower end of the support.
    pub fn support_min(&self) -> f64 {
        self.x[0]
    }

    /// Upper end of the support.
    pub fn support_max(&self) -> f64 {
        self.x[self.x.len() - 1]
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

    /// The staircase this model is: cell boundaries, cumulative values and heights.
    ///
    /// Point evaluation answers what the density is at a value a caller picked; this answers
    /// what the density *is*, on the boundaries the model itself uses. Drawing the second from
    /// the first means guessing a grid and rebuilding the normalization on top of it -- a second
    /// reconstruction of the same object that drifts from this one silently. Every height is the
    /// expression [`LinearDistModel::density_at`] evaluates inside the same cell, so the two
    /// agree bit for bit rather than approximately.
    pub fn get_landscape(&self) -> LinearLandscape {
        let n = self.x.len();
        // The general cumulative_at path for an exact order statistic x[i] always resolves
        // j = i and t = 0.0, so it reduces to exactly i as f64 / (n - 1) as f64 -- computing
        // that directly skips the O(log n) binary search per boundary without changing a
        // single bit of the result.
        let cumulative: Vec<f64> = (0..n).map(|i| i as f64 / (n - 1) as f64).collect();
        let densities: Vec<f64> = (0..n - 1)
            .map(|j| 1.0 / (n - 1) as f64 / (self.x[j + 1] - self.x[j]))
            .collect();
        LinearLandscape::new(self.x.clone(), cumulative, densities)
    }
}

impl Distribution for LinearDistModel {
    fn sample(&self, rng: &mut Rng) -> f64 {
        self.quantile_at(rng.uniform_f64())
    }

    /// NaN first: every branch below compares false against it, and the fallback arithmetic
    /// would otherwise depend on how the platform casts a NaN cumulance to an index. The same
    /// guard opens `cumulative_at` and `density_at`, so all three answer the same way to the
    /// same non-input.
    fn quantile_at(&self, p: f64) -> f64 {
        if p.is_nan() {
            return f64::NAN;
        }

        let n = self.x.len();
        let value = if p <= 0.0 {
            self.x[0]
        } else if p >= 1.0 {
            self.x[n - 1]
        } else {
            let r = (n - 1) as f64 * p;
            let j = r.floor() as usize;
            let g = r - j as f64;
            lerp(self.x[j], self.x[j + 1], g)
        };

        // Every branch hands back a value the sample supplied -- the clamped ones return an
        // order statistic verbatim, and lerp reproduces its endpoints exactly -- so a -0.0 among
        // the observations would otherwise reach the caller unnormalized.
        normalize_zero(value)
    }

    fn cumulative_at(&self, x: f64) -> f64 {
        if x.is_nan() {
            return f64::NAN;
        }

        let n = self.x.len();
        if x <= self.x[0] {
            return 0.0;
        }
        if x >= self.x[n - 1] {
            return 1.0;
        }

        let j = upper_bound(&self.x, x) - 1;
        let t = crate::dist_numerics::unit_ratio(x - self.x[j], self.x[j + 1] - self.x[j]);
        (j as f64 + t) / (n - 1) as f64
    }

    fn density_at(&self, x: f64) -> f64 {
        if x.is_nan() {
            return f64::NAN;
        }

        let n = self.x.len();
        if x < self.x[0] || x > self.x[n - 1] {
            return 0.0;
        }

        let j = if x == self.x[n - 1] {
            n - 2
        } else {
            upper_bound(&self.x, x) - 1
        };
        1.0 / (n - 1) as f64 / (self.x[j + 1] - self.x[j])
    }
}

impl MeasuredModel for LinearDistModel {
    fn quantile(&self, cumulances: &[f64]) -> Result<Vec<f64>, EstimatorError> {
        LinearDistModel::quantile(self, cumulances)
    }

    fn cumulative(&self, values: &[f64]) -> Result<Vec<f64>, EstimatorError> {
        LinearDistModel::cumulative(self, values)
    }

    fn density(&self, values: &[f64]) -> Result<Vec<f64>, EstimatorError> {
        LinearDistModel::density(self, values)
    }

    fn unit(&self) -> &MeasurementUnit {
        LinearDistModel::unit(self)
    }
}
