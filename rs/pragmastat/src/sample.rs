//! Sample: a collection of values with optional weights and a measurement unit.
//!
//! [`Sample`] is the primary input type for all estimators in the metrology layer.
//! It validates inputs at construction time (no empty, NaN, or infinite values)
//! and lazily computes sorted values on demand.

use crate::assumptions::{AssumptionError, EstimatorError, Subject};
use crate::measurement_unit::{
    conversion_factor, finer, is_compatible, MeasurementUnit, UnitMismatchError,
};
use std::ops::Mul;
use std::sync::OnceLock;

/// A validated collection of numeric values with optional weights and a measurement unit.
///
/// Samples are validated at construction time: empty inputs, NaN, and infinite values
/// are rejected with an appropriate error. Sorted values are computed lazily on first access.
#[derive(Debug, Clone)]
pub struct Sample {
    values: Vec<f64>,
    weights: Option<Vec<f64>>,
    unit: MeasurementUnit,
    is_weighted: bool,
    total_weight: f64,
    weighted_size: f64,
    sorted_values: OnceLock<Vec<f64>>,
}

impl Sample {
    /// Creates an unweighted sample from numeric values with the default number unit.
    ///
    /// # Errors
    ///
    /// Returns [`EstimatorError`] if `values` is empty or contains NaN/infinite values.
    pub fn new(values: Vec<f64>) -> Result<Self, EstimatorError> {
        Self::with_unit(values, MeasurementUnit::number())
    }

    /// Creates an unweighted sample with a specified unit.
    ///
    /// # Errors
    ///
    /// Returns [`EstimatorError`] if `values` is empty or contains NaN/infinite values.
    pub fn with_unit(values: Vec<f64>, unit: MeasurementUnit) -> Result<Self, EstimatorError> {
        Self::build(values, None, unit)
    }

    /// Creates a weighted sample with a specified unit.
    ///
    /// # Errors
    ///
    /// Returns [`EstimatorError`] if:
    /// - `values` is empty or contains NaN/infinite values
    /// - `weights` length does not match `values` length
    /// - Any weight is negative
    /// - Total weight is near zero
    pub fn weighted(
        values: Vec<f64>,
        weights: Vec<f64>,
        unit: MeasurementUnit,
    ) -> Result<Self, EstimatorError> {
        Self::build(values, Some(weights), unit)
    }

    fn build(
        values: Vec<f64>,
        weights: Option<Vec<f64>>,
        unit: MeasurementUnit,
    ) -> Result<Self, EstimatorError> {
        // Construction can't know argument position, so validity errors here always
        // report subject "x" (matches the sample-construction fixtures across languages).
        if values.is_empty() {
            return Err(EstimatorError::from(AssumptionError::validity(Subject::X)));
        }
        for &v in &values {
            if !v.is_finite() {
                return Err(EstimatorError::from(AssumptionError::validity(Subject::X)));
            }
        }

        let (is_weighted, total_weight, weighted_size, stored_weights) = match weights {
            Some(w) => {
                if w.len() != values.len() {
                    return Err(EstimatorError::Other(
                        "weights length must match values length".to_string(),
                    ));
                }
                let mut total_w = 0.0_f64;
                let mut total_w_sq = 0.0_f64;
                let mut min_w = f64::MAX;
                for &wi in &w {
                    total_w += wi;
                    total_w_sq = wi.mul_add(wi, total_w_sq);
                    if wi < min_w {
                        min_w = wi;
                    }
                }
                if min_w < 0.0 {
                    return Err(EstimatorError::Other(
                        "all weights must be non-negative".to_string(),
                    ));
                }
                if total_w < 1e-9 {
                    return Err(EstimatorError::Other(
                        "total weight must be positive".to_string(),
                    ));
                }
                let ws = (total_w * total_w) / total_w_sq;
                (true, total_w, ws, Some(w))
            }
            None => (false, 1.0, values.len() as f64, None),
        };

        Ok(Self {
            values,
            weights: stored_weights,
            unit,
            is_weighted,
            total_weight,
            weighted_size,
            sorted_values: OnceLock::new(),
        })
    }

    /// Returns the number of values in this sample.
    pub fn size(&self) -> usize {
        self.values.len()
    }

