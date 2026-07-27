package dev.pragmastat

import kotlin.test.fail

/**
 * Asserts a single binary64 value against a fixture bit for bit.
 *
 * Every suite that uses this returns an element selected out of a finite set built
 * from the input (a pairwise average, a pairwise absolute difference, an order
 * statistic), or an average of two such elements. A divergence is therefore never
 * a small error: either the same element was selected and the answer is
 * bit-identical, or a different one was, and then the gap is data-dependent and no
 * epsilon bounds it. A tolerance hides exactly the failure it appears to guard
 * against.
 *
 * Exactness here is measured, not assumed: recomputing every estimator with each
 * call to log, exp, pow and cos returning the neighbouring representable value
 * (the largest difference two conforming libm implementations can legitimately
 * have) left center, spread, shift, disparity, avg-spread, all of their bounds,
 * compare1 and the margins unmoved on every input.
 *
 * The suites that keep a tolerance are genuinely approximate, and the reason sits
 * on `ReferenceTest.assertClose`.
 *
 * [label] names the position inside the fixture (which field, which index, which
 * bound); the fixture itself is the dynamic test name. A one-ULP report has to be
 * readable, so the message carries both decimal values and both raw payloads.
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
