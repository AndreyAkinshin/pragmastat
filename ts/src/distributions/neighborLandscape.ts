/**
 * The staircase a NeighborDistModel is: cell boundaries, cumulative values, masses and heights.
 *
 * Point evaluation answers what the density is at a value a caller picked; this answers what the
 * density is, on the boundaries the model itself uses. Drawing the second from the first means
 * guessing a grid and rebuilding the normalization on top of it, which is a second reconstruction
 * of the same object and drifts from this one silently.
 *
 * Every array is a copy handed out by the model, so holding a landscape can never mutate it.
 */
export class NeighborLandscape {
  /** Smoothing scale index k0 = ceil(sqrt(n)). */
  readonly scaleIndex: number;

  /** Window radius r = floor(k0 / 2). */
  readonly radius: number;

  /** Strictly increasing cell boundaries, length cellCount + 1. */
  readonly boundaries: readonly number[];

  /** Cumulative values at the boundaries, from 0 to exactly 1. */
  readonly cumulative: readonly number[];

  /** Strictly positive cell masses, length cellCount. */
  readonly masses: readonly number[];

  /** Cell heights, mass divided by width. */
  readonly densities: readonly number[];

  /** Index of the sorted observation each surviving cell was derived from. */
  readonly sources: readonly number[];

  constructor(
    scaleIndex: number,
    radius: number,
    boundaries: readonly number[],
    cumulative: readonly number[],
    masses: readonly number[],
    densities: readonly number[],
    sources: readonly number[],
  ) {
    this.scaleIndex = scaleIndex;
    this.radius = radius;
    this.boundaries = boundaries;
    this.cumulative = cumulative;
    this.masses = masses;
    this.densities = densities;
    this.sources = sources;
  }

  /** Number of surviving cells. */
  get cellCount(): number {
    return this.masses.length;
  }
}