    /// Returns a reference to the raw values.
    pub fn values(&self) -> &[f64] {
        &self.values
    }

    /// Returns true if this is a weighted sample.
    pub fn is_weighted(&self) -> bool {
        self.is_weighted
    }

    /// Returns the total weight (1.0 for unweighted samples).
    pub fn total_weight(&self) -> f64 {
        self.total_weight
    }

    /// Returns the effective sample size (Kish's formula for weighted, n for unweighted).
    pub fn weighted_size(&self) -> f64 {
        self.weighted_size
    }

    /// Returns a reference to the unit.
    pub fn unit(&self) -> &MeasurementUnit {
        &self.unit
    }

    /// Returns a sorted copy of the values (lazily computed, cached).
    pub fn sorted_values(&self) -> &[f64] {
        self.sorted_values.get_or_init(|| {
            let mut sorted = self.values.clone();
            sorted.sort_unstable_by(|a, b| a.total_cmp(b));
            sorted
        })
    }

    /// Converts this sample to a different (compatible) unit.
    ///
    /// # Errors
    ///
    /// Returns an error if the target unit is in a different family.
    pub fn convert_to(&self, target: &MeasurementUnit) -> Result<Self, UnitMismatchError> {
        if !is_compatible(&self.unit, target) {
            return Err(UnitMismatchError::new(&self.unit, target));
        }
        if self.unit.id() == target.id() && self.unit.base_units() == target.base_units() {
            return Ok(self.clone());
        }
        let factor = conversion_factor(&self.unit, target);
        let converted: Vec<f64> = self.values.iter().map(|&v| v * factor).collect();
        Ok(Self {
            values: converted,
            weights: self.weights.clone(),
            unit: target.clone(),
            is_weighted: self.is_weighted,
            total_weight: self.total_weight,
            weighted_size: self.weighted_size,
            sorted_values: OnceLock::new(),
        })
    }
}

/// Multiplies all values in the sample by a scalar.
impl Mul<f64> for &Sample {
    type Output = Result<Sample, EstimatorError>;

    fn mul(self, rhs: f64) -> Self::Output {
        let scaled: Vec<f64> = self.values.iter().map(|&v| v * rhs).collect();
        Sample::build(scaled, self.weights.clone(), self.unit.clone())
    }
}

// =============================================================================
// Helpers for two-sample estimators
// =============================================================================

/// Returns an error if the sample is weighted.
pub(crate) fn check_non_weighted(name: &str, s: &Sample) -> Result<(), EstimatorError> {
    if s.is_weighted() {
        return Err(EstimatorError::Other(format!(
            "weighted samples are not supported for {name}"
        )));
    }
    Ok(())
}

/// Returns an error if two samples have incompatible units.
pub(crate) fn check_compatible_units(a: &Sample, b: &Sample) -> Result<(), EstimatorError> {
    if !is_compatible(a.unit(), b.unit()) {
        return Err(EstimatorError::Other(format!(
            "can't convert {} to {}",
            a.unit().full_name(),
            b.unit().full_name()
        )));
    }
    Ok(())
}

/// Prepares two samples for a two-sample estimator: checks unit compatibility
/// and converts both to the finer unit.
///
/// When both samples already share the same unit, the originals are borrowed
/// (via [`Cow::Borrowed`]) so their warm sorted-value caches are reused — no
/// clone or re-sort. Only when a unit conversion is required are new samples
/// materialized. The error "subject" is supplied positionally by the estimator
/// (it is not stored on a sample), so no relabeling happens here.
pub(crate) fn prepare_pair<'a>(
    a: &'a Sample,
    b: &'a Sample,
) -> Result<(std::borrow::Cow<'a, Sample>, std::borrow::Cow<'a, Sample>), EstimatorError> {
    use std::borrow::Cow;
    check_compatible_units(a, b)?;
    if a.unit().id() == b.unit().id() && a.unit().base_units() == b.unit().base_units() {
        return Ok((Cow::Borrowed(a), Cow::Borrowed(b)));
    }
    let target = finer(a.unit(), b.unit());
    let new_a = a
        .convert_to(target)
        .map_err(|e| EstimatorError::Other(e.to_string()))?;
    let new_b = b
        .convert_to(target)
        .map_err(|e| EstimatorError::Other(e.to_string()))?;
    Ok((Cow::Owned(new_a), Cow::Owned(new_b)))
}

