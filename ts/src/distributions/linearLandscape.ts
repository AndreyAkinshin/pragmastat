/**
 * The staircase a LinearDistModel is: cell boundaries, cumulative values and heights.
 *
 * The model interpolates the order statistics linearly, so its density is a staircase: one cell
 * between each pair of adjacent observations, each carrying the same mass 1/(n-1) and therefore a
 * height that is that mass over its width. Every array is a copy handed out by the model, so
 * holding a landscape can never mutate it.
 */
export class LinearLandscape {
  /** The sorted observations, which are the cell boundaries; length cellCount + 1. */
  readonly boundaries: readonly number[];

  /** Cumulative values at the boundaries, from 0 to exactly 1. */
  readonly cumulative: readonly number[];

  /** Cell heights, mass divided by width; length cellCount. */
  readonly densities: readonly number[];

  constructor(
    boundaries: readonly number[],
    cumulative: readonly number[],
    densities: readonly number[],
  ) {
    this.boundaries = boundaries;
    this.cumulative = cumulative;
    this.densities = densities;
  }

  /** Number of cells, one fewer than the number of observations. */
  get cellCount(): number {
    return this.densities.length;
  }
}
