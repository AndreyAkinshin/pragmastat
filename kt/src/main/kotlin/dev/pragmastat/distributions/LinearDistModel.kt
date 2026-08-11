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
import kotlin.math.floor

/**
 * Piecewise-linear distribution reconstructed from a sample (algorithm `dist-linear-v1`).
 *
 * The model interpolates the order statistics at the anchors (i-1)/(n-1), which is the
 * Hyndman-Fan type 7 rule. It is immutable: the constructor copies and, unless the caller
 * promises otherwise, sorts the input, so concurrent quantile/cumulative/density reads on a
 * built model are safe.
 *
 * Support is [min(x), max(x)]. Both entry points are available: a native numeric one and a
 * measured one built from a [Sample]. The numeric vector methods expose plain numbers; the
 * measured ones, reached through [measured], attach the model's own unit.
 */
class LinearDistModel private constructor(
    private val x: DoubleArray,
    /** Unit of the observations this model was built from. */
    val unit: MeasurementUnit,
) : Distribution {
    /**
     * Builds the model from a native numeric vector.
     *
     * @param values Fitting vector; must be non-empty, finite, of size at least two,
     *   with a finite representable range and pairwise distinct elements.
     * @param assumeSorted When true, the caller promises the input is already ascending
     *   and the internal copy is not sorted. Passing an unsorted list under this flag is a
     *   contract violation with an unspecified but terminating result.
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

    /** Number of observations backing the model. */
    val size: Int get() = x.size

    /** Lower end of the support. */
    val supportMin: Double get() = x[0]

    /** Upper end of the support. */
    val supportMax: Double get() = x[x.size - 1]

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

    /** Measured queries against a built [LinearDistModel]. */
    class MeasuredView internal constructor(
        private val model: LinearDistModel,
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
     * NaN first: every branch below compares false against it, and the fallback would otherwise
     * depend on how the platform casts a NaN cumulance to an index. The same guard opens
     * [cumulativeAt] and [densityAt].
     */
    override fun quantileAt(p: Double): Double {
        if (p.isNaN()) {
            return Double.NaN
        }

        val n = x.size
        // Clamped rather than compared: the contract answers a cumulance outside (0, 1)
        // with the endpoints of the support, and the vector path rejects one earlier.
        val value =
            if (p <= 0.0) {
                x[0]
            } else if (p >= 1.0) {
                x[n - 1]
            } else {
                val r = (n - 1) * p
                val j = floor(r).toInt()
                val g = r - j
                DistNumerics.lerp(x[j], x[j + 1], g)
            }

        // Every branch hands back a value the sample supplied -- the clamped ones return an
        // order statistic verbatim, and lerp reproduces its endpoints exactly -- so a -0.0 among
        // the observations would otherwise reach the caller unnormalized.
        return normalizeZero(value)
    }

    override fun cumulativeAt(x: Double): Double {
        if (x.isNaN()) {
            return Double.NaN
        }

        val n = this.x.size
        if (x <= this.x[0]) {
            return 0.0
        }
        if (x >= this.x[n - 1]) {
            return 1.0
        }

        val j = DistNumerics.upperBound(this.x, x) - 1
        val t = DistNumerics.unitRatio(x - this.x[j], this.x[j + 1] - this.x[j])
        return (j + t) / (n - 1)
    }

    override fun densityAt(x: Double): Double {
        if (x.isNaN()) {
            return Double.NaN
        }

        val n = this.x.size
        if (x < this.x[0] || x > this.x[n - 1]) {
            return 0.0
        }

        val j = if (x == this.x[n - 1]) n - 2 else DistNumerics.upperBound(this.x, x) - 1
        return 1.0 / (n - 1) / (this.x[j + 1] - this.x[j])
    }

    /**
     * The staircase this model is: cell boundaries, cumulative values and heights.
     *
     * Point evaluation answers what the density is at a value a caller picked; this answers what
     * the density *is*, on the boundaries the model itself uses. Every height is the expression
     * [densityAt] evaluates inside the same cell, so the two agree bit for bit rather than
     * approximately.
     */
    fun getLandscape(): LinearLandscape {
        val n = x.size
        // cumulativeAt(x[i]) always resolves j = i and t = 0.0 for an exact order statistic, so
        // it reduces to exactly i / (n - 1) -- computing that directly skips the O(log n) binary
        // search per boundary without changing a single bit of the result.
        val cumulative = (0 until n).map { it.toDouble() / (n - 1) }
        val densities = (0 until n - 1).map { j -> 1.0 / (n - 1) / (x[j + 1] - x[j]) }
        return LinearLandscape(x.toList(), cumulative, densities)
    }

    companion object {
        /** Normative algorithm identifier. */
        const val ALGORITHM: String = "dist-linear-v1"

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
