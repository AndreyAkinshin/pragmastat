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
        return Sample(converted, weights?.toList(), target)
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
                    throw AssumptionException(Violation(AssumptionId.POSITIVITY, Subject.X))
                }
                ln(v)
            }
        return Sample(logValues, weights?.toList(), NumberUnit)
    }

    /** Returns a new sample with each value multiplied by [scalar]. */
    operator fun times(scalar: Double): Sample {
        return Sample(values.map { it * scalar }, weights?.toList(), unit)
    }

    /** Returns a new sample with [scalar] added to each value. */
    operator fun plus(scalar: Double): Sample {
        return Sample(values.map { it + scalar }, weights?.toList(), unit)
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
        ): Sample = create(values, null, unit)

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
        ): Sample = create(values, weights, unit)

        // Construction validity errors are always reported with subject "x":
        // construction cannot know which argument position the sample will occupy.
        internal fun create(
            values: List<Double>,
            weights: List<Double>?,
            unit: MeasurementUnit,
        ): Sample {
            if (values.isEmpty()) {
                throw AssumptionException(Violation(AssumptionId.VALIDITY, Subject.X))
            }
            for (v in values) {
                if (v.isNaN() || v.isInfinite()) {
                    throw AssumptionException(Violation(AssumptionId.VALIDITY, Subject.X))
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
            return Sample(values.toList(), weights?.toList(), unit)
        }

        // ====================================================================
        // Static estimator methods (delegating to the free functions)
        // ====================================================================

        // These companion methods are thin adapters: they delegate to the single
        // raw List-based estimator implementations (passing the cached sortedValues
        // with assumeSorted=true) and attach the appropriate measurement unit.

        internal fun center(x: Sample): Measurement {
            checkNonWeighted("x", x)
            val result = dev.pragmastat.center(x.sortedValues, assumeSorted = true)
            return Measurement(result, x.unit)
        }

        internal fun spread(x: Sample): Measurement {
            checkNonWeighted("x", x)
            val spreadVal = dev.pragmastat.spread(x.sortedValues, assumeSorted = true)
            return Measurement(spreadVal, x.unit)
        }

        internal fun shift(
            x: Sample,
            y: Sample,
        ): Measurement {
            checkNonWeighted("x", x)
            checkNonWeighted("y", y)
            val (cx, cy) = preparePair(x, y)
            val result = dev.pragmastat.shift(cx.sortedValues, cy.sortedValues, assumeSorted = true)
            return Measurement(result, cx.unit)
        }

        internal fun ratio(
            x: Sample,
            y: Sample,
        ): Measurement {
            checkNonWeighted("x", x)
            checkNonWeighted("y", y)
            val (cx, cy) = preparePair(x, y)
            val result = dev.pragmastat.ratio(cx.sortedValues, cy.sortedValues, assumeSorted = true)
            return Measurement(result, RatioUnit)
        }

        internal fun disparity(
            x: Sample,
            y: Sample,
        ): Measurement {
            checkNonWeighted("x", x)
            checkNonWeighted("y", y)
            val (cx, cy) = preparePair(x, y)
            val result = dev.pragmastat.disparity(cx.sortedValues, cy.sortedValues, assumeSorted = true)
            return Measurement(result, DisparityUnit)
        }

        internal fun centerBounds(
            x: Sample,
            misrate: Double,
        ): Bounds {
            checkNonWeighted("x", x)
            return dev.pragmastat.centerBounds(x.sortedValues, misrate, assumeSorted = true)
                .withUnit(x.unit)
        }

        internal fun spreadBounds(
            x: Sample,
            misrate: Double,
            seed: String?,
        ): Bounds {
            checkNonWeighted("x", x)
            // Shuffle runs on the original order; the cached sorted view is sparity-only.
            return spreadBoundsImpl(
                x.values,
                misrate,
                seed,
                sortedX = x.sortedValues,
            ).withUnit(x.unit)
        }

        internal fun shiftBounds(
            x: Sample,
            y: Sample,
            misrate: Double,
        ): Bounds {
            checkNonWeighted("x", x)
            checkNonWeighted("y", y)
            val (cx, cy) = preparePair(x, y)
            return dev.pragmastat.shiftBounds(cx.sortedValues, cy.sortedValues, misrate, assumeSorted = true)
                .withUnit(cx.unit)
        }

        internal fun ratioBounds(
            x: Sample,
            y: Sample,
            misrate: Double,
        ): Bounds {
            checkNonWeighted("x", x)
            checkNonWeighted("y", y)
            val (cx, cy) = preparePair(x, y)
            // ratioBounds is order-independent and ln is monotonic, so log(sortedValues)
            // stays sorted — reuse the cached sorted view to skip a re-sort (matches ratio/shiftBounds).
            return dev.pragmastat.ratioBounds(cx.sortedValues, cy.sortedValues, misrate, assumeSorted = true)
                .withUnit(RatioUnit)
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
            // Shuffles run on the original order; the cached sorted views are sparity-only.
            return disparityBoundsImpl(
                cx.values,
                cy.values,
                misrate,
                seed,
                sortedX = cx.sortedValues,
                sortedY = cy.sortedValues,
            ).withUnit(DisparityUnit)
        }

        // ====================================================================
        // Validation helpers
        // ====================================================================

        internal fun checkNonWeighted(
            name: String,
            s: Sample,
        ) {
            if (s.isWeighted) {
                throw AssumptionException("weighted samples are not supported for $name")
            }
        }

        internal fun checkCompatibleUnits(
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

        /**
         * Check unit compatibility and convert to the finer unit.
         *
         * No subject relabeling: the error subject is supplied positionally by the
         * raw estimator impls these pairs are passed to. Converting to the finer
         * unit returns the original samples unchanged when units already match, so
         * the warm sorted cache is reused across repeated two-sample calls.
         */
        private fun preparePair(
            x: Sample,
            y: Sample,
        ): Pair<Sample, Sample> {
            checkCompatibleUnits(x, y)
            return convertToFiner(x, y)
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
    misrate: Probability = Probability(DEFAULT_MISRATE),
): Bounds = Sample.centerBounds(x, misrate.value)

/** Provides distribution-free bounds for spread of [x]. */
fun spreadBounds(
    x: Sample,
    misrate: Probability = Probability(DEFAULT_MISRATE),
    seed: String? = null,
): Bounds = Sample.spreadBounds(x, misrate.value, seed)

/** Provides bounds on shift between [x] and [y]. */
fun shiftBounds(
    x: Sample,
    y: Sample,
    misrate: Probability = Probability(DEFAULT_MISRATE),
): Bounds = Sample.shiftBounds(x, y, misrate.value)

/** Provides bounds on ratio between [x] and [y]. */
fun ratioBounds(
    x: Sample,
    y: Sample,
    misrate: Probability = Probability(DEFAULT_MISRATE),
): Bounds = Sample.ratioBounds(x, y, misrate.value)

/** Provides bounds on disparity between [x] and [y]. */
fun disparityBounds(
    x: Sample,
    y: Sample,
    misrate: Probability = Probability(DEFAULT_MISRATE),
    seed: String? = null,
): Bounds = Sample.disparityBounds(x, y, misrate.value, seed)

/** One-sample confirmatory analysis against practical thresholds. */
fun Sample.compare1(thresholds: List<Threshold>): List<Projection> = CompareEngine.compare1(this, thresholds, null)

/** One-sample confirmatory analysis with seed for reproducibility. */
fun Sample.compare1(
    thresholds: List<Threshold>,
    seed: String,
): List<Projection> = CompareEngine.compare1(this, thresholds, seed)

/** Two-sample confirmatory analysis against practical thresholds. */
fun Sample.compare2(
    y: Sample,
    thresholds: List<Threshold>,
): List<Projection> = CompareEngine.compare2(this, y, thresholds, null)

/** Two-sample confirmatory analysis with seed for reproducibility. */
fun Sample.compare2(
    y: Sample,
    thresholds: List<Threshold>,
    seed: String,
): List<Projection> = CompareEngine.compare2(this, y, thresholds, seed)
