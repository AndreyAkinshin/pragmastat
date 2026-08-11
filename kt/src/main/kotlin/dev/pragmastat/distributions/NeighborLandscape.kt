package dev.pragmastat.distributions

/**
 * The staircase a [NeighborDistModel] is: cell boundaries, cumulative values, masses and heights.
 *
 * Point evaluation answers what the density is at a value a caller picked; this answers what the
 * density *is*, on the boundaries the model itself uses. Drawing the second from the first means
 * guessing a grid and rebuilding the normalization on top of it, which is a second reconstruction
 * of the same object and drifts from this one silently.
 *
 * Every list is a copy handed out by the model, so holding a landscape can never mutate it.
 *
 * @property scaleIndex Smoothing scale index k0 = ceil(sqrt(n)).
 * @property radius Window radius r = floor(k0 / 2).
 * @property boundaries Strictly increasing cell boundaries, length [cellCount] + 1.
 * @property cumulative Cumulative values at the boundaries, from 0 to exactly 1.
 * @property masses Strictly positive cell masses, length [cellCount].
 * @property densities Cell heights, mass divided by width.
 * @property sources Index of the sorted observation each surviving cell was derived from.
 */
class NeighborLandscape(
    val scaleIndex: Int,
    val radius: Int,
    val boundaries: List<Double>,
    val cumulative: List<Double>,
    val masses: List<Double>,
    val densities: List<Double>,
    val sources: List<Int>,
) {
    /** Number of surviving cells. */
    val cellCount: Int get() = masses.size
}
