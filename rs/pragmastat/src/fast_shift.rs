use std::collections::BTreeSet;

use crate::assumptions::{log, Subject};

/// Fast O((m+n) log precision) implementation of the Shift estimator.
/// Computes the median of all pairwise differences {x[i] - y[j]}.
///
/// Internal implementation - not part of public API.
pub(crate) fn fast_shift(x: &[f64], y: &[f64]) -> Result<f64, &'static str> {
    let result = fast_shift_quantiles(x, y, &[0.5], false)?;
    Ok(result[0])
}

/// Computes quantiles of all pairwise differences {x[i] - y[j]}.
/// Time complexity: O((m+n) log precision) per unique rank.
/// Space complexity: O(1) - avoids materializing all m*n differences.
///
/// # Arguments
/// * `x` - First sample (will be sorted if assume_sorted is false)
/// * `y` - Second sample (will be sorted if assume_sorted is false)
/// * `p` - Slice of probabilities in [0, 1]
/// * `assume_sorted` - If true, assumes inputs are already sorted
pub(crate) fn fast_shift_quantiles(
    x: &[f64],
    y: &[f64],
    p: &[f64],
    assume_sorted: bool,
) -> Result<Vec<f64>, &'static str> {
    if x.is_empty() || y.is_empty() {
        return Err("Input slices cannot be empty");
    }

    // Validate probabilities
    for &pk in p {
        if pk.is_nan() || !(0.0..=1.0).contains(&pk) {
            return Err("Probabilities must be within [0, 1]");
        }
    }

    // Validate for NaN/infinite values
    if x.iter().any(|v| !v.is_finite()) || y.iter().any(|v| !v.is_finite()) {
        return Err("Input contains NaN or infinite values");
    }

    let mut x_sorted_buf;
    let mut y_sorted_buf;
    let (xs, ys): (&[f64], &[f64]) = if assume_sorted {
        (x, y)
    } else {
        x_sorted_buf = x.to_vec();
        y_sorted_buf = y.to_vec();
        x_sorted_buf.sort_by(|a, b| a.total_cmp(b));
        y_sorted_buf.sort_by(|a, b| a.total_cmp(b));
        (&x_sorted_buf, &y_sorted_buf)
    };

    let m = xs.len();
    let n = ys.len();
    let total = (m as i64) * (n as i64);

    // Collect all required ranks using Type-7 quantile interpolation
    struct InterpolationParams {
        lower_rank: i64,
        upper_rank: i64,
        weight: f64,
    }

    let mut params = Vec::with_capacity(p.len());
    let mut required_ranks = BTreeSet::new();

    for &pk in p {
        let h = ((total - 1) as f64).mul_add(pk, 1.0);
        let mut lower_rank = h.floor() as i64;
        let mut upper_rank = h.ceil() as i64;
        let weight = h - lower_rank as f64;

        if lower_rank < 1 {
            lower_rank = 1;
        }
        if upper_rank > total {
            upper_rank = total;
        }

        params.push(InterpolationParams {
            lower_rank,
            upper_rank,
            weight,
        });
        required_ranks.insert(lower_rank);
        required_ranks.insert(upper_rank);
    }

    // Compute values for all required ranks
    let mut rank_values: Vec<(i64, f64)> = Vec::with_capacity(required_ranks.len());
    for rank in required_ranks {
        rank_values.push((rank, select_kth_pairwise_diff(xs, ys, rank)?));
    }

    // Interpolate to get final results
    let find_rank = |r: i64| -> f64 { rank_values.iter().find(|(rank, _)| *rank == r).unwrap().1 };
    let result: Vec<f64> = params
        .iter()
        .map(|param| {
            let lower = find_rank(param.lower_rank);
            let upper = find_rank(param.upper_rank);
            if param.weight == 0.0 {
                lower
            } else {
                param.weight.mul_add(upper, (1.0 - param.weight) * lower)
            }
        })
        .collect();

    Ok(result)
}

