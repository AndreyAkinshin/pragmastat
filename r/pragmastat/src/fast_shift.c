#include <R.h>
#include <Rinternals.h>
#include <math.h>
#include <stdlib.h>

#define MIN(a, b) ((a) < (b) ? (a) : (b))
#define MAX(a, b) ((a) > (b) ? (a) : (b))

static inline double midpoint(double a, double b) {
    return a + (b - a) * 0.5;
}

/*
 * Two-pointer algorithm: counts pairs where x[i] - y[j] <= threshold,
 * and tracks the closest actual differences on either side of threshold.
 */
static void count_and_neighbors(
    const double *x, int m,
    const double *y, int n,
    double threshold,
    long long *count_less_or_equal,
    double *closest_below,
    double *closest_above)
{
    long long count = 0;
    double max_below = R_NegInf;
    double min_above = R_PosInf;

    int j = 0;
    for (int i = 0; i < m; i++) {
        // Find the first y[j] where x[i] - y[j] <= threshold
        while (j < n && x[i] - y[j] > threshold) {
            j++;
        }

        // All pairs (x[i], y[k]) for k >= j satisfy x[i] - y[k] <= threshold
        count += (n - j);

        // Track boundaries
        if (j < n) {
            double diff = x[i] - y[j];
            if (diff > max_below) max_below = diff;
        }

        if (j > 0) {
            double diff = x[i] - y[j - 1];
            if (diff < min_above) min_above = diff;
        }
    }

    // Fallback to actual min/max if no boundaries found
    if (isinf(max_below) && max_below < 0) {
        max_below = x[0] - y[n - 1];
    }
    if (isinf(min_above) && min_above > 0) {
        min_above = x[m - 1] - y[0];
    }

    *count_less_or_equal = count;
    *closest_below = max_below;
    *closest_above = min_above;
}

/*
 * Binary search to find the k-th smallest pairwise difference.
 * Returns the actual difference value at rank k (1-indexed).
 */
static double select_kth_pairwise_diff(
    const double *x, int m,
    const double *y, int n,
    long long k)
{
    long long total = (long long)m * n;

    if (k < 1 || k > total) {
        error("k must be between 1 and m*n");
    }

    double search_min = x[0] - y[n - 1];
    double search_max = x[m - 1] - y[0];

    if (ISNAN(search_min) || ISNAN(search_max)) {
        error("NaN in input values");
    }

    const int max_iterations = 128;
    double prev_min = R_NegInf;
    double prev_max = R_PosInf;

    for (int iter = 0; iter < max_iterations && search_min != search_max; iter++) {
        double mid = midpoint(search_min, search_max);
        long long count_le;
        double closest_below, closest_above;

        count_and_neighbors(x, m, y, n, mid,
                          &count_le, &closest_below, &closest_above);

        if (closest_below == closest_above) {
            return closest_below;
        }

        // No progress means we're stuck between two discrete values
        if (search_min == prev_min && search_max == prev_max) {
            return count_le >= k ? closest_below : closest_above;
        }

        prev_min = search_min;
        prev_max = search_max;

        if (count_le >= k) {
            search_max = closest_below;
        } else {
            search_min = closest_above;
        }
    }

    if (search_min != search_max) {
        error("Convergence failure in binary search");
    }

    return search_min;
}

/*
 * Computes quantiles of all pairwise differences { x_i - y_j }.
 * Time: O((m + n) * log(precision)) per quantile. Space: O(1).
 *
 * @param x_sexp Numeric vector (will be sorted if needed)
 * @param y_sexp Numeric vector (will be sorted if needed)
 * @param p_sexp Numeric vector of probabilities in [0, 1]
 * @param assume_sorted_sexp Logical: if TRUE, assume x and y are already sorted
 * @return Numeric vector of quantile values
 */
