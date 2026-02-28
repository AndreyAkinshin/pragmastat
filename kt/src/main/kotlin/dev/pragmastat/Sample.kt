package dev.pragmastat

import kotlin.math.ln

/**
 * A statistical sample: a list of numeric values with optional weights and a measurement unit.
 *
 * Samples are immutable. All transformations (convertTo, log, arithmetic operators)
 * return new Sample instances.
 *
 * Use the companion [of] and [weighted] factory methods to create instances.
 *
 * @property values The raw numeric values
 * @property weights Optional weights (null for unweighted samples)
 * @property unit The measurement unit associated with these values
 */
class Sample private constructor(
    val values: List<Double>,
    val weights: List<Double>?,
    val unit: MeasurementUnit,
    internal val subject: Subject = Subject.X,
) {
    /** Number of values in the sample. */
    val size: Int get() = values.size

    /** True if this sample has explicit weights. */
    val isWeighted: Boolean get() = weights != null

    /** Sum of weights (1.0 for unweighted samples). */
    val totalWeight: Double

    /** Effective sample size: (sum w_i)^2 / sum w_i^2. Equals [size] for unweighted samples. */
    val weightedSize: Double

    /** Lazily computed sorted copy of [values]. */
    val sortedValues: List<Double> by lazy { values.sorted() }

    init {
        if (weights != null) {
            var tw = 0.0
            var twSq = 0.0
            for (w in weights) {
                tw += w
                twSq += w * w
            }
            totalWeight = tw
            weightedSize = (tw * tw) / twSq
        } else {
            totalWeight = 1.0
            weightedSize = values.size.toDouble()
        }
    }

    /**
     * Convert this sample to a different (compatible) unit.
     *
     * @throws UnitMismatchException if [target] is not compatible with [unit]
     */
    fun convertTo(target: MeasurementUnit): Sample {
        if (!unit.isCompatible(target)) {
            throw UnitMismatchException(unit, target)
        }
        if (unit == target) return this
        val factor = conversionFactor(unit, target)
        val converted = values.map { it * factor }
        return Sample(converted, weights?.toList(), target, subject)
    }

    /**
     * Log-transform: returns a new sample with ln(value) for each value, unit becomes [NumberUnit].
     *
     * @throws AssumptionException with POSITIVITY violation if any value is non-positive
     */
    fun log(): Sample {
        val logValues =
            values.map { v ->
                if (v <= 0.0) {
                    throw AssumptionException(Violation(AssumptionId.POSITIVITY, subject))
                }
                ln(v)
            }
        return Sample(logValues, weights?.toList(), NumberUnit, subject)
    }

    /** Returns a copy of this sample with a different subject label. */
    internal fun withSubject(newSubject: Subject): Sample {
        return Sample(values, weights, unit, newSubject)
    }

    /** Returns a new sample with each value multiplied by [scalar]. */
    operator fun times(scalar: Double): Sample {
        return Sample(values.map { it * scalar }, weights?.toList(), unit, subject)
    }

    /** Returns a new sample with [scalar] added to each value. */
    operator fun plus(scalar: Double): Sample {
        return Sample(values.map { it + scalar }, weights?.toList(), unit, subject)
    }

    override fun toString(): String = "Sample(size=$size, unit=${unit.id})"

    companion object {
        /**
         * Create an unweighted sample from vararg doubles with [NumberUnit].
         */
        fun of(vararg values: Double): Sample = of(values.toList(), NumberUnit)

        /**
         * Create an unweighted sample from a list of doubles.
         *
         * @throws AssumptionException with VALIDITY violation if values is empty or contains NaN/Inf
         */
        fun of(
            values: List<Double>,
            unit: MeasurementUnit = NumberUnit,
        ): Sample = create(values, null, unit, Subject.X)

        /**
         * Create a weighted sample.
         *
         * @throws AssumptionException with VALIDITY violation if values is empty or contains NaN/Inf
         * @throws IllegalArgumentException if weights length differs from values or weights are invalid
         */
        fun weighted(
            values: List<Double>,
            weights: List<Double>,
            unit: MeasurementUnit = NumberUnit,
        ): Sample = create(values, weights, unit, Subject.X)

        internal fun create(
            values: List<Double>,
            weights: List<Double>?,
            unit: MeasurementUnit,
            subject: Subject,
        ): Sample {
            if (values.isEmpty()) {
                throw AssumptionException(Violation(AssumptionId.VALIDITY, subject))
            }
            for (v in values) {
                if (v.isNaN() || v.isInfinite()) {
                    throw AssumptionException(Violation(AssumptionId.VALIDITY, subject))
                }
            }
            if (weights != null) {
                if (weights.size != values.size) {
                    throw AssumptionException(
                        "weights length (${weights.size}) must match values length (${values.size})",
                    )
                }
                var totalW = 0.0
                var minW = Double.MAX_VALUE
                for (w in weights) {
                    totalW += w
                    if (w < minW) minW = w
                }
                if (minW < 0.0) throw AssumptionException("all weights must be non-negative")
                if (totalW < 1e-9) throw AssumptionException("total weight must be positive")
            }
            return Sample(values.toList(), weights?.toList(), unit, subject)
        }

        // ====================================================================
        // Static estimator methods (delegating to the free functions)
        // ====================================================================

        internal fun center(x: Sample): Measurement {
            checkNonWeighted("x", x)
            val result = fastCenter(x.values)
            return Measurement(result, x.unit)
        }

        internal fun spread(x: Sample): Measurement {
            checkNonWeighted("x", x)
            val spreadVal = fastSpread(x.values)
            if (spreadVal <= 0.0) {
                throw AssumptionException(Violation(AssumptionId.SPARITY, x.subject))
            }
            return Measurement(spreadVal, x.unit)
        }

        internal fun shift(
            x: Sample,
            y: Sample,
        ): Measurement {
            checkNonWeighted("x", x)
            checkNonWeighted("y", y)
            val (cx, cy) = preparePair(x, y)
            val result = fastShift(cx.values, cy.values)[0]
            return Measurement(result, cx.unit)
        }

        internal fun ratio(
            x: Sample,
            y: Sample,
        ): Measurement {
            checkNonWeighted("x", x)
            checkNonWeighted("y", y)
            val (cx, cy) = preparePair(x, y)
            checkPositivity(cx.values, cx.subject)
            checkPositivity(cy.values, cy.subject)
            val result = fastRatio(cx.values, cy.values)[0]
            return Measurement(result, RatioUnit)
        }

        internal fun disparity(
            x: Sample,
            y: Sample,
        ): Measurement {
            checkNonWeighted("x", x)
            checkNonWeighted("y", y)
            val (cx, cy) = preparePair(x, y)
            val n = cx.size
            val m = cy.size
            val spreadX = fastSpread(cx.values)
            if (spreadX <= 0.0) {
                throw AssumptionException(Violation(AssumptionId.SPARITY, cx.subject))
            }
            val spreadY = fastSpread(cy.values)
            if (spreadY <= 0.0) {
                throw AssumptionException(Violation(AssumptionId.SPARITY, cy.subject))
            }
            val shiftVal = fastShift(cx.values, cy.values)[0]
            val avgSpreadVal = (n * spreadX + m * spreadY) / (n + m).toDouble()
            return Measurement(shiftVal / avgSpreadVal, DisparityUnit)
        }

        internal fun centerBounds(
            x: Sample,
            misrate: Double,
        ): Bounds {
            checkNonWeighted("x", x)
            return dev.pragmastat.centerBounds(x.values, misrate).withUnit(x.unit)
        }

        internal fun spreadBounds(
            x: Sample,
            misrate: Double,
            seed: String?,
        ): Bounds {
            checkNonWeighted("x", x)
            return dev.pragmastat.spreadBounds(x.values, misrate, seed).withUnit(x.unit)
        }

        internal fun shiftBounds(
            x: Sample,
            y: Sample,
            misrate: Double,
        ): Bounds {
            checkNonWeighted("x", x)
            checkNonWeighted("y", y)
            val (cx, cy) = preparePair(x, y)
            return dev.pragmastat.shiftBounds(cx.values, cy.values, misrate).withUnit(cx.unit)
        }

        internal fun ratioBounds(
            x: Sample,
            y: Sample,
            misrate: Double,
        ): Bounds {
            checkNonWeighted("x", x)
            checkNonWeighted("y", y)
            val (cx, cy) = preparePair(x, y)
            return dev.pragmastat.ratioBounds(cx.values, cy.values, misrate).withUnit(RatioUnit)
        }

        internal fun disparityBounds(
            x: Sample,
            y: Sample,
            misrate: Double,
            seed: String?,
        ): Bounds {
            checkNonWeighted("x", x)
            checkNonWeighted("y", y)
            val (cx, cy) = preparePair(x, y)
            return dev.pragmastat.disparityBounds(cx.values, cy.values, misrate, seed)
                .withUnit(DisparityUnit)
        }

        // ====================================================================
        // Validation helpers
        // ====================================================================

        private fun checkNonWeighted(
            name: String,
            s: Sample,
        ) {
            if (s.isWeighted) {
                throw AssumptionException("weighted samples are not supported for $name")
            }
        }

        private fun checkCompatibleUnits(
            a: Sample,
            b: Sample,
        ) {
            if (!a.unit.isCompatible(b.unit)) {
                throw UnitMismatchException(a.unit, b.unit)
            }
        }

        private fun convertToFiner(
            a: Sample,
            b: Sample,
        ): Pair<Sample, Sample> {
            if (!a.unit.isCompatible(b.unit)) {
                throw UnitMismatchException(a.unit, b.unit)
            }
            if (a.unit == b.unit) return Pair(a, b)
            val target = finer(a.unit, b.unit)
            return Pair(a.convertTo(target), b.convertTo(target))
        }

        /** Set subjects, check unit compatibility, convert to finer unit. */
        private fun preparePair(
            x: Sample,
            y: Sample,
        ): Pair<Sample, Sample> {
            checkCompatibleUnits(x, y)
            return convertToFiner(
                x.withSubject(Subject.X),
                y.withSubject(Subject.Y),
            )
        }
    }
}

