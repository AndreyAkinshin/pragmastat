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
pub fn median(values: &[f64]) -> Result<f64, EstimatorError> {
    // Check validity (priority 0)
    check_validity(values, Subject::X)?;
    let mut sorted = values.to_vec();
    sorted.sort_by(|a, b| a.total_cmp(b));
    median_sorted(&sorted).map_err(|_| EstimatorError::from(AssumptionError::validity(Subject::X)))
}

/// Estimates the central value of the data (center)
///
/// Calculates the median of all pairwise averages (x[i] + x[j])/2.
/// More robust than the mean and more efficient than the median.
/// Uses fast O(n log n) algorithm.
pub fn center(x: &[f64]) -> Result<f64, EstimatorError> {
    // Check validity (priority 0)
    check_validity(x, Subject::X)?;
    crate::fast_center::fast_center(x)
        .map_err(|_| EstimatorError::from(AssumptionError::validity(Subject::X)))
}

/// Estimates data dispersion (spread)
///
/// Calculates the median of all pairwise absolute differences |x[i] - x[j]|.
/// More robust than standard deviation and more efficient than MAD.
/// Uses fast O(n log n) algorithm.
///
/// # Assumptions
///
/// - `sparity(x)` - sample must be non tie-dominant (spread > 0)
///
/// # Errors
///
/// Returns `AssumptionError` if:
/// - Input is empty, contains NaN, or contains infinite values (validity)
/// - Sample is tie-dominant or has fewer than two elements (sparity)
pub fn spread(x: &[f64]) -> Result<f64, EstimatorError> {
    // Check validity first (priority 0)
    check_validity(x, Subject::X)?;

    // Check sparity (priority 2)
    check_sparity(x, Subject::X)?;

    // Use the internal fast implementation
    // We know at this point the input is valid, so unwrap is safe for internal errors
    crate::fast_spread::fast_spread(x)
        .map_err(|_| EstimatorError::from(AssumptionError::validity(Subject::X)))
}

/// Measures the relative dispersion of a sample (rel_spread)
///
/// Calculates the ratio of spread to absolute center.
/// Robust alternative to the coefficient of variation.
///
/// # Assumptions
///
/// - `positivity(x)` - all values must be strictly positive (ensures center > 0)
///
/// # Errors
///
/// Returns `AssumptionError` if:
/// - Input is empty, contains NaN, or contains infinite values (validity)
/// - Any value is zero or negative (positivity)
pub fn rel_spread(x: &[f64]) -> Result<f64, EstimatorError> {
    // Check validity (priority 0)
    check_validity(x, Subject::X)?;

    // Check positivity (priority 1)
    check_positivity(x, Subject::X)?;

    // Calculate center (we know x is valid, center should succeed)
    let center_val = crate::fast_center::fast_center(x)
        .map_err(|_| EstimatorError::from(AssumptionError::validity(Subject::X)))?;

    // Calculate spread (we know x is valid)
    // Note: spread now requires sparity, but for rel_spread we require positivity
    // which is checked above. We use the internal implementation directly.
    let spread_val = crate::fast_spread::fast_spread(x)
        .map_err(|_| EstimatorError::from(AssumptionError::validity(Subject::X)))?;

    // center_val is guaranteed positive because all values are positive
    Ok(spread_val / center_val.abs())
}

/// Measures the typical difference between elements of x and y (shift)
///
/// Calculates the median of all pairwise differences (x[i] - y[j]).
/// Positive values mean x is typically larger, negative means y is typically larger.
/// Uses fast O((m+n) log precision) algorithm.
pub fn shift(x: &[f64], y: &[f64]) -> Result<f64, EstimatorError> {
    // Check validity (priority 0)
    check_validity(x, Subject::X)?;
    check_validity(y, Subject::Y)?;
    crate::fast_shift::fast_shift(x, y)
        .map_err(|_| EstimatorError::from(AssumptionError::validity(Subject::X)))
}

