#define PY_SSIZE_T_CLEAN
#include <Python.h>
#include <numpy/arrayobject.h>
#include <math.h>
#include <stdlib.h>
#include <stdint.h>
#include <string.h>

#define MIN(a, b) ((a) < (b) ? (a) : (b))
#define MAX(a, b) ((a) > (b) ? (a) : (b))
#define ABS(a) ((a) < 0 ? -(a) : (a))

// Comparison function for qsort
static int compare_doubles(const void *a, const void *b) {
    double da = *(const double *)a;
    double db = *(const double *)b;
    if (da < db) return -1;
    if (da > db) return 1;
    return 0;
}

// Deterministic RNG mirroring py/pragmastat/xoshiro256.py bit for bit, so the
// C kernel and the pure-Python fallback follow identical narrowing paths for
// the same input. No global state: the generator lives on the caller's stack.

static uint64_t rotl_u64(uint64_t x, int k) {
    return (x << k) | (x >> (64 - k));
}

typedef struct { uint64_t s[4]; } xoshiro256pp_state;

// SplitMix64 seed expansion (Xoshiro256PlusPlus.__init__)
static void xoshiro256pp_init(xoshiro256pp_state *rng, uint64_t seed) {
    uint64_t state = seed;
    for (int i = 0; i < 4; i++) {
        state += 0x9E3779B97F4A7C15ULL;
        uint64_t z = state;
        z = (z ^ (z >> 30)) * 0xBF58476D1CE4E5B9ULL;
        z = (z ^ (z >> 27)) * 0x94D049BB133111EBULL;
        rng->s[i] = z ^ (z >> 31);
    }
}

static uint64_t xoshiro256pp_next(xoshiro256pp_state *rng) {
    uint64_t *s = rng->s;
    uint64_t result = rotl_u64(s[0] + s[3], 23) + s[0];
    uint64_t t = s[1] << 17;

    s[2] ^= s[0];
    s[3] ^= s[1];
    s[1] ^= s[2];
    s[0] ^= s[3];

    s[2] ^= t;
    s[3] = rotl_u64(s[3], 45);

    return result;
}

// FNV-1a over the IEEE-754 bytes of each input value, least significant byte
// first (_derive_seed). Hashes the ORIGINAL input order, before any sorting.
static uint64_t derive_seed(PyArrayObject *values_array, npy_intp n) {
    uint64_t hash = 0xCBF29CE484222325ULL;  // FNV offset basis
    for (npy_intp i = 0; i < n; i++) {
        double v = *(double*)PyArray_GETPTR1(values_array, i);
        uint64_t bits;
        memcpy(&bits, &v, sizeof(bits));
        for (int b = 0; b < 8; b++) {
            hash ^= (bits >> (b * 8)) & 0xFF;
            hash *= 0x00000100000001B3ULL;  // FNV prime
        }
    }
    return hash;
}

// Uniform integer in [0, limit_exclusive), mirroring Rng.uniform_int(0, limit)
static long long next_index(xoshiro256pp_state *rng, long long limit_exclusive) {
    if (limit_exclusive <= 0) return 0;
    return (long long)(xoshiro256pp_next(rng) % (uint64_t)limit_exclusive);
}

/*
 * O(n log n) implementation of the Spread (Shamos) estimator
 * Computes the median of all pairwise absolute differences efficiently
 */
