#include <R.h>
#include <Rinternals.h>
#include <math.h>
#include <stdlib.h>

#define MIN(a, b) ((a) < (b) ? (a) : (b))
#define MAX(a, b) ((a) > (b) ? (a) : (b))

/*
 * Fast O(n log n) implementation of the Center (Hodges-Lehmann) estimator
 * Based on Monahan's Algorithm 616 (1984)
 * Computes the median of all pairwise averages efficiently
 */
SEXP fast_center_c(SEXP values_sexp) {
    // Input validation
    if (!isReal(values_sexp)) {
        error("Input must be a numeric vector");
    }

    int n = length(values_sexp);
    if (n == 0) {
        error("Input vector cannot be empty");
    }

    // Handle trivial cases
    if (n == 1) {
        return values_sexp;
    }

    if (n == 2) {
        double *values = REAL(values_sexp);
        SEXP result = PROTECT(allocVector(REALSXP, 1));
        REAL(result)[0] = (values[0] + values[1]) / 2.0;
        UNPROTECT(1);
        return result;
    }

    // Allocate working memory (sorted copy)
    double *sorted_values = (double *) R_alloc(n, sizeof(double));
    double *values = REAL(values_sexp);
    for (int i = 0; i < n; i++) {
        sorted_values[i] = values[i];
    }

    // Sort the values
    R_rsort(sorted_values, n);

    // Calculate target median rank(s)
    long long total_pairs = ((long long)n * (n + 1)) / 2;
    long long median_rank_low = (total_pairs + 1) / 2;
    long long median_rank_high = (total_pairs + 2) / 2;

    // Initialize search bounds (using 0-based indexing internally)
    long long *left_bounds = (long long *) R_alloc(n, sizeof(long long));
    long long *right_bounds = (long long *) R_alloc(n, sizeof(long long));
    long long *partition_counts = (long long *) R_alloc(n, sizeof(long long));

    for (int i = 0; i < n; i++) {
        left_bounds[i] = i;      // Row i can pair with columns [i..n-1]
        right_bounds[i] = n - 1; // Initially all columns available
    }

    // Initial pivot: sum of middle elements
    double pivot = sorted_values[(n - 1) / 2] + sorted_values[n / 2];
    long long active_set_size = total_pairs;
    long long previous_count = 0;

    // Random number generator for pivot selection
    GetRNGstate();

    while (1) {
        // === PARTITION STEP ===
        long long count_below_pivot = 0;
        long long current_column = n - 1;

        for (int row = 0; row < n; row++) {
            partition_counts[row] = 0;

            // Move left from current column until we find sums < pivot
            while (current_column >= row &&
                   sorted_values[row] + sorted_values[current_column] >= pivot) {
                current_column--;
            }

            // Count elements in this row that are < pivot
            if (current_column >= row) {
                long long elements_below = current_column - row + 1;
                partition_counts[row] = elements_below;
                count_below_pivot += elements_below;
            }
        }

        // === CONVERGENCE CHECK ===
        if (count_below_pivot == previous_count) {
            double min_active_sum = R_PosInf;
            double max_active_sum = R_NegInf;

            for (int i = 0; i < n; i++) {
                if (left_bounds[i] > right_bounds[i]) continue;

                double row_value = sorted_values[i];
                double smallest_in_row = sorted_values[left_bounds[i]] + row_value;
                double largest_in_row = sorted_values[right_bounds[i]] + row_value;

                min_active_sum = MIN(min_active_sum, smallest_in_row);
                max_active_sum = MAX(max_active_sum, largest_in_row);
            }

            pivot = (min_active_sum + max_active_sum) / 2.0;
            if (pivot <= min_active_sum || pivot > max_active_sum) {
                pivot = max_active_sum;
            }

            if (min_active_sum == max_active_sum || active_set_size <= 2) {
                PutRNGstate();
                SEXP result = PROTECT(allocVector(REALSXP, 1));
                REAL(result)[0] = pivot / 2.0;
                UNPROTECT(1);
                return result;
            }

            continue;
        }

        // === TARGET CHECK ===
        int at_target_rank = (count_below_pivot == median_rank_low) ||
                             (count_below_pivot == median_rank_high - 1);

        if (at_target_rank) {
            double largest_below_pivot = R_NegInf;
            double smallest_at_or_above_pivot = R_PosInf;

            for (int i = 0; i < n; i++) {
                long long count_in_row = partition_counts[i];
                double row_value = sorted_values[i];
                long long total_in_row = n - i;

                // Find largest sum in this row that's < pivot
                if (count_in_row > 0) {
                    long long last_below_index = i + count_in_row - 1;
                    double last_below_value = row_value + sorted_values[last_below_index];
                    largest_below_pivot = MAX(largest_below_pivot, last_below_value);
                }

                // Find smallest sum in this row that's >= pivot
                if (count_in_row < total_in_row) {
                    long long first_at_or_above_index = i + count_in_row;
                    double first_at_or_above_value = row_value + sorted_values[first_at_or_above_index];
                    smallest_at_or_above_pivot = MIN(smallest_at_or_above_pivot, first_at_or_above_value);
                }
            }

            PutRNGstate();
            SEXP result = PROTECT(allocVector(REALSXP, 1));

            if (median_rank_low < median_rank_high) {
                // Even total: average the two middle values
                REAL(result)[0] = (smallest_at_or_above_pivot + largest_below_pivot) / 4.0;
            } else {
                // Odd total: return the single middle value
                int need_largest = (count_below_pivot == median_rank_low);
                REAL(result)[0] = (need_largest ? largest_below_pivot : smallest_at_or_above_pivot) / 2.0;
            }

            UNPROTECT(1);
            return result;
        }

        // === UPDATE BOUNDS ===
        if (count_below_pivot < median_rank_low) {
            // Too few values below pivot - search higher
            for (int i = 0; i < n; i++) {
                left_bounds[i] = i + partition_counts[i];
            }
        } else {
            // Too many values below pivot - search lower
            for (int i = 0; i < n; i++) {
                right_bounds[i] = i + partition_counts[i] - 1;
            }
        }

        // === PREPARE NEXT ITERATION ===
        previous_count = count_below_pivot;

        // Recalculate active set size
        active_set_size = 0;
        for (int i = 0; i < n; i++) {
            long long row_size = right_bounds[i] - left_bounds[i] + 1;
            active_set_size += MAX(0, row_size);
        }

        // Choose next pivot
        if (active_set_size > 2) {
            // Use randomized row median strategy
            double random_fraction = unif_rand();
            long long target_index = (long long)(random_fraction * active_set_size);
            int selected_row = 0;

            long long cumulative_size = 0;
            for (int i = 0; i < n; i++) {
                long long row_size = MAX(0, right_bounds[i] - left_bounds[i] + 1);
                if (target_index < cumulative_size + row_size) {
                    selected_row = i;
                    break;
                }
                cumulative_size += row_size;
            }

            // Use median element of the selected row as pivot
            long long median_column_in_row = (left_bounds[selected_row] + right_bounds[selected_row]) / 2;
            pivot = sorted_values[selected_row] + sorted_values[median_column_in_row];

        } else {
            // Few elements remain - use midrange strategy
            double min_remaining_sum = R_PosInf;
            double max_remaining_sum = R_NegInf;

            for (int i = 0; i < n; i++) {
                if (left_bounds[i] > right_bounds[i]) continue;

                double row_value = sorted_values[i];
                double min_in_row = sorted_values[left_bounds[i]] + row_value;
                double max_in_row = sorted_values[right_bounds[i]] + row_value;

                min_remaining_sum = MIN(min_remaining_sum, min_in_row);
                max_remaining_sum = MAX(max_remaining_sum, max_in_row);
            }

            pivot = (min_remaining_sum + max_remaining_sum) / 2.0;
            if (pivot <= min_remaining_sum || pivot > max_remaining_sum) {
                pivot = max_remaining_sum;
            }

            if (min_remaining_sum == max_remaining_sum) {
                PutRNGstate();
                SEXP result = PROTECT(allocVector(REALSXP, 1));
                REAL(result)[0] = pivot / 2.0;
                UNPROTECT(1);
                return result;
            }
        }
    }

    // Should never reach here
    PutRNGstate();
    error("Algorithm failed to converge");
    return R_NilValue;
}
