#include <R.h>
#include <Rinternals.h>

/*
 * Exact binomial coefficient C(n, k) via 64-bit integer arithmetic,
 * mirroring the go (int64) and rust (u128) ports. R's built-in choose()
 * runs in floating point and is provably inexact even below 2^53 (e.g.
 * choose(56, 27) yields ...078 instead of the exact ...080), which made
 * R's pairwise margin and min-misrate disagree with the other six ports.
 *
 * Used only for the exact-distribution branch (n < 62). There the running
 * product stays below 2^63, so unsigned 64-bit arithmetic is exact; the
 * final cast to double matches float64(int64) in go and (u128 as f64) in rust.
 */
SEXP binomial_coefficient_c(SEXP n_sexp, SEXP k_sexp) {
    int n = asInteger(n_sexp);
    int k = asInteger(k_sexp);
    if (k < 0 || k > n) return ScalarReal(0.0);
    if (k > n - k) k = n - k;
    unsigned long long result = 1ULL;
    for (int i = 0; i < k; i++) {
        result = result * (unsigned long long)(n - i) / (unsigned long long)(i + 1);
    }
    return ScalarReal((double)result);
}
