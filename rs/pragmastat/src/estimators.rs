//! Statistical estimators for one-sample and two-sample analysis

use crate::assumptions::{
    check_positivity, check_validity, log, AssumptionError, EstimatorError, Subject,
};

/// Estimates the central value of the data (center)
///
/// Calculates the median of all pairwise averages (x[i] + x[j])/2.
/// More robust than the mean and more efficient than the median.
/// Uses fast O(n log n) algorithm.
pub fn center(x: &[f64]) -> Result<f64, EstimatorError> {
    // Check validity (priority 0)
    check_validity(x, Subject::X)?;
    crate::fast_center::fast_center(x).map_err(EstimatorError::from)
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

    let spread_val = crate::fast_spread::fast_spread(x).map_err(EstimatorError::from)?;
    if spread_val <= 0.0 {
        return Err(EstimatorError::from(AssumptionError::sparity(Subject::X)));
    }
    Ok(spread_val)
}

#[deprecated(since = "10.0.0", note = "use spread(x) / center(x).abs() instead")]
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
    let center_val = crate::fast_center::fast_center(x).map_err(EstimatorError::from)?;

    // Calculate spread (we know x is valid)
    // Note: spread now requires sparity, but for rel_spread we require positivity
    // which is checked above. We use the internal implementation directly.
    let spread_val = crate::fast_spread::fast_spread(x).map_err(EstimatorError::from)?;

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
    crate::fast_shift::fast_shift(x, y).map_err(EstimatorError::from)
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

    crate::fast_shift::fast_ratio(x, y).map_err(EstimatorError::from)
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
#[cfg(test)]
pub(crate) fn avg_spread(x: &[f64], y: &[f64]) -> Result<f64, EstimatorError> {
    // Check validity for x (priority 0, subject x)
    check_validity(x, Subject::X)?;

    // Check validity for y (priority 0, subject y)
    check_validity(y, Subject::Y)?;

    let n = x.len();
    let m = y.len();

    let spread_x = crate::fast_spread::fast_spread(x).map_err(EstimatorError::from)?;
    if spread_x <= 0.0 {
        return Err(EstimatorError::from(AssumptionError::sparity(Subject::X)));
    }
    let spread_y = crate::fast_spread::fast_spread(y).map_err(EstimatorError::from)?;
    if spread_y <= 0.0 {
        return Err(EstimatorError::from(AssumptionError::sparity(Subject::Y)));
    }

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

    let n = x.len();
    let m = y.len();

    let spread_x = crate::fast_spread::fast_spread(x).map_err(EstimatorError::from)?;
    if spread_x <= 0.0 {
        return Err(EstimatorError::from(AssumptionError::sparity(Subject::X)));
    }
    let spread_y = crate::fast_spread::fast_spread(y).map_err(EstimatorError::from)?;
    if spread_y <= 0.0 {
        return Err(EstimatorError::from(AssumptionError::sparity(Subject::Y)));
    }

    // Calculate shift (we know inputs are valid)
    let shift_val = crate::fast_shift::fast_shift(x, y).map_err(EstimatorError::from)?;

    let avg_spread_val = (n as f64 * spread_x + m as f64 * spread_y) / (n + m) as f64;

    Ok(shift_val / avg_spread_val)
}

/// Default misclassification rate for bounds estimators.
pub const DEFAULT_MISRATE: f64 = 1e-3;

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

    let min_misrate = crate::min_misrate::min_achievable_misrate_two_sample(n, m)
        .map_err(EstimatorError::from)?;
    if misrate < min_misrate {
        return Err(EstimatorError::from(AssumptionError::domain(
            Subject::Misrate,
        )));
    }

    // Sort both arrays
    let mut xs = x.to_vec();
    let mut ys = y.to_vec();
    xs.sort_by(|a, b| a.total_cmp(b));
    ys.sort_by(|a, b| a.total_cmp(b));

    let total = n as u64 * m as u64;

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

    if misrate.is_nan() || !(0.0..=1.0).contains(&misrate) {
        return Err(EstimatorError::from(AssumptionError::domain(
            Subject::Misrate,
        )));
    }

    let min_misrate = crate::min_misrate::min_achievable_misrate_two_sample(x.len(), y.len())
        .map_err(EstimatorError::from)?;
    if misrate < min_misrate {
        return Err(EstimatorError::from(AssumptionError::domain(
            Subject::Misrate,
        )));
    }

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

