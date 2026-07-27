package dev.pragmastat

import kotlin.test.fail

/**
 * Asserts one binary64 value against another bit for bit.
 *
 * Two kinds of comparison route through here, and the case for exactness is
 * different for each.
 *
 * ACROSS IMPLEMENTATIONS (a fixture against this port, in `ReferenceTest` and
 * `MetrologyTest`): the value is an element selected out of a finite set built
 * from the input (a pairwise average, a pairwise absolute difference, an order
 * statistic), or an average of two such elements. A divergence is therefore never
 * a small error: either the same element was selected and the answer is
 * bit-identical, or a different one was, and then the gap is data-dependent and no
 * epsilon bounds it. A tolerance hides exactly the failure it appears to guard
 * against. Exactness here is measured, not assumed: recomputing every estimator
 * with each call to log, exp, pow and cos returning the neighbouring representable
 * value (the largest difference two conforming libm implementations can
 * legitimately have) left center, spread, shift, disparity, avg-spread, all of
 * their bounds, compare1 and the margins unmoved on every input.
 *
 * WITHIN THIS PORT (two entry points, or a sorted input against the same input
 * unsorted, in `AssumeSortedTest` and `CenterMidpointSymmetryTest`): the argument
 * is stronger still, and does not depend on the measurement above. Both sides are
 * the SAME implementation reaching the SAME kernel over the SAME values; there is
 * no arithmetic between them for a rounding to enter. Either the payloads agree to
 * the last bit or one path does something the other does not, which is a defect. A
 * tolerance there states no numerical fact, it only widens the window in which the
 * defect goes unreported. This covers ratio as well: it does go through log and
 * exp, but both sides take that route on identical inputs, so whatever the
 * platform libm returns, it returns it twice.
 *
 * The suites that keep a tolerance are genuinely approximate, and the reasons sit
 * on `ReferenceTest.assertClose` (ratio and compare2 across implementations) and
 * on `InvarianceTest` (equivariance properties, where the expectation carries its
 * own inexact arithmetic).
 *
 * [label] names the position being compared (which field, which index, which
 * bound, or which pair of paths); for the fixture suites the fixture itself is the
 * dynamic test name. A one-ULP report has to be readable, so the message carries
 * both decimal values and both raw payloads.
 */
internal fun assertBitwise(
    expected: Double,
    actual: Double,
    label: String,
) {
    val expectedBits = expected.toRawBits()
    val actualBits = actual.toRawBits()
    if (expectedBits == actualBits) {
        return
    }
    fail(
        "$label: expected $expected (${hexBits(expectedBits)}) " +
            "but got $actual (${hexBits(actualBits)})",
    )
}

private fun hexBits(bits: Long): String = "0x" + bits.toULong().toString(16).padStart(16, '0')
