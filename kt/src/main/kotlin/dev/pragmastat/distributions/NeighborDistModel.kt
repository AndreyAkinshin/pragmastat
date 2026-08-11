package dev.pragmastat.distributions

import dev.pragmastat.DistNumerics
import dev.pragmastat.DistSampleGuard
import dev.pragmastat.Measurement
import dev.pragmastat.MeasurementUnit
import dev.pragmastat.NumberUnit
import dev.pragmastat.Probability
import dev.pragmastat.Rng
import dev.pragmastat.Sample
import dev.pragmastat.Subject
import dev.pragmastat.checkCumulance
import dev.pragmastat.checkQueryValidity
import dev.pragmastat.checkValidity
import dev.pragmastat.normalizeZero
import kotlin.math.max
import kotlin.math.min

/**
 * Local step-density distribution reconstructed from a sample (algorithm `dist-neighbor-v1`).
 *
 * Each observation contributes a cell whose height reflects how tightly its nearest neighbors
 * are packed, so the reconstruction follows local structure instead of imposing a single global
 * shape. The window is indexed by k0 = ceil(sqrt(n)) with radius r = floor(k0 / 2); windows near
 * the ends are truncated and the numerator is always the actual neighbor count.
 *
 * The construction is kNN-inspired but is not the Loftsgaarden-Quesenberry estimator: support is
 * bounded by the observed range and the cell masses are explicitly normalized, so the density
 * integrates to one.
 *
 * The model is immutable, so concurrent quantile/cumulative/density reads on a built model are
 * safe.
 *
 * Both entry points are available: a native numeric one and a measured one built from a [Sample].
 */