static PyObject* spread_impl_c(PyObject* self, PyObject* args) {
    PyArrayObject *values_array;
    int assume_sorted = 0;

    // Parse input: array and optional assume_sorted flag
    if (!PyArg_ParseTuple(args, "O!|i", &PyArray_Type, &values_array, &assume_sorted)) {
        return NULL;
    }

    // Ensure it's a 1D array
    if (PyArray_NDIM(values_array) != 1) {
        PyErr_SetString(PyExc_ValueError, "Input must be a 1-dimensional array");
        return NULL;
    }

    npy_intp n = PyArray_DIM(values_array, 0);
    if (n <= 1) {
        return PyFloat_FromDouble(0.0);
    }

    if (n == 2) {
        double v0 = *(double*)PyArray_GETPTR1(values_array, 0);
        double v1 = *(double*)PyArray_GETPTR1(values_array, 1);
        return PyFloat_FromDouble(fabs(v1 - v0));
    }

    // Use input directly when sorted; otherwise sort a copy
    double *a;
    int allocated_a = 0;
    if (assume_sorted && PyArray_IS_C_CONTIGUOUS(values_array)) {
        a = (double*)PyArray_DATA(values_array);
    } else {
        a = (double*)malloc(n * sizeof(double));
        if (!a) {
            PyErr_NoMemory();
            return NULL;
        }
        allocated_a = 1;
        for (npy_intp i = 0; i < n; i++) {
            a[i] = *(double*)PyArray_GETPTR1(values_array, i);
        }
        if (!assume_sorted) {
            qsort(a, n, sizeof(double), compare_doubles);
        }
    }

    // Total number of pairwise differences with i < j
    long long N = ((long long)n * (n - 1)) / 2;
    long long k_low = (N + 1) / 2;
    long long k_high = (N + 2) / 2;

    // Per-row active bounds
    int *L = (int*)malloc(n * sizeof(int));
    int *R_bounds = (int*)malloc(n * sizeof(int));
    long long *row_counts = (long long*)malloc(n * sizeof(long long));

    if (!L || !R_bounds || !row_counts) {
        if (allocated_a) free(a);
        free(L);
        free(R_bounds);
        free(row_counts);
        PyErr_NoMemory();
        return NULL;
    }

    for (npy_intp i = 0; i < n; i++) {
        L[i] = i + 1;
        R_bounds[i] = n - 1;
        if (L[i] > R_bounds[i]) {
            L[i] = 1;
            R_bounds[i] = 0;
        }
    }

    // Initial pivot: a central gap
    double pivot = a[n / 2] - a[(n - 1) / 2];
    long long prev_count_below = -1;

    // Deterministic RNG seeded from the input values in their original order,
    // matching the pure-Python kernel (Rng(_derive_seed(values))).
    xoshiro256pp_state rng;
    xoshiro256pp_init(&rng, derive_seed(values_array, n));

    double result_value = 0.0;
    int converged = 0;

    // Bound the selection loop. On valid sorted input the Monahan-style
    // selection converges in O(log n) iterations; this cap is far higher than
    // ever needed for sorted input but guarantees termination on misuse (e.g.,
    // assume_sorted=True on UNSORTED input, which is undefined behavior and
    // would otherwise loop forever and wedge the process: an unkillable
    // infinite loop inside the C extension). The cap scales with n so large
    // valid inputs are never starved. We also track no-progress (stall) on the
    // active set to bail out deterministically. Mirrors the center cap
    // (256 + 4 * n).
    const long long base_iterations = 256;
    long long max_iterations = base_iterations + 4 * (long long)n;
    long long prev_active_size = -1;
    int stall_count = 0;
    const int max_stall = 8;
    for (long long iter = 0; iter < max_iterations; iter++) {
        // === PARTITION: count how many differences are < pivot ===
        long long count_below = 0;
        double largest_below = -INFINITY;
        double smallest_at_or_above = INFINITY;

        int j = 1;
        for (npy_intp i = 0; i < n - 1; i++) {
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
            if (k_low < k_high) {
                // Even N: average the two central order stats
                result_value = 0.5 * largest_below + 0.5 * smallest_at_or_above;
            } else {
                // Odd N: pick the single middle
                int need_largest = (count_below == k_low);
                result_value = need_largest ? largest_below : smallest_at_or_above;
            }
            converged = 1;
            break;
        }

        // === STALL HANDLING ===
        if (count_below == prev_count_below) {
            double min_active = INFINITY;
            double max_active = -INFINITY;
            long long active = 0;

            for (npy_intp i = 0; i < n - 1; i++) {
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
                if (k_low < k_high) {
                    result_value = 0.5 * largest_below + 0.5 * smallest_at_or_above;
                } else {
                    result_value = (count_below >= k_low) ? largest_below : smallest_at_or_above;
                }
                converged = 1;
                break;
            }

            if (max_active <= min_active) {
                result_value = min_active;
                converged = 1;
                break;
            }

            double mid = 0.5 * min_active + 0.5 * max_active;
            pivot = (mid > min_active && mid <= max_active) ? mid : max_active;
            prev_count_below = count_below;
            continue;
        }

        // === SHRINK ACTIVE WINDOW ===
        if (count_below < k_low) {
            // Need larger differences: discard all strictly below pivot
            for (npy_intp i = 0; i < n - 1; i++) {
                int new_L = i + 1 + (int)row_counts[i];
                if (new_L > L[i]) L[i] = new_L;
                if (L[i] > R_bounds[i]) {
                    L[i] = 1;
                    R_bounds[i] = 0;
                }
            }
        } else {
            // Too many below: keep only those strictly below pivot
            for (npy_intp i = 0; i < n - 1; i++) {
                int new_R = i + (int)row_counts[i];
                if (new_R < R_bounds[i]) R_bounds[i] = new_R;
                if (R_bounds[i] < i + 1) {
                    L[i] = 1;
                    R_bounds[i] = 0;
                }
            }
        }

        prev_count_below = count_below;

        // === CHOOSE NEXT PIVOT FROM ACTIVE SET ===
        long long active_size = 0;
        for (npy_intp i = 0; i < n - 1; i++) {
            if (L[i] <= R_bounds[i]) {
                active_size += (R_bounds[i] - L[i] + 1);
            }
        }

        // Stall detection: on valid sorted input the active set strictly
        // shrinks toward the target. If it fails to shrink for several
        // consecutive iterations, the input is pathological (e.g.,
        // assume_sorted=True on unsorted data) and we bail deterministically;
        // the shared error path below reports the convergence failure.
        if (active_size >= prev_active_size && prev_active_size >= 0) {
            stall_count++;
            if (stall_count >= max_stall) {
                break;
            }
        } else {
            stall_count = 0;
        }
        prev_active_size = active_size;

        if (active_size <= 2) {
            // Few candidates left: return midrange of remaining
            double min_rem = INFINITY;
            double max_rem = -INFINITY;

            for (npy_intp i = 0; i < n - 1; i++) {
                if (L[i] > R_bounds[i]) continue;
                double lo = a[L[i]] - a[i];
                double hi = a[R_bounds[i]] - a[i];
                if (lo < min_rem) min_rem = lo;
                if (hi > max_rem) max_rem = hi;
            }

            if (active_size <= 0) {
                if (k_low < k_high) {
                    result_value = 0.5 * largest_below + 0.5 * smallest_at_or_above;
                } else {
                    result_value = (count_below >= k_low) ? largest_below : smallest_at_or_above;
                }
                converged = 1;
                break;
            }

            if (k_low < k_high) {
                result_value = 0.5 * min_rem + 0.5 * max_rem;
            } else {
                long long dist_low = llabs((k_low - 1) - count_below);
                long long dist_high = llabs(count_below - k_low);
                result_value = (dist_low <= dist_high) ? min_rem : max_rem;
            }
            converged = 1;
            break;

        } else {
            // Weighted random row selection
            long long t = next_index(&rng, active_size);
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

    // Cleanup
    if (allocated_a) free(a);
    free(L);
    free(R_bounds);
    free(row_counts);

    if (!converged) {
        PyErr_SetString(PyExc_RuntimeError, "Convergence failure (pathological input)");
        return NULL;
    }

    return PyFloat_FromDouble(result_value);
}

// Method definitions
static PyMethodDef SpreadImplMethods[] = {
    {"spread_impl_c", spread_impl_c, METH_VARARGS, "Spread estimator in C"},
    {NULL, NULL, 0, NULL}
};

// Module definition
static struct PyModuleDef spread_impl_module = {
    PyModuleDef_HEAD_INIT,
    "_spread_impl_c",
    "Spread estimator C extension",
    -1,
    SpreadImplMethods
};

// Module initialization
PyMODINIT_FUNC PyInit__spread_impl_c(void) {
    import_array();
    return PyModule_Create(&spread_impl_module);
}