/// Measures how many times larger x is compared to y (ratio)
///
/// Calculates the median of all pairwise ratios (x[i] / y[j]) via log-transformation.
/// Equivalent to: exp(shift(log(x), log(y)))
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
pub fn ratio(x: &[f64], y: &[f64]) -> Result<f64, EstimatorError> {
    // Check validity for x (priority 0, subject x)
    check_validity(x, Subject::X)?;

    // Check validity for y (priority 0, subject y)
    check_validity(y, Subject::Y)?;

    // Check positivity for x (priority 1, subject x)
    check_positivity(x, Subject::X)?;

    // Check positivity for y (priority 1, subject y)
    check_positivity(y, Subject::Y)?;

    crate::fast_shift::fast_ratio(x, y)
        .map_err(|_| EstimatorError::from(AssumptionError::validity(Subject::X)))
}

/// Measures the typical variability when considering both samples together (avg_spread)
///
/// Computes the weighted average of individual spreads: (n*spread(x) + m*spread(y))/(n+m).
///
/// # Assumptions
///
/// - `sparity(x)` - first sample must be non tie-dominant (spread > 0)
/// - `sparity(y)` - second sample must be non tie-dominant (spread > 0)
///
/// # Errors
///
/// Returns `AssumptionError` if:
/// - Either input is empty, contains NaN, or contains infinite values (validity)
/// - Either sample is tie-dominant or has fewer than two elements (sparity)
pub fn avg_spread(x: &[f64], y: &[f64]) -> Result<f64, EstimatorError> {
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
        .map_err(|_| EstimatorError::from(AssumptionError::validity(Subject::X)))?;
    let spread_y = crate::fast_spread::fast_spread(y)
        .map_err(|_| EstimatorError::from(AssumptionError::validity(Subject::Y)))?;

    Ok((n as f64 * spread_x + m as f64 * spread_y) / (n + m) as f64)
}

/// Measures effect size: a normalized difference between x and y (disparity)
///
/// Calculated as shift / avg_spread. Robust alternative to Cohen's d.
///
/// # Assumptions
///
/// - `sparity(x)` - first sample must be non tie-dominant (spread > 0)
/// - `sparity(y)` - second sample must be non tie-dominant (spread > 0)
///
/// # Errors
///
/// Returns `AssumptionError` if:
/// - Either input is empty, contains NaN, or contains infinite values (validity)
/// - Either sample is tie-dominant or has fewer than two elements (sparity)
pub fn disparity(x: &[f64], y: &[f64]) -> Result<f64, EstimatorError> {
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
        .map_err(|_| EstimatorError::from(AssumptionError::validity(Subject::X)))?;

    // Calculate avg_spread (we know inputs are valid and non-degenerate)
    let n = x.len();
    let m = y.len();
    let spread_x = crate::fast_spread::fast_spread(x)
        .map_err(|_| EstimatorError::from(AssumptionError::validity(Subject::X)))?;
    let spread_y = crate::fast_spread::fast_spread(y)
        .map_err(|_| EstimatorError::from(AssumptionError::validity(Subject::Y)))?;
    let avg_spread_val = (n as f64 * spread_x + m as f64 * spread_y) / (n + m) as f64;

    Ok(shift_val / avg_spread_val)
}

/// Represents an interval with lower and upper bounds
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Bounds {
    pub lower: f64,
    pub upper: f64,
}

/// Provides bounds on the shift estimator with specified misclassification rate (shift_bounds)
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

    if misrate.is_nan() || !(0.0..=1.0).contains(&misrate) {
        return Err(EstimatorError::from(AssumptionError::domain(
            Subject::Misrate,
        )));
    }

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

/// Provides bounds on the ratio estimator with specified misclassification rate (ratio_bounds)
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

