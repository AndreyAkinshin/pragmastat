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

/*
 * O(n log n) implementation of the Center (Hodges-Lehmann) estimator
 * Based on Monahan's Algorithm 616 (1984)
 * Computes the median of all pairwise averages efficiently
 */
static PyObject* center_impl_c(PyObject* self, PyObject* args) {
    PyArrayObject *values_array;
    int assume_sorted = 0;

    // Parse input: array and optional assume_sorted flag
    if (!PyArg_ParseTuple(args, "O!|i", &PyArray_Type, &values_array, &assume_sorted)) {
        return NULL;
    }

    // Ensure it's a 1D array of doubles
    if (PyArray_NDIM(values_array) != 1) {
        PyErr_SetString(PyExc_ValueError, "Input must be a 1-dimensional array");
        return NULL;
    }

    npy_intp n = PyArray_DIM(values_array, 0);
    if (n == 0) {
        PyErr_SetString(PyExc_ValueError, "Input array cannot be empty");
        return NULL;
    }

    // Handle trivial cases
    if (n == 1) {
        double val = *(double*)PyArray_GETPTR1(values_array, 0);
        return PyFloat_FromDouble(val);
    }

    if (n == 2) {
        double v0 = *(double*)PyArray_GETPTR1(values_array, 0);
        double v1 = *(double*)PyArray_GETPTR1(values_array, 1);
        // Overflow-safe, order-symmetric midpoint: 0.5*a + 0.5*b
        // (halve before summing; never overflows; operand order is irrelevant).
        return PyFloat_FromDouble(0.5 * v0 + 0.5 * v1);
    }

    // Use input directly when sorted; otherwise sort a copy
    double *sorted_values;
    int allocated_sorted = 0;
    if (assume_sorted && PyArray_IS_C_CONTIGUOUS(values_array)) {
        sorted_values = (double*)PyArray_DATA(values_array);
    } else {
        sorted_values = (double*)malloc(n * sizeof(double));
        if (!sorted_values) {
            PyErr_NoMemory();
            return NULL;
        }
        allocated_sorted = 1;
        for (npy_intp i = 0; i < n; i++) {
            sorted_values[i] = *(double*)PyArray_GETPTR1(values_array, i);
        }
        if (!assume_sorted) {
            qsort(sorted_values, n, sizeof(double), compare_doubles);
        }
    }

    // Calculate target median rank(s)
    long long total_pairs = ((long long)n * (n + 1)) / 2;
    long long median_rank_low = (total_pairs + 1) / 2;
    long long median_rank_high = (total_pairs + 2) / 2;

    // Initialize search bounds
    long long *left_bounds = (long long*)malloc(n * sizeof(long long));
    long long *right_bounds = (long long*)malloc(n * sizeof(long long));
    long long *partition_counts = (long long*)malloc(n * sizeof(long long));

    if (!left_bounds || !right_bounds || !partition_counts) {
        if (allocated_sorted) free(sorted_values);
        free(left_bounds);
        free(right_bounds);
        free(partition_counts);
        PyErr_NoMemory();
        return NULL;
    }

    for (npy_intp i = 0; i < n; i++) {
        left_bounds[i] = i;
        right_bounds[i] = n - 1;
    }

    // Initial pivot: sum of middle elements
    double pivot = sorted_values[(n - 1) / 2] + sorted_values[n / 2];
    long long active_set_size = total_pairs;
    long long previous_count = 0;

    double result_value = 0.0;
    int converged = 0;

    // Bound the selection loop. On valid sorted input the Monahan selection
    // converges in O(log n) iterations; this cap is far higher than ever needed
    // for sorted input but guarantees termination on misuse (e.g.,
    // assume_sorted=True on UNSORTED input, which is undefined behavior and
    // would otherwise loop forever and wedge the process: an unkillable
    // infinite loop inside the C extension). The cap scales with n so large
    // valid inputs are never starved. We also track no-progress (stall) on the
    // active set to bail out deterministically.
    const long long base_iterations = 256;
    long long max_iterations = base_iterations + 4 * (long long)n;
    long long prev_active_set_size = -1;
    int stall_count = 0;
    const int max_stall = 8;
    for (long long iter = 0; iter < max_iterations; iter++) {
        // === PARTITION STEP ===
        long long count_below_pivot = 0;
        long long current_column = n - 1;

        for (npy_intp row = 0; row < n; row++) {
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
            double min_active_sum = INFINITY;
            double max_active_sum = -INFINITY;

            for (npy_intp i = 0; i < n; i++) {
                if (left_bounds[i] > right_bounds[i]) continue;

                double row_value = sorted_values[i];
                double smallest_in_row = sorted_values[left_bounds[i]] + row_value;
                double largest_in_row = sorted_values[right_bounds[i]] + row_value;

                min_active_sum = MIN(min_active_sum, smallest_in_row);
                max_active_sum = MAX(max_active_sum, largest_in_row);
            }

            pivot = 0.5 * min_active_sum + 0.5 * max_active_sum;
            if (pivot <= min_active_sum || pivot > max_active_sum) {
                pivot = max_active_sum;
            }

            if (min_active_sum == max_active_sum || active_set_size <= 2) {
                result_value = pivot / 2.0;
                converged = 1;
                break;
            }

            continue;
        }

        // === TARGET CHECK ===
        int at_target_rank = (count_below_pivot == median_rank_low) ||
                             (count_below_pivot == median_rank_high - 1);

        if (at_target_rank) {
            double largest_below_pivot = -INFINITY;
            double smallest_at_or_above_pivot = INFINITY;

            for (npy_intp i = 0; i < n; i++) {
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

            if (median_rank_low < median_rank_high) {
                // Even total: average the two middle values. Overflow-safe: quarter each
                // pair-sum before summing (both operands can be near the double max).
                result_value = 0.25 * smallest_at_or_above_pivot + 0.25 * largest_below_pivot;
            } else {
                // Odd total: return the single middle value
                int need_largest = (count_below_pivot == median_rank_low);
                result_value = (need_largest ? largest_below_pivot : smallest_at_or_above_pivot) / 2.0;
            }

            converged = 1;
            break;
        }

        // === UPDATE BOUNDS ===
        if (count_below_pivot < median_rank_low) {
            // Too few values below pivot - search higher
            for (npy_intp i = 0; i < n; i++) {
                left_bounds[i] = i + partition_counts[i];
            }
        } else {
            // Too many values below pivot - search lower
            for (npy_intp i = 0; i < n; i++) {
                right_bounds[i] = i + partition_counts[i] - 1;
            }
        }

        // === PREPARE NEXT ITERATION ===
        previous_count = count_below_pivot;

        // Recalculate active set size
        active_set_size = 0;
        for (npy_intp i = 0; i < n; i++) {
            long long row_size = right_bounds[i] - left_bounds[i] + 1;
            active_set_size += MAX(0, row_size);
        }

        // Stall detection: on valid sorted input the active set strictly
        // shrinks toward the target. If it fails to shrink for several
        // consecutive iterations, the input is pathological (e.g.,
        // assume_sorted=True on unsorted data) and we bail deterministically;
        // the shared error path below reports the convergence failure.
        if (active_set_size >= prev_active_set_size && prev_active_set_size >= 0) {
            stall_count++;
            if (stall_count >= max_stall) {
                break;
            }
        } else {
            stall_count = 0;
        }
        prev_active_set_size = active_set_size;

        // Choose next pivot
        if (active_set_size > 2) {
            // Use deterministic middle-element strategy
            long long target_index = active_set_size / 2;
            npy_intp selected_row = 0;

            long long cumulative_size = 0;
            for (npy_intp i = 0; i < n; i++) {
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
            double min_remaining_sum = INFINITY;
            double max_remaining_sum = -INFINITY;

            for (npy_intp i = 0; i < n; i++) {
                if (left_bounds[i] > right_bounds[i]) continue;

                double row_value = sorted_values[i];
                double min_in_row = sorted_values[left_bounds[i]] + row_value;
                double max_in_row = sorted_values[right_bounds[i]] + row_value;

                min_remaining_sum = MIN(min_remaining_sum, min_in_row);
                max_remaining_sum = MAX(max_remaining_sum, max_in_row);
            }

            pivot = 0.5 * min_remaining_sum + 0.5 * max_remaining_sum;
            if (pivot <= min_remaining_sum || pivot > max_remaining_sum) {
                pivot = max_remaining_sum;
            }

            if (min_remaining_sum == max_remaining_sum) {
                result_value = pivot / 2.0;
                converged = 1;
                break;
            }
        }
    }

    // Cleanup
    if (allocated_sorted) free(sorted_values);
    free(left_bounds);
    free(right_bounds);
    free(partition_counts);

    if (!converged) {
        PyErr_SetString(PyExc_RuntimeError, "Convergence failure (pathological input)");
        return NULL;
    }

    return PyFloat_FromDouble(result_value);
}

// Method definitions
static PyMethodDef CenterImplMethods[] = {
    {"center_impl_c", center_impl_c, METH_VARARGS, "Center estimator in C"},
    {NULL, NULL, 0, NULL}
};

// Module definition
static struct PyModuleDef center_impl_module = {
    PyModuleDef_HEAD_INIT,
    "_center_impl_c",
    "Center estimator C extension",
    -1,
    CenterImplMethods
};

// Module initialization
PyMODINIT_FUNC PyInit__center_impl_c(void) {
    import_array();
    return PyModule_Create(&center_impl_module);
}
