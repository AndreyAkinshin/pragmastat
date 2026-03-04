//! Compare1 and Compare2: confirmatory analysis for one-sample and two-sample estimators.
//!
//! These high-level APIs compare estimates (Center, Spread, Shift, Ratio, Disparity)
//! against practical thresholds and return verdicts (Less, Greater, or Inconclusive).

use crate::assumptions::{AssumptionError, EstimatorError, Subject};
use crate::bounds::Bounds;
use crate::estimators::raw;
use crate::measurement::Measurement;
use crate::measurement_unit::{
    conversion_factor, finer, is_compatible, MeasurementUnit, UnitMismatchError,
};
use crate::sample::{check_compatible_units, check_non_weighted, Sample};

/// Metric types supported by Compare1 and Compare2.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Metric {
    /// Central value of a sample.
    Center,
    /// Data dispersion (spread).
    Spread,
    /// Typical difference between two samples.
    Shift,
    /// Multiplicative factor between two samples.
    Ratio,
    /// Normalized difference (effect size).
    Disparity,
}

impl Metric {
    /// Returns the string identifier for this metric.
    pub fn as_str(&self) -> &'static str {
        match self {
            Metric::Center => "center",
            Metric::Spread => "spread",
            Metric::Shift => "shift",
            Metric::Ratio => "ratio",
            Metric::Disparity => "disparity",
        }
    }
}

/// Verdict from comparing an estimate against a threshold.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComparisonVerdict {
    /// The estimate is statistically less than the threshold.
    Less,
    /// The estimate is statistically greater than the threshold.
    Greater,
    /// Not enough evidence to conclude (interval contains threshold).
    Inconclusive,
}

impl ComparisonVerdict {
    /// Returns the string identifier for this verdict.
    pub fn as_str(&self) -> &'static str {
        match self {
            ComparisonVerdict::Less => "less",
            ComparisonVerdict::Greater => "greater",
            ComparisonVerdict::Inconclusive => "inconclusive",
        }
    }
}

/// A threshold value with a metric type and misrate for comparison.
#[derive(Debug, Clone)]
pub struct Threshold {
    metric: Metric,
    value: Measurement,
    misrate: f64,
}

impl Threshold {
    /// Creates a new threshold.
    ///
    /// # Arguments
    ///
    /// * `metric` - The metric type (Center, Spread, Shift, Ratio, or Disparity)
    /// * `value` - The threshold value as a Measurement
    /// * `misrate` - The per-threshold misclassification rate
    ///
    /// # Errors
    ///
    /// Returns an error if misrate is not in (0, 1] or value is not finite.
    pub fn new(metric: Metric, value: Measurement, misrate: f64) -> Result<Self, EstimatorError> {
        if !misrate.is_finite() || misrate <= 0.0 || misrate > 1.0 {
            return Err(EstimatorError::from(AssumptionError::domain(
                Subject::Misrate,
            )));
        }
        if !value.value.is_finite() {
            return Err(EstimatorError::Other(
                "threshold value must be finite".to_string(),
            ));
        }
        Ok(Self {
            metric,
            value,
            misrate,
        })
    }

    /// Returns the metric type.
    pub fn metric(&self) -> Metric {
        self.metric
    }

    /// Returns the threshold value.
    pub fn value(&self) -> &Measurement {
        &self.value
    }

    /// Returns the misrate.
    pub fn misrate(&self) -> f64 {
        self.misrate
    }
}

/// A projection containing estimate, bounds, and verdict for a single threshold.
#[derive(Debug, Clone)]
pub struct Projection {
    threshold: Threshold,
    estimate: Measurement,
    bounds: Bounds,
    verdict: ComparisonVerdict,
}

impl Projection {
    /// Creates a new projection.
    pub fn new(
        threshold: Threshold,
        estimate: Measurement,
        bounds: Bounds,
        verdict: ComparisonVerdict,
    ) -> Self {
        Self {
            threshold,
            estimate,
            bounds,
            verdict,
        }
    }

    /// Returns the threshold that was evaluated.
    pub fn threshold(&self) -> &Threshold {
        &self.threshold
    }

    /// Returns the point estimate.
    pub fn estimate(&self) -> &Measurement {
        &self.estimate
    }

    /// Returns the confidence bounds.
    pub fn bounds(&self) -> &Bounds {
        &self.bounds
    }

    /// Returns the comparison verdict.
    pub fn verdict(&self) -> ComparisonVerdict {
        self.verdict
    }
}

// =============================================================================
// Internal implementation
// =============================================================================