/// Provides exact distribution-free bounds for the population median (median_bounds)
///
/// No symmetry requirement. Uses order statistics with binomial distribution for exact coverage.
///
/// # Arguments
///
/// * `x` - Sample slice
/// * `misrate` - Misclassification rate (probability that true median falls outside bounds)
///
/// # Returns
///
/// A `Bounds` struct containing the lower and upper bounds, or an error if inputs are invalid.
pub fn median_bounds(x: &[f64], misrate: f64) -> Result<Bounds, EstimatorError> {
    check_validity(x, Subject::X)?;

    if misrate.is_nan() || !(0.0..=1.0).contains(&misrate) {
        return Err(EstimatorError::from(AssumptionError::domain(
            Subject::Misrate,
        )));
    }

    let n = x.len();
    if n < 2 {
        return Err(EstimatorError::from(AssumptionError::domain(Subject::X)));
    }

    let min_misrate = crate::min_misrate::min_achievable_misrate_one_sample(n)?;
    if misrate < min_misrate {
        return Err(EstimatorError::from(AssumptionError::domain(
            Subject::Misrate,
        )));
    }

    // Sort the input
    let mut sorted = x.to_vec();
    sorted.sort_by(|a, b| a.total_cmp(b));

    // Compute order statistic indices
    let (lo, hi) = compute_median_bounds_indices(n, misrate);

    Ok(Bounds {
        lower: sorted[lo],
        upper: sorted[hi],
    })
}

/// Find order statistic indices that achieve the specified coverage.
fn compute_median_bounds_indices(n: usize, misrate: f64) -> (usize, usize) {
    let alpha = misrate / 2.0;

    // Find the largest k where P(Bin(n,0.5) <= k) <= alpha
    let mut lo = 0;
    for k in 0..n.div_ceil(2) {
        let tail_prob = binomial_tail_probability(n, k);
        if tail_prob <= alpha {
            lo = k;
        } else {
            break;
        }
    }

    // Symmetric interval: hi = n - 1 - lo
    let mut hi = n - 1 - lo;

    if hi < lo {
        hi = lo;
    }
    if hi >= n {
        hi = n - 1;
    }

    (lo, hi)
}

/// Compute P(X <= k) for X ~ Binomial(n, 0.5).
/// Note: 2^n overflows f64 for n > 1024.
fn binomial_tail_probability(n: usize, k: usize) -> f64 {
    if k >= n {
        return 1.0;
    }

    // Normal approximation with continuity correction for large n
    // (2^n overflows f64 for n > 1024)
    if n > 1023 {
        let mean = n as f64 / 2.0;
        let std = (n as f64 / 4.0).sqrt();
        let z = (k as f64 + 0.5 - mean) / std;
        return crate::gauss_cdf::gauss_cdf(z);
    }

    let total = 2.0_f64.powf(n as f64);
    let mut sum = 0.0;
    let mut coef = 1.0;

    for i in 0..=k {
        sum += coef;
        coef = coef * (n - i) as f64 / (i + 1) as f64;
    }

    sum / total
}

/// Provides exact distribution-free bounds for center (Hodges-Lehmann pseudomedian)
///
/// Requires weak symmetry assumption: distribution symmetric around unknown center.
///
/// # Arguments
///
/// * `x` - Sample slice
/// * `misrate` - Misclassification rate (probability that true center falls outside bounds)
///
/// # Returns
///
/// A `Bounds` struct containing the lower and upper bounds, or an error if inputs are invalid.
pub fn center_bounds(x: &[f64], misrate: f64) -> Result<Bounds, EstimatorError> {
    check_validity(x, Subject::X)?;

    if misrate.is_nan() || !(0.0..=1.0).contains(&misrate) {
        return Err(EstimatorError::from(AssumptionError::domain(
            Subject::Misrate,
        )));
    }

    let n = x.len();
    if n < 2 {
        return Err(EstimatorError::from(AssumptionError::domain(Subject::X)));
    }

    let min_misrate = crate::min_misrate::min_achievable_misrate_one_sample(n)?;
    if misrate < min_misrate {
        return Err(EstimatorError::from(AssumptionError::domain(
            Subject::Misrate,
        )));
    }

    let margin = crate::signed_rank_margin::signed_rank_margin(n, misrate)?;

    let total_pairs = (n * (n + 1) / 2) as i64;

    let mut half_margin = (margin / 2) as i64;
    let max_half_margin = (total_pairs - 1) / 2;
    if half_margin > max_half_margin {
        half_margin = max_half_margin;
    }

    let k_left = half_margin + 1;
    let k_right = total_pairs - half_margin;

    // Sort the input
    let mut sorted = x.to_vec();
    sorted.sort_by(|a, b| a.total_cmp(b));

    let (lo, hi) =
        crate::fast_center_quantiles::fast_center_quantile_bounds(&sorted, k_left, k_right);
    Ok(Bounds {
        lower: lo,
        upper: hi,
    })
}

