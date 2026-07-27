package dev.pragmastat

import org.junit.jupiter.api.Test

/**
 * Regression guard: the raw (native-array) List-based API must NOT mutate the
 * caller's list. Both branches of the `assumeSorted` flag are covered:
 *
 * - Default (`assumeSorted = false`): the kernels sort a copy internally, so
 *   every point and bounds estimator is fed a MUTABLE, UNSORTED list and the
 *   list must be unchanged afterward.
 * - Aliasing (`assumeSorted = true`): the defensive sort is skipped and the
 *   kernels operate on the caller's list DIRECTLY, so any in-place mutation
 *   would corrupt caller data. Every estimator is fed a MUTABLE, pre-SORTED
 *   list (the flag is undefined behavior on unsorted input) and the list must
 *   be unchanged afterward.
 *
 * Both one- and two-sample shapes are covered for each branch.
 *
 * The list-against-snapshot comparison is BITWISE, through [assertBitwise]: the
 * claim is that the caller's values are untouched, so any change to any payload
 * is a defect, and there is no rounding for a tolerance to absorb.
 */
class MutationTest {
    private val misrate = 0.3

    // Unsorted, 8 elements: large enough that a misrate of 0.3 is valid for the
    // bounds estimators.
    private fun fresh(): MutableList<Double> = mutableListOf(5.0, 1.0, 8.0, 3.0, 2.0, 7.0, 4.0, 6.0)

    // Sorted ascending: assumeSorted = true is only defined on sorted input.
    private fun freshSorted(): MutableList<Double> = fresh().apply { sort() }

    @Test
    fun oneSampleDoesNotMutate() {
        run {
            val x = fresh()
            val snapshot = x.toList()
            center(x)
            assertBitwise(snapshot, x, "center mutated its input")
        }
        run {
            val x = fresh()
            val snapshot = x.toList()
            spread(x)
            assertBitwise(snapshot, x, "spread mutated its input")
        }
        run {
            val x = fresh()
            val snapshot = x.toList()
            centerBounds(x, misrate)
            assertBitwise(snapshot, x, "centerBounds mutated its input")
        }
        run {
            val x = fresh()
            val snapshot = x.toList()
            spreadBounds(x, misrate)
            assertBitwise(snapshot, x, "spreadBounds mutated its input")
        }
    }

    @Test
    fun twoSampleDoesNotMutate() {
        val pointEstimators: List<Pair<String, (List<Double>, List<Double>) -> Unit>> =
            listOf(
                "shift" to { x, y -> shift(x, y) },
                "ratio" to { x, y -> ratio(x, y) },
                "disparity" to { x, y -> disparity(x, y) },
                "avgSpread" to { x, y -> avgSpread(x, y) },
            )
        for ((name, estimator) in pointEstimators) {
            val x = fresh()
            val y = fresh()
            val snapshotX = x.toList()
            val snapshotY = y.toList()
            estimator(x, y)
            assertBitwise(snapshotX, x, "$name mutated its x input")
            assertBitwise(snapshotY, y, "$name mutated its y input")
        }

        val boundsEstimators: List<Pair<String, (List<Double>, List<Double>) -> Unit>> =
            listOf(
                "shiftBounds" to { x, y -> shiftBounds(x, y, misrate) },
                "ratioBounds" to { x, y -> ratioBounds(x, y, misrate) },
                "disparityBounds" to { x, y -> disparityBounds(x, y, misrate) },
            )
        for ((name, estimator) in boundsEstimators) {
            val x = fresh()
            val y = fresh()
            val snapshotX = x.toList()
            val snapshotY = y.toList()
            estimator(x, y)
            assertBitwise(snapshotX, x, "$name mutated its x input")
            assertBitwise(snapshotY, y, "$name mutated its y input")
        }
    }

    // --- Aliasing branch: assumeSorted = true skips the defensive copy, so the
    // kernels see the caller's MutableList directly and must treat it read-only.

    @Test
    fun oneSampleAssumeSortedDoesNotMutate() {
        val estimators: List<Pair<String, (List<Double>) -> Unit>> =
            listOf(
                "center" to { x -> center(x, assumeSorted = true) },
                "spread" to { x -> spread(x, assumeSorted = true) },
                "centerBounds" to { x -> centerBounds(x, misrate, assumeSorted = true) },
                "spreadBounds" to { x -> spreadBounds(x, misrate, assumeSorted = true) },
            )
        for ((name, estimator) in estimators) {
            val x = freshSorted()
            val snapshot = x.toList()
            estimator(x)
            assertBitwise(snapshot, x, "$name(assumeSorted = true) mutated its input")
        }
    }

    @Test
    fun twoSampleAssumeSortedDoesNotMutate() {
        val estimators: List<Pair<String, (List<Double>, List<Double>) -> Unit>> =
            listOf(
                "shift" to { x, y -> shift(x, y, assumeSorted = true) },
                "ratio" to { x, y -> ratio(x, y, assumeSorted = true) },
                "disparity" to { x, y -> disparity(x, y, assumeSorted = true) },
                "avgSpread" to { x, y -> avgSpread(x, y, assumeSorted = true) },
                "shiftBounds" to { x, y -> shiftBounds(x, y, misrate, assumeSorted = true) },
                "ratioBounds" to { x, y -> ratioBounds(x, y, misrate, assumeSorted = true) },
                "disparityBounds" to { x, y -> disparityBounds(x, y, misrate, assumeSorted = true) },
            )
        for ((name, estimator) in estimators) {
            val x = freshSorted()
            val y = freshSorted()
            val snapshotX = x.toList()
            val snapshotY = y.toList()
            estimator(x, y)
            assertBitwise(snapshotX, x, "$name(assumeSorted = true) mutated its x input")
            assertBitwise(snapshotY, y, "$name(assumeSorted = true) mutated its y input")
        }
    }
}
