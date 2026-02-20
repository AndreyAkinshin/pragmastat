#include <R.h>
#include <Rinternals.h>
#include <math.h>
#include <stdlib.h>

#define MIN(a, b) ((a) < (b) ? (a) : (b))
#define MAX(a, b) ((a) > (b) ? (a) : (b))
#define ABS(a) ((a) < 0 ? -(a) : (a))

/*
 * Fast O(n log n) implementation of the Spread (Shamos) estimator
 * Computes the median of all pairwise absolute differences efficiently
 */
SEXP fast_spread_c(SEXP values_sexp) {
    // Input validation
    if (!isReal(values_sexp)) {
        error("Input must be a numeric vector");
    }

    int n = length(values_sexp);
    if (n <= 1) {
        SEXP result = PROTECT(allocVector(REALSXP, 1));
        REAL(result)[0] = 0.0;
        UNPROTECT(1);
        return result;
    }

    if (n == 2) {
        double *values = REAL(values_sexp);
        SEXP result = PROTECT(allocVector(REALSXP, 1));
        REAL(result)[0] = fabs(values[1] - values[0]);
        UNPROTECT(1);
        return result;
    }

    // Allocate and sort working copy
    double *a = (double *) R_alloc(n, sizeof(double));
    double *values = REAL(values_sexp);
    for (int i = 0; i < n; i++) {
        a[i] = values[i];
    }
    R_rsort(a, n);

    // Total number of pairwise differences with i < j
    long long N = ((long long)n * (n - 1)) / 2;
    long long k_low = (N + 1) / 2;
    long long k_high = (N + 2) / 2;

    // Per-row active bounds (0-based indexing)
    int *L = (int *) R_alloc(n, sizeof(int));
    int *R_bounds = (int *) R_alloc(n, sizeof(int));
    long long *row_counts = (long long *) R_alloc(n, sizeof(long long));

    for (int i = 0; i < n; i++) {
        L[i] = i + 1;      // Row i allows columns [i+1, n-1]
        R_bounds[i] = n - 1;
        if (L[i] > R_bounds[i]) {
            L[i] = 1;
            R_bounds[i] = 0; // mark empty
        }
    }

    // Initial pivot: a central gap
    double pivot = a[n / 2] - a[(n - 1) / 2];
    long long prev_count_below = -1;

    while (1) {
        // === PARTITION: count how many differences are < pivot ===
        long long count_below = 0;
        double largest_below = R_NegInf;
        double smallest_at_or_above = R_PosInf;

        int j = 1; // global two-pointer (0-based)
        for (int i = 0; i < n - 1; i++) {
            if (j < i + 1) j = i + 1;
            while (j < n && a[j] - a[i] < pivot) j++;

            long long cnt_row = j - (i + 1);
            if (cnt_row < 0) cnt_row = 0;
            row_counts[i] = cnt_row;
            count_below += cnt_row;

            // Boundary elements for this row
            if (cnt_row > 0) {
                double cand_below = a[j - 1] - a[i];
                if (cand_below > largest_below) largest_below = cand_below;
            }

            if (j < n) {
                double cand_at_or_above = a[j] - a[i];
                if (cand_at_or_above < smallest_at_or_above) {
                    smallest_at_or_above = cand_at_or_above;
                }
            }
        }

        // === TARGET CHECK ===
        int at_target = (count_below == k_low) || (count_below == k_high - 1);

        if (at_target) {
            SEXP result = PROTECT(allocVector(REALSXP, 1));

            if (k_low < k_high) {
                // Even N: average the two central order stats
                REAL(result)[0] = 0.5 * (largest_below + smallest_at_or_above);
            } else {
                // Odd N: pick the single middle
                int need_largest = (count_below == k_low);
                REAL(result)[0] = need_largest ? largest_below : smallest_at_or_above;
            }

            UNPROTECT(1);
            return result;
        }

        // === STALL HANDLING ===
        if (count_below == prev_count_below) {
            double min_active = R_PosInf;
            double max_active = R_NegInf;
            long long active = 0;

            for (int i = 0; i < n - 1; i++) {
                int Li = L[i];
                int Ri = R_bounds[i];
                if (Li > Ri) continue;

                double row_min = a[Li] - a[i];
                double row_max = a[Ri] - a[i];
                if (row_min < min_active) min_active = row_min;
                if (row_max > max_active) max_active = row_max;
                active += (Ri - Li + 1);
            }

            if (active <= 0) {
                SEXP result = PROTECT(allocVector(REALSXP, 1));
                if (k_low < k_high) {
                    REAL(result)[0] = 0.5 * (largest_below + smallest_at_or_above);
                } else {
                    REAL(result)[0] = (count_below >= k_low) ? largest_below : smallest_at_or_above;
                }
                UNPROTECT(1);
                return result;
            }

            if (max_active <= min_active) {
                SEXP result = PROTECT(allocVector(REALSXP, 1));
                REAL(result)[0] = min_active;
                UNPROTECT(1);
                return result;
            }

            double mid = 0.5 * (min_active + max_active);
            pivot = (mid > min_active && mid <= max_active) ? mid : max_active;
            prev_count_below = count_below;
            continue;
        }

        // === SHRINK ACTIVE WINDOW ===
        if (count_below < k_low) {
            // Need larger differences: discard all strictly below pivot
            for (int i = 0; i < n - 1; i++) {
                int new_L = i + 1 + (int)row_counts[i];
                if (new_L > L[i]) L[i] = new_L;
                if (L[i] > R_bounds[i]) {
                    L[i] = 1;
                    R_bounds[i] = 0; // mark empty
                }
            }
        } else {
            // Too many below: keep only those strictly below pivot
            for (int i = 0; i < n - 1; i++) {
                int new_R = i + (int)row_counts[i];
                if (new_R < R_bounds[i]) R_bounds[i] = new_R;
                if (R_bounds[i] < i + 1) {
                    L[i] = 1;
                    R_bounds[i] = 0; // empty row
                }
            }
        }

        prev_count_below = count_below;

        // === CHOOSE NEXT PIVOT FROM ACTIVE SET ===
        long long active_size = 0;
        for (int i = 0; i < n - 1; i++) {
            if (L[i] <= R_bounds[i]) {
                active_size += (R_bounds[i] - L[i] + 1);
            }
        }

        if (active_size <= 2) {
            // Few candidates left: return midrange of remaining
            double min_rem = R_PosInf;
            double max_rem = R_NegInf;

            for (int i = 0; i < n - 1; i++) {
                if (L[i] > R_bounds[i]) continue;
                double lo = a[L[i]] - a[i];
                double hi = a[R_bounds[i]] - a[i];
                if (lo < min_rem) min_rem = lo;
                if (hi > max_rem) max_rem = hi;
            }

            if (active_size <= 0) {
                SEXP result = PROTECT(allocVector(REALSXP, 1));
                if (k_low < k_high) {
                    REAL(result)[0] = 0.5 * (largest_below + smallest_at_or_above);
                } else {
                    REAL(result)[0] = (count_below >= k_low) ? largest_below : smallest_at_or_above;
                }
                UNPROTECT(1);
                return result;
            }

            SEXP result = PROTECT(allocVector(REALSXP, 1));
            if (k_low < k_high) {
                REAL(result)[0] = 0.5 * (min_rem + max_rem);
            } else {
                long long dist_low = llabs((k_low - 1) - count_below);
                long long dist_high = llabs(count_below - k_low);
                REAL(result)[0] = (dist_low <= dist_high) ? min_rem : max_rem;
            }
            UNPROTECT(1);
            return result;

        } else {
            // Deterministic middle-element selection
            long long t = active_size / 2;
            long long acc = 0;
            int row = 0;

            for (int r = 0; r < n - 1; r++) {
                if (L[r] > R_bounds[r]) continue;
                long long size = R_bounds[r] - L[r] + 1;
                if (t < acc + size) {
                    row = r;
                    break;
                }
                acc += size;
            }

            // Median column of the selected row
            int col = (L[row] + R_bounds[row]) / 2;
            pivot = a[col] - a[row];
        }
    }

    // Should never reach here
    error("Algorithm failed to converge");
    return R_NilValue;
}
