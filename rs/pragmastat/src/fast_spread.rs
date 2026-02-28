/// Fast O(n log n) implementation of the Spread (Shamos) estimator.
/// Based on Monahan's selection algorithm adapted for pairwise differences.
///
/// Internal implementation - not part of public API.
use crate::fnv1a::hash_f64_slice;
use crate::rng::Rng;

pub(crate) fn fast_spread(values: &[f64]) -> Result<f64, &'static str> {
    let n = values.len();
    if n <= 1 {
        return Ok(0.0);
    }
    if n == 2 {
        return Ok((values[1] - values[0]).abs());
    }

    // Validate for NaN/infinite values
    if values.iter().any(|v| !v.is_finite()) {
        return Err("Input contains NaN or infinite values");
    }

    if n > u32::MAX as usize {
        return Err("fast_spread: input too large");
    }

    // Sort the values
    let mut a = values.to_vec();
    a.sort_by(|x, y| x.total_cmp(y));

    // Total number of pairwise differences with i < j
    let total_pairs = (n as u64) * ((n - 1) as u64) / 2;
    let k_low = total_pairs.div_ceil(2); // 1-based rank of lower middle
    let k_high = (total_pairs + 2) / 2; // 1-based rank of upper middle

    // Per-row active bounds over columns j (0-based indices)
    // Row i allows j in [i+1, n-1] initially
    let mut left_bounds: Vec<u32> = (0..n).map(|i| (i + 1).min(n) as u32).collect();
    let mut right_bounds = vec![(n - 1) as u32; n];

    for i in 0..n {
        if left_bounds[i] > right_bounds[i] {
            left_bounds[i] = 1;
            right_bounds[i] = 0; // mark empty
        }
    }

    let mut row_counts: Vec<u32> = vec![0; n];

    // Initial pivot: a central gap
    let mut pivot = a[n / 2] - a[(n - 1) / 2];
    let mut prev_count_below = -1i64;

    let mut rng = Rng::from_seed(hash_f64_slice(values));

    loop {
        // === PARTITION: count how many differences are < pivot ===
        let mut count_below: u64 = 0;
        let mut largest_below = f64::NEG_INFINITY;
        let mut smallest_at_or_above = f64::INFINITY;

        let mut j = 1; // global two-pointer (non-decreasing across rows)
        for i in 0..n - 1 {
            if j < i + 1 {
                j = i + 1;
            }
            while j < n && a[j] - a[i] < pivot {
                j += 1;
            }

            let cnt_row = j.saturating_sub(i + 1);
            row_counts[i] = cnt_row as u32;
            count_below += cnt_row as u64;

            // boundary elements for this row
            if cnt_row > 0 {
                let cand_below = a[j - 1] - a[i];
                largest_below = largest_below.max(cand_below);
            }

            if j < n {
                let cand_at_or_above = a[j] - a[i];
                smallest_at_or_above = smallest_at_or_above.min(cand_at_or_above);
            }
        }

        // === TARGET CHECK ===
        let at_target = count_below == k_low || count_below == k_high - 1;

        if at_target {
            if k_low < k_high {
                // Even N: average the two central order stats
                return Ok(0.5 * (largest_below + smallest_at_or_above));
            } else {
                // Odd N: pick the single middle
                let need_largest = count_below == k_low;
                return Ok(if need_largest {
                    largest_below
                } else {
                    smallest_at_or_above
                });
            }
        }

        // === STALL HANDLING ===
        if count_below as i64 == prev_count_below {
            let mut min_active = f64::INFINITY;
            let mut max_active = f64::NEG_INFINITY;
            let mut active: u64 = 0;

            for i in 0..n - 1 {
                let li = left_bounds[i] as usize;
                let ri = right_bounds[i] as usize;
                if li > n || ri >= n {
                    return Err("fast_spread: bounds corrupted");
                }
                if li > ri {
                    continue;
                }

                let row_min = a[li] - a[i];
                let row_max = a[ri] - a[i];
                min_active = min_active.min(row_min);
                max_active = max_active.max(row_max);
                active += (ri - li + 1) as u64;
            }

            if active == 0 {
                if k_low < k_high {
                    return Ok(0.5 * (largest_below + smallest_at_or_above));
                }
                return Ok(if count_below >= k_low {
                    largest_below
                } else {
                    smallest_at_or_above
                });
            }

            if max_active <= min_active {
                return Ok(min_active);
            }

            let mid = 0.5 * (min_active + max_active);
            pivot = if mid > min_active && mid <= max_active {
                mid
            } else {
                max_active
            };
            prev_count_below = count_below as i64;
            continue;
        }

        // === SHRINK ACTIVE WINDOW ===
        if count_below < k_low {
            // Need larger differences: discard all strictly below pivot
            for i in 0..n - 1 {
                let i_u32 = i as u32;
                let new_left = i_u32
                    .checked_add(1)
                    .and_then(|v| v.checked_add(row_counts[i]))
                    .unwrap_or(n as u32); // overflow => mark empty below
                if new_left > left_bounds[i] {
                    left_bounds[i] = new_left;
                }
                if left_bounds[i] > right_bounds[i] {
                    left_bounds[i] = 1;
                    right_bounds[i] = 0;
                }
                let li = left_bounds[i] as usize;
                let ri = right_bounds[i] as usize;
                if li > n || ri >= n {
                    return Err("fast_spread: bounds corrupted");
                }
            }
        } else {
            // Too many below: keep only those strictly below pivot
            for i in 0..n - 1 {
                let i_u32 = i as u32;
                let new_right = match i_u32.checked_add(row_counts[i]) {
                    Some(v) => v,
                    None => {
                        left_bounds[i] = 1;
                        right_bounds[i] = 0;
                        continue;
                    }
                };
                if new_right < right_bounds[i] {
                    right_bounds[i] = new_right;
                }
                if right_bounds[i] < i_u32 + 1 {
                    left_bounds[i] = 1;
                    right_bounds[i] = 0;
                }
                let li = left_bounds[i] as usize;
                let ri = right_bounds[i] as usize;
                if li > n || ri >= n {
                    return Err("fast_spread: bounds corrupted");
                }
            }
        }

        prev_count_below = count_below as i64;

        // === CHOOSE NEXT PIVOT FROM ACTIVE SET ===
        let active_size: u64 = (0..n - 1)
            .filter(|&i| left_bounds[i] <= right_bounds[i])
            .map(|i| (right_bounds[i] - left_bounds[i] + 1) as u64)
            .sum();

        if active_size <= 2 {
            // Few candidates left: return midrange of remaining
            let mut min_rem = f64::INFINITY;
            let mut max_rem = f64::NEG_INFINITY;
            for i in 0..n - 1 {
                if left_bounds[i] > right_bounds[i] {
                    continue;
                }
                let lo = a[left_bounds[i] as usize] - a[i];
                let hi = a[right_bounds[i] as usize] - a[i];
                min_rem = min_rem.min(lo);
                max_rem = max_rem.max(hi);
            }

            if active_size == 0 {
                if k_low < k_high {
                    return Ok(0.5 * (largest_below + smallest_at_or_above));
                }
                return Ok(if count_below >= k_low {
                    largest_below
                } else {
                    smallest_at_or_above
                });
            }

            if k_low < k_high {
                return Ok(0.5 * (min_rem + max_rem));
            }
            // In this code path count_below < k_low, so min_rem is always the correct result:
            // |k_low-1-count_below| = d-1 <= d = |count_below-k_low| for all d > 0.
            return Ok(min_rem);
        } else {
            // Weighted random row selection
            let t = rng.uniform_i64(0, active_size as i64) as u64;
            let mut acc: u64 = 0;
            let mut row = 0;
            for r in 0..n - 1 {
                if left_bounds[r] > right_bounds[r] {
                    continue;
                }
                let size = (right_bounds[r] - left_bounds[r] + 1) as u64;
                if t < acc + size {
                    row = r;
                    break;
                }
                acc += size;
            }

            // Median column of the selected row
            let left = left_bounds[row] as usize;
            let right = right_bounds[row] as usize;
            if left >= n || right >= n || left > right {
                return Err("fast_spread: active bounds out of range");
            }
            let col = (left + right) / 2;
            if col >= n {
                return Err("fast_spread: pivot index out of range");
            }
            pivot = a[col] - a[row];
        }
    }
}
