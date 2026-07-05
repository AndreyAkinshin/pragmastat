package dev.pragmastat

import org.junit.jupiter.api.Test
import org.junit.jupiter.api.Timeout
import org.junit.jupiter.api.assertThrows
import java.util.concurrent.TimeUnit
import kotlin.math.abs
import kotlin.test.assertEquals
import kotlin.test.assertTrue

/**
 * Direct coverage of the raw List-based API's `assumeSorted=true` branch.
 *
 * The reference suite only ever calls the raw estimators with
 * `assumeSorted=false`; the `=true` branch is otherwise reached only
 * transitively via [Sample]. These tests hit it directly through the raw API.
 *
 * Two contracts are checked:
 *
 * - ORDER-INDEPENDENT estimators (center, spread, shift, ratio, disparity,
 *   centerBounds, shiftBounds, ratioBounds, and the internal avgSpread/
 *   avgSpreadBounds): sorting the input ascending and calling with
 *   `assumeSorted=true` must equal the call on the UNSORTED input with
 *   `assumeSorted=false` (within ~1e-9).
 * - SHUFFLE-based bounds (spreadBounds, disparityBounds): `assumeSorted` only
 *   affects the order-independent sparity / shift-bounds sub-computations, never
 *   the disjoint-pair shuffle. On an already-SORTED array with a fixed seed the
 *   result must therefore be byte-identical for true vs false. A SORTED array is
 *   used so the order-independent sub-checks receive valid input under both flag
 *   values (the kernels treat `assumeSorted=true` on unsorted input as undefined
 *   behavior); the shuffle then runs on the identical order regardless.
 *
 * A spreadBounds unsorted-inertness assertion (`true == false` on UNSORTED
 * input) is deliberately absent: with `assumeSorted = true` on unsorted input
 * the spread kernel raises a convergence error instead of returning a value, so
 * there is nothing to compare. Details in the note at the bottom of this class.
 */
class AssumeSortedTest {
    private val misrate = 0.3
    private val seed = "assume-sorted-roundtrip"
    private val tol = 1e-9

    // Unsorted, strictly positive (so ratio is defined), 8 elements (large enough
    // for the 0.3 misrate used by the bounds estimators).
    private val xUnsorted = listOf(5.0, 1.0, 8.0, 3.0, 2.0, 7.0, 4.0, 6.0)
    private val yUnsorted = listOf(12.0, 9.0, 15.0, 10.0, 13.0, 11.0, 16.0, 14.0)

    private val xSorted = xUnsorted.sorted()
    private val ySorted = yUnsorted.sorted()

    private fun assertClose(
        expected: Double,
        actual: Double,
    ) {
        assertTrue(
            abs(expected - actual) < tol,
            "Expected $expected but got $actual (difference: ${abs(expected - actual)})",
        )
    }

    // --- Order-independent point estimators ---

    @Test
    fun centerRoundtrip() {
        assertClose(center(xUnsorted, assumeSorted = false), center(xSorted, assumeSorted = true))
    }

    @Test
    fun spreadRoundtrip() {
        assertClose(spread(xUnsorted, assumeSorted = false), spread(xSorted, assumeSorted = true))
    }

    @Test
    fun shiftRoundtrip() {
        assertClose(
            shift(xUnsorted, yUnsorted, assumeSorted = false),
            shift(xSorted, ySorted, assumeSorted = true),
        )
    }

    @Test
    fun ratioRoundtrip() {
        assertClose(
            ratio(xUnsorted, yUnsorted, assumeSorted = false),
            ratio(xSorted, ySorted, assumeSorted = true),
        )
    }

    @Test
    fun disparityRoundtrip() {
        assertClose(
            disparity(xUnsorted, yUnsorted, assumeSorted = false),
            disparity(xSorted, ySorted, assumeSorted = true),
        )
    }

    @Test
    fun avgSpreadRoundtrip() {
        assertClose(
            avgSpread(xUnsorted, yUnsorted, assumeSorted = false),
            avgSpread(xSorted, ySorted, assumeSorted = true),
        )
    }

    // --- Order-independent bounds estimators ---

    @Test
    fun centerBoundsRoundtrip() {
        val unsorted = centerBounds(xUnsorted, misrate, assumeSorted = false)
        val presorted = centerBounds(xSorted, misrate, assumeSorted = true)
        assertClose(unsorted.lower, presorted.lower)
        assertClose(unsorted.upper, presorted.upper)
    }

    @Test
    fun shiftBoundsRoundtrip() {
        val unsorted = shiftBounds(xUnsorted, yUnsorted, misrate, assumeSorted = false)
        val presorted = shiftBounds(xSorted, ySorted, misrate, assumeSorted = true)
        assertClose(unsorted.lower, presorted.lower)
        assertClose(unsorted.upper, presorted.upper)
    }

    @Test
    fun ratioBoundsRoundtrip() {
        val unsorted = ratioBounds(xUnsorted, yUnsorted, misrate, assumeSorted = false)
        val presorted = ratioBounds(xSorted, ySorted, misrate, assumeSorted = true)
        assertClose(unsorted.lower, presorted.lower)
        assertClose(unsorted.upper, presorted.upper)
    }