SEXP fast_shift_c(SEXP x_sexp, SEXP y_sexp, SEXP p_sexp, SEXP assume_sorted_sexp) {
    // Input validation
    if (!isReal(x_sexp) || !isReal(y_sexp) || !isReal(p_sexp)) {
        error("x, y, and p must be numeric vectors");
    }
    if (!isLogical(assume_sorted_sexp)) {
        error("assume_sorted must be logical");
    }

    int m = length(x_sexp);
    int n = length(y_sexp);
    int np = length(p_sexp);

    if (m == 0 || n == 0) {
        error("x and y must be non-empty");
    }

    double *p = REAL(p_sexp);
    for (int i = 0; i < np; i++) {
        if (ISNAN(p[i]) || p[i] < 0.0 || p[i] > 1.0) {
            error("Probabilities must be within [0, 1]");
        }
    }

    int assume_sorted = asLogical(assume_sorted_sexp);

    // Prepare sorted arrays
    double *xs = (double *) R_alloc(m, sizeof(double));
    double *ys = (double *) R_alloc(n, sizeof(double));

    double *x_vals = REAL(x_sexp);
    double *y_vals = REAL(y_sexp);

    for (int i = 0; i < m; i++) xs[i] = x_vals[i];
    for (int i = 0; i < n; i++) ys[i] = y_vals[i];

    if (!assume_sorted) {
        R_rsort(xs, m);
        R_rsort(ys, n);
    }

    long long total = (long long)m * n;

    // Compute Type-7 quantile parameters for each probability
    typedef struct {
        long long lower_rank;
        long long upper_rank;
        double weight;
    } QuantileParams;

    QuantileParams *params = (QuantileParams *) R_alloc(np, sizeof(QuantileParams));

    // Track unique ranks we need to compute
    int *required_ranks = (int *) R_alloc(2 * np, sizeof(int));
    double *rank_values = (double *) R_alloc(2 * np, sizeof(double));
    int n_ranks = 0;

    for (int i = 0; i < np; i++) {
        // Type-7 quantile: h = 1 + (n-1)*p
        double h = 1.0 + (total - 1) * p[i];
        long long lower_rank = (long long)floor(h);
        long long upper_rank = (long long)ceil(h);
        double weight = h - lower_rank;

        if (lower_rank < 1) lower_rank = 1;
        if (upper_rank > total) upper_rank = total;

        params[i].lower_rank = lower_rank;
        params[i].upper_rank = upper_rank;
        params[i].weight = weight;

        // Add to required ranks if not already present
        int found_lower = 0, found_upper = 0;
        for (int j = 0; j < n_ranks; j++) {
            if (required_ranks[j] == lower_rank) found_lower = 1;
            if (required_ranks[j] == upper_rank) found_upper = 1;
        }
        if (!found_lower) {
            required_ranks[n_ranks++] = lower_rank;
        }
        if (!found_upper && upper_rank != lower_rank) {
            required_ranks[n_ranks++] = upper_rank;
        }
    }

    // Compute values for required ranks
    for (int i = 0; i < n_ranks; i++) {
        rank_values[i] = select_kth_pairwise_diff(xs, m, ys, n, required_ranks[i]);
    }

    // Interpolate to get final quantiles
    SEXP result = PROTECT(allocVector(REALSXP, np));
    double *result_ptr = REAL(result);

    for (int i = 0; i < np; i++) {
        long long lower_rank = params[i].lower_rank;
        long long upper_rank = params[i].upper_rank;
        double weight = params[i].weight;

        // Find values for lower and upper ranks
        double lower_val = 0.0, upper_val = 0.0;
        for (int j = 0; j < n_ranks; j++) {
            if (required_ranks[j] == lower_rank) lower_val = rank_values[j];
            if (required_ranks[j] == upper_rank) upper_val = rank_values[j];
        }

        result_ptr[i] = (weight == 0.0) ? lower_val : (1.0 - weight) * lower_val + weight * upper_val;
    }

    UNPROTECT(1);
    return result;
}
