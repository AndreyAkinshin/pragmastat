#include <R.h>
#include <Rinternals.h>
#include <stdint.h>
#include <string.h>
#include <stdlib.h>
#include "fast_center_impl.h"

/* ========================================================================
 * xoshiro256++ PRNG
 * Reference: https://prng.di.unimi.it/xoshiro256plusplus.c
 * ======================================================================== */

static inline uint64_t rotl64(uint64_t x, int k) {
    return (x << k) | (x >> (64 - k));
}

static uint64_t xoshiro256pp_next(uint64_t s[4]) {
    uint64_t result = rotl64(s[0] + s[3], 23) + s[0];
    uint64_t t = s[1] << 17;
    s[2] ^= s[0];
    s[3] ^= s[1];
    s[1] ^= s[2];
    s[0] ^= s[3];
    s[2] ^= t;
    s[3] = rotl64(s[3], 45);
    return result;
}

/* ========================================================================
 * SplitMix64 for seed expansion
 * ======================================================================== */

static uint64_t splitmix64_next(uint64_t *state) {
    *state += UINT64_C(0x9e3779b97f4a7c15);
    uint64_t z = *state;
    z = (z ^ (z >> 30)) * UINT64_C(0xbf58476d1ce4e5b9);
    z = (z ^ (z >> 27)) * UINT64_C(0x94d049bb133111eb);
    return z ^ (z >> 31);
}

static void xoshiro256pp_seed(uint64_t s[4], uint64_t seed) {
    uint64_t sm = seed;
    s[0] = splitmix64_next(&sm);
    s[1] = splitmix64_next(&sm);
    s[2] = splitmix64_next(&sm);
    s[3] = splitmix64_next(&sm);
}

/* ========================================================================
 * FNV-1a hash
 * ======================================================================== */

static uint64_t fnv1a_hash(const char *data, size_t len) {
    uint64_t hash = UINT64_C(0xcbf29ce484222325);
    for (size_t i = 0; i < len; i++) {
        hash ^= (uint64_t)(unsigned char)data[i];
        hash *= UINT64_C(0x100000001b3);
    }
    return hash;
}

/* ========================================================================
 * Comparison function for qsort
 * ======================================================================== */

static int cmp_double_bootstrap(const void *a, const void *b) {
    double da = *(const double *)a;
    double db = *(const double *)b;
    if (da < db) return -1;
    if (da > db) return 1;
    return 0;
}

/* ========================================================================
 * Bootstrap function for center_bounds_approx
 *
 * Performs the full bootstrap loop in C:
 * - Initializes xoshiro256++ from seed string
 * - For each iteration: resamples with replacement, computes fast_center
 * - Returns sorted vector of bootstrap center estimates
 *
 * Arguments:
 *   sorted_x_sexp    - REALSXP: sorted input values
 *   m_sexp           - INTSXP: resample size (min(n, max_subsample))
 *   iterations_sexp  - INTSXP: number of bootstrap iterations
 *   seed_sexp        - STRSXP: seed string for RNG
 *
 * Returns: REALSXP vector of sorted bootstrap centers
 * ======================================================================== */

SEXP center_bounds_approx_bootstrap_c(SEXP sorted_x_sexp, SEXP m_sexp,
                                       SEXP iterations_sexp, SEXP seed_sexp) {
    double *sorted_x = REAL(sorted_x_sexp);
    int n = length(sorted_x_sexp);
    int m = INTEGER(m_sexp)[0];
    int iterations = INTEGER(iterations_sexp)[0];

    /* Initialize RNG from seed string */
    const char *seed_str = CHAR(STRING_ELT(seed_sexp, 0));
    uint64_t seed_hash = fnv1a_hash(seed_str, strlen(seed_str));
    uint64_t rng_state[4];
    xoshiro256pp_seed(rng_state, seed_hash);

    /* Allocate result vector */
    SEXP result = PROTECT(allocVector(REALSXP, iterations));
    double *centers = REAL(result);

    /* Working buffer for resampled values */
    double *resample_buf = (double *)malloc(m * sizeof(double));
    if (!resample_buf) {
        UNPROTECT(1);
        error("Failed to allocate resample buffer");
    }

    /* Bootstrap loop */
    for (int iter = 0; iter < iterations; iter++) {
        /* Resample with replacement using modulo reduction.
         * Bias: at most 2^-55 for n < 512. Acceptable for bootstrap sampling;
         * matches the modulo-based uniform_int used in TS and Python.
         * Note: directly calls xoshiro256pp_next for performance, bypassing the
         * Rng class. Any change to Rng.uniformInt must be mirrored here. */
        for (int i = 0; i < m; i++) {
            uint64_t u = xoshiro256pp_next(rng_state);
            int idx = (int)(u % (uint64_t)n);
            resample_buf[i] = sorted_x[idx];
        }

        centers[iter] = fast_center_compute(resample_buf, m);
    }

    free(resample_buf);

    /* Sort bootstrap centers */
    qsort(centers, iterations, sizeof(double), cmp_double_bootstrap);

    UNPROTECT(1);
    return result;
}