    @Test
    fun avgSpreadBoundsRoundtrip() {
        // avgSpreadBounds is internal and shuffle-based; its only assumeSorted
        // effect is via the pre-sorted sparity views. On a SORTED array + fixed
        // seed the shuffle is identical, so passing sorted views must be
        // byte-identical to not passing them.
        val withoutViews = avgSpreadBounds(xSorted, ySorted, misrate, seed)
        val withViews = avgSpreadBounds(xSorted, ySorted, misrate, seed, xSorted, ySorted)
        assertEquals(withoutViews.lower, withViews.lower)
        assertEquals(withoutViews.upper, withViews.upper)
    }

    // --- centerImpl convergence guard on pathological (unsorted) input ---
    //
    // Passing assumeSorted = true on UNSORTED input is undefined behavior. The
    // centerImpl Monahan-selection loop relies on ascending order; on unsorted
    // input it could otherwise spin forever (an unkillable process wedge). The
    // iteration cap + no-progress guard must instead raise a deterministic
    // convergence error QUICKLY rather than hang. This is a LOCAL regression test
    // (not a shared fixture); it asserts termination, not a specific value.
    @Test
    @Timeout(value = 10, unit = TimeUnit.SECONDS)
    fun centerAssumeSortedUnsortedRaisesConvergenceError() {
        // Strongly anti-sorted, n >= 3 so we reach the selection loop (n==1/n==2
        // short-circuit). Descending order is the worst case for the kernel.
        val unsorted = listOf(100.0, 90.0, 80.0, 5.0, 70.0, 1.0, 60.0, 50.0, 3.0, 40.0)
        val ex =
            assertThrows<IllegalStateException> {
                center(unsorted, assumeSorted = true)
            }
        assertTrue(
            ex.message?.contains("Convergence failure", ignoreCase = true) == true,
            "Expected a convergence-failure error, got: ${ex.message}",
        )
    }

    // --- spreadImpl convergence guard on pathological (unsorted) input ---
    //
    // Parallel to centerAssumeSortedUnsortedRaisesConvergenceError above, but for
    // spreadImpl. Its Monahan-selection loop also relies on ascending order;
    // without the iteration cap, spread(UNSORTED, assumeSorted = true) would spin
    // forever inside the kernel (an unkillable process wedge). The cap must
    // instead raise a deterministic convergence error QUICKLY rather than hang.
    // LOCAL regression test (not a shared fixture); asserts termination, not a
    // value.
    @Test
    @Timeout(value = 10, unit = TimeUnit.SECONDS)
    fun spreadAssumeSortedUnsortedRaisesConvergenceError() {
        // Strongly anti-sorted, n >= 3 so we reach the selection loop (n==1/n==2
        // short-circuit). Descending order is the worst case for the kernel.
        val unsorted = listOf(100.0, 90.0, 80.0, 5.0, 70.0, 1.0, 60.0, 50.0, 3.0, 40.0)
        val ex =
            assertThrows<IllegalStateException> {
                spread(unsorted, assumeSorted = true)
            }
        assertTrue(
            ex.message?.contains("Convergence failure", ignoreCase = true) == true,
            "Expected a convergence-failure error, got: ${ex.message}",
        )
    }

    // --- Shuffle-based bounds: assumeSorted true==false on a SORTED array + seed ---

    @Test
    fun spreadBoundsSortedFlagInvariant() {
        val falseResult = spreadBounds(xSorted, misrate, seed, assumeSorted = false)
        val trueResult = spreadBounds(xSorted, misrate, seed, assumeSorted = true)
        assertEquals(falseResult.lower, trueResult.lower)
        assertEquals(falseResult.upper, trueResult.upper)
    }

    @Test
    fun disparityBoundsSortedFlagInvariant() {
        val falseResult = disparityBounds(xSorted, ySorted, misrate, seed, assumeSorted = false)
        val trueResult = disparityBounds(xSorted, ySorted, misrate, seed, assumeSorted = true)
        assertEquals(falseResult.lower, trueResult.lower)
        assertEquals(falseResult.upper, trueResult.upper)
    }

    // --- spreadBounds UNSORTED-inertness ---
    //
    // A tempting additional assertion would be that, for spreadBounds ONLY,
    //
    //     spreadBounds(UNSORTED, misrate, seed, assumeSorted = true) ==
    //     spreadBounds(UNSORTED, misrate, seed, assumeSorted = false)
    //
    // byte-identical, on the premise that the flag only reaches the
    // order-independent `spread > 0` sparity predicate.
    //
    // That premise holds MATHEMATICALLY but NOT for this implementation. When
    // assumeSorted = true is passed on UNSORTED input, spreadBoundsImpl wires the
    // sparity check to spreadImpl(x, assumeSorted = true) with x still unsorted
    // (see Estimators.kt: `sortedX = if (assumeSorted) x else null`). The Monahan
    // selection kernel in spreadImpl relies on `a` being sorted ascending; on
    // unsorted input with assumeSorted = true its two-pointer partition never
    // converges. Without the iteration cap that would spin forever; with the cap
    // it raises a deterministic convergence error instead (pinned by
    // spreadAssumeSortedUnsortedRaisesConvergenceError above). Either way the
    // call cannot return a meaningful value, so this is the documented "passing
    // assumeSorted = true on unsorted input is undefined behavior" contract
    // biting.
    //
    // Because the unsorted true-flag call ERRORS rather than returning, an
    // equality assertion against the false-flag result is not meaningful. The
    // terminating, well-defined coverage is the sorted-array flag invariant
    // (spreadBoundsSortedFlagInvariant above), matching the Go and Python suites.
}
