//! Sample: a collection of values with optional weights and a measurement unit.
//!
//! [`Sample`] is the primary input type for all estimators in the metrology layer.
//! It validates inputs at construction time (no empty, NaN, or infinite values)
//! and lazily computes sorted values on demand.

use crate::assumptions::{AssumptionError, EstimatorError, Subject};
use crate::measurement_unit::{
    conversion_factor, finer, is_compatible, MeasurementUnit, NumberUnit, UnitMismatchError,
};
use std::cell::OnceCell;
use std::ops::Mul;

/// A validated collection of numeric values with optional weights and a measurement unit.
///
/// Samples are validated at construction time: empty inputs, NaN, and infinite values
/// are rejected with an appropriate error. Sorted values are computed lazily on first access.
#[derive(Debug, Clone)]
pub struct Sample {
    values: Vec<f64>,
    weights: Option<Vec<f64>>,
    unit: Box<dyn MeasurementUnit>,
    is_weighted: bool,
    total_weight: f64,
    weighted_size: f64,
    subject: Subject,
    sorted_values: OnceCell<Vec<f64>>,
}

impl Sample {
    /// Creates an unweighted sample from numeric values with the default [`NumberUnit`].
    ///
    /// # Errors
    ///
    /// Returns [`EstimatorError`] if `values` is empty or contains NaN/infinite values.
    pub fn new(values: Vec<f64>) -> Result<Self, EstimatorError> {
        Self::with_unit(values, Box::new(NumberUnit))
    }

    /// Creates an unweighted sample with a specified unit.
    ///
    /// # Errors
    ///
    /// Returns [`EstimatorError`] if `values` is empty or contains NaN/infinite values.
    pub fn with_unit(
        values: Vec<f64>,
        unit: Box<dyn MeasurementUnit>,
    ) -> Result<Self, EstimatorError> {
        Self::build(values, None, unit, Subject::X)
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
        unit: Box<dyn MeasurementUnit>,
    ) -> Result<Self, EstimatorError> {
        Self::build(values, Some(weights), unit, Subject::X)
    }

    /// Internal constructor with configurable subject (used for two-sample estimators).
    pub(crate) fn with_subject(mut self, subject: Subject) -> Self {
        self.subject = subject;
        self
    }