/// Provides distribution-free bounds for spread using disjoint pairs with sign-test inversion.
///
/// Uses a value-independent disjoint pairing and randomizes the sign-test cutoff
/// between adjacent ranks to match the requested misrate.
/// Requires misrate >= 2^(1 - floor(n/2)).
///
/// # Arguments
///
/// * `x` - Sample slice
/// * `misrate` - Misclassification rate (probability that true spread falls outside bounds)
///
/// # Assumptions
///
/// - `sparity(x)` - sample must be non tie-dominant (spread > 0)
///
/// # Returns
///
/// A `Bounds` struct containing the lower and upper bounds, or an error if inputs are invalid.
pub fn spread_bounds(x: &[f64], misrate: f64) -> Result<Bounds, EstimatorError> {
    let mut rng = crate::rng::Rng::new();
    spread_bounds_with_rng(x, misrate, &mut rng)
}

/// Provides distribution-free bounds for spread with a deterministic seed.
///
/// Same as `spread_bounds` but uses a string seed for reproducible randomization.
pub fn spread_bounds_with_seed(
    x: &[f64],
    misrate: f64,
    seed: &str,
) -> Result<Bounds, EstimatorError> {
    let mut rng = crate::rng::Rng::from_string(seed);
    spread_bounds_with_rng(x, misrate, &mut rng)
}

/// Provides distribution-free bounds for avg_spread using Bonferroni combination.
///
/// Computes SpreadBounds for each sample with equal split alpha = misrate / 2,
/// then combines bounds via the weighted average:
/// AvgSpread = (n * Spread(x) + m * Spread(y)) / (n + m).
///
/// Requires alpha >= 2^(1 - floor(n/2)) for x and y.
///
/// # Arguments
///
/// * `x` - First sample slice
/// * `y` - Second sample slice
/// * `misrate` - Misclassification rate (probability that true avg_spread falls outside bounds)
///
/// # Assumptions
///
/// - `sparity(x)` - first sample must be non tie-dominant (spread > 0)
/// - `sparity(y)` - second sample must be non tie-dominant (spread > 0)
///
/// # Returns
///
/// A `Bounds` struct containing the lower and upper bounds, or an error if inputs are invalid.
#[cfg(test)]
pub(crate) fn avg_spread_bounds(
    x: &[f64],
    y: &[f64],
    misrate: f64,
) -> Result<Bounds, EstimatorError> {
    let mut rng_x = crate::rng::Rng::new();
    let mut rng_y = crate::rng::Rng::new();
    avg_spread_bounds_with_rngs(x, y, misrate, &mut rng_x, &mut rng_y)
}

/// Provides distribution-free avg_spread bounds with a deterministic seed.
///
/// Same as `avg_spread_bounds` but uses two identical RNG streams derived
/// from the provided seed for reproducible randomization.
#[cfg(test)]
pub(crate) fn avg_spread_bounds_with_seed(
    x: &[f64],
    y: &[f64],
    misrate: f64,
    seed: &str,
) -> Result<Bounds, EstimatorError> {
    let mut rng_x = crate::rng::Rng::from_string(seed);
    let mut rng_y = crate::rng::Rng::from_string(seed);
    avg_spread_bounds_with_rngs(x, y, misrate, &mut rng_x, &mut rng_y)
}

fn avg_spread_bounds_with_rngs(
    x: &[f64],
    y: &[f64],
    misrate: f64,
    rng_x: &mut crate::rng::Rng,
    rng_y: &mut crate::rng::Rng,
) -> Result<Bounds, EstimatorError> {
    // Check validity (priority 0)
    check_validity(x, Subject::X)?;
    check_validity(y, Subject::Y)?;

    // Check misrate domain
    if misrate.is_nan() || !(0.0..=1.0).contains(&misrate) {
        return Err(EstimatorError::from(AssumptionError::domain(
            Subject::Misrate,
        )));
    }

    let n = x.len();
    let m = y.len();

    if n < 2 {
        return Err(EstimatorError::from(AssumptionError::domain(Subject::X)));
    }
    if m < 2 {
        return Err(EstimatorError::from(AssumptionError::domain(Subject::Y)));
    }

    let mx = n / 2;
    let my = m / 2;
    let min_x = crate::min_misrate::min_achievable_misrate_one_sample(mx)?;
    let min_y = crate::min_misrate::min_achievable_misrate_one_sample(my)?;

    let alpha = misrate / 2.0;
    if alpha < min_x || alpha < min_y {
        return Err(EstimatorError::from(AssumptionError::domain(
            Subject::Misrate,
        )));
    }

    if crate::fast_spread::fast_spread(x).map_err(EstimatorError::from)? <= 0.0 {
        return Err(EstimatorError::from(AssumptionError::sparity(Subject::X)));
    }
    if crate::fast_spread::fast_spread(y).map_err(EstimatorError::from)? <= 0.0 {
        return Err(EstimatorError::from(AssumptionError::sparity(Subject::Y)));
    }

    let bounds_x = spread_bounds_with_rng(x, alpha, rng_x)?;
    let bounds_y = spread_bounds_with_rng(y, alpha, rng_y)?;

    let weight_x = n as f64 / (n + m) as f64;
    let weight_y = m as f64 / (n + m) as f64;

    Ok(Bounds {
        lower: weight_x * bounds_x.lower + weight_y * bounds_y.lower,
        upper: weight_x * bounds_x.upper + weight_y * bounds_y.upper,
    })
}

