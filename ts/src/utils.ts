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
    throw new Error('Input array cannot be empty');
  }

  const sorted = [...values].sort((a, b) => a - b);
  const mid = Math.floor(sorted.length / 2);

  if (sorted.length % 2 === 0) {
    return (sorted[mid - 1] + sorted[mid]) / 2;
  } else {
    return sorted[mid];
  }
}
