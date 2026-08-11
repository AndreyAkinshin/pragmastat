package dev.pragmastat.distributions

/**
 * The staircase a [LinearDistModel] is: cell boundaries, cumulative values and heights.
 *
 * The model interpolates the order statistics linearly, so its density is a staircase: one cell
 * between each pair of adjacent observations, each carrying the same mass 1/(n-1) and therefore a
 * height that is that mass over its width.
 *
 * Every list is a copy handed out by the model, so holding a landscape can never mutate it.
 *
 * @property boundaries The sorted observations, which are the cell boundaries; length [cellCount] + 1.
 * @property cumulative Cumulative values at the boundaries, from 0 to exactly 1.
 * @property densities Cell heights, mass divided by width; length [cellCount].
 */
class LinearLandscape(
    val boundaries: List<Double>,
    val cumulative: List<Double>,
    val densities: List<Double>,
) {
    /** Number of cells, one fewer than the number of observations. */
    val cellCount: Int get() = densities.size
}
