package dev.pragmastat

import org.junit.jupiter.api.Test

/**
 * Pins the sign of zero on every value the public estimators report.
 *
 * A sample holding both `+0.0` and `-0.0` leaves the reported sign to the sorting algorithm rather
 * than to the data, because comparison cannot tell the two apart, and each port sorts its own way.
 * The ports disagreed on `center(listOf(0.0, -0.0, 0.0, -0.0, 1.0))`, which the `exact` conformance
 * class forbids: identical inputs must give identical bits. [normalizeZero] closes that on the way
 * out of every estimator, and this suite is what keeps it closed.
 *
 * The comparison is BITWISE, through [assertBitwise] where the expected value is known and through
 * [assertNotNegativeZero] where only the sign is at stake. `==` would prove nothing here:
 * `-0.0 == 0.0` is true, so an equality check passes on exactly the divergence this file exists to
 * report.
 *
 * Two routes reach a negative zero arithmetically: a pairwise average of negative zeros (center and
 * its bounds), and a pairwise difference `x[i] - y[j]` where `x[i]` is `-0.0` and `y[j]` is `+0.0`
 * (shift, its bounds, and everything built on them). The named tests take those routes, and each
 * one goes red without the normalization. The two sweeps state the guarantee for the whole public
 * surface, which includes estimators whose output is structurally out of reach today: `spread` and
 * `spreadBounds` report absolute differences, `ratio` and `ratioBounds` report an exponential, and
 * none of those can carry the sign. The guarantee is about what the API promises, not about which
 * kernel currently computes it.
 */
class NegativeZeroTest {
    /** The samples the guarantee was written against: center must report `+0.0` for each. */
    private val zeroCenterSamples =
        listOf(
            listOf(0.0, -0.0, 0.0, -0.0, 1.0),
            listOf(-0.0, -0.0),
            listOf(-0.0, 0.0),
            listOf(0.0, -0.0),
            listOf(-0.0, -0.0, -0.0),
            listOf(-1.0, 1.0),
            listOf(-2.0, -0.0, 2.0),
        )

    // A tied pair that differs only in the sign of its zeros. Every pairwise difference is one of
    // -1, -0.0, +0.0, +1, so the middle order statistics ARE the signed zeros, which is what puts
    // the sign in reach of shift, disparity and shift bounds. Six elements keep the misrate
    // minimums of every bounds estimator satisfiable.
    private val tiedX = listOf(-0.0, -0.0, -0.0, 1.0, 1.0, 1.0)
    private val tiedY = listOf(0.0, 0.0, 0.0, 1.0, 1.0, 1.0)

    // The same pair shifted into the strictly positive range that ratio and ratioBounds require.
    private val positiveX = tiedX.map { it + 1.0 }
    private val positiveY = tiedY.map { it + 1.0 }

    @Test
    fun centerReportsPositiveZero() {
        for (sample in zeroCenterSamples) {
            assertBitwise(0.0, center(sample), "center($sample)")
        }
    }

    @Test
    fun centerBoundsReportPositiveZero() {
        val x = listOf(0.0, -0.0, 0.0, -0.0, 1.0, -1.0)
        val bounds = centerBounds(x, MISRATE)
        assertBitwise(0.0, bounds.lower, "centerBounds($x).lower")
        assertBitwise(0.0, bounds.upper, "centerBounds($x).upper")
    }

    @Test
    fun shiftAndDisparityReportPositiveZero() {
        // A single pair: the reported value IS the difference -0.0 - 0.0, with no selection or
        // averaging in between, so this is the shortest route from a signed zero to the output.
        assertBitwise(0.0, shift(listOf(-0.0), listOf(0.0)), "shift([-0.0], [0.0])")

        // The same route through samples that pass the sparity check, so disparity
        // (shift / avgSpread, over a strictly positive divisor) carries the sign as well.
        assertBitwise(0.0, shift(tiedX, tiedY), "shift(tiedX, tiedY)")
        assertBitwise(0.0, disparity(tiedX, tiedY), "disparity(tiedX, tiedY)")
    }

