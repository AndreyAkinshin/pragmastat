#include <R.h>
#include <Rinternals.h>

/*
 * Exact Wilcoxon signed-rank margin via 64-bit integer arithmetic, mirroring the
 * uint64 counters of the go, rust, kotlin and csharp ports, the BigInt of the
 * typescript one and the unbounded int of python.
 *
 * R has no 64-bit integer, so this used to run in a double vector. The counts
 * themselves fit: the largest is 4.5e14 at n = 57, well inside the 2^53 that a
 * double represents exactly. The CUMULATIVE sum does not. It climbs to 2^(n-1),
 * and the comparison it feeds is against p, so the lost bits decide an integer.
 *
 * They did. The signed-rank distribution is symmetric, so where max_w is odd the
 * cumulative at the midpoint is exactly half the total, and `cdf >= p` at p = 1/2
 * is then an exact equality. Accumulated in doubles it comes out a hair below:
 * signed_rank_margin(57, 1) returned 1654 in R against 1652 in the other six, in
 * a suite tests/manifest.json declares exact.
 *
 * This is the same answer binomial_coefficient_c gives to the same question, for
 * the same reason: the quantity is an integer by definition, so accumulate it as
 * one and convert once at the end. The division below is the single conversion,
 * matching float64(cumulative) / float64(total) in go.
 */
SEXP signed_rank_margin_exact_raw_c(SEXP n_sexp, SEXP p_sexp) {
    int n = asInteger(n_sexp);
    double p = asReal(p_sexp);

    /* 2^n must fit an unsigned 64-bit integer, which is what bounds n at 63. */
    if (n <= 0 || n > 63) return ScalarInteger(0);

    long long max_w = (long long)n * (n + 1) / 2;

    unsigned long long *count =
        (unsigned long long *)R_alloc((size_t)max_w + 1, sizeof(unsigned long long));
    for (long long w = 0; w <= max_w; w++) count[w] = 0ULL;
    count[0] = 1ULL;

    for (int i = 1; i <= n; i++) {
        long long max_wi = (long long)i * (i + 1) / 2;
        if (max_wi > max_w) max_wi = max_w;
        for (long long w = max_wi; w >= i; w--) {
            count[w] += count[w - i];
        }
    }

    unsigned long long total = 1ULL << n;
    unsigned long long cumulative = 0ULL;
    for (long long w = 0; w <= max_w; w++) {
        cumulative += count[w];
        if ((double)cumulative / (double)total >= p) {
            return ScalarInteger((int)w);
        }
    }

    return ScalarInteger((int)max_w);
}
