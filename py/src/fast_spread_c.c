#define PY_SSIZE_T_CLEAN
#include <Python.h>
#include <numpy/arrayobject.h>
#include <math.h>
#include <stdlib.h>
#include <time.h>

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

// Random double in [0, 1)
static double uniform_random(void) {
    return (double)rand() / ((double)RAND_MAX + 1.0);
}

// Helper function for uniform random selection
static long long next_index(long long limit_exclusive) {
    if (limit_exclusive <= 0) return 0;
    if (limit_exclusive <= 2147483647LL) {
        return (long long)(uniform_random() * limit_exclusive);
    }

    // For large ranges, use rejection sampling
    unsigned long long u_limit = (unsigned long long)limit_exclusive;
    while (1) {
        unsigned long long u = ((unsigned long long)(uniform_random() * 4294967296.0)) << 32;
        u |= (unsigned long long)(uniform_random() * 4294967296.0);
        unsigned long long r = u % u_limit;
        if (u - r <= ULLONG_MAX - (ULLONG_MAX % u_limit)) {
            return (long long)r;
        }
    }
}

/*
 * Fast O(n log n) implementation of the Spread (Shamos) estimator
 * Computes the median of all pairwise absolute differences efficiently
 */
static PyObject* fast_spread_c(PyObject* self, PyObject* args) {
    PyArrayObject *values_array;

    // Parse input
    if (!PyArg_ParseTuple(args, "O!", &PyArray_Type, &values_array)) {
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

    // Allocate and sort working copy
    double *a = (double*)malloc(n * sizeof(double));
    if (!a) {
        PyErr_NoMemory();
        return NULL;
    }

    for (npy_intp i = 0; i < n; i++) {
        a[i] = *(double*)PyArray_GETPTR1(values_array, i);
    }
    qsort(a, n, sizeof(double), compare_doubles);

    // Total number of pairwise differences with i < j
    long long N = ((long long)n * (n - 1)) / 2;
    long long k_low = (N + 1) / 2;
    long long k_high = (N + 2) / 2;

    // Per-row active bounds
    int *L = (int*)malloc(n * sizeof(int));
    int *R_bounds = (int*)malloc(n * sizeof(int));
    long long *row_counts = (long long*)malloc(n * sizeof(long long));

    if (!L || !R_bounds || !row_counts) {
        free(a);
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

    // Initialize random seed
    static int seeded = 0;
    if (!seeded) {
        srand((unsigned int)time(NULL));
        seeded = 1;
    }

    double result_value = 0.0;
    int converged = 0;

    while (1) {
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
                result_value = 0.5 * (largest_below + smallest_at_or_above);
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
                    result_value = 0.5 * (largest_below + smallest_at_or_above);
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

            double mid = 0.5 * (min_active + max_active);
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
                    result_value = 0.5 * (largest_below + smallest_at_or_above);
                } else {
                    result_value = (count_below >= k_low) ? largest_below : smallest_at_or_above;
                }
                converged = 1;
                break;
            }

            if (k_low < k_high) {
                result_value = 0.5 * (min_rem + max_rem);
            } else {
                long long dist_low = llabs((k_low - 1) - count_below);
                long long dist_high = llabs(count_below - k_low);
                result_value = (dist_low <= dist_high) ? min_rem : max_rem;
            }
            converged = 1;
            break;

        } else {
            // Weighted random row selection
            long long t = next_index(active_size);
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
    free(a);
    free(L);
    free(R_bounds);
    free(row_counts);

    if (!converged) {
        PyErr_SetString(PyExc_RuntimeError, "Algorithm failed to converge");
        return NULL;
    }

    return PyFloat_FromDouble(result_value);
}

// Method definitions
static PyMethodDef FastSpreadMethods[] = {
    {"fast_spread_c", fast_spread_c, METH_VARARGS, "Fast spread estimator in C"},
    {NULL, NULL, 0, NULL}
};

// Module definition
static struct PyModuleDef fast_spread_module = {
    PyModuleDef_HEAD_INIT,
    "_fast_spread_c",
    "Fast spread estimator C extension",
    -1,
    FastSpreadMethods
};

// Module initialization
PyMODINIT_FUNC PyInit__fast_spread_c(void) {
    import_array();
    return PyModule_Create(&fast_spread_module);
}
