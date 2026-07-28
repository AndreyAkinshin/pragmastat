package dev.pragmastat

import kotlin.test.fail

/**
 * Asserts one binary64 value against another bit for bit.
 *
 * This is the ONLY spelling of "these two floating-point values are identical" in
 * the suite. Everything that claims exactness routes through it, scalars and
 * sequences alike, so that the claim means one thing everywhere.
 *
 * The comparison is on the RAW payload (`toRawBits`, i.e.
 * `Double.doubleToRawLongBits`), not on `==` and not on boxed `equals`:
 *
 * - `==` is not bit equality. `-0.0 == 0.0` is true, so a sign-of-zero divergence
 *   passes a check whose whole claim is that seven implementations return
 *   identical bits; and `NaN == NaN` is false, so two identical NaNs fail a check
 *   that bit equality passes. Neither direction is uniformly stronger, and only
 *   bit equality matches what these predicates say.
 * - Boxed `equals` (what `assertEquals` reaches through a generic parameter) does
 *   separate the zeros, but it routes through `doubleToLongBits`, which collapses
 *   every NaN to one canonical payload. A port that emits a different NaN payload
 *   than the fixture is a divergence, and this is the one comparison that reports
 *   it.
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
 * with each call to log, exp, pow and cos returning the neighboring representable
 * value (the largest difference two conforming libm implementations can
 * legitimately have) left center, spread, shift, disparity, avg-spread, all of
 * their bounds, compare1 and the margins unmoved on every input. The seeded
 * streams (`Rng`, the uniform distribution, shuffle/sample/resample) are bitwise
 * for a simpler reason: the randomization contract is that a seed produces the
 * same sequence in every language, so a one-ULP drift, such as an arm64 compiler
 * fusing a multiply into an add, is a broken contract and not a rounding.
 *
 * WITHIN THIS PORT (two entry points, or a sorted input against the same input
 * unsorted, in `AssumeSortedTest` and `CenterMidpointSymmetryTest`; or an input
 * list against its pre-call snapshot in `MutationTest`): the argument is stronger
 * still, and does not depend on the measurement above. Both sides are the SAME
 * implementation reaching the SAME kernel over the SAME values; there is no
 * arithmetic between them for a rounding to enter. Either the payloads agree to
 * the last bit or one path does something the other does not, which is a defect. A
 * tolerance there states no numerical fact, it only widens the window in which the
 * defect goes unreported. This covers ratio as well: it does go through log and
 * exp, but both sides take that route on identical inputs, so whatever the
 * platform libm returns, it returns it twice.
 *
 * The suites that keep a tolerance are genuinely approximate, and the reasons sit
 * on `ReferenceTest.assertClose` (ratio, and the ratio projections of compare2,
 * across implementations) and on `InvarianceTest` (equivariance properties, where
 * the expectation carries its own inexact arithmetic). compare2 is resolved per
 * projection rather than per suite: a projection is compared here unless its own
 * threshold names the ratio metric.
 *
 * [label] names the position being compared (which field, which index, which
 * bound, or which pair of paths); for the fixture suites the fixture itself is the
 * dynamic test name. A one-ULP report has to be readable, and a sign-of-zero
 * report is unreadable without the bits, so the message carries both decimal
 * values and both raw payloads.
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

/**
 * Asserts one binary32 value against another bit for bit.
 *
 * The `uniformFloat` stream is part of the same randomization contract as the
 * binary64 draws, and `==` on a Float carries the identical defect described on
 * the binary64 overload above.
 */
internal fun assertBitwise(
    expected: Float,
    actual: Float,
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

/**
 * Asserts two binary64 sequences against each other element by element, bit for
 * bit.
 *
 * A sequence comparison that walks the elements with `==` (or leans on `List`
 * equality, which walks them with boxed `equals`) has exactly the defect the
 * scalar overload documents, so it delegates to the scalar overload rather than
 * repeating it. Length is reported before any element, since a length mismatch
 * makes every index report misleading.
 */
@JvmName("assertBitwiseDoubles")
internal fun assertBitwise(
    expected: List<Double>,
    actual: List<Double>,
    label: String,
) {
    assertSameSize(expected.size, actual.size, label)
    for (i in expected.indices) {
        assertBitwise(expected[i], actual[i], "$label: element $i")
    }
}

/**
 * Asserts that a binary64 value is not a negative zero.
 *
 * The negated form of the same claim, for the positions where the expected value is not known in
 * advance and only the sign of a possible zero is being pinned: no public estimator may report
 * `-0.0` (see `normalizeZero`). It lives here because the payload is the only way to state it —
 * `actual == -0.0` is true of `+0.0` as well, and so asserts nothing at all.
 */
internal fun assertNotNegativeZero(
    actual: Double,
    label: String,
) {
    val actualBits = actual.toRawBits()
    if (actualBits != (-0.0).toRawBits()) {
        return
    }
    fail("$label: expected 0.0 (${hexBits(0.0.toRawBits())}) but got $actual (${hexBits(actualBits)})")
}

/** Sequence form of the binary32 comparison. See the binary64 overload. */
@JvmName("assertBitwiseFloats")
internal fun assertBitwise(
    expected: List<Float>,
    actual: List<Float>,
    label: String,
) {
    assertSameSize(expected.size, actual.size, label)
    for (i in expected.indices) {
        assertBitwise(expected[i], actual[i], "$label: element $i")
    }
}

private fun assertSameSize(
    expected: Int,
    actual: Int,
    label: String,
) {
    if (expected != actual) {
        fail("$label: expected $expected elements but got $actual")
    }
}

private fun hexBits(bits: Long): String = "0x" + bits.toULong().toString(16).padStart(16, '0')

private fun hexBits(bits: Int): String = "0x" + bits.toUInt().toString(16).padStart(8, '0')
