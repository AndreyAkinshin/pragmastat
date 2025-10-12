#define PY_SSIZE_T_CLEAN
#include <Python.h>
#include <numpy/arrayobject.h>
#include <math.h>
#include <stdlib.h>

#define MIN(a, b) ((a) < (b) ? (a) : (b))
#define MAX(a, b) ((a) > (b) ? (a) : (b))

// Comparison function for qsort
static int compare_doubles(const void *a, const void *b) {
    double da = *(const double *)a;
    double db = *(const double *)b;
    if (da < db) return -1;
    if (da > db) return 1;
    return 0;
}

// Numerically stable midpoint
static double midpoint(double a, double b) {
    return a + (b - a) * 0.5;
}

// Two-pointer algorithm to count pairs where x[i] - y[j] <= threshold
// Also tracks the closest actual differences on either side of threshold
static void count_and_neighbors(
    double *x, npy_intp m,
    double *y, npy_intp n,
    double threshold,
    long long *count_le,
    double *closest_below,
    double *closest_above)
{
    long long count = 0;
    double max_below = -INFINITY;
    double min_above = INFINITY;

    npy_intp j = 0;
    for (npy_intp i = 0; i < m; i++) {
        // Move j forward while x[i] - y[j] > threshold
        while (j < n && x[i] - y[j] > threshold) {
            j++;
        }

        // All elements from y[j] to y[n-1] satisfy x[i] - y[j] <= threshold
        count += (n - j);

        // Track boundary values
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

    *count_le = count;
    *closest_below = max_below;
    *closest_above = min_above;
}

// Select the k-th smallest pairwise difference (1-indexed)
static double select_kth_pairwise_diff(
    double *x, npy_intp m,
    double *y, npy_intp n,
    long long k)
{
    long long total = (long long)m * n;

    if (k < 1 || k > total) {
        PyErr_Format(PyExc_ValueError, "k must be in [1, %lld], got %lld", total, k);
        return NAN;
    }

    // Initialize search bounds
    double search_min = x[0] - y[n - 1];
    double search_max = x[m - 1] - y[0];

    if (isnan(search_min) || isnan(search_max)) {
        PyErr_SetString(PyExc_ValueError, "NaN in input values");
        return NAN;
    }

    const int max_iterations = 128;
    double prev_min = -INFINITY;
    double prev_max = INFINITY;

    for (int iter = 0; iter < max_iterations && search_min != search_max; iter++) {
        double mid = midpoint(search_min, search_max);
        long long count_le;
        double closest_below, closest_above;

        count_and_neighbors(x, m, y, n, mid, &count_le, &closest_below, &closest_above);

        // Check if we found the exact value
        if (closest_below == closest_above) {
            return closest_below;
        }

        // No progress means we're stuck between two discrete values
        if (search_min == prev_min && search_max == prev_max) {
            return (count_le >= k) ? closest_below : closest_above;
        }

        prev_min = search_min;
        prev_max = search_max;

        // Narrow the search space
        if (count_le >= k) {
            search_max = closest_below;
        } else {
            search_min = closest_above;
        }
    }

    if (search_min != search_max) {
        PyErr_SetString(PyExc_RuntimeError, "Convergence failure (pathological input)");
        return NAN;
    }

    return search_min;
}

/*
 * Fast O((m+n) log L) implementation of the Shift estimator
 * Computes quantiles of all pairwise differences without materializing them
 */
static PyObject* fast_shift_c(PyObject* self, PyObject* args) {
    PyArrayObject *x_array, *y_array, *p_array;

    // Parse input
    if (!PyArg_ParseTuple(args, "O!O!O!", &PyArray_Type, &x_array,
                          &PyArray_Type, &y_array, &PyArray_Type, &p_array)) {
        return NULL;
    }

    // Ensure arrays are 1D
    if (PyArray_NDIM(x_array) != 1 || PyArray_NDIM(y_array) != 1 || PyArray_NDIM(p_array) != 1) {
        PyErr_SetString(PyExc_ValueError, "All inputs must be 1-dimensional arrays");
        return NULL;
    }

    npy_intp m = PyArray_DIM(x_array, 0);
    npy_intp n = PyArray_DIM(y_array, 0);
    npy_intp num_quantiles = PyArray_DIM(p_array, 0);

    if (m == 0 || n == 0) {
        PyErr_SetString(PyExc_ValueError, "x and y must be non-empty");
        return NULL;
    }

    // Allocate and sort x and y
    double *xs = (double*)malloc(m * sizeof(double));
    double *ys = (double*)malloc(n * sizeof(double));

    if (!xs || !ys) {
        free(xs);
        free(ys);
        PyErr_NoMemory();
        return NULL;
    }

    for (npy_intp i = 0; i < m; i++) {
        xs[i] = *(double*)PyArray_GETPTR1(x_array, i);
        if (isnan(xs[i])) {
            free(xs);
            free(ys);
            PyErr_SetString(PyExc_ValueError, "NaN values not allowed in x");
            return NULL;
        }
    }

    for (npy_intp i = 0; i < n; i++) {
        ys[i] = *(double*)PyArray_GETPTR1(y_array, i);
        if (isnan(ys[i])) {
            free(xs);
            free(ys);
            PyErr_SetString(PyExc_ValueError, "NaN values not allowed in y");
            return NULL;
        }
    }

    qsort(xs, m, sizeof(double), compare_doubles);
    qsort(ys, n, sizeof(double), compare_doubles);

    long long total = (long long)m * n;

    // Process quantiles
    // First, collect all required ranks and interpolation parameters
    typedef struct {
        long long lower_rank;
        long long upper_rank;
        double weight;
    } InterpolationParam;

    InterpolationParam *interp_params = (InterpolationParam*)malloc(num_quantiles * sizeof(InterpolationParam));
    if (!interp_params) {
        free(xs);
        free(ys);
        PyErr_NoMemory();
        return NULL;
    }

    // Use a simple array to track unique ranks (could be optimized with hash set)
    long long *required_ranks = (long long*)malloc(2 * num_quantiles * sizeof(long long));
    int num_required = 0;

    if (!required_ranks) {
        free(xs);
        free(ys);
        free(interp_params);
        PyErr_NoMemory();
        return NULL;
    }

    // Collect required ranks
    for (npy_intp i = 0; i < num_quantiles; i++) {
        double pk = *(double*)PyArray_GETPTR1(p_array, i);

        if (isnan(pk) || pk < 0.0 || pk > 1.0) {
            free(xs);
            free(ys);
            free(interp_params);
            free(required_ranks);
            PyErr_Format(PyExc_ValueError, "Probabilities must be within [0, 1], got %f", pk);
            return NULL;
        }

        // Type-7 quantile: h = 1 + (n-1)*p
        double h = 1.0 + (total - 1) * pk;
        long long lower_rank = (long long)floor(h);
        long long upper_rank = (long long)ceil(h);
        double weight = h - lower_rank;

        // Clamp to valid range
        if (lower_rank < 1) lower_rank = 1;
        if (upper_rank > total) upper_rank = total;
        if (lower_rank > total) lower_rank = total;
        if (upper_rank < 1) upper_rank = 1;

        interp_params[i].lower_rank = lower_rank;
        interp_params[i].upper_rank = upper_rank;
        interp_params[i].weight = weight;

        // Add to required ranks if not already present
        int found_lower = 0, found_upper = 0;
        for (int j = 0; j < num_required; j++) {
            if (required_ranks[j] == lower_rank) found_lower = 1;
            if (required_ranks[j] == upper_rank) found_upper = 1;
        }
        if (!found_lower) required_ranks[num_required++] = lower_rank;
        if (!found_upper && upper_rank != lower_rank) required_ranks[num_required++] = upper_rank;
    }

    // Compute rank values
    double *rank_values = (double*)malloc(num_required * sizeof(double));
    if (!rank_values) {
        free(xs);
        free(ys);
        free(interp_params);
        free(required_ranks);
        PyErr_NoMemory();
        return NULL;
    }

    for (int i = 0; i < num_required; i++) {
        rank_values[i] = select_kth_pairwise_diff(xs, m, ys, n, required_ranks[i]);
        if (isnan(rank_values[i])) {
            // Error was set by select_kth_pairwise_diff
            free(xs);
            free(ys);
            free(interp_params);
            free(required_ranks);
            free(rank_values);
            return NULL;
        }
    }

    // Create result array
    npy_intp dims[1] = {num_quantiles};
    PyArrayObject *result = (PyArrayObject*)PyArray_SimpleNew(1, dims, NPY_DOUBLE);
    if (!result) {
        free(xs);
        free(ys);
        free(interp_params);
        free(required_ranks);
        free(rank_values);
        return NULL;
    }

    // Interpolate to get final quantile values
    for (npy_intp i = 0; i < num_quantiles; i++) {
        long long lower_rank = interp_params[i].lower_rank;
        long long upper_rank = interp_params[i].upper_rank;
        double weight = interp_params[i].weight;

        // Find rank values
        double lower_val = 0.0, upper_val = 0.0;
        for (int j = 0; j < num_required; j++) {
            if (required_ranks[j] == lower_rank) lower_val = rank_values[j];
            if (required_ranks[j] == upper_rank) upper_val = rank_values[j];
        }

        double result_val;
        if (weight == 0.0) {
            result_val = lower_val;
        } else {
            result_val = (1.0 - weight) * lower_val + weight * upper_val;
        }

        *(double*)PyArray_GETPTR1(result, i) = result_val;
    }

    // Cleanup
    free(xs);
    free(ys);
    free(interp_params);
    free(required_ranks);
    free(rank_values);

    return (PyObject*)result;
}

// Method definitions
static PyMethodDef FastShiftMethods[] = {
    {"fast_shift_c", fast_shift_c, METH_VARARGS, "Fast shift estimator in C"},
    {NULL, NULL, 0, NULL}
};

// Module definition
static struct PyModuleDef fast_shift_module = {
    PyModuleDef_HEAD_INIT,
    "_fast_shift_c",
    "Fast shift estimator C extension",
    -1,
    FastShiftMethods
};

// Module initialization
PyMODINIT_FUNC PyInit__fast_shift_c(void) {
    import_array();
    return PyModule_Create(&fast_shift_module);
}