const CENTER_BOUNDS_APPROX_ITERATIONS: usize = 10000;
const CENTER_BOUNDS_APPROX_MAX_SUBSAMPLE: usize = 5000;
const CENTER_BOUNDS_APPROX_DEFAULT_SEED: &str = "center-bounds-approx";

/// Provides bootstrap-based nominal bounds for center (Hodges-Lehmann pseudomedian)
///
/// No symmetry requirement, but provides only nominal (not exact) coverage.
///
/// WARNING: Bootstrap percentile method has known undercoverage for small samples.
/// When requesting 95% confidence (misrate = 0.05), actual coverage is typically 85-92% for n < 30.
/// Users requiring exact coverage should use center_bounds (if symmetry holds) or median_bounds.
pub fn center_bounds_approx(x: &[f64], misrate: f64) -> Result<Bounds, EstimatorError> {
    center_bounds_approx_with_seed(x, misrate, None)
}

/// Provides bootstrap-based nominal bounds for center with optional seed.
pub fn center_bounds_approx_with_seed(
    x: &[f64],
    misrate: f64,
    seed: Option<&str>,
) -> Result<Bounds, EstimatorError> {
    check_validity(x, Subject::X)?;

    if misrate.is_nan() || !(0.0..=1.0).contains(&misrate) {
        return Err(EstimatorError::from(AssumptionError::domain(
            Subject::Misrate,
        )));
    }

    let n = x.len();
    if n < 2 {
        return Err(EstimatorError::from(AssumptionError::domain(Subject::X)));
    }

    let one_sample_min = crate::min_misrate::min_achievable_misrate_one_sample(n)?;
    let min_misrate = (2.0 / CENTER_BOUNDS_APPROX_ITERATIONS as f64).max(one_sample_min);
    if misrate < min_misrate {
        return Err(EstimatorError::from(AssumptionError::domain(
            Subject::Misrate,
        )));
    }

    // Sort for permutation invariance
    let mut sorted = x.to_vec();
    sorted.sort_by(|a, b| a.total_cmp(b));

    // Use default seed for cross-language determinism when no seed provided
    let mut rng = crate::Rng::from_string(seed.unwrap_or(CENTER_BOUNDS_APPROX_DEFAULT_SEED));

    // m-out-of-n subsampling: cap at MaxSubsampleSize for performance
    let m = n.min(CENTER_BOUNDS_APPROX_MAX_SUBSAMPLE);

    let mut centers: Vec<f64> = Vec::with_capacity(CENTER_BOUNDS_APPROX_ITERATIONS);
    for _ in 0..CENTER_BOUNDS_APPROX_ITERATIONS {
        let resample = rng.resample(&sorted, m);
        // fast_center cannot fail here: resample has m >= 2 elements, all finite
        let c = crate::fast_center::fast_center(&resample)
            .expect("fast_center failed on valid resample");
        centers.push(c);
    }

    centers.sort_by(|a, b| a.total_cmp(b));

    let alpha = misrate / 2.0;
    let lo_idx = ((alpha * CENTER_BOUNDS_APPROX_ITERATIONS as f64).floor() as usize).max(0);
    let hi_idx = (((1.0 - alpha) * CENTER_BOUNDS_APPROX_ITERATIONS as f64).ceil() as usize - 1)
        .min(CENTER_BOUNDS_APPROX_ITERATIONS - 1);
    let lo_idx = lo_idx.min(hi_idx);

    let bootstrap_lo = centers[lo_idx];
    let bootstrap_hi = centers[hi_idx];

    // Scale bounds to full n using asymptotic sqrt(n) rate
    if m < n {
        let center_val =
            crate::fast_center::fast_center(&sorted).expect("fast_center failed on valid input");
        let scale_factor = ((m as f64) / (n as f64)).sqrt();
        let lo = center_val + (bootstrap_lo - center_val) / scale_factor;
        let hi = center_val + (bootstrap_hi - center_val) / scale_factor;
        return Ok(Bounds {
            lower: lo,
            upper: hi,
        });
    }

    Ok(Bounds {
        lower: bootstrap_lo,
        upper: bootstrap_hi,
    })
}