    fn build(
        values: Vec<f64>,
        weights: Option<Vec<f64>>,
        unit: Box<dyn MeasurementUnit>,
        subject: Subject,
    ) -> Result<Self, EstimatorError> {
        if values.is_empty() {
            return Err(EstimatorError::from(AssumptionError::validity(subject)));
        }
        for &v in &values {
            if !v.is_finite() {
                return Err(EstimatorError::from(AssumptionError::validity(subject)));
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
                    total_w_sq += wi * wi;
                    if wi < min_w {
                        min_w = wi;
                    }
                }
                if min_w < 0.0 {
                    return Err(EstimatorError::Other("all weights must be non-negative".to_string()));
                }
                if total_w < 1e-9 {
                    return Err(EstimatorError::Other("total weight must be positive".to_string()));
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
            subject,
            sorted_values: OnceCell::new(),
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
    pub fn unit(&self) -> &dyn MeasurementUnit {
        self.unit.as_ref()
    }

    /// Returns the subject label (X or Y) for error reporting.
    pub(crate) fn subject(&self) -> Subject {
        self.subject
    }

    /// Returns a sorted copy of the values (lazily computed, cached).
    pub fn sorted_values(&self) -> &[f64] {
        self.sorted_values.get_or_init(|| {
            let mut sorted = self.values.clone();
            sorted.sort_by(|a, b| a.total_cmp(b));
            sorted
        })
    }

    /// Converts this sample to a different (compatible) unit.
    ///
    /// # Errors
    ///
    /// Returns an error if the target unit is in a different family.
    pub fn convert_to(&self, target: &dyn MeasurementUnit) -> Result<Self, UnitMismatchError> {
        if !is_compatible(self.unit.as_ref(), target) {
            return Err(UnitMismatchError::new(self.unit.as_ref(), target));
        }
        if self.unit.id() == target.id() && self.unit.base_units() == target.base_units() {
            return Ok(self.clone());
        }
        let factor = conversion_factor(self.unit.as_ref(), target);
        let converted: Vec<f64> = self.values.iter().map(|&v| v * factor).collect();
        Ok(Self {
            values: converted,
            weights: self.weights.clone(),
            unit: target.clone_box(),
            is_weighted: self.is_weighted,
            total_weight: self.total_weight,
            weighted_size: self.weighted_size,
            subject: self.subject,
            sorted_values: OnceCell::new(),
        })
    }

    /// Log-transforms the values. Returns a new sample with NumberUnit.
    ///
    /// # Errors
    ///
    /// Returns a positivity error if any value is non-positive.
    #[allow(dead_code)] // Part of the Sample API; used by ratio estimators in other languages
    pub(crate) fn log_transform(&self) -> Result<Self, AssumptionError> {
        let mut log_values = Vec::with_capacity(self.values.len());
        for &v in &self.values {
            if v <= 0.0 {
                return Err(AssumptionError::positivity(self.subject));
            }
            log_values.push(v.ln());
        }
        Ok(Self {
            values: log_values,
            weights: self.weights.clone(),
            unit: Box::new(NumberUnit),
            is_weighted: self.is_weighted,
            total_weight: self.total_weight,
            weighted_size: self.weighted_size,
            subject: self.subject,
            sorted_values: OnceCell::new(),
        })
    }
}

/// Multiplies all values in the sample by a scalar.
impl Mul<f64> for &Sample {
    type Output = Result<Sample, EstimatorError>;

    fn mul(self, rhs: f64) -> Self::Output {
        let scaled: Vec<f64> = self.values.iter().map(|&v| v * rhs).collect();
        Sample::build(
            scaled,
            self.weights.clone(),
            self.unit.clone(),
            self.subject,
        )
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

/// Prepares two samples for a two-sample estimator: sets subjects, checks
/// unit compatibility, and converts both to the finer unit.
pub(crate) fn prepare_pair(
    a: &Sample,
    b: &Sample,
) -> Result<(Sample, Sample), EstimatorError> {
    check_compatible_units(a, b)?;
    let (a, b) = convert_to_finer(a, b)?;
    Ok((a.with_subject(Subject::X), b.with_subject(Subject::Y)))
}

/// Converts both samples to the finer (more precise) unit.
/// Returns clones if they already share the same unit identity.
pub(crate) fn convert_to_finer(a: &Sample, b: &Sample) -> Result<(Sample, Sample), EstimatorError> {
    if a.unit().id() == b.unit().id() && a.unit().base_units() == b.unit().base_units() {
        return Ok((a.clone(), b.clone()));
    }
    let target = finer(a.unit(), b.unit());
    let new_a = a
        .convert_to(target)
        .map_err(|e| EstimatorError::Other(e.to_string()))?;
    let new_b = b
        .convert_to(target)
        .map_err(|e| EstimatorError::Other(e.to_string()))?;
    Ok((new_a, new_b))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::measurement_unit::CustomUnit;

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
        // Second call returns same cached result
        let sorted2 = s.sorted_values();
        assert_eq!(sorted, sorted2);
    }

    #[test]
    fn convert_to_compatible() {
        let ms = CustomUnit::new("ms", "Time", "ms", "Millisecond", 1_000_000);
        let ns = CustomUnit::new("ns", "Time", "ns", "Nanosecond", 1);
        let s = Sample::with_unit(vec![1.0, 2.0], Box::new(ms)).unwrap();
        let converted = s.convert_to(&ns).unwrap();
        assert!((converted.values()[0] - 1_000_000.0).abs() < 1e-6);
        assert!((converted.values()[1] - 2_000_000.0).abs() < 1e-6);
        assert_eq!(converted.unit().id(), "ns");
    }

    #[test]
    fn convert_to_incompatible_fails() {
        use crate::measurement_unit::RatioUnit;
        let s = Sample::new(vec![1.0, 2.0]).unwrap();
        assert!(s.convert_to(&RatioUnit).is_err());
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
            Box::new(NumberUnit),
        )
        .unwrap();
        assert!(s.is_weighted());
        assert!((s.total_weight() - 4.0).abs() < 1e-9);
    }

    #[test]
    fn weighted_negative_fails() {
        let result = Sample::weighted(vec![1.0, 2.0], vec![1.0, -1.0], Box::new(NumberUnit));
        assert!(result.is_err());
    }

    #[test]
    fn weighted_zero_total_fails() {
        let result = Sample::weighted(vec![1.0, 2.0], vec![0.0, 0.0], Box::new(NumberUnit));
        assert!(result.is_err());
    }

    #[test]
    fn weighted_length_mismatch_fails() {
        let result = Sample::weighted(vec![1.0, 2.0], vec![1.0], Box::new(NumberUnit));
        assert!(result.is_err());
    }

    #[test]
    fn log_transform_positive() {
        let s = Sample::new(vec![1.0, std::f64::consts::E]).unwrap();
        let log_s = s.log_transform().unwrap();
        assert!((log_s.values()[0] - 0.0).abs() < 1e-15);
        assert!((log_s.values()[1] - 1.0).abs() < 1e-15);
        assert_eq!(log_s.unit().id(), "number");
    }

    #[test]
    fn log_transform_non_positive_fails() {
        let s = Sample::new(vec![1.0, 0.0]).unwrap();
        assert!(s.log_transform().is_err());
    }
}