/// Computes the verdict by comparing bounds against a threshold value.
fn compute_verdict(bounds: &Bounds, threshold_value: f64) -> ComparisonVerdict {
    if bounds.lower > threshold_value {
        ComparisonVerdict::Greater
    } else if bounds.upper < threshold_value {
        ComparisonVerdict::Less
    } else {
        ComparisonVerdict::Inconclusive
    }
}

/// Function type for seeded bounds computation.
type SeededBoundsFn = fn(&Sample, Option<&Sample>, f64, &str) -> Result<Bounds, EstimatorError>;

/// Specification for a metric's validation, estimation, and bounds computation.
struct MetricSpec {
    metric: Metric,
    validate_and_normalize:
        fn(&Threshold, &Sample, Option<&Sample>) -> Result<Measurement, EstimatorError>,
    estimate: fn(&Sample, Option<&Sample>) -> Result<Measurement, EstimatorError>,
    bounds: fn(&Sample, Option<&Sample>, f64) -> Result<Bounds, EstimatorError>,
    seeded_bounds: Option<SeededBoundsFn>,
}

/// Compare1 metric specifications.
const COMPARE1_SPECS: &[MetricSpec] = &[
    MetricSpec {
        metric: Metric::Center,
        validate_and_normalize: validate_center,
        estimate: |x, _| {
            let val = raw::center(x.values())?;
            Ok(Measurement::new(val, x.unit().clone()))
        },
        bounds: |x, _, misrate| {
            let rb = raw::center_bounds(x.values(), misrate)?;
            Ok(Bounds::new(rb.lower, rb.upper, x.unit().clone()))
        },
        seeded_bounds: None,
    },
    MetricSpec {
        metric: Metric::Spread,
        validate_and_normalize: validate_spread,
        estimate: |x, _| {
            let val = raw::spread(x.values())?;
            Ok(Measurement::new(val, x.unit().clone()))
        },
        bounds: |x, _, misrate| {
            let rb = raw::spread_bounds(x.values(), misrate)?;
            Ok(Bounds::new(rb.lower, rb.upper, x.unit().clone()))
        },
        seeded_bounds: Some(|x, _, misrate, seed| {
            let rb = raw::spread_bounds_with_seed(x.values(), misrate, seed)?;
            Ok(Bounds::new(rb.lower, rb.upper, x.unit().clone()))
        }),
    },
];

/// Compare2 metric specifications.
const COMPARE2_SPECS: &[MetricSpec] = &[
    MetricSpec {
        metric: Metric::Shift,
        validate_and_normalize: validate_shift,
        estimate: |x, y| {
            let y = y.expect("Shift requires y sample");
            let val = raw::shift(x.values(), y.values())?;
            // Unit is already the finer one from prepare_pair
            Ok(Measurement::new(val, x.unit().clone()))
        },
        bounds: |x, y, misrate| {
            let y = y.expect("Shift requires y sample");
            let rb = raw::shift_bounds(x.values(), y.values(), misrate)?;
            Ok(Bounds::new(rb.lower, rb.upper, x.unit().clone()))
        },
        seeded_bounds: None,
    },
    MetricSpec {
        metric: Metric::Ratio,
        validate_and_normalize: validate_ratio,
        estimate: |x, y| {
            let y = y.expect("Ratio requires y sample");
            let val = raw::ratio(x.values(), y.values())?;
            Ok(Measurement::new(val, MeasurementUnit::ratio()))
        },
        bounds: |x, y, misrate| {
            let y = y.expect("Ratio requires y sample");
            let rb = raw::ratio_bounds(x.values(), y.values(), misrate)?;
            Ok(Bounds::new(rb.lower, rb.upper, MeasurementUnit::ratio()))
        },
        seeded_bounds: None,
    },
    MetricSpec {
        metric: Metric::Disparity,
        validate_and_normalize: validate_disparity,
        estimate: |x, y| {
            let y = y.expect("Disparity requires y sample");
            let val = raw::disparity(x.values(), y.values())?;
            Ok(Measurement::new(val, MeasurementUnit::disparity()))
        },
        bounds: |x, y, misrate| {
            let y = y.expect("Disparity requires y sample");
            let rb = raw::disparity_bounds(x.values(), y.values(), misrate)?;
            Ok(Bounds::new(
                rb.lower,
                rb.upper,
                MeasurementUnit::disparity(),
            ))
        },
        seeded_bounds: Some(|x, y, misrate, seed| {
            let y = y.expect("Disparity requires y sample");
            let rb = raw::disparity_bounds_with_seed(x.values(), y.values(), misrate, seed)?;
            Ok(Bounds::new(
                rb.lower,
                rb.upper,
                MeasurementUnit::disparity(),
            ))
        }),
    },
];