/// Binary search to find the k-th smallest pairwise difference x[i] - y[j]
/// without materializing all m*n differences.
pub(crate) fn select_kth_pairwise_diff(x: &[f64], y: &[f64], k: i64) -> Result<f64, &'static str> {
    let m = x.len();
    let n = y.len();
    let total = (m as i64) * (n as i64);

    if k < 1 || k > total {
        return Err("k out of range");
    }

    // Initial search bounds: [min_diff, max_diff]
    let mut search_min = x[0] - y[n - 1];
    let mut search_max = x[m - 1] - y[0];

    if search_min.is_nan() || search_max.is_nan() {
        return Err("NaN in input values");
    }

    const MAX_ITERATIONS: usize = 128; // Sufficient for double precision
    let mut prev_min = f64::NEG_INFINITY;
    let mut prev_max = f64::INFINITY;

    for _ in 0..MAX_ITERATIONS {
        if search_min == search_max {
            return Ok(search_min);
        }

        let mid = midpoint(search_min, search_max);
        let (count_le, closest_below, closest_above) = count_and_neighbors(x, y, mid);

        // If we found the exact value
        if closest_below == closest_above {
            return Ok(closest_below);
        }

        // Check if we're stuck (no progress)
        if search_min == prev_min && search_max == prev_max {
            return Ok(if count_le >= k {
                closest_below
            } else {
                closest_above
            });
        }

        prev_min = search_min;
        prev_max = search_max;

        // Update search bounds based on count
        if count_le >= k {
            search_max = closest_below;
        } else {
            search_min = closest_above;
        }
    }

    Err("Convergence failure in fast_shift")
}

/// Counts how many pairs x[i] - y[j] <= threshold using a two-pointer algorithm.
/// Also tracks the closest actual differences on either side of threshold.
/// Returns (count_less_or_equal, closest_below, closest_above).
fn count_and_neighbors(x: &[f64], y: &[f64], threshold: f64) -> (i64, f64, f64) {
    let m = x.len();
    let n = y.len();
    let mut count: i64 = 0;
    let mut max_below = f64::NEG_INFINITY;
    let mut min_above = f64::INFINITY;

    // Two-pointer algorithm: for each x[i], find the largest j where x[i] - y[j] > threshold
    let mut j = 0;
    for &xi in x.iter() {
        // Move j forward while xi - y[j] > threshold
        while j < n && xi - y[j] > threshold {
            j += 1;
        }

        // Count pairs for this xi: all y[j..n] satisfy xi - y[j] <= threshold
        count += (n - j) as i64;

        // Track boundaries
        if j < n {
            let diff = xi - y[j];
            if diff > max_below {
                max_below = diff;
            }
        }

        if j > 0 {
            let diff = xi - y[j - 1];
            if diff < min_above {
                min_above = diff;
            }
        }
    }

    // Fallback to actual min/max if no boundaries found
    if max_below.is_infinite() && max_below.is_sign_negative() {
        max_below = x[0] - y[n - 1];
    }
    if min_above.is_infinite() && min_above.is_sign_positive() {
        min_above = x[m - 1] - y[0];
    }

    (count, max_below, min_above)
}

/// Computes the midpoint of two numbers, avoiding overflow
fn midpoint(a: f64, b: f64) -> f64 {
    (b - a).mul_add(0.5, a)
}

/// Fast O((m+n) log precision) implementation of the Ratio estimator via log-transformation.
/// Computes the median of all pairwise ratios {x[i] / y[j]} as exp(Shift(log x, log y)).
///
/// Internal implementation - not part of public API.
pub(crate) fn fast_ratio(x: &[f64], y: &[f64]) -> Result<f64, &'static str> {
    let result = fast_ratio_quantiles(x, y, &[0.5], false)?;
    Ok(result[0])
}

/// Computes quantiles of all pairwise ratios {x[i] / y[j]} via log-transformation.
/// Time complexity: O((m+n) log precision) per unique rank.
/// Space complexity: O(m+n) for log-transformed arrays.
///
/// # Arguments
/// * `x` - First sample (must be positive; will be sorted if assume_sorted is false)
/// * `y` - Second sample (must be positive; will be sorted if assume_sorted is false)
/// * `p` - Slice of probabilities in [0, 1]
/// * `assume_sorted` - If true, assumes inputs are already sorted
pub(crate) fn fast_ratio_quantiles(
    x: &[f64],
    y: &[f64],
    p: &[f64],
    assume_sorted: bool,
) -> Result<Vec<f64>, &'static str> {
    if x.is_empty() || y.is_empty() {
        return Err("Input slices cannot be empty");
    }

    // Log-transform both samples (includes positivity check)
    let log_x = log(x, Subject::X).map_err(|_| "x must contain only positive values")?;
    let log_y = log(y, Subject::Y).map_err(|_| "y must contain only positive values")?;

    // Delegate to fast_shift_quantiles in log-space
    let log_result = fast_shift_quantiles(&log_x, &log_y, p, assume_sorted)?;

    // Exp-transform back to ratio-space
    Ok(log_result.iter().map(|&v| v.exp()).collect())
}
