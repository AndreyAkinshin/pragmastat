#include <R.h>
#include <Rinternals.h>
#include <math.h>
#include <stdlib.h>
#include <string.h>
#include "center_impl.h"

#define MIN(a, b) ((a) < (b) ? (a) : (b))
#define MAX(a, b) ((a) > (b) ? (a) : (b))

static inline double midpoint_fc(double a, double b) {
    return 0.5 * a + 0.5 * b;
}

static int cmp_double_fc(const void *a, const void *b) {
    double da = *(const double *)a;
    double db = *(const double *)b;
    if (da < db) return -1;
    if (da > db) return 1;
    return 0;
}

/*
 * Core computation: Center (Hodges-Lehmann) estimator.
 * Uses Monahan's Algorithm 616 with deterministic pivot selection.
 * Allocates and frees its own working memory.
 */
double center_impl_compute(const double *values, int n, int assume_sorted) {
    if (n == 1) return values[0];
    if (n == 2) return midpoint_fc(values[0], values[1]);

    /* Use input directly when sorted; otherwise sort a copy */
    double *sorted_values;
    int allocated_sorted = 0;
    if (assume_sorted) {
        sorted_values = (double *)values;
    } else {
        sorted_values = (double *)malloc(n * sizeof(double));
        if (!sorted_values) {
            error("center_impl: memory allocation failed");
        }
        memcpy(sorted_values, values, n * sizeof(double));
        qsort(sorted_values, n, sizeof(double), cmp_double_fc);
        allocated_sorted = 1;
    }

    /* Calculate target median rank(s) */
    long long total_pairs = ((long long)n * (n + 1)) / 2;
    long long median_rank_low = (total_pairs + 1) / 2;
    long long median_rank_high = (total_pairs + 2) / 2;

    /* Initialize search bounds */
    long long *left_bounds = (long long *)malloc(n * sizeof(long long));
    if (!left_bounds) {
        if (allocated_sorted) free(sorted_values);
        error("center_impl: memory allocation failed");
    }
    long long *right_bounds = (long long *)malloc(n * sizeof(long long));
    if (!right_bounds) {
        if (allocated_sorted) free(sorted_values);
        free(left_bounds);
        error("center_impl: memory allocation failed");
    }
    long long *partition_counts = (long long *)malloc(n * sizeof(long long));
    if (!partition_counts) {
        if (allocated_sorted) free(sorted_values);
        free(left_bounds);
        free(right_bounds);
        error("center_impl: memory allocation failed");
    }

    for (int i = 0; i < n; i++) {
        left_bounds[i] = i;
        right_bounds[i] = n - 1;
    }

    double pivot = sorted_values[(n - 1) / 2] + sorted_values[n / 2];
    long long active_set_size = total_pairs;
    long long previous_count = 0;

    double result = 0.0;

    /*
     * Bound the selection loop. On valid sorted input the Monahan selection
     * converges in O(log n) iterations; this cap is far higher than ever
     * needed for sorted input but guarantees termination on misuse (e.g.,
     * assume_sorted=TRUE on UNSORTED input, which is undefined behavior and
     * would otherwise wedge the process in an unkillable infinite loop inside
     * the C extension). The cap scales with n so large valid inputs are never
     * starved, and it counts EVERY pass through the loop, so it also
     * terminates the convergence-check ping-pong mode. We additionally track
     * no-progress (stall) on the active set to bail out deterministically.
     */
    const int base_iterations = 256;
    const int max_iterations = base_iterations + 4 * n;
    long long previous_active_set_size = -1;
    int stall_count = 0;
    const int max_stall = 8;

    for (int iter = 0; iter < max_iterations; iter++) {
        /* === PARTITION STEP === */
        long long count_below_pivot = 0;
        long long current_column = n - 1;

        for (int row = 0; row < n; row++) {
            partition_counts[row] = 0;

            while (current_column >= row &&
                   sorted_values[row] + sorted_values[current_column] >= pivot) {
                current_column--;
            }

            if (current_column >= row) {
                long long elements_below = current_column - row + 1;
                partition_counts[row] = elements_below;
                count_below_pivot += elements_below;
            }
        }

        /* === CONVERGENCE CHECK === */
        if (count_below_pivot == previous_count) {
            double min_active_sum = INFINITY;
            double max_active_sum = -INFINITY;

            for (int i = 0; i < n; i++) {
                if (left_bounds[i] > right_bounds[i]) continue;

                double row_value = sorted_values[i];
                double smallest_in_row = sorted_values[left_bounds[i]] + row_value;
                double largest_in_row = sorted_values[right_bounds[i]] + row_value;

                if (smallest_in_row < min_active_sum) min_active_sum = smallest_in_row;
                if (largest_in_row > max_active_sum) max_active_sum = largest_in_row;
            }

            pivot = 0.5 * min_active_sum + 0.5 * max_active_sum;
            if (pivot <= min_active_sum || pivot > max_active_sum) {
                pivot = max_active_sum;
            }

            if (min_active_sum == max_active_sum || active_set_size <= 2) {
                result = pivot / 2.0;
                goto cleanup;
            }

            continue;
        }

        /* === TARGET CHECK === */
        int at_target_rank = (count_below_pivot == median_rank_low) ||
                             (count_below_pivot == median_rank_high - 1);

        if (at_target_rank) {
            double largest_below_pivot = -INFINITY;
            double smallest_at_or_above_pivot = INFINITY;

            for (int i = 0; i < n; i++) {
                long long count_in_row = partition_counts[i];
                double row_value = sorted_values[i];
                long long total_in_row = n - i;

                if (count_in_row > 0) {
                    long long last_below_index = i + count_in_row - 1;
                    double last_below_value = row_value + sorted_values[last_below_index];
                    if (last_below_value > largest_below_pivot)
                        largest_below_pivot = last_below_value;
                }

                if (count_in_row < total_in_row) {
                    long long first_at_or_above_index = i + count_in_row;
                    double first_at_or_above_value = row_value + sorted_values[first_at_or_above_index];
                    if (first_at_or_above_value < smallest_at_or_above_pivot)
                        smallest_at_or_above_pivot = first_at_or_above_value;
                }
            }

            if (median_rank_low < median_rank_high) {
                result = (smallest_at_or_above_pivot + largest_below_pivot) / 4.0;
            } else {
                int need_largest = (count_below_pivot == median_rank_low);
                result = (need_largest ? largest_below_pivot : smallest_at_or_above_pivot) / 2.0;
            }
            goto cleanup;
        }

        /* === UPDATE BOUNDS === */
        if (count_below_pivot < median_rank_low) {
            for (int i = 0; i < n; i++) {
                left_bounds[i] = i + partition_counts[i];
            }
        } else {
            for (int i = 0; i < n; i++) {
                right_bounds[i] = i + partition_counts[i] - 1;
            }
        }

        /* === PREPARE NEXT ITERATION === */
        previous_count = count_below_pivot;

        active_set_size = 0;
        for (int i = 0; i < n; i++) {
            long long row_size = right_bounds[i] - left_bounds[i] + 1;
            active_set_size += MAX(0, row_size);
        }

        /*
         * Stall detection: on valid sorted input the active set strictly
         * shrinks toward the target. If it fails to shrink for several
         * consecutive iterations, the input is pathological (e.g.,
         * assume_sorted=TRUE on unsorted data) and we bail deterministically.
         */
        if (active_set_size >= previous_active_set_size && previous_active_set_size >= 0) {
            if (++stall_count >= max_stall) {
                break;
            }
        } else {
            stall_count = 0;
        }
        previous_active_set_size = active_set_size;

        /* Choose next pivot */
        if (active_set_size > 2) {
            /* Deterministic pivot: pick middle element of active set */
            long long target_index = active_set_size / 2;
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

            long long median_column_in_row = (left_bounds[selected_row] + right_bounds[selected_row]) / 2;
            pivot = sorted_values[selected_row] + sorted_values[median_column_in_row];
        } else {
            double min_remaining_sum = INFINITY;
            double max_remaining_sum = -INFINITY;

            for (int i = 0; i < n; i++) {
                if (left_bounds[i] > right_bounds[i]) continue;

                double row_value = sorted_values[i];
                double min_in_row = sorted_values[left_bounds[i]] + row_value;
                double max_in_row = sorted_values[right_bounds[i]] + row_value;

                if (min_in_row < min_remaining_sum) min_remaining_sum = min_in_row;
                if (max_in_row > max_remaining_sum) max_remaining_sum = max_in_row;
            }

            pivot = 0.5 * min_remaining_sum + 0.5 * max_remaining_sum;
            if (pivot <= min_remaining_sum || pivot > max_remaining_sum) {
                pivot = max_remaining_sum;
            }

            if (min_remaining_sum == max_remaining_sum) {
                result = pivot / 2.0;
                goto cleanup;
            }
        }
    }

    /* Non-convergence: iteration cap reached or the stall guard tripped. */
    if (allocated_sorted) free(sorted_values);
    free(left_bounds);
    free(right_bounds);
    free(partition_counts);
    error("Convergence failure (pathological input)");

cleanup:
    if (allocated_sorted) free(sorted_values);
    free(left_bounds);
    free(right_bounds);
    free(partition_counts);
    return result;
}

/*
 * R-callable wrapper for center_impl_compute.
 */
SEXP center_impl_c(SEXP values_sexp, SEXP assume_sorted_sexp) {
    if (!isReal(values_sexp)) {
        error("Input must be a numeric vector");
    }

    int n = length(values_sexp);
    if (n == 0) {
        error("Input vector cannot be empty");
    }

    int assume_sorted = asLogical(assume_sorted_sexp);
    double result = center_impl_compute(REAL(values_sexp), n, assume_sorted);

    SEXP result_sexp = PROTECT(allocVector(REALSXP, 1));
    REAL(result_sexp)[0] = result;
    UNPROTECT(1);
    return result_sexp;
}
