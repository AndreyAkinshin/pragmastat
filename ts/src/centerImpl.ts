/**
 * O(n log n) implementation of the Center (Hodges-Lehmann) estimator.
 * Based on Monahan's Algorithm 616 (1984).
 *
 * Internal implementation - not part of public API.
 */

import { Rng } from './rng';

const FNV_OFFSET_BASIS = 0xcbf29ce484222325n;
const FNV_PRIME = 0x00000100000001b3n;
const MASK64 = (1n << 64n) - 1n;

/**
 * Convert a float64 to its IEEE 754 binary representation as bigint.
 */
function float64ToBits(value: number): bigint {
  const buffer = new ArrayBuffer(8);
  new Float64Array(buffer)[0] = value;
  const view = new DataView(buffer);
  return (BigInt(view.getUint32(4, true)) << 32n) | BigInt(view.getUint32(0, true));
}

/**
 * Derive a deterministic seed from input values using FNV-1a hash.
 */
function deriveSeed(values: number[]): bigint {
  let hash = FNV_OFFSET_BASIS;
  for (const v of values) {
    const bits = float64ToBits(v);
    for (let i = 0; i < 8; i++) {
      hash ^= (bits >> BigInt(i * 8)) & 0xffn;
      hash = (hash * FNV_PRIME) & MASK64;
    }
  }
  return BigInt.asIntN(64, hash);
}

/**
 * Compute the median of all pairwise averages (xi + xj)/2 efficiently.
 *
 * Time complexity: O(n log n) expected
 * Space complexity: O(n)
 *
 * @param values Array of numeric values
 * @returns The center estimate (Hodges-Lehmann estimator)
 * @internal
 */
