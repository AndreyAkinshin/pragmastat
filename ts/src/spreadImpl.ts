/**
 * O(n log n) implementation of the Spread (Shamos) estimator.
 * Based on Monahan's selection algorithm adapted for pairwise differences.
 *
 * Internal implementation - not part of public API.
 */

import { Rng } from './rng';
import { deriveSeed } from './fnv1a';

/**
 * Compute the median of all pairwise absolute differences |xi - xj| efficiently.
 *
 * Time complexity: O(n log n) expected
 * Space complexity: O(n)
 *
 * @param values Array of numeric values
 * @returns The spread estimate (Shamos estimator)
 * @internal
 */
export function spreadImpl(values: readonly number[], assumeSorted = false): number {
  const n = values.length;
  if (n === 0) {
    throw new Error('Input array cannot be empty');
  }
  if (n === 1) {
    return 0;
  }
  if (n === 2) {
    return Math.abs(values[1] - values[0]);
  }

  // Create deterministic RNG from input values
  const rng = new Rng(deriveSeed(values));

  // Sort the values
  const a = assumeSorted ? values : [...values].sort((a, b) => a - b);

  // Total number of pairwise differences with i < j
  // Use BigInt to prevent 53-bit precision overflow for large n
  const N = Number((BigInt(n) * BigInt(n - 1)) / 2n);
  const kLow = Math.floor((N + 1) / 2); // 1-based rank of lower middle
  const kHigh = Math.floor((N + 2) / 2); // 1-based rank of upper middle

  // Per-row active bounds over columns j (0-based indices)
  // Row i allows j in [i+1, n-1] initially
  const L: number[] = Array.from({ length: n }, (_, i) => Math.min(i + 1, n));
  const R: number[] = Array(n).fill(n - 1);

  for (let i = 0; i < n; i++) {
    if (L[i] > R[i]) {
      L[i] = 1;
      R[i] = 0; // mark empty
    }
  }

  const rowCounts: number[] = Array(n).fill(0);

  // Initial pivot: a central gap
  let pivot = a[Math.floor(n / 2)] - a[Math.floor((n - 1) / 2)];
  let prevCountBelow = -1;

  // Bound the selection loop. On valid sorted input the Monahan selection
  // converges in O(log n) iterations; this cap is far higher than ever needed
  // for sorted input but guarantees termination on misuse (e.g.,
  // `assumeSorted=true` on UNSORTED input, which is undefined behavior and
  // would otherwise spin forever). The cap scales with n so large valid
  // inputs are never starved. We also track no-progress (stall) on the
  // active set to bail out deterministically.
  const baseIterations = 256;
  const maxIterations = baseIterations + 4 * n;
  let prevActiveSize = -1;
  let stallCount = 0;
  const maxStall = 8;
  let iterations = 0;

  while (true) {
    if (++iterations > maxIterations) {
      throw new Error('Convergence failure (pathological input)');
    }

    // === PARTITION: count how many differences are < pivot ===
    let countBelow = 0;
    let largestBelow = -Infinity;
    let smallestAtOrAbove = Infinity;

    let j = 1; // global two-pointer (non-decreasing across rows)
    for (let i = 0; i < n - 1; i++) {
      if (j < i + 1) {
        j = i + 1;
      }
      while (j < n && a[j] - a[i] < pivot) {
        j++;
      }

      const cntRow = Math.max(0, j - (i + 1));
      rowCounts[i] = cntRow;
      countBelow += cntRow;

      // boundary elements for this row
      if (cntRow > 0) {
        const candBelow = a[j - 1] - a[i];
        largestBelow = Math.max(largestBelow, candBelow);
      }

      if (j < n) {
        const candAtOrAbove = a[j] - a[i];
        smallestAtOrAbove = Math.min(smallestAtOrAbove, candAtOrAbove);
      }
    }

    // === TARGET CHECK ===
    const atTarget = countBelow === kLow || countBelow === kHigh - 1;

    if (atTarget) {
      if (kLow < kHigh) {
        // Even N: average the two central order stats
        return 0.5 * largestBelow + 0.5 * smallestAtOrAbove;
      } else {
        // Odd N: pick the single middle
        const needLargest = countBelow === kLow;
        return needLargest ? largestBelow : smallestAtOrAbove;
      }
    }

    // === STALL HANDLING ===
    if (countBelow === prevCountBelow) {
      let minActive = Infinity;
      let maxActive = -Infinity;
      let active = 0;

      for (let i = 0; i < n - 1; i++) {
        const Li = L[i];
        const Ri = R[i];
        if (Li > Ri) {
          continue;
        }

        const rowMin = a[Li] - a[i];
        const rowMax = a[Ri] - a[i];
        minActive = Math.min(minActive, rowMin);
        maxActive = Math.max(maxActive, rowMax);
        active += Ri - Li + 1;
      }

      if (active <= 0) {
        if (kLow < kHigh) {
          return 0.5 * largestBelow + 0.5 * smallestAtOrAbove;
        }
        return countBelow >= kLow ? largestBelow : smallestAtOrAbove;
      }

      if (maxActive <= minActive) {
        return minActive;
      }

      const mid = 0.5 * minActive + 0.5 * maxActive;
      pivot = mid > minActive && mid <= maxActive ? mid : maxActive;
      prevCountBelow = countBelow;
      continue;
    }

    // === SHRINK ACTIVE WINDOW ===
    if (countBelow < kLow) {
      // Need larger differences: discard all strictly below pivot
      for (let i = 0; i < n - 1; i++) {
        const newL = i + 1 + rowCounts[i];
        if (newL > L[i]) {
          L[i] = newL;
        }
        if (L[i] > R[i]) {
          L[i] = 1;
          R[i] = 0;
        }
      }
    } else {
      // Too many below: keep only those strictly below pivot
      for (let i = 0; i < n - 1; i++) {
        const newR = i + rowCounts[i];
        if (newR < R[i]) {
          R[i] = newR;
        }
        if (R[i] < i + 1) {
          L[i] = 1;
          R[i] = 0;
        }
      }
    }

    prevCountBelow = countBelow;

    // === CHOOSE NEXT PIVOT FROM ACTIVE SET ===
    let activeSize = 0;
    for (let i = 0; i < n - 1; i++) {
      if (L[i] <= R[i]) {
        activeSize += R[i] - L[i] + 1;
      }
    }

    // Stall detection: on valid sorted input the active set strictly
    // shrinks toward the target. If it fails to shrink for several
    // consecutive iterations, the input is pathological (e.g.,
    // assumeSorted=true on unsorted data) and we bail deterministically.
    if (activeSize >= prevActiveSize && prevActiveSize >= 0) {
      stallCount++;
      if (stallCount >= maxStall) {
        throw new Error('Convergence failure (pathological input)');
      }
    } else {
      stallCount = 0;
    }
    prevActiveSize = activeSize;

    if (activeSize <= 2) {
      // Few candidates left: return midrange of remaining
      let minRem = Infinity;
      let maxRem = -Infinity;
      for (let i = 0; i < n - 1; i++) {
        if (L[i] > R[i]) {
          continue;
        }
        const lo = a[L[i]] - a[i];
        const hi = a[R[i]] - a[i];
        minRem = Math.min(minRem, lo);
        maxRem = Math.max(maxRem, hi);
      }

      if (activeSize <= 0) {
        if (kLow < kHigh) {
          return 0.5 * largestBelow + 0.5 * smallestAtOrAbove;
        }
        return countBelow >= kLow ? largestBelow : smallestAtOrAbove;
      }

      if (kLow < kHigh) {
        return 0.5 * minRem + 0.5 * maxRem;
      }
      return Math.abs(kLow - 1 - countBelow) <= Math.abs(countBelow - kLow) ? minRem : maxRem;
    } else {
      // Weighted random row selection
      const t = rng.uniformInt(0, activeSize);
      let acc = 0;
      let row = 0;
      for (row = 0; row < n - 1; row++) {
        if (L[row] > R[row]) {
          continue;
        }
        const size = R[row] - L[row] + 1;
        if (t < acc + size) {
          break;
        }
        acc += size;
      }

      // Median column of the selected row
      const col = Math.floor((L[row] + R[row]) / 2);
      pivot = a[col] - a[row];
    }
  }
}
