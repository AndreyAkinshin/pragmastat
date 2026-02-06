//! Fast algorithm for computing quantiles from pairwise averages
//!
//! Efficiently computes quantiles from all pairwise averages (x[i] + x[j]) / 2 for i <= j.
//! Uses binary search with counting function to avoid materializing all N(N+1)/2 pairs.

/// Relative epsilon for floating-point comparisons in binary search convergence.
const RELATIVE_EPSILON: f64 = 1e-14;

/// Compute both lower and upper bounds from pairwise averages.
pub fn fast_center_quantile_bounds(sorted: &[f64], margin_lo: i64, margin_hi: i64) -> (f64, f64) {
    debug_assert!(
        sorted.windows(2).all(|w| w[0] <= w[1]),
        "fast_center_quantile_bounds: input must be sorted"
    );
    let n = sorted.len();
    let total_pairs = (n * (n + 1) / 2) as i64;

    let margin_lo = margin_lo.clamp(1, total_pairs);
    let margin_hi = margin_hi.clamp(1, total_pairs);

    let lo = fast_center_find_exact_quantile(sorted, margin_lo);
    let hi = fast_center_find_exact_quantile(sorted, margin_hi);

    if lo > hi {
        (hi, lo)
    } else {
        (lo, hi)
    }
}

/// Count pairwise averages <= target value.
/// Uses O(n) two-pointer algorithm.
fn count_pairs_less_or_equal(sorted: &[f64], target: f64) -> i64 {
    let n = sorted.len();
    let mut count: i64 = 0;
    // j is not reset: as i increases, threshold decreases monotonically
    let mut j = n as i64 - 1;

    for i in 0..n {
        let threshold = 2.0 * target - sorted[i];

        while j >= 0 && sorted[j as usize] > threshold {
            j -= 1;
        }

        if j >= i as i64 {
            count += j - i as i64 + 1;
        }
    }

    count
}

/// Find the exact k-th pairwise average using selection algorithm.
fn fast_center_find_exact_quantile(sorted: &[f64], k: i64) -> f64 {
    let n = sorted.len();
    let total_pairs = (n * (n + 1) / 2) as i64;

    if n == 1 {
        return sorted[0];
    }

    if k == 1 {
        return sorted[0];
    }

    if k == total_pairs {
        return sorted[n - 1];
    }

    let mut lo = sorted[0];
    let mut hi = sorted[n - 1];
    const EPS: f64 = RELATIVE_EPSILON;

    while hi - lo > EPS * 1.0_f64.max(lo.abs().max(hi.abs())) {
        let mid = (lo + hi) / 2.0;
        let count_less_or_equal = count_pairs_less_or_equal(sorted, mid);

        if count_less_or_equal >= k {
            hi = mid;
        } else {
            lo = mid;
        }
    }

    let target = (lo + hi) / 2.0;
    let mut candidates: Vec<f64> = Vec::new();

    for i in 0..n {
        let threshold = 2.0 * target - sorted[i];

        let mut left = i;
        let mut right = n;

        while left < right {
            let m = (left + right) / 2;
            if sorted[m] < threshold - EPS {
                left = m + 1;
            } else {
                right = m;
            }
        }

        if left < n
            && left >= i
            && (sorted[left] - threshold).abs() < EPS * 1.0_f64.max(threshold.abs())
        {
            candidates.push((sorted[i] + sorted[left]) / 2.0);
        }

        if left > i {
            let avg_before = (sorted[i] + sorted[left - 1]) / 2.0;
            if avg_before <= target + EPS {
                candidates.push(avg_before);
            }
        }
    }

    if candidates.is_empty() {
        return target;
    }

    candidates.sort_by(|a, b| a.partial_cmp(b).unwrap());

    for candidate in &candidates {
        let count_at_candidate = count_pairs_less_or_equal(sorted, *candidate);
        if count_at_candidate >= k {
            return *candidate;
        }
    }

    target
}