#[cfg(test)]
mod tests {
    use super::*;

    // Sample must stay shareable across threads (e.g. Arc<Sample>, rayon &Sample).
    // The lazily-cached sorted view uses OnceLock (Sync), not OnceCell (!Sync) — this
    // compile-time guard fails if that ever regresses.
    #[test]
    fn sample_is_send_and_sync() {
        fn assert_send_sync<T: Send + Sync>() {}
        assert_send_sync::<Sample>();
    }

    #[test]
    fn new_valid() {
        let s = Sample::new(vec![1.0, 2.0, 3.0]).unwrap();
        assert_eq!(s.size(), 3);
        assert!(!s.is_weighted());
        assert_eq!(s.unit().id(), "number");
    }

    #[test]
    fn new_empty_fails() {
        assert!(Sample::new(vec![]).is_err());
    }

    #[test]
    fn new_nan_fails() {
        assert!(Sample::new(vec![1.0, f64::NAN]).is_err());
    }

    #[test]
    fn new_inf_fails() {
        assert!(Sample::new(vec![1.0, f64::INFINITY]).is_err());
    }

    #[test]
    fn sorted_values_cached() {
        let s = Sample::new(vec![3.0, 1.0, 2.0]).unwrap();
        let sorted = s.sorted_values();
        assert_eq!(sorted, &[1.0, 2.0, 3.0]);
        let sorted2 = s.sorted_values();
        assert_eq!(sorted, sorted2);
        // The cache must be REUSED, not recomputed: both calls must hand back the
        // exact same backing allocation. Value-equality alone would pass even with
        // no caching, so assert pointer-identity of the slices.
        assert!(
            std::ptr::eq(sorted, sorted2),
            "sorted_values() returned a freshly computed slice; cache was not reused"
        );
        assert_eq!(sorted.as_ptr(), sorted2.as_ptr());
    }

    #[test]
    fn convert_to_compatible() {
        let ms = MeasurementUnit::new("ms", "Time", "ms", "Millisecond", 1_000_000);
        let ns = MeasurementUnit::new("ns", "Time", "ns", "Nanosecond", 1);
        let s = Sample::with_unit(vec![1.0, 2.0], ms).unwrap();
        let converted = s.convert_to(&ns).unwrap();
        assert!((converted.values()[0] - 1_000_000.0).abs() < 1e-6);
        assert!((converted.values()[1] - 2_000_000.0).abs() < 1e-6);
        assert_eq!(converted.unit().id(), "ns");
    }

    #[test]
    fn convert_to_incompatible_fails() {
        let s = Sample::new(vec![1.0, 2.0]).unwrap();
        assert!(s.convert_to(&MeasurementUnit::ratio()).is_err());
    }

    #[test]
    fn mul_scalar() {
        let s = Sample::new(vec![1.0, 2.0, 3.0]).unwrap();
        let result = (&s * 2.0).unwrap();
        assert_eq!(result.values(), &[2.0, 4.0, 6.0]);
    }

    #[test]
    fn weighted_sample() {
        let s = Sample::weighted(
            vec![1.0, 2.0, 3.0],
            vec![1.0, 2.0, 1.0],
            MeasurementUnit::number(),
        )
        .unwrap();
        assert!(s.is_weighted());
        assert!((s.total_weight() - 4.0).abs() < 1e-9);
    }

    #[test]
    fn weighted_negative_fails() {
        let result = Sample::weighted(vec![1.0, 2.0], vec![1.0, -1.0], MeasurementUnit::number());
        assert!(result.is_err());
    }

    #[test]
    fn weighted_zero_total_fails() {
        let result = Sample::weighted(vec![1.0, 2.0], vec![0.0, 0.0], MeasurementUnit::number());
        assert!(result.is_err());
    }

    #[test]
    fn weighted_length_mismatch_fails() {
        let result = Sample::weighted(vec![1.0, 2.0], vec![1.0], MeasurementUnit::number());
        assert!(result.is_err());
    }
}
