/// O(n log n) implementation of the Center (Hodges-Lehmann) estimator.
/// Based on Monahan's Algorithm 616 (1984).
///
/// Internal implementation - not part of public API.
use crate::fnv1a::hash_f64_slice;
use crate::rng::Rng;

pub(crate) fn center_impl(values: &[f64], assume_sorted: bool) -> Result<f64, &'static str> {
    let n = values.len();
    if n == 0 {
        return Err("Input slice cannot be empty");
    }
    if n == 1 {
        return Ok(values[0]);
    }
    if n == 2 {
        return Ok(0.5 * values[0] + 0.5 * values[1]);
    }

    // Validate for NaN/infinite values
    if values.iter().any(|v| !v.is_finite()) {
        return Err("Input contains NaN or infinite values");
    }

    let owned_sorted;
    let sorted_values: &[f64] = if assume_sorted {
        values
    } else {
        owned_sorted = {
            let mut v = values.to_vec();
            v.sort_unstable_by(|a, b| a.total_cmp(b));
            v
        };
        &owned_sorted
    };

    // Calculate target median rank(s) among all pairwise sums
    let total_pairs = (n * (n + 1)) / 2;
    let median_rank_low = total_pairs.div_ceil(2); // 1-based rank
    let median_rank_high = (total_pairs + 2) / 2;

    // Initialize search bounds for each row (1-based indexing)
    let mut left_bounds: Vec<usize> = (0..n).map(|i| i + 1).collect();
    let mut right_bounds = vec![n; n];

    // Start with a good pivot: sum of middle elements
    let mut pivot = sorted_values[(n - 1) / 2] + sorted_values[n / 2];
    let mut active_set_size = total_pairs;
    let mut previous_count = 0;

    let mut rng = Rng::from_seed(hash_f64_slice(values));

    let mut partition_counts = vec![0; n];

    // Bound the selection loop. On valid sorted input the Monahan selection
    // converges in O(log n) iterations; this cap is far higher than ever
    // needed for sorted input but guarantees termination on misuse (e.g.,
    // assume_sorted=true on UNSORTED input, which is undefined behavior and
    // would otherwise spin forever). The cap scales with n so large valid
    // inputs are never starved. We also track no-progress (stall) on the
    // active set to bail out deterministically with a plain error (NOT an
    // assumption error).
    const BASE_ITERATIONS: usize = 256;
    const MAX_STALL: usize = 8;
    let max_iterations = BASE_ITERATIONS + 4 * n;
    let mut prev_active_set_size: i64 = -1;
    let mut stall_count: usize = 0;

    for _ in 0..max_iterations {
        // === PARTITION STEP ===
        let mut count_below_pivot = 0;
        let mut current_column = n;
        partition_counts.fill(0);

        for row in 1..=n {
            // Move left from current column until we find sums < pivot
            while current_column >= row
                && sorted_values[row - 1] + sorted_values[current_column - 1] >= pivot
            {
                current_column -= 1;
            }

            // Count elements in this row that are < pivot
            let elements_below = if current_column >= row {
                current_column - row + 1
            } else {
                0
            };
            partition_counts[row - 1] = elements_below;
            count_below_pivot += elements_below;
        }

        // === CONVERGENCE CHECK ===
        if count_below_pivot == previous_count {
            let mut min_active_sum = f64::INFINITY;
            let mut max_active_sum = f64::NEG_INFINITY;

            for i in 0..n {
                if left_bounds[i] > right_bounds[i] {
                    continue;
                }

                let row_value = sorted_values[i];
                let smallest_in_row = sorted_values[left_bounds[i] - 1] + row_value;
                let largest_in_row = sorted_values[right_bounds[i] - 1] + row_value;

                min_active_sum = min_active_sum.min(smallest_in_row);
                max_active_sum = max_active_sum.max(largest_in_row);
            }

            pivot = 0.5 * min_active_sum + 0.5 * max_active_sum;
            if pivot <= min_active_sum || pivot > max_active_sum {
                pivot = max_active_sum;
            }

            if min_active_sum == max_active_sum || active_set_size <= 2 {
                return Ok(pivot / 2.0);
            }

            continue;
        }

        // === TARGET CHECK ===
        let at_target_rank =
            count_below_pivot == median_rank_low || count_below_pivot == median_rank_high - 1;

        if at_target_rank {
            let mut largest_below_pivot = f64::NEG_INFINITY;
            let mut smallest_at_or_above_pivot = f64::INFINITY;

            for i in 0..n {
                let count_in_row = partition_counts[i];
                let row_value = sorted_values[i];
                let total_in_row = n - i;

                // Find largest sum in this row that's < pivot
                if count_in_row > 0 {
                    let last_below_index = i + count_in_row;
                    let last_below_value = row_value + sorted_values[last_below_index - 1];
                    largest_below_pivot = largest_below_pivot.max(last_below_value);
                }

                // Find smallest sum in this row that's >= pivot
                if count_in_row < total_in_row {
                    let first_at_or_above_index = i + count_in_row + 1;
                    let first_at_or_above_value =
                        row_value + sorted_values[first_at_or_above_index - 1];
                    smallest_at_or_above_pivot =
                        smallest_at_or_above_pivot.min(first_at_or_above_value);
                }
            }

            // Calculate final result
            if median_rank_low < median_rank_high {
                // Even total: average the two middle values
                return Ok((smallest_at_or_above_pivot + largest_below_pivot) / 4.0);
            } else {
                // Odd total: return the single middle value
                let need_largest = count_below_pivot == median_rank_low;
                return Ok(if need_largest {
                    largest_below_pivot
                } else {
                    smallest_at_or_above_pivot
                } / 2.0);
            }
        }

        // === UPDATE BOUNDS ===
        if count_below_pivot < median_rank_low {
            // Too few values below pivot - search higher
            for i in 0..n {
                left_bounds[i] = i + partition_counts[i] + 1;
            }
        } else {
            // Too many values below pivot - search lower
            for i in 0..n {
                right_bounds[i] = i + partition_counts[i];
            }
        }

        // === PREPARE NEXT ITERATION ===
        previous_count = count_below_pivot;

        // Recalculate active set size
        active_set_size = left_bounds
            .iter()
            .zip(right_bounds.iter())
            .map(|(l, r)| if r >= l { r - l + 1 } else { 0 })
            .sum();

        // Stall detection: on valid sorted input the active set strictly
        // shrinks toward the target. If it fails to shrink for several
        // consecutive iterations, the input is pathological (e.g.,
        // assume_sorted=true on unsorted data) and we bail deterministically.
        if active_set_size as i64 >= prev_active_set_size && prev_active_set_size >= 0 {
            stall_count += 1;
            if stall_count >= MAX_STALL {
                return Err("Convergence failure (pathological input)");
            }
        } else {
            stall_count = 0;
        }
        prev_active_set_size = active_set_size as i64;

        // Choose next pivot
        if active_set_size > 2 {
            // Use randomized row median strategy
            let target_index = rng.uniform_i64(0, active_set_size as i64) as usize;
            let mut cumulative_size = 0;
            let mut selected_row = 0;

            for i in 0..n {
                let row_size = if right_bounds[i] >= left_bounds[i] {
                    right_bounds[i] - left_bounds[i] + 1
                } else {
                    0
                };
                if target_index < cumulative_size + row_size {
                    selected_row = i;
                    break;
                }
                cumulative_size += row_size;
            }

            // Use median element of the selected row as pivot
            let median_column_in_row =
                usize::midpoint(left_bounds[selected_row], right_bounds[selected_row]);
            pivot = sorted_values[selected_row] + sorted_values[median_column_in_row - 1];
        } else {
            // Few elements remain - use midrange strategy
            let mut min_remaining_sum = f64::INFINITY;
            let mut max_remaining_sum = f64::NEG_INFINITY;

            for i in 0..n {
                if left_bounds[i] > right_bounds[i] {
                    continue;
                }

                let row_value = sorted_values[i];
                let min_in_row = sorted_values[left_bounds[i] - 1] + row_value;
                let max_in_row = sorted_values[right_bounds[i] - 1] + row_value;

                min_remaining_sum = min_remaining_sum.min(min_in_row);
                max_remaining_sum = max_remaining_sum.max(max_in_row);
            }

            pivot = 0.5 * min_remaining_sum + 0.5 * max_remaining_sum;
            if pivot <= min_remaining_sum || pivot > max_remaining_sum {
                pivot = max_remaining_sum;
            }

            if min_remaining_sum == max_remaining_sum {
                return Ok(pivot / 2.0);
            }
        }
    }

    Err("Convergence failure (pathological input)")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sorted_input_converges() {
        // Valid sorted input must converge well within the iteration cap.
        let values: Vec<f64> = (0..101).map(|i| i as f64).collect();
        let result = center_impl(&values, true).unwrap();
        assert!((result - 50.0).abs() < 1e-9);
    }

    #[test]
    fn unsorted_with_assume_sorted_does_not_hang() {
        // assume_sorted=true on UNSORTED input violates the contract (UB).
        // Without the iteration cap and stall detection, the Monahan selection
        // loop could spin forever on such input (an unkillable process wedge).
        // This test pins the guard: the call must terminate quickly with a
        // deterministic convergence-failure error rather than hang.
        //
        // This adversarial input is crafted to defeat the selection invariant so
        // the active set fails to shrink; we assert the bailout, not any value.
        let values = vec![
            0.0, 100.0, 1.0, 99.0, 2.0, 98.0, 3.0, 97.0, 50.0, 4.0, 96.0, 5.0, 95.0, 49.0, 51.0,
        ];
        let result = center_impl(&values, true);
        // The contract is "does not hang"; if the loop were uncapped this test
        // would never complete. Assert the bailout unconditionally.
        assert!(result.is_err());
        assert!(
            result.unwrap_err().contains("Convergence failure"),
            "expected convergence-failure error"
        );
    }

    #[test]
    fn pathological_unsorted_returns_convergence_error() {
        // A strongly anti-sorted sequence under assume_sorted=true. The stall
        // guard detects that the active set fails to shrink and bails out with
        // a deterministic convergence-failure error instead of spinning.
        let n = 64;
        let mut values = Vec::with_capacity(n);
        for i in 0..n {
            if i % 2 == 0 {
                values.push((n - i) as f64);
            } else {
                values.push(i as f64);
            }
        }
        let result = center_impl(&values, true);
        assert_eq!(result, Err("Convergence failure (pathological input)"));
    }
}
