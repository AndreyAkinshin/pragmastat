//! Statistical estimators for one-sample and two-sample analysis

/// Validates that all values in the slice are finite (not NaN or infinite).
fn validate_finite(values: &[f64]) -> Result<(), &'static str> {
    if values.iter().any(|v| !v.is_finite()) {
        return Err("Input contains NaN or infinite values");
    }
    Ok(())
}

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
pub fn median(values: &[f64]) -> Result<f64, &'static str> {
    if values.is_empty() {
        return Err("Input slice cannot be empty");
    }
    validate_finite(values)?;
    let mut sorted = values.to_vec();
    sorted.sort_by(|a, b| a.total_cmp(b));
    median_sorted(&sorted)
}

/// Estimates the central value of the data (Center)
///
/// Calculates the median of all pairwise averages (x[i] + x[j])/2.
/// More robust than the mean and more efficient than the median.
/// Uses fast O(n log n) algorithm.
pub fn center(x: &[f64]) -> Result<f64, &'static str> {
    crate::fast_center::fast_center(x)
}

/// Estimates data dispersion (Spread)
///
/// Calculates the median of all pairwise absolute differences |x[i] - x[j]|.
/// More robust than standard deviation and more efficient than MAD.
/// Uses fast O(n log n) algorithm.
pub fn spread(x: &[f64]) -> Result<f64, &'static str> {
    crate::fast_spread::fast_spread(x)
}

/// Measures the relative dispersion of a sample (RelSpread)
///
/// Calculates the ratio of Spread to absolute Center.
/// Robust alternative to the coefficient of variation.
pub fn rel_spread(x: &[f64]) -> Result<f64, &'static str> {
    let center_val = center(x)?;
    if center_val == 0.0 {
        return Err("RelSpread is undefined when Center equals zero");
    }
    let spread_val = spread(x)?;
    Ok(spread_val / center_val.abs())
}

/// Measures the typical difference between elements of x and y (Shift)
///
/// Calculates the median of all pairwise differences (x[i] - y[j]).
/// Positive values mean x is typically larger, negative means y is typically larger.
/// Uses fast O((m+n) log precision) algorithm.
pub fn shift(x: &[f64], y: &[f64]) -> Result<f64, &'static str> {
    crate::fast_shift::fast_shift(x, y)
}

/// Measures how many times larger x is compared to y (Ratio)
///
/// Calculates the median of all pairwise ratios (x[i] / y[j]).
/// For example, ratio = 1.2 means x is typically 20% larger than y.
pub fn ratio(x: &[f64], y: &[f64]) -> Result<f64, &'static str> {
    if x.is_empty() || y.is_empty() {
        return Err("Input slices cannot be empty");
    }

    // Check that all y values are strictly positive
    if y.iter().any(|&val| val <= 0.0) {
        return Err("All values in y must be strictly positive");
    }

    let mut pairwise_ratios = Vec::new();
    for &xi in x {
        for &yj in y {
            pairwise_ratios.push(xi / yj);
        }
    }

    median(&pairwise_ratios)
}

/// Measures the typical variability when considering both samples together (AvgSpread)
///
/// Computes the weighted average of individual spreads: (n*Spread(x) + m*Spread(y))/(n+m).
pub fn avg_spread(x: &[f64], y: &[f64]) -> Result<f64, &'static str> {
    if x.is_empty() || y.is_empty() {
        return Err("Input slices cannot be empty");
    }

    let n = x.len();
    let m = y.len();
    let spread_x = spread(x)?;
    let spread_y = spread(y)?;

    Ok((n as f64 * spread_x + m as f64 * spread_y) / (n + m) as f64)
}

/// Measures effect size: a normalized difference between x and y (Disparity)
///
/// Calculated as Shift / AvgSpread. Robust alternative to Cohen's d.
/// Returns infinity if avg_spread is zero.
pub fn disparity(x: &[f64], y: &[f64]) -> Result<f64, &'static str> {
    let shift_val = shift(x, y)?;
    let avg_spread_val = avg_spread(x, y)?;
    if avg_spread_val == 0.0 {
        return Ok(f64::INFINITY);
    }
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
pub fn shift_bounds(x: &[f64], y: &[f64], misrate: f64) -> Result<Bounds, &'static str> {
    if x.is_empty() || y.is_empty() {
        return Err("Input slices cannot be empty");
    }

    let n = x.len();
    let m = y.len();

    // Validate inputs for NaN/infinite values
    validate_finite(x)?;
    validate_finite(y)?;

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

    let margin = crate::pairwise_margin::pairwise_margin(n, m, misrate)?;
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

    let bounds = crate::fast_shift::fast_shift_quantiles(&xs, &ys, &p, true)?;

    let lower = bounds[0].min(bounds[1]);
    let upper = bounds[0].max(bounds[1]);

    Ok(Bounds { lower, upper })
}
