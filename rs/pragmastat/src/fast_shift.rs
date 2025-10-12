/// Fast O((m+n) log precision) implementation of the Shift estimator.
/// Computes the median of all pairwise differences {x[i] - y[j]}.
///
/// Internal implementation - not part of public API.
pub(crate) fn fast_shift(x: &[f64], y: &[f64]) -> Result<f64, &'static str> {
    if x.is_empty() || y.is_empty() {
        return Err("Input slices cannot be empty");
    }

    // Sort the input arrays
    let mut x_sorted = x.to_vec();
    let mut y_sorted = y.to_vec();
    x_sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
    y_sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let m = x_sorted.len();
    let n = y_sorted.len();
    let total = (m * n) as i64;

    // Type-7 quantile for p=0.5 (median)
    let h = 1.0 + (total - 1) as f64 * 0.5;
    let k_low = h.floor() as i64; // 1-based rank of lower middle
    let k_high = h.ceil() as i64; // 1-based rank of upper middle
    let weight = h - k_low as f64;

    // Find the k_low-th and k_high-th order statistics
    let value_low = select_kth_pairwise_diff(&x_sorted, &y_sorted, k_low);

    if k_low == k_high {
        // Odd number of pairs: single middle value
        return Ok(value_low);
    }

    // Even number of pairs: interpolate between two middle values
    let value_high = select_kth_pairwise_diff(&x_sorted, &y_sorted, k_high);
    Ok((1.0 - weight) * value_low + weight * value_high)
}

/// Binary search to find the k-th smallest pairwise difference x[i] - y[j]
/// without materializing all m*n differences.
fn select_kth_pairwise_diff(x: &[f64], y: &[f64], k: i64) -> f64 {
    let m = x.len();
    let n = y.len();
    let total = (m * n) as i64;

    if k < 1 || k > total {
        panic!("k out of range: k={}, total={}", k, total);
    }

    // Initial search bounds: [min_diff, max_diff]
    let mut search_min = x[0] - y[n - 1];
    let mut search_max = x[m - 1] - y[0];

    if search_min.is_nan() || search_max.is_nan() {
        panic!("NaN in input values");
    }

    const MAX_ITERATIONS: usize = 128; // Sufficient for double precision
    let mut prev_min = f64::NEG_INFINITY;
    let mut prev_max = f64::INFINITY;

    for _ in 0..MAX_ITERATIONS {
        if search_min == search_max {
            return search_min;
        }

        let mid = midpoint(search_min, search_max);
        let (count_le, closest_below, closest_above) = count_and_neighbors(x, y, mid);

        // If we found the exact value
        if closest_below == closest_above {
            return closest_below;
        }

        // Check if we're stuck (no progress)
        if search_min == prev_min && search_max == prev_max {
            return if count_le >= k {
                closest_below
            } else {
                closest_above
            };
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

    // Should converge within MAX_ITERATIONS
    panic!("Convergence failure in fast_shift");
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
    a + (b - a) * 0.5
}