/// Validates and normalizes a Center threshold.
fn validate_center(
    threshold: &Threshold,
    x: &Sample,
    _: Option<&Sample>,
) -> Result<Measurement, EstimatorError> {
    if !is_compatible(&threshold.value.unit, x.unit()) {
        return Err(UnitMismatchError::new(&threshold.value.unit, x.unit()).into());
    }
    let factor = conversion_factor(&threshold.value.unit, x.unit());
    Ok(Measurement::new(
        threshold.value.value * factor,
        x.unit().clone(),
    ))
}

/// Validates and normalizes a Spread threshold.
fn validate_spread(
    threshold: &Threshold,
    x: &Sample,
    _: Option<&Sample>,
) -> Result<Measurement, EstimatorError> {
    validate_center(threshold, x, None) // Same validation as Center
}

/// Validates and normalizes a Shift threshold.
fn validate_shift(
    threshold: &Threshold,
    x: &Sample,
    y: Option<&Sample>,
) -> Result<Measurement, EstimatorError> {
    let y = y.expect("Shift requires y sample");
    if !is_compatible(&threshold.value.unit, x.unit()) {
        return Err(UnitMismatchError::new(&threshold.value.unit, x.unit()).into());
    }
    let target = finer(x.unit(), y.unit());
    let factor = conversion_factor(&threshold.value.unit, target);
    Ok(Measurement::new(
        threshold.value.value * factor,
        target.clone(),
    ))
}

/// Validates and normalizes a Ratio threshold.
fn validate_ratio(
    threshold: &Threshold,
    _: &Sample,
    _: Option<&Sample>,
) -> Result<Measurement, EstimatorError> {
    let unit = &threshold.value.unit;
    if unit.id() != "ratio" && unit.id() != "number" {
        return Err(UnitMismatchError::new(unit, &MeasurementUnit::ratio()).into());
    }
    let value = threshold.value.value;
    if value <= 0.0 {
        return Err(EstimatorError::Other(
            "Ratio threshold value must be positive".to_string(),
        ));
    }
    Ok(Measurement::new(value, MeasurementUnit::ratio()))
}

/// Validates and normalizes a Disparity threshold.
fn validate_disparity(
    threshold: &Threshold,
    _: &Sample,
    _: Option<&Sample>,
) -> Result<Measurement, EstimatorError> {
    let unit = &threshold.value.unit;
    if unit.id() != "disparity" && unit.id() != "number" {
        return Err(UnitMismatchError::new(unit, &MeasurementUnit::disparity()).into());
    }
    let value = threshold.value.value;
    if !value.is_finite() {
        return Err(EstimatorError::Other(
            "Disparity threshold value must be finite".to_string(),
        ));
    }
    Ok(Measurement::new(value, MeasurementUnit::disparity()))
}

fn get_spec(specs: &[MetricSpec], metric: Metric) -> Option<&MetricSpec> {
    specs.iter().find(|s| s.metric == metric)
}

// =============================================================================
// Public API
// =============================================================================

/// One-sample confirmatory analysis: compares Center/Spread against practical thresholds.
///
/// # Arguments
///
/// * `x` - The sample to analyze
/// * `thresholds` - List of thresholds to compare against
///
/// # Returns
///
/// A vector of [`Projection`]s in the same order as the input thresholds.
///
/// # Errors
///
/// Returns an error if:
/// - The sample is weighted
/// - The threshold list is empty
/// - Any threshold has an unsupported metric (Shift, Ratio, Disparity)
/// - Any threshold value is incompatible with the sample unit
pub fn compare1(x: &Sample, thresholds: &[Threshold]) -> Result<Vec<Projection>, EstimatorError> {
    compare1_impl(x, thresholds, None)
}

/// One-sample confirmatory analysis with seed for reproducibility.
///
/// The seed is used for randomized bounds (Spread bounds only).
pub fn compare1_with_seed(
    x: &Sample,
    thresholds: &[Threshold],
    seed: &str,
) -> Result<Vec<Projection>, EstimatorError> {
    compare1_impl(x, thresholds, Some(seed))
}