/// Provides distribution-free bounds for disparity using Bonferroni combination.
///
/// Splits the misrate between ShiftBounds and AvgSpreadBounds with a minimal-feasible
/// Bonferroni split:
/// `alpha_shift + alpha_avg = misrate`, with each at least its minimum achievable misrate.
///
/// If the AvgSpreadBounds lower endpoint is zero, the bounds may be unbounded.
///
/// # Arguments
///
/// * `x` - First sample slice
/// * `y` - Second sample slice
/// * `misrate` - Misclassification rate (probability that true disparity falls outside bounds)
///
/// # Assumptions
///
/// - `sparity(x)` - first sample must be non tie-dominant (spread > 0)
/// - `sparity(y)` - second sample must be non tie-dominant (spread > 0)
///
/// # Returns
///
/// A `Bounds` struct containing the lower and upper bounds, or an error if inputs are invalid.
pub fn disparity_bounds(x: &[f64], y: &[f64], misrate: f64) -> Result<Bounds, EstimatorError> {
    let mut rng_x = crate::rng::Rng::new();
    let mut rng_y = crate::rng::Rng::new();
    disparity_bounds_with_rngs(x, y, misrate, &mut rng_x, &mut rng_y)
}

/// Provides distribution-free disparity bounds with a deterministic seed.
///
/// Same as `disparity_bounds` but uses two identical RNG streams derived
/// from the provided seed for reproducible randomization. Both RNG streams
/// are intentionally correlated (same seed) to ensure the Bonferroni split
/// between shift bounds and avg-spread bounds uses consistent randomization.
pub fn disparity_bounds_with_seed(
    x: &[f64],
    y: &[f64],
    misrate: f64,
    seed: &str,
) -> Result<Bounds, EstimatorError> {
    let mut rng_x = crate::rng::Rng::from_string(seed);
    let mut rng_y = crate::rng::Rng::from_string(seed);
    disparity_bounds_with_rngs(x, y, misrate, &mut rng_x, &mut rng_y)
}

