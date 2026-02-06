/**
 * Fast algorithm for finding quantiles of pairwise averages (x[i] + x[j])/2.
 *
 * Uses binary search with counting function to find exact quantiles in O(n log(range)) time.
 */

/** Relative epsilon for floating-point comparisons in binary search convergence. */
const RELATIVE_EPSILON = 1e-14;

/**
 * Counts how many pairwise averages (sorted[i] + sorted[j])/2 where i <= j are <= threshold.
 *
 * @param sorted Sorted array of values
 * @param threshold Threshold to count against
 * @returns Number of pairwise averages <= threshold
 */
function countPairsLessOrEqual(sorted: number[], threshold: number): number {
  const n = sorted.length;
  let count = 0;
  // j is not reset: as i increases, threshold decreases monotonically
  let j = n - 1;

  for (let i = 0; i < n; i++) {
    const target = 2 * threshold - sorted[i];
    while (j >= 0 && sorted[j] > target) {
      j--;
    }
    if (j >= i) {
      count += j - i + 1;
    }
  }

  return count;
}

/**
 * Finds the k-th smallest pairwise average using binary search.
 *
 * @param sorted Sorted array of values
 * @param k 1-based rank of the desired quantile
 * @returns The k-th smallest pairwise average
 */
function findExactQuantile(sorted: number[], k: number): number {
  const n = sorted.length;
  const totalPairs = (n * (n + 1)) / 2;

  if (n === 1) return sorted[0];
  if (k === 1) return sorted[0];
  if (k === totalPairs) return sorted[n - 1];

  const min = sorted[0];
  const max = sorted[n - 1];

  // Binary search on value range
  let lo = min;
  let hi = max;

  while (hi - lo > RELATIVE_EPSILON * Math.max(1.0, Math.abs(lo), Math.abs(hi))) {
    const mid = (lo + hi) / 2;
    const count = countPairsLessOrEqual(sorted, mid);
    if (count < k) {
      lo = mid;
    } else {
      hi = mid;
    }
  }

  const target = (lo + hi) / 2;

  // Extract candidates that are close to the target
  const candidates: number[] = [];
  for (let i = 0; i < n; i++) {
    const threshold = 2 * target - sorted[i];

    // Find left boundary using binary search
    let left = i;
    let right = n;
    while (left < right) {
      const m = Math.floor((left + right) / 2);
      if (sorted[m] < threshold - RELATIVE_EPSILON) {
        left = m + 1;
      } else {
        right = m;
      }
    }

    if (
      left < n &&
      left >= i &&
      Math.abs(sorted[left] - threshold) < RELATIVE_EPSILON * Math.max(1.0, Math.abs(threshold))
    ) {
      candidates.push((sorted[i] + sorted[left]) / 2);
    }

    if (left > i) {
      const avgBefore = (sorted[i] + sorted[left - 1]) / 2;
      if (avgBefore <= target + RELATIVE_EPSILON) {
        candidates.push(avgBefore);
      }
    }
  }

  if (candidates.length === 0) {
    return target;
  }

  candidates.sort((a, b) => a - b);

  // Return the candidate that gives exactly k pairs <= it
  for (const c of candidates) {
    if (countPairsLessOrEqual(sorted, c) >= k) {
      return c;
    }
  }

  return target;
}

/**
 * Finds both lower and upper quantile bounds for pairwise averages.
 *
 * @param sorted Sorted array of values
 * @param kLo 1-based rank for lower bound
 * @param kHi 1-based rank for upper bound
 * @returns Tuple of [lower bound, upper bound]
 */
export function fastCenterQuantileBounds(
  sorted: number[],
  kLo: number,
  kHi: number,
): [number, number] {
  const n = sorted.length;
  const totalPairs = (n * (n + 1)) / 2;

  // Clamp ranks to valid range
  const clampedLo = Math.max(1, Math.min(kLo, totalPairs));
  const clampedHi = Math.max(1, Math.min(kHi, totalPairs));

  const lower = findExactQuantile(sorted, clampedLo);
  const upper = findExactQuantile(sorted, clampedHi);

  // Ensure lower <= upper
  return lower > upper ? [upper, lower] : [lower, upper];
}