/// Two-sample confirmatory analysis: compares Shift/Ratio/Disparity against practical thresholds.
///
/// # Arguments
///
/// * `x` - The first sample
/// * `y` - The second sample
/// * `thresholds` - List of thresholds to compare against
///
/// # Returns
///
/// A vector of [`Projection`]s in the same order as the input thresholds.
///
/// # Errors
///
/// Returns an error if:
/// - Either sample is weighted
/// - The samples have incompatible units
/// - The threshold list is empty
/// - Any threshold has an unsupported metric (Center, Spread)
/// - Any threshold value is incompatible
pub fn compare2(
    x: &Sample,
    y: &Sample,
    thresholds: &[Threshold],
) -> Result<Vec<Projection>, EstimatorError> {
    compare2_impl(x, y, thresholds, None)
}

/// Two-sample confirmatory analysis with seed for reproducibility.
///
/// The seed is used for randomized bounds (Disparity bounds only).
pub fn compare2_with_seed(
    x: &Sample,
    y: &Sample,
    thresholds: &[Threshold],
    seed: &str,
) -> Result<Vec<Projection>, EstimatorError> {
    compare2_impl(x, y, thresholds, Some(seed))
}

// =============================================================================
// Inner implementations
// =============================================================================

fn compare1_impl(
    x: &Sample,
    thresholds: &[Threshold],
    seed: Option<&str>,
) -> Result<Vec<Projection>, EstimatorError> {
    check_non_weighted("x", x)?;

    if thresholds.is_empty() {
        return Err(EstimatorError::Other(
            "thresholds list cannot be empty".to_string(),
        ));
    }

    for threshold in thresholds {
        if !matches!(threshold.metric, Metric::Center | Metric::Spread) {
            return Err(EstimatorError::Other(format!(
                "Metric {} is not supported by Compare1. Use Compare2 instead.",
                threshold.metric.as_str()
            )));
        }
    }

    let mut normalized_values = Vec::with_capacity(thresholds.len());
    for threshold in thresholds {
        let spec = get_spec(COMPARE1_SPECS, threshold.metric).expect("spec exists");
        let normalized = (spec.validate_and_normalize)(threshold, x, None)?;
        normalized_values.push(normalized);
    }

    let mut results: Vec<Option<Projection>> = vec![None; thresholds.len()];

    for spec in COMPARE1_SPECS {
        let entries: Vec<(usize, &Threshold, &Measurement)> = thresholds
            .iter()
            .zip(normalized_values.iter())
            .enumerate()
            .filter(|(_, (t, _))| t.metric == spec.metric)
            .map(|(i, (t, n))| (i, t, n))
            .collect();

        if entries.is_empty() {
            continue;
        }

        let estimate = (spec.estimate)(x, None)?;

        for (input_idx, threshold, normalized_value) in entries {
            let bounds = match (seed, spec.seeded_bounds) {
                (Some(s), Some(seeded_fn)) => seeded_fn(x, None, threshold.misrate, s)?,
                _ => (spec.bounds)(x, None, threshold.misrate)?,
            };
            let verdict = compute_verdict(&bounds, normalized_value.value);
            results[input_idx] = Some(Projection::new(
                threshold.clone(),
                estimate.clone(),
                bounds,
                verdict,
            ));
        }
    }

    Ok(results.into_iter().flatten().collect())
}