export function centerImpl(values: number[]): number {
  const n = values.length;
  if (n === 0) {
    throw new Error('Input array cannot be empty');
  }
  if (n === 1) {
    return values[0];
  }
  if (n === 2) {
    // Overflow-safe, order-symmetric midpoint: 0.5*a + 0.5*b (halve before
    // summing; never overflows; operand order is irrelevant).
    const a = values[0];
    const b = values[1];
    return 0.5 * a + 0.5 * b;
  }

  // Create deterministic RNG from input values
  const rng = new Rng(deriveSeed(values));

  // Sort the values
  const sortedValues = [...values].sort((a, b) => a - b);

  // Calculate target median rank(s) among all pairwise sums
  // Use BigInt to prevent 53-bit precision overflow for large n
  const totalPairs = Number((BigInt(n) * BigInt(n + 1)) / 2n);
  const medianRankLow = Math.floor((totalPairs + 1) / 2); // 1-based rank
  const medianRankHigh = Math.floor((totalPairs + 2) / 2);

  // Initialize search bounds for each row (1-based indexing)
  const leftBounds: number[] = Array.from({ length: n }, (_, i) => i + 1);
  const rightBounds: number[] = Array(n).fill(n);

  // Start with a good pivot: sum of middle elements
  let pivot = sortedValues[Math.floor((n - 1) / 2)] + sortedValues[Math.floor(n / 2)];
  let activeSetSize = totalPairs;
  let previousCount = 0;

  // Bound the selection loop. On valid sorted input the Monahan selection
  // converges in O(log n) iterations; this cap is far higher than ever needed
  // for sorted input but guarantees termination on misuse (e.g.,
  // `assumeSorted=true` on UNSORTED input, which is undefined behavior and
  // would otherwise spin forever). The cap scales with n so large valid
  // inputs are never starved. We also track no-progress (stall) on the
  // active set to bail out deterministically.
  const baseIterations = 256;
  const maxIterations = baseIterations + 4 * n;
  let prevActiveSetSize = -1;
  let stallCount = 0;
  const maxStall = 8;
  let iterations = 0;

  while (true) {
    if (++iterations > maxIterations) {
      throw new Error('Convergence failure (pathological input)');
    }

    // === PARTITION STEP ===
    let countBelowPivot = 0;
    let currentColumn = n;
    const partitionCounts: number[] = [];

    for (let row = 1; row <= n; row++) {
      // Move left from current column until we find sums < pivot
      while (
        currentColumn >= row &&
        sortedValues[row - 1] + sortedValues[currentColumn - 1] >= pivot
      ) {
        currentColumn--;
      }

      // Count elements in this row that are < pivot
      const elementsBelow = currentColumn >= row ? currentColumn - row + 1 : 0;
      partitionCounts.push(elementsBelow);
      countBelowPivot += elementsBelow;
    }

    // === CONVERGENCE CHECK ===
    if (countBelowPivot === previousCount) {
      let minActiveSum = Infinity;
      let maxActiveSum = -Infinity;

      for (let i = 0; i < n; i++) {
        if (leftBounds[i] > rightBounds[i]) {
          continue;
        }

        const rowValue = sortedValues[i];
        const smallestInRow = sortedValues[leftBounds[i] - 1] + rowValue;
        const largestInRow = sortedValues[rightBounds[i] - 1] + rowValue;

        minActiveSum = Math.min(minActiveSum, smallestInRow);
        maxActiveSum = Math.max(maxActiveSum, largestInRow);
      }

      pivot = 0.5 * minActiveSum + 0.5 * maxActiveSum;
      if (pivot <= minActiveSum || pivot > maxActiveSum) {
        pivot = maxActiveSum;
      }

      if (minActiveSum === maxActiveSum || activeSetSize <= 2) {
        return pivot / 2;
      }

      continue;
    }

    // === TARGET CHECK ===
    const atTargetRank =
      countBelowPivot === medianRankLow || countBelowPivot === medianRankHigh - 1;

    if (atTargetRank) {
      let largestBelowPivot = -Infinity;
      let smallestAtOrAbovePivot = Infinity;

      for (let i = 0; i < n; i++) {
        const countInRow = partitionCounts[i];
        const rowValue = sortedValues[i];
        const totalInRow = n - i;

        // Find largest sum in this row that's < pivot
        if (countInRow > 0) {
          const lastBelowIndex = i + countInRow;
          const lastBelowValue = rowValue + sortedValues[lastBelowIndex - 1];
          largestBelowPivot = Math.max(largestBelowPivot, lastBelowValue);
        }

        // Find smallest sum in this row that's >= pivot
        if (countInRow < totalInRow) {
          const firstAtOrAboveIndex = i + countInRow + 1;
          const firstAtOrAboveValue = rowValue + sortedValues[firstAtOrAboveIndex - 1];
          smallestAtOrAbovePivot = Math.min(smallestAtOrAbovePivot, firstAtOrAboveValue);
        }
      }

      // Calculate final result
      if (medianRankLow < medianRankHigh) {
        // Even total: average the two middle values
        return (smallestAtOrAbovePivot + largestBelowPivot) / 4;
      } else {
        // Odd total: return the single middle value
        const needLargest = countBelowPivot === medianRankLow;
        return (needLargest ? largestBelowPivot : smallestAtOrAbovePivot) / 2;
      }
    }

    // === UPDATE BOUNDS ===
    if (countBelowPivot < medianRankLow) {
      // Too few values below pivot - search higher
      for (let i = 0; i < n; i++) {
        leftBounds[i] = i + partitionCounts[i] + 1;
      }
    } else {
      // Too many values below pivot - search lower
      for (let i = 0; i < n; i++) {
        rightBounds[i] = i + partitionCounts[i];
      }
    }

    // === PREPARE NEXT ITERATION ===
    previousCount = countBelowPivot;

    // Recalculate active set size
    activeSetSize = 0;
    for (let i = 0; i < n; i++) {
      const rowSize = rightBounds[i] - leftBounds[i] + 1;
      if (rowSize > 0) {
        activeSetSize += rowSize;
      }
    }

    // Stall detection: on valid sorted input the active set strictly
    // shrinks toward the target. If it fails to shrink for several
    // consecutive iterations, the input is pathological (e.g.,
    // assumeSorted=true on unsorted data) and we bail deterministically.
    if (activeSetSize >= prevActiveSetSize && prevActiveSetSize >= 0) {
      stallCount++;
      if (stallCount >= maxStall) {
        throw new Error('Convergence failure (pathological input)');
      }
    } else {
      stallCount = 0;
    }
    prevActiveSetSize = activeSetSize;

    // Choose next pivot
    if (activeSetSize > 2) {
      // Use randomized row median strategy
      const targetIndex = rng.uniformInt(0, activeSetSize);
      let cumulativeSize = 0;
      let selectedRow = 0;

      for (let i = 0; i < n; i++) {
        const rowSize = rightBounds[i] - leftBounds[i] + 1;
        if (rowSize > 0) {
          if (targetIndex < cumulativeSize + rowSize) {
            selectedRow = i;
            break;
          }
          cumulativeSize += rowSize;
        }
      }

      // Use median element of the selected row as pivot
      const medianColumnInRow = Math.floor(
        (leftBounds[selectedRow] + rightBounds[selectedRow]) / 2,
      );
      pivot = sortedValues[selectedRow] + sortedValues[medianColumnInRow - 1];
    } else {
      // Few elements remain - use midrange strategy
      let minRemainingSum = Infinity;
      let maxRemainingSum = -Infinity;

      for (let i = 0; i < n; i++) {
        if (leftBounds[i] > rightBounds[i]) {
          continue;
        }

        const rowValue = sortedValues[i];
        const minInRow = sortedValues[leftBounds[i] - 1] + rowValue;
        const maxInRow = sortedValues[rightBounds[i] - 1] + rowValue;

        minRemainingSum = Math.min(minRemainingSum, minInRow);
        maxRemainingSum = Math.max(maxRemainingSum, maxInRow);
      }

      pivot = 0.5 * minRemainingSum + 0.5 * maxRemainingSum;
      if (pivot <= minRemainingSum || pivot > maxRemainingSum) {
        pivot = maxRemainingSum;
      }

      if (minRemainingSum === maxRemainingSum) {
        return pivot / 2;
      }
    }
  }
}