// =============================================================================
// Sample-based estimator functions
// =============================================================================

/** Estimates the central value of a [Sample]. */
fun center(x: Sample): Measurement = Sample.center(x)

/** Estimates data dispersion of a [Sample]. */
fun spread(x: Sample): Measurement = Sample.spread(x)

/** Measures the typical difference between [x] and [y]. */
fun shift(
    x: Sample,
    y: Sample,
): Measurement = Sample.shift(x, y)

/** Measures how many times larger [x] is compared to [y]. */
fun ratio(
    x: Sample,
    y: Sample,
): Measurement = Sample.ratio(x, y)

/** Measures effect size between [x] and [y]. */
fun disparity(
    x: Sample,
    y: Sample,
): Measurement = Sample.disparity(x, y)

/** Provides distribution-free bounds for center of [x]. */
fun centerBounds(
    x: Sample,
    misrate: Double = DEFAULT_MISRATE,
): Bounds = Sample.centerBounds(x, misrate)

/** Provides distribution-free bounds for spread of [x]. */
fun spreadBounds(
    x: Sample,
    misrate: Double = DEFAULT_MISRATE,
    seed: String? = null,
): Bounds = Sample.spreadBounds(x, misrate, seed)

/** Provides bounds on shift between [x] and [y]. */
fun shiftBounds(
    x: Sample,
    y: Sample,
    misrate: Double = DEFAULT_MISRATE,
): Bounds = Sample.shiftBounds(x, y, misrate)

/** Provides bounds on ratio between [x] and [y]. */
fun ratioBounds(
    x: Sample,
    y: Sample,
    misrate: Double = DEFAULT_MISRATE,
): Bounds = Sample.ratioBounds(x, y, misrate)

/** Provides bounds on disparity between [x] and [y]. */
fun disparityBounds(
    x: Sample,
    y: Sample,
    misrate: Double = DEFAULT_MISRATE,
    seed: String? = null,
): Bounds = Sample.disparityBounds(x, y, misrate, seed)
