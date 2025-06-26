/**
 * Utility functions for statistical calculations
 */

/**
 * Calculate the median of an array of numbers
 * @param values Array of numbers
 * @returns The median value
 */
export function median(values: number[]): number {
  if (values.length === 0) {
    return 0;
  }

  const sorted = [...values].sort((a, b) => a - b);
  const mid = Math.floor(sorted.length / 2);

  if (sorted.length % 2 === 0) {
    return (sorted[mid - 1] + sorted[mid]) / 2;
  } else {
    return sorted[mid];
  }
}

/**
 * Generate all pairwise combinations of indices
 * @param n Number of elements
 * @param includeDiagonal Whether to include pairs (i, i)
 * @returns Array of index pairs [i, j]
 */
export function getPairs(n: number, includeDiagonal: boolean = true): [number, number][] {
  const pairs: [number, number][] = [];

  for (let i = 0; i < n; i++) {
    const startJ = includeDiagonal ? i : i + 1;
    for (let j = startJ; j < n; j++) {
      pairs.push([i, j]);
    }
  }

  return pairs;
}