fn compare2_impl(
    x: &Sample,
    y: &Sample,
    thresholds: &[Threshold],
    seed: Option<&str>,
) -> Result<Vec<Projection>, EstimatorError> {
    check_non_weighted("x", x)?;
    check_non_weighted("y", y)?;
    check_compatible_units(x, y)?;

    if thresholds.is_empty() {
        return Err(EstimatorError::Other(
            "thresholds list cannot be empty".to_string(),
        ));
    }

    for threshold in thresholds {
        if !matches!(
            threshold.metric,
            Metric::Shift | Metric::Ratio | Metric::Disparity
        ) {
            return Err(EstimatorError::Other(format!(
                "Metric {} is not supported by Compare2. Use Compare1 instead.",
                threshold.metric.as_str()
            )));
        }
    }

    // Convert both samples to the finer unit before any estimation
    let target = finer(x.unit(), y.unit());
    let x_conv = x
        .convert_to(target)
        .map_err(|e| EstimatorError::Other(e.to_string()))?;
    let y_conv = y
        .convert_to(target)
        .map_err(|e| EstimatorError::Other(e.to_string()))?;

    let mut normalized_values = Vec::with_capacity(thresholds.len());
    for threshold in thresholds {
        let spec = get_spec(COMPARE2_SPECS, threshold.metric).expect("spec exists");
        let normalized = (spec.validate_and_normalize)(threshold, &x_conv, Some(&y_conv))?;
        normalized_values.push(normalized);
    }

    let mut results: Vec<Option<Projection>> = vec![None; thresholds.len()];

    for spec in COMPARE2_SPECS {
        let entries: Vec<(usize, &Threshold, &Measurement)> = thresholds
            .iter()
            .zip(normalized_values.iter())
            .enumerate()
            .filter(|(_, (t, _))| t.metric == spec.metric)
            .map(|(i, (t, n))| (i, t, n))
            .collect();

        if entries.is_empty() {
            continue;
        }

        let estimate = (spec.estimate)(&x_conv, Some(&y_conv))?;

        for (input_idx, threshold, normalized_value) in entries {
            let bounds = match (seed, spec.seeded_bounds) {
                (Some(s), Some(seeded_fn)) => {
                    seeded_fn(&x_conv, Some(&y_conv), threshold.misrate, s)?
                }
                _ => (spec.bounds)(&x_conv, Some(&y_conv), threshold.misrate)?,
            };
            let verdict = compute_verdict(&bounds, normalized_value.value);
            results[input_idx] = Some(Projection::new(
                threshold.clone(),
                estimate.clone(),
                bounds,
                verdict,
            ));
        }
    }

    Ok(results.into_iter().flatten().collect())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sample::Sample;

    #[test]
    fn test_compare1_center() {
        let x = Sample::new(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0]).unwrap();
        let threshold = Threshold::new(Metric::Center, Measurement::number(5.5), 0.05).unwrap();

        let projections = compare1(&x, &[threshold]).unwrap();
        assert_eq!(projections.len(), 1);
        assert!(matches!(
            projections[0].verdict,
            ComparisonVerdict::Inconclusive
        ));
    }

    #[test]
    fn test_compare1_spread() {
        let x = Sample::new(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0]).unwrap();
        let threshold = Threshold::new(Metric::Spread, Measurement::number(0.1), 0.2).unwrap();

        let projections = compare1_with_seed(&x, &[threshold], "test-seed").unwrap();
        assert_eq!(projections.len(), 1);
        assert!(matches!(projections[0].verdict, ComparisonVerdict::Greater));
    }

    #[test]
    fn test_compare1_wrong_metric() {
        let x = Sample::new(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0]).unwrap();
        let threshold = Threshold::new(Metric::Shift, Measurement::number(0.0), 0.1).unwrap();

        assert!(compare1(&x, &[threshold]).is_err());
    }

    #[test]
    fn test_compare1_empty_thresholds() {
        let x = Sample::new(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0]).unwrap();
        assert!(compare1(&x, &[]).is_err());
    }

    #[test]
    fn test_compare2_shift() {
        let x = Sample::new((1..=30).map(|v| v as f64).collect()).unwrap();
        let y = Sample::new((21..=50).map(|v| v as f64).collect()).unwrap();
        let threshold = Threshold::new(Metric::Shift, Measurement::number(0.0), 0.02).unwrap();

        let projections = compare2(&x, &y, &[threshold]).unwrap();
        assert_eq!(projections.len(), 1);
        assert!(matches!(projections[0].verdict, ComparisonVerdict::Less));
    }

    #[test]
    fn test_compare2_wrong_metric() {
        let x = Sample::new(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0]).unwrap();
        let y = Sample::new(vec![6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0]).unwrap();
        let threshold = Threshold::new(Metric::Center, Measurement::number(5.0), 0.1).unwrap();

        assert!(compare2(&x, &y, &[threshold]).is_err());
    }

    #[test]
    fn test_compare2_empty_thresholds() {
        let x = Sample::new(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0]).unwrap();
        let y = Sample::new(vec![6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0]).unwrap();
        assert!(compare2(&x, &y, &[]).is_err());
    }

    #[test]
    fn test_threshold_non_finite_value() {
        assert!(Threshold::new(
            Metric::Center,
            Measurement::new(f64::INFINITY, MeasurementUnit::number()),
            0.1,
        )
        .is_err());
    }

    #[test]
    fn test_threshold_invalid_misrate() {
        assert!(Threshold::new(Metric::Center, Measurement::number(5.0), -0.1).is_err());
        assert!(Threshold::new(Metric::Center, Measurement::number(5.0), 0.0).is_err());
        assert!(Threshold::new(Metric::Center, Measurement::number(5.0), 1.5).is_err());
        // boundary: exactly 1.0 is valid
        assert!(Threshold::new(Metric::Center, Measurement::number(5.0), 1.0).is_ok());
    }
}
