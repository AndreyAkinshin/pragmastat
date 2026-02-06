//! Statistical estimators for one-sample and two-sample analysis

use crate::assumptions::{
    check_positivity, check_sparity, check_validity, log, AssumptionError, EstimatorError, Subject,
};

/// Calculates the median of a sorted slice
fn median_sorted(sorted: &[f64]) -> Result<f64, &'static str> {
    let n = sorted.len();
    if n == 0 {
        return Err("Input slice cannot be empty");
    }
    if n.is_multiple_of(2) {
        Ok((sorted[n / 2 - 1] + sorted[n / 2]) / 2.0)
    } else {
        Ok(sorted[n / 2])
    }
}

/// Calculates the median of a slice
pub fn median(values: &[f64]) -> Result<f64, AssumptionError> {
    // Check validity (priority 0)
    check_validity(values, Subject::X)?;
    let mut sorted = values.to_vec();
    sorted.sort_by(|a, b| a.total_cmp(b));
    median_sorted(&sorted).map_err(|_| AssumptionError::validity("Median", Subject::X))
}

/// Estimates the central value of the data (Center)
///
/// Calculates the median of all pairwise averages (x[i] + x[j])/2.
/// More robust than the mean and more efficient than the median.
/// Uses fast O(n log n) algorithm.
pub fn center(x: &[f64]) -> Result<f64, AssumptionError> {
    // Check validity (priority 0)
    check_validity(x, Subject::X)?;
    crate::fast_center::fast_center(x).map_err(|_| AssumptionError::validity("Center", Subject::X))
}

/// Estimates data dispersion (Spread)
///
/// Calculates the median of all pairwise absolute differences |x[i] - x[j]|.
/// More robust than standard deviation and more efficient than MAD.
/// Uses fast O(n log n) algorithm.
///
/// # Assumptions
///
/// - `sparity(x)` - sample must be non tie-dominant (Spread > 0)
///
/// # Errors
///
/// Returns `AssumptionError` if:
/// - Input is empty, contains NaN, or contains infinite values (validity)
/// - Sample is tie-dominant or has fewer than two elements (sparity)
pub fn spread(x: &[f64]) -> Result<f64, AssumptionError> {
    // Check validity first (priority 0)
    check_validity(x, Subject::X)?;

    // Check sparity (priority 2)
    check_sparity(x, Subject::X)?;

    // Use the internal fast implementation
    // We know at this point the input is valid, so unwrap is safe for internal errors
    crate::fast_spread::fast_spread(x).map_err(|_| AssumptionError::validity("Spread", Subject::X))
}

/// Measures the relative dispersion of a sample (RelSpread)
///
/// Calculates the ratio of Spread to absolute Center.
/// Robust alternative to the coefficient of variation.
///
/// # Assumptions
///
/// - `positivity(x)` - all values must be strictly positive (ensures Center > 0)
///
/// # Errors
///
/// Returns `AssumptionError` if:
/// - Input is empty, contains NaN, or contains infinite values (validity)
/// - Any value is zero or negative (positivity)
pub fn rel_spread(x: &[f64]) -> Result<f64, AssumptionError> {
    // Check validity (priority 0)
    check_validity(x, Subject::X)?;

    // Check positivity (priority 1)
    check_positivity(x, Subject::X)?;

    // Calculate center (we know x is valid, center should succeed)
    let center_val = crate::fast_center::fast_center(x)
        .map_err(|_| AssumptionError::validity("RelSpread", Subject::X))?;

    // Calculate spread (we know x is valid)
    // Note: spread now requires sparity, but for RelSpread we require positivity
    // which is checked above. We use the internal implementation directly.
    let spread_val = crate::fast_spread::fast_spread(x)
        .map_err(|_| AssumptionError::validity("RelSpread", Subject::X))?;

    // center_val is guaranteed positive because all values are positive
    Ok(spread_val / center_val.abs())
}

/// Measures the typical difference between elements of x and y (Shift)
///
/// Calculates the median of all pairwise differences (x[i] - y[j]).
/// Positive values mean x is typically larger, negative means y is typically larger.
/// Uses fast O((m+n) log precision) algorithm.
pub fn shift(x: &[f64], y: &[f64]) -> Result<f64, AssumptionError> {
    // Check validity (priority 0)
    check_validity(x, Subject::X)?;
    check_validity(y, Subject::Y)?;
    crate::fast_shift::fast_shift(x, y).map_err(|_| AssumptionError::validity("Shift", Subject::X))
}

