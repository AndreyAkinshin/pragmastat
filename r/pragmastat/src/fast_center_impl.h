#ifndef FAST_CENTER_IMPL_H
#define FAST_CENTER_IMPL_H

/*
 * Compute the Center (Hodges-Lehmann) estimator: median of all pairwise averages.
 * Uses Monahan's Algorithm 616 (1984) for O(n log n) computation.
 *
 * The input array is NOT modified. A sorted copy is made internally.
 * Caller is responsible for ensuring n > 0.
 */
double fast_center_compute(const double *values, int n);

#endif
