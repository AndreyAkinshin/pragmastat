/// Fast O(n log n) implementation of the Spread (Shamos) estimator.
/// Based on Monahan's selection algorithm adapted for pairwise differences.
///
/// Internal implementation - not part of public API.
use rand::Rng;

pub(crate) fn fast_spread(values: &[f64]) -> Result<f64, &'static str> {
    let n = values.len();
    if n <= 1 {
        return Ok(0.0);
    }
    if n == 2 {
        return Ok((values[1] - values[0]).abs());
    }

    // Sort the values
    let mut a = values.to_vec();
    a.sort_by(|x, y| x.partial_cmp(y).unwrap());

    // Total number of pairwise differences with i < j
    let total_pairs = (n * (n - 1)) / 2;
    let k_low = total_pairs.div_ceil(2); // 1-based rank of lower middle
    let k_high = (total_pairs + 2) / 2; // 1-based rank of upper middle

    // Per-row active bounds over columns j (0-based indices)
    // Row i allows j in [i+1, n-1] initially
    let mut left_bounds: Vec<usize> = (0..n).map(|i| (i + 1).min(n)).collect();
    let mut right_bounds = vec![n - 1; n];

    for i in 0..n {
        if left_bounds[i] > right_bounds[i] {
            left_bounds[i] = 1;
            right_bounds[i] = 0; // mark empty
        }
    }

    let mut row_counts = vec![0; n];

    // Initial pivot: a central gap
    let mut pivot = a[n / 2] - a[(n - 1) / 2];
    let mut prev_count_below = -1i64;

    let mut rng = rand::thread_rng();

    loop {
        // === PARTITION: count how many differences are < pivot ===
        let mut count_below = 0;
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
            row_counts[i] = cnt_row;
            count_below += cnt_row;

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
            let mut active = 0;

            for i in 0..n - 1 {
                let li = left_bounds[i];
                let ri = right_bounds[i];
                if li > ri {
                    continue;
                }

                let row_min = a[li] - a[i];
                let row_max = a[ri] - a[i];
                min_active = min_active.min(row_min);
                max_active = max_active.max(row_max);
                active += ri - li + 1;
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
                let new_left = i + 1 + row_counts[i];
                if new_left > left_bounds[i] {
                    left_bounds[i] = new_left;
                }
                if left_bounds[i] > right_bounds[i] {
                    left_bounds[i] = 1;
                    right_bounds[i] = 0;
                }
            }
        } else {
            // Too many below: keep only those strictly below pivot
            for i in 0..n - 1 {
                let new_right = i + row_counts[i];
                if new_right < right_bounds[i] {
                    right_bounds[i] = new_right;
                }
                if right_bounds[i] < i + 1 {
                    left_bounds[i] = 1;
                    right_bounds[i] = 0;
                }
            }
        }

        prev_count_below = count_below as i64;

        // === CHOOSE NEXT PIVOT FROM ACTIVE SET ===
        let active_size: usize = (0..n - 1)
            .filter(|&i| left_bounds[i] <= right_bounds[i])
            .map(|i| right_bounds[i] - left_bounds[i] + 1)
            .sum();

        if active_size <= 2 {
            // Few candidates left: return midrange of remaining
            let mut min_rem = f64::INFINITY;
            let mut max_rem = f64::NEG_INFINITY;
            for i in 0..n - 1 {
                if left_bounds[i] > right_bounds[i] {
                    continue;
                }
                let lo = a[left_bounds[i]] - a[i];
                let hi = a[right_bounds[i]] - a[i];
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
            return Ok(
                if ((k_low - 1) as i64 - count_below as i64).abs()
                    <= (count_below as i64 - k_low as i64).abs()
                {
                    min_rem
                } else {
                    max_rem
                },
            );
        } else {
            // Weighted random row selection
            let t = rng.gen_range(0..active_size);
            let mut acc = 0;
            let mut row = 0;
            for r in 0..n - 1 {
                if left_bounds[r] > right_bounds[r] {
                    continue;
                }
                let size = right_bounds[r] - left_bounds[r] + 1;
                if t < acc + size {
                    row = r;
                    break;
                }
                acc += size;
            }

            // Median column of the selected row
            let col = (left_bounds[row] + right_bounds[row]) / 2;
            pivot = a[col] - a[row];
        }
    }
}