class NeighborDistModel private constructor(
    x: DoubleArray,
    /** Unit of the observations this model was built from. */
    val unit: MeasurementUnit,
) : Distribution {
    private val b: DoubleArray
    private val f: DoubleArray
    private val p: DoubleArray
    private val d: DoubleArray
    private val sources: IntArray
    private val scaleIndex: Int
    private val radius: Int

    init {
        val n = x.size
        scaleIndex = DistNumerics.ceilSqrt(n)
        radius = scaleIndex / 2

        val counts = IntArray(n)
        val spans = DoubleArray(n)
        for (i in 0 until n) {
            val left = max(0, i - radius)
            val right = min(n - 1, i + radius)
            counts[i] = right - left + 1
            spans[i] = x[right] - x[left]
        }

        val e = DoubleArray(n + 1)
        e[0] = x[0]
        for (i in 1 until n) {
            e[i] = DistNumerics.midpoint(x[i - 1], x[i])
        }
        e[n] = x[n - 1]

        // Rounding can collapse the midpoint of two adjacent binary64 observations onto a
        // neighboring boundary. Such a cell carries no mass and is dropped; the surviving cells
        // keep the index of the observation they came from.
        val boundaries = ArrayList<Double>(n + 1)
        boundaries.add(e[0])
        val surviving = ArrayList<Int>(n)
        for (i in 0 until n) {
            if (e[i + 1] == e[i]) {
                continue
            }
            surviving.add(i)
            boundaries.add(e[i + 1])
        }

        val s = surviving.size
        val a = DoubleArray(s)
        for (j in 0 until s) {
            val i = surviving[j]
            val width = e[i + 1] - e[i]
            a[j] = counts[i].toDouble() / n.toDouble() * (width / spans[i])
        }

        var total = 0.0
        for (j in 0 until s) {
            total += a[j]
        }
        // A broken assumeSorted promise can make a cell wider than the window it normalizes
        // against, so its unnormalized mass overflows, or it can collapse every cell and leave
        // a zero total. Either way the divisor is unusable and NaN would reach the query
        // kernels. The sample itself satisfies domain(x) and distinctness(x), so this is not an
        // assumption violation: it is a numerical-conformance failure, reported through the
        // same channel unitRatio uses. For a genuinely ascending sample the total is finite and
        // strictly positive. It is not bounded by 1: the total is a normalizer, not a
        // probability. It sits above 1 and decays toward it as n grows.
        if (!total.isFinite() || total <= 0.0) {
            throw IllegalStateException(
                "Numerical conformance failure: the NeighborDistModel normalizer is not usable; " +
                    "this indicates an assumeSorted promise that the input does not satisfy",
            )
        }

        // Reserve one representable step for every cell that has not been placed yet, so a mass
        // too small to register at this point in the prefix still advances the cumulative value
        // instead of flattening it into a plateau.
        val cap = DoubleArray(s + 1)
        cap[s] = 1.0
        for (j in s - 1 downTo 0) {
            cap[j] = DistNumerics.nextDown(cap[j + 1])
        }

        f = DoubleArray(s + 1)
        f[0] = 0.0
        for (j in 0 until s - 1) {
            val raw = f[j] + a[j] / total
            f[j + 1] = min(cap[j + 1], max(DistNumerics.nextUp(f[j]), raw))
        }

        f[s] = 1.0

        b = boundaries.toDoubleArray()
        sources = surviving.toIntArray()
        p = DoubleArray(s)
        d = DoubleArray(s)
        for (j in 0 until s) {
            p[j] = f[j + 1] - f[j]
            d[j] = p[j] / (b[j + 1] - b[j])
        }
    }

    /**
     * Builds the model from a native numeric vector.
     *
     * @param values Fitting vector; must be non-empty, finite, of size at least two,
     *   with a finite representable range and pairwise distinct elements.
     * @param assumeSorted When true, the caller promises the input is already ascending
     *   and the internal copy is not sorted.
     */
    constructor(
        values: List<Double>,
        assumeSorted: Boolean = false,
    ) : this(checked(values, assumeSorted), NumberUnit)

    /**
     * Builds the model from a measured sample.
     *
     * The sample already stores its values in ascending order, so the model does not sort again.
     * Weighted samples are refused: this reconstruction has no weighted form, and silently
     * ignoring the weights would answer a different question than the caller asked.
     */
    constructor(x: Sample) : this(prepared(x), x.unit)

    /** Number of surviving cells in the reconstruction. */
    val cellCount: Int get() = p.size

    /** Lower end of the support. */
    val supportMin: Double get() = b[0]

    /** Upper end of the support. */
    val supportMax: Double get() = b[b.size - 1]

    /**
     * Value at each requested cumulance, in the order the cumulances were given.
     */
    fun quantile(cumulances: List<Double>): List<Double> {
        checkQueryValidity(cumulances)
        checkCumulance(cumulances)
        return quantileTrusted(cumulances)
    }

    /**
     * Probability of not exceeding each requested value, in the order the values were given.
     */
    fun cumulative(values: List<Double>): List<Double> {
        checkQueryValidity(values)
        return cumulativeTrusted(values)
    }

    /**
     * Density at each requested value, in the order the values were given.
     */
    fun density(values: List<Double>): List<Double> {
        checkQueryValidity(values)
        return densityTrusted(values)
    }

    // ── Measured facade ───────────────────────────────────────────────────────
    //
    // The metrology path, alongside the numeric one, exactly as every estimator offers both. Only
    // Quantile carries a unit -- the sample's own; a probability and a density are plain numbers.

    /**
     * The measured view of this model.
     *
     * The metrology path lives behind its own accessor rather than as overloads of the numeric
     * methods. `List<Double>` and `List<Probability>` erase to the same JVM signature, so the
     * three pairs would clash on the platform and could only coexist behind `@JvmName` aliases
     * that no Kotlin caller ever writes and every Java caller has to learn. A separate view keeps
     * both paths under their natural names and states which world a call belongs to.
     */
    val measured: MeasuredView get() = MeasuredView(this)

    /** Measured queries against a built [NeighborDistModel]. */
    class MeasuredView internal constructor(
        private val model: NeighborDistModel,
    ) {
        /** Values at the requested cumulances, in the unit of the sample. */
        fun quantile(cumulances: List<Probability>): List<Measurement> =
            DistSampleGuard.toMeasurements(model.quantile(cumulances.map { it.value }), model.unit)

        /** Probabilities of not exceeding the requested measurements. */
        fun cumulative(values: List<Measurement>): List<Probability> =
            model.cumulative(DistSampleGuard.toRaw(values, model.unit)).map { Probability(it) }

        /** Densities at the requested measurements, as plain numbers. */
        fun density(values: List<Measurement>): List<Double> = model.density(DistSampleGuard.toRaw(values, model.unit))
    }

    override fun sample(rng: Rng): Double = quantileAt(rng.uniformDouble())

    private fun quantileTrusted(cumulances: List<Double>): List<Double> = cumulances.map { quantileAt(it) }

    private fun cumulativeTrusted(values: List<Double>): List<Double> = values.map { cumulativeAt(it) }

    private fun densityTrusted(values: List<Double>): List<Double> = values.map { densityAt(it) }

    /**
     * NaN first, for the same reason [LinearDistModel.quantileAt] checks it: every branch below
     * compares false against it, and the fallback would otherwise depend on how the platform
     * casts a NaN cumulance to an index.
     */
    override fun quantileAt(p: Double): Double {
        if (p.isNaN()) {
            return Double.NaN
        }

        val s = this.p.size
        val value =
            if (p <= 0.0) {
                b[0]
            } else if (p >= 1.0) {
                b[s]
            } else {
                val j = DistNumerics.lowerBound(f, 1, s + 1, p) - 1
                val t = DistNumerics.unitRatio(p - f[j], this.p[j])
                DistNumerics.lerp(b[j], b[j + 1], t)
            }

        // Every branch hands back a value built from the sample's own boundaries -- the clamped
        // ones return one verbatim, and lerp reproduces its endpoints exactly -- so a -0.0 among
        // the observations would otherwise reach the caller unnormalized.
        return normalizeZero(value)
    }

    override fun cumulativeAt(x: Double): Double {
        if (x.isNaN()) {
            return Double.NaN
        }

        val s = p.size
        if (x <= b[0]) {
            return 0.0
        }
        if (x >= b[s]) {
            return 1.0
        }

        val j = DistNumerics.upperBound(b, x) - 1
        val t = DistNumerics.unitRatio(x - b[j], b[j + 1] - b[j])
        return DistNumerics.lerp(f[j], f[j + 1], t)
    }

    override fun densityAt(x: Double): Double {
        if (x.isNaN()) {
            return Double.NaN
        }

        val s = p.size
        if (x < b[0] || x > b[s]) {
            return 0.0
        }

        val j = if (x == b[s]) s - 1 else DistNumerics.upperBound(b, x) - 1
        return p[j] / (b[j + 1] - b[j])
    }

    /**
     * The staircase this model is: cell boundaries, cumulative values, masses and heights.
     *
     * Point evaluation answers what the density is at a value a caller picked; this answers what
     * the density *is*, on the boundaries the model itself uses.
     */
    fun getLandscape(): NeighborLandscape =
        NeighborLandscape(
            scaleIndex,
            radius,
            b.toList(),
            f.toList(),
            p.toList(),
            d.toList(),
            sources.toList(),
        )

    companion object {
        /** Normative algorithm identifier. */
        const val ALGORITHM: String = "dist-neighbor-v1"

        private fun checked(
            values: List<Double>,
            assumeSorted: Boolean,
        ): DoubleArray {
            checkValidity(values, Subject.X)
            return DistSampleGuard.prepare(values, assumeSorted)
        }

        // Sample construction already enforced validity, and its values are ascending.
        private fun prepared(x: Sample): DoubleArray {
            Sample.checkNonWeighted("x", x)
            return DistSampleGuard.prepare(x.sortedValues, true)
        }
    }
}