/// Measures how many times larger x is compared to y (Ratio)
///
/// Calculates the median of all pairwise ratios (x[i] / y[j]) via log-transformation.
/// Equivalent to: exp(Shift(log(x), log(y)))
/// For example, ratio = 1.2 means x is typically 20% larger than y.
/// Uses fast O((m+n) log precision) algorithm.
///
/// # Assumptions
///
/// - `positivity(x)` - all values in x must be strictly positive
/// - `positivity(y)` - all values in y must be strictly positive
///
/// # Errors
///
/// Returns `AssumptionError` if:
/// - Either input is empty, contains NaN, or contains infinite values (validity)
/// - Either sample contains zero or negative values (positivity)
pub fn ratio(x: &[f64], y: &[f64]) -> Result<f64, AssumptionError> {
    // Check validity for x (priority 0, subject x)
    check_validity(x, Subject::X)?;

    // Check validity for y (priority 0, subject y)
    check_validity(y, Subject::Y)?;

    // Check positivity for x (priority 1, subject x)
    check_positivity(x, Subject::X)?;

    // Check positivity for y (priority 1, subject y)
    check_positivity(y, Subject::Y)?;

    crate::fast_shift::fast_ratio(x, y).map_err(|_| AssumptionError::validity("Ratio", Subject::X))
}

/// Measures the typical variability when considering both samples together (AvgSpread)
///
/// Computes the weighted average of individual spreads: (n*Spread(x) + m*Spread(y))/(n+m).
///
/// # Assumptions
///
/// - `sparity(x)` - first sample must be non tie-dominant (Spread > 0)
/// - `sparity(y)` - second sample must be non tie-dominant (Spread > 0)
///
/// # Errors
///
/// Returns `AssumptionError` if:
/// - Either input is empty, contains NaN, or contains infinite values (validity)
/// - Either sample is tie-dominant or has fewer than two elements (sparity)
pub fn avg_spread(x: &[f64], y: &[f64]) -> Result<f64, AssumptionError> {
    // Check validity for x (priority 0, subject x)
    check_validity(x, Subject::X)?;

    // Check validity for y (priority 0, subject y)
    check_validity(y, Subject::Y)?;

    // Check sparity for x (priority 2, subject x)
    check_sparity(x, Subject::X)?;

    // Check sparity for y (priority 2, subject y)
    check_sparity(y, Subject::Y)?;

    let n = x.len();
    let m = y.len();

    // Calculate spreads (we know inputs are valid and non-degenerate)
    let spread_x = crate::fast_spread::fast_spread(x)
        .map_err(|_| AssumptionError::validity("AvgSpread", Subject::X))?;
    let spread_y = crate::fast_spread::fast_spread(y)
        .map_err(|_| AssumptionError::validity("AvgSpread", Subject::Y))?;

    Ok((n as f64 * spread_x + m as f64 * spread_y) / (n + m) as f64)
}

/// Measures effect size: a normalized difference between x and y (Disparity)
///
/// Calculated as Shift / AvgSpread. Robust alternative to Cohen's d.
///
/// # Assumptions
///
/// - `sparity(x)` - first sample must be non tie-dominant (Spread > 0)
/// - `sparity(y)` - second sample must be non tie-dominant (Spread > 0)
///
/// # Errors
///
/// Returns `AssumptionError` if:
/// - Either input is empty, contains NaN, or contains infinite values (validity)
/// - Either sample is tie-dominant or has fewer than two elements (sparity)
pub fn disparity(x: &[f64], y: &[f64]) -> Result<f64, AssumptionError> {
    // Check validity for x (priority 0, subject x)
    check_validity(x, Subject::X)?;

    // Check validity for y (priority 0, subject y)
    check_validity(y, Subject::Y)?;

    // Check sparity for x (priority 2, subject x)
    check_sparity(x, Subject::X)?;

    // Check sparity for y (priority 2, subject y)
    check_sparity(y, Subject::Y)?;

    // Calculate shift (we know inputs are valid)
    let shift_val = crate::fast_shift::fast_shift(x, y)
        .map_err(|_| AssumptionError::validity("Disparity", Subject::X))?;

    // Calculate avg_spread (we know inputs are valid and non-degenerate)
    let n = x.len();
    let m = y.len();
    let spread_x = crate::fast_spread::fast_spread(x)
        .map_err(|_| AssumptionError::validity("Disparity", Subject::X))?;
    let spread_y = crate::fast_spread::fast_spread(y)
        .map_err(|_| AssumptionError::validity("Disparity", Subject::Y))?;
    let avg_spread_val = (n as f64 * spread_x + m as f64 * spread_y) / (n + m) as f64;

    Ok(shift_val / avg_spread_val)
}