fn disparity_bounds_with_rngs(
    x: &[f64],
    y: &[f64],
    misrate: f64,
    rng_x: &mut crate::rng::Rng,
    rng_y: &mut crate::rng::Rng,
) -> Result<Bounds, EstimatorError> {
    // Check validity (priority 0)
    check_validity(x, Subject::X)?;
    check_validity(y, Subject::Y)?;

    // Check misrate domain
    if misrate.is_nan() || !(0.0..=1.0).contains(&misrate) {
        return Err(EstimatorError::from(AssumptionError::domain(
            Subject::Misrate,
        )));
    }

    let n = x.len();
    let m = y.len();

    if n < 2 {
        return Err(EstimatorError::from(AssumptionError::domain(Subject::X)));
    }
    if m < 2 {
        return Err(EstimatorError::from(AssumptionError::domain(Subject::Y)));
    }

    let min_shift = crate::min_misrate::min_achievable_misrate_two_sample(n, m)
        .map_err(EstimatorError::from)?;
    let min_x = crate::min_misrate::min_achievable_misrate_one_sample(n / 2)?;
    let min_y = crate::min_misrate::min_achievable_misrate_one_sample(m / 2)?;
    let min_avg = 2.0 * min_x.max(min_y);

    if misrate < min_shift + min_avg {
        return Err(EstimatorError::from(AssumptionError::domain(
            Subject::Misrate,
        )));
    }

    let extra = misrate - (min_shift + min_avg);
    let alpha_shift = min_shift + extra / 2.0;
    let alpha_avg = min_avg + extra / 2.0;

    if crate::fast_spread::fast_spread(x).map_err(EstimatorError::from)? <= 0.0 {
        return Err(EstimatorError::from(AssumptionError::sparity(Subject::X)));
    }
    if crate::fast_spread::fast_spread(y).map_err(EstimatorError::from)? <= 0.0 {
        return Err(EstimatorError::from(AssumptionError::sparity(Subject::Y)));
    }

    let shift_bounds = shift_bounds(x, y, alpha_shift)?;
    let avg_bounds = avg_spread_bounds_with_rngs(x, y, alpha_avg, rng_x, rng_y)?;

    let la = avg_bounds.lower;
    let ua = avg_bounds.upper;
    let ls = shift_bounds.lower;
    let us = shift_bounds.upper;

    if la > 0.0 {
        let r1 = ls / la;
        let r2 = ls / ua;
        let r3 = us / la;
        let r4 = us / ua;
        let lower = r1.min(r2).min(r3).min(r4);
        let upper = r1.max(r2).max(r3).max(r4);
        return Ok(Bounds { lower, upper });
    }

    if ua <= 0.0 {
        if ls == 0.0 && us == 0.0 {
            return Ok(Bounds {
                lower: 0.0,
                upper: 0.0,
            });
        }
        if ls >= 0.0 {
            return Ok(Bounds {
                lower: 0.0,
                upper: f64::INFINITY,
            });
        }
        if us <= 0.0 {
            return Ok(Bounds {
                lower: f64::NEG_INFINITY,
                upper: 0.0,
            });
        }
        return Ok(Bounds {
            lower: f64::NEG_INFINITY,
            upper: f64::INFINITY,
        });
    }

    if ls > 0.0 {
        return Ok(Bounds {
            lower: ls / ua,
            upper: f64::INFINITY,
        });
    }
    if us < 0.0 {
        return Ok(Bounds {
            lower: f64::NEG_INFINITY,
            upper: us / ua,
        });
    }
    if ls == 0.0 && us == 0.0 {
        return Ok(Bounds {
            lower: 0.0,
            upper: 0.0,
        });
    }
    if ls == 0.0 && us > 0.0 {
        return Ok(Bounds {
            lower: 0.0,
            upper: f64::INFINITY,
        });
    }
    if ls < 0.0 && us == 0.0 {
        return Ok(Bounds {
            lower: f64::NEG_INFINITY,
            upper: 0.0,
        });
    }

    Ok(Bounds {
        lower: f64::NEG_INFINITY,
        upper: f64::INFINITY,
    })
}

fn spread_bounds_with_rng(
    x: &[f64],
    misrate: f64,
    rng: &mut crate::rng::Rng,
) -> Result<Bounds, EstimatorError> {
    check_validity(x, Subject::X)?;

    if misrate.is_nan() || !(0.0..=1.0).contains(&misrate) {
        return Err(EstimatorError::from(AssumptionError::domain(
            Subject::Misrate,
        )));
    }

    let n = x.len();
    let m = n / 2;
    let min_misrate = crate::min_misrate::min_achievable_misrate_one_sample(m)?;
    if misrate < min_misrate {
        return Err(EstimatorError::from(AssumptionError::domain(
            Subject::Misrate,
        )));
    }

    if x.len() < 2 {
        return Err(EstimatorError::from(AssumptionError::sparity(Subject::X)));
    }
    if crate::fast_spread::fast_spread(x).map_err(EstimatorError::from)? <= 0.0 {
        return Err(EstimatorError::from(AssumptionError::sparity(Subject::X)));
    }

    let margin = crate::sign_margin::sign_margin_randomized(m, misrate, rng)
        .map_err(EstimatorError::from)?;
    let mut half_margin = margin / 2;
    let max_half_margin = (m - 1) / 2;
    if half_margin > max_half_margin {
        half_margin = max_half_margin;
    }

    let k_left = half_margin + 1;
    let k_right = m - half_margin;

    let indices: Vec<usize> = (0..n).collect();
    let shuffled = rng.shuffle(&indices);
    let mut diffs = Vec::with_capacity(m);
    for i in 0..m {
        let a = shuffled[2 * i];
        let b = shuffled[2 * i + 1];
        diffs.push((x[a] - x[b]).abs());
    }
    diffs.sort_by(|a, b| a.total_cmp(b));

    let lower = diffs[k_left - 1];
    let upper = diffs[k_right - 1];

    Ok(Bounds { lower, upper })
}
