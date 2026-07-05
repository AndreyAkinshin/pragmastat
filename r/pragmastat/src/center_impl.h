#ifndef CENTER_IMPL_H
#define CENTER_IMPL_H

/*
 * Compute the Center (Hodges-Lehmann) estimator: median of all pairwise averages.
 * Uses Monahan's Algorithm 616 (1984) for O(n log n) computation.
 *
 * The input array is NOT modified. When assume_sorted == 0 a sorted copy is made
 * internally; when assume_sorted != 0 the caller guarantees `values` is already
 * sorted ascending and no copy/sort is performed.
 * Caller is responsible for ensuring n > 0.
 */
double center_impl_compute(const double *values, int n, int assume_sorted);

#endif