/// Represents an interval with lower and upper bounds
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Bounds {
    pub lower: f64,
    pub upper: f64,
}

/// Provides bounds on the Shift estimator with specified misclassification rate (ShiftBounds)
///
/// The misrate represents the probability that the true shift falls outside the computed bounds.
/// This is a pragmatic alternative to traditional confidence intervals for the Hodges-Lehmann estimator.
///
/// # Arguments
///
/// * `x` - First sample slice
/// * `y` - Second sample slice
/// * `misrate` - Misclassification rate (probability that true shift falls outside bounds)
///
/// # Returns
///
/// A `Bounds` struct containing the lower and upper bounds, or an error if inputs are invalid.
pub fn shift_bounds(x: &[f64], y: &[f64], misrate: f64) -> Result<Bounds, EstimatorError> {
    // Check validity for x
    check_validity(x, Subject::X)?;

    // Check validity for y
    check_validity(y, Subject::Y)?;

    let n = x.len();
    let m = y.len();

    // Sort both arrays
    let mut xs = x.to_vec();
    let mut ys = y.to_vec();
    xs.sort_by(|a, b| a.total_cmp(b));
    ys.sort_by(|a, b| a.total_cmp(b));

    let total = n * m;

    // Special case: when there's only one pairwise difference, bounds collapse to a single value
    if total == 1 {
        let value = xs[0] - ys[0];
        return Ok(Bounds {
            lower: value,
            upper: value,
        });
    }

    let margin =
        crate::pairwise_margin::pairwise_margin(n, m, misrate).map_err(EstimatorError::from)?;
    let max_half_margin = (total - 1) / 2;
    let mut half_margin = margin / 2;
    if half_margin > max_half_margin {
        half_margin = max_half_margin;
    }
    let k_left = half_margin;
    let k_right = total - 1 - half_margin;

    // Compute quantile positions
    let denominator = (total - 1) as f64;
    let p = vec![k_left as f64 / denominator, k_right as f64 / denominator];

    let bounds = crate::fast_shift::fast_shift_quantiles(&xs, &ys, &p, true)
        .map_err(EstimatorError::from)?;

    let lower = bounds[0].min(bounds[1]);
    let upper = bounds[0].max(bounds[1]);

    Ok(Bounds { lower, upper })
}

/// Provides bounds on the Ratio estimator with specified misclassification rate (RatioBounds)
///
/// Computes bounds via log-transformation and shift_bounds delegation:
/// `ratio_bounds(x, y, misrate) = exp(shift_bounds(log(x), log(y), misrate))`
///
/// # Arguments
///
/// * `x` - First sample slice (must be positive)
/// * `y` - Second sample slice (must be positive)
/// * `misrate` - Misclassification rate (probability that true ratio falls outside bounds)
///
/// # Assumptions
///
/// - `positivity(x)` - all values in x must be strictly positive
/// - `positivity(y)` - all values in y must be strictly positive
///
/// # Returns
///
/// A `Bounds` struct containing the lower and upper bounds, or an error if inputs are invalid.
pub fn ratio_bounds(x: &[f64], y: &[f64], misrate: f64) -> Result<Bounds, EstimatorError> {
    check_validity(x, Subject::X)?;
    check_validity(y, Subject::Y)?;

    // Log-transform samples (includes positivity check)
    let log_x = log(x, Subject::X)?;
    let log_y = log(y, Subject::Y)?;

    // Delegate to shift_bounds in log-space
    let log_bounds = shift_bounds(&log_x, &log_y, misrate)?;

    // Exp-transform back to ratio-space
    Ok(Bounds {
        lower: log_bounds.lower.exp(),
        upper: log_bounds.upper.exp(),
    })
}