    @Test
    fun shiftBoundsReportPositiveZero() {
        // A wide misrate collapses the interval onto the middle order statistics, which for this
        // pair are the signed zeros. Both endpoints therefore land on the value under test.
        val bounds = shiftBounds(tiedX, tiedY, WIDE_MISRATE)
        assertBitwise(0.0, bounds.lower, "shiftBounds(tiedX, tiedY).lower")
        assertBitwise(0.0, bounds.upper, "shiftBounds(tiedX, tiedY).upper")
    }

    @Test
    fun noScalarEstimatorReportsNegativeZero() {
        val estimators: List<Pair<String, () -> Double>> =
            listOf(
                "center" to { center(tiedX) },
                "spread" to { spread(tiedX) },
                "shift" to { shift(tiedX, tiedY) },
                "ratio" to { ratio(positiveX, positiveY) },
                "disparity" to { disparity(tiedX, tiedY) },
                "avgSpread" to { avgSpread(tiedX, tiedY) },
            )
        for ((name, estimator) in estimators) {
            assertNotNegativeZero(estimator(), name)
        }
    }

    @Test
    fun noBoundsEstimatorReportsNegativeZero() {
        val estimators: List<Pair<String, () -> Bounds>> =
            listOf(
                "centerBounds" to { centerBounds(tiedX, WIDE_MISRATE) },
                "spreadBounds" to { spreadBounds(tiedX, WIDE_MISRATE) },
                "spreadBounds(seeded)" to { spreadBounds(tiedX, WIDE_MISRATE, SEED) },
                "shiftBounds" to { shiftBounds(tiedX, tiedY, WIDE_MISRATE) },
                "ratioBounds" to { ratioBounds(positiveX, positiveY, WIDE_MISRATE) },
                "disparityBounds" to { disparityBounds(tiedX, tiedY, WIDE_MISRATE) },
                "disparityBounds(seeded)" to { disparityBounds(tiedX, tiedY, WIDE_MISRATE, SEED) },
                "avgSpreadBounds" to { avgSpreadBounds(tiedX, tiedY, WIDE_MISRATE) },
                "avgSpreadBounds(seeded)" to { avgSpreadBounds(tiedX, tiedY, WIDE_MISRATE, SEED) },
            )
        for ((name, estimator) in estimators) {
            val bounds = estimator()
            assertNotNegativeZero(bounds.lower, "$name.lower")
            assertNotNegativeZero(bounds.upper, "$name.upper")
        }
    }

    /**
     * The unit-aware [Sample] API re-labels bounds built by the raw API instead of rebuilding them,
     * so it inherits the guarantee. This is the guard on that "instead of".
     */
    @Test
    fun theSampleApiReportsPositiveZero() {
        val sec = MeasurementUnit("s", "Time", "s", "Second", 1_000_000_000)
        val x = Sample.of(listOf(0.0, -0.0, 0.0, -0.0, 1.0, -1.0), sec)
        val bounds = centerBounds(x, Probability(MISRATE))
        assertBitwise(0.0, bounds.lower, "Sample centerBounds.lower")
        assertBitwise(0.0, bounds.upper, "Sample centerBounds.upper")
        assertBitwise(0.0, center(x).value, "Sample center")
    }

    /**
     * Only outputs are normalized. An estimator that reached into its argument to drop the sign
     * there would satisfy every check above and silently rewrite caller data.
     */
    @Test
    fun negativeZeroInputsAreLeftAlone() {
        val x = mutableListOf(-0.0, 0.0, 1.0, -1.0, 2.0, -2.0)
        val snapshot = x.toList()
        center(x)
        shift(x, x)
        centerBounds(x, MISRATE)
        assertBitwise(snapshot, x, "an estimator normalized the zeros in its input")
    }

    private companion object {
        // Comfortably above the minimum achievable misrate of centerBounds at six elements.
        const val MISRATE = 0.3

        // Deliberately wide: the interval collapses onto the middle order statistics, which is
        // where the signed zeros of the tied pair sit. It is also above the minimum achievable
        // misrate of the shuffle-based bounds at this sample size.
        const val WIDE_MISRATE = 0.9

        const val SEED = "negative-zero"
    }
}
