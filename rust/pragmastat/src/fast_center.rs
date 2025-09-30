/// Fast O(n log n) implementation of the Center (Hodges-Lehmann) estimator.
/// Based on Monahan's Algorithm 616 (1984).
///
/// Internal implementation - not part of public API.
use rand::Rng;

pub(crate) fn fast_center(values: &[f64]) -> Result<f64, &'static str> {
    let n = values.len();
    if n == 0 {
        return Err("Input slice cannot be empty");
    }
    if n == 1 {
        return Ok(values[0]);
    }
    if n == 2 {
        return Ok((values[0] + values[1]) / 2.0);
    }

    // Sort the values
    let mut sorted_values = values.to_vec();
    sorted_values.sort_by(|a, b| a.partial_cmp(b).unwrap());

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

    let mut rng = rand::thread_rng();

    loop {
        // === PARTITION STEP ===
        let mut count_below_pivot = 0;
        let mut current_column = n;
        let mut partition_counts = vec![0; n];

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

            pivot = (min_active_sum + max_active_sum) / 2.0;
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

        // Choose next pivot
        if active_set_size > 2 {
            // Use randomized row median strategy
            let target_index = rng.gen_range(0..active_set_size);
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
            let median_column_in_row = (left_bounds[selected_row] + right_bounds[selected_row]) / 2;
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

            pivot = (min_remaining_sum + max_remaining_sum) / 2.0;
            if pivot <= min_remaining_sum || pivot > max_remaining_sum {
                pivot = max_remaining_sum;
            }

            if min_remaining_sum == max_remaining_sum {
                return Ok(pivot / 2.0);
            }
        }
    }
}
