package dev.pragmastat

/**
 * Metric types supported by Compare1 and Compare2.
 */
enum class Metric {
    Center,
    Spread,
    Shift,
    Ratio,
    Disparity,
}

/**
 * Verdict from comparing an estimate against a threshold.
 */
enum class ComparisonVerdict {
    Less,
    Greater,
    Inconclusive,
}

/**
 * A threshold value with a metric type and misrate for comparison.
 *
 * @property metric The metric type (Center, Spread, Shift, Ratio, or Disparity)
 * @property value The threshold value as a Measurement
 * @property misrate The per-threshold misclassification rate (must be in (0, 1])
 */
data class Threshold(
    val metric: Metric,
    val value: Measurement,
    val misrate: Double,
) {
    init {
        require(misrate.isFinite() && misrate > 0.0 && misrate <= 1.0) {
            "misrate must be in (0, 1], got $misrate"
        }
        require(value.value.isFinite()) {
            "threshold value must be finite"
        }
    }
}

/**
 * A projection containing estimate, bounds, and verdict for a single threshold.
 *
 * @property threshold The threshold that was evaluated
 * @property estimate The point estimate
 * @property bounds The confidence bounds
 * @property verdict The comparison verdict
 */
data class Projection(
    val threshold: Threshold,
    val estimate: Measurement,
    val bounds: Bounds,
    val verdict: ComparisonVerdict,
)

/**
 * CompareEngine provides Compare1 and Compare2 functionality.
 *
 * Compare1: One-sample confirmatory analysis (Center, Spread)
 * Compare2: Two-sample confirmatory analysis (Shift, Ratio, Disparity)
 */
internal object CompareEngine {
    private data class MetricSpec(
        val metric: Metric,
        val validateAndNormalize: (Threshold, Sample, Sample?) -> Measurement,
        val estimate: (Sample, Sample?) -> Measurement,
        val bounds: (Sample, Sample?, Double) -> Bounds,
        val seededBounds: ((Sample, Sample?, Double, String) -> Bounds)? = null,
    )

    private val compare1Specs =
        listOf(
            MetricSpec(
                metric = Metric.Center,
                validateAndNormalize = ::validateCenter,
                estimate = { x, _ -> Sample.center(x) },
                bounds = { x, _, misrate -> Sample.centerBounds(x, misrate) },
            ),
            MetricSpec(
                metric = Metric.Spread,
                validateAndNormalize = ::validateSpread,
                estimate = { x, _ -> Sample.spread(x) },
                bounds = { x, _, misrate -> Sample.spreadBounds(x, misrate, null) },
                seededBounds = { x, _, misrate, seed -> Sample.spreadBounds(x, misrate, seed) },
            ),
        )

    private val compare2Specs =
        listOf(
            MetricSpec(
                metric = Metric.Shift,
                validateAndNormalize = ::validateShift,
                estimate = { x, y -> Sample.shift(x!!, y!!) },
                bounds = { x, y, misrate -> Sample.shiftBounds(x!!, y!!, misrate) },
            ),
            MetricSpec(
                metric = Metric.Ratio,
                validateAndNormalize = ::validateRatio,
                estimate = { x, y -> Sample.ratio(x!!, y!!) },
                bounds = { x, y, misrate -> Sample.ratioBounds(x!!, y!!, misrate) },
            ),
            MetricSpec(
                metric = Metric.Disparity,
                validateAndNormalize = ::validateDisparity,
                estimate = { x, y -> Sample.disparity(x!!, y!!) },
                bounds = { x, y, misrate -> Sample.disparityBounds(x!!, y!!, misrate, null) },
                seededBounds = { x, y, misrate, seed -> Sample.disparityBounds(x!!, y!!, misrate, seed) },
            ),
        )

    private fun validateCenter(
        threshold: Threshold,
        x: Sample,
        unused: Sample?,
    ): Measurement {
        if (!threshold.value.unit.isCompatible(x.unit)) {
            throw UnitMismatchException(threshold.value.unit, x.unit)
        }
        val factor = conversionFactor(threshold.value.unit, x.unit)
        return Measurement(threshold.value.value * factor, x.unit)
    }

    private fun validateSpread(
        threshold: Threshold,
        x: Sample,
        unused: Sample?,
    ): Measurement {
        return validateCenter(threshold, x, null)
    }

    private fun validateShift(
        threshold: Threshold,
        x: Sample,
        y: Sample?,
    ): Measurement {
        if (!threshold.value.unit.isCompatible(x.unit)) {
            throw UnitMismatchException(threshold.value.unit, x.unit)
        }
        val target = finer(x.unit, y!!.unit)
        val factor = conversionFactor(threshold.value.unit, target)
        return Measurement(threshold.value.value * factor, target)
    }

    private fun validateRatio(
        threshold: Threshold,
        unused1: Sample?,
        unused2: Sample?,
    ): Measurement {
        val unit = threshold.value.unit
        if (unit != RatioUnit && unit != NumberUnit) {
            throw IllegalArgumentException("Ratio threshold must have Ratio or dimensionless unit, got ${unit.id}")
        }
        val value = threshold.value.value
        if (value <= 0.0) {
            throw IllegalArgumentException("Ratio threshold value must be positive, got $value")
        }
        return Measurement(value, RatioUnit)
    }

    private fun validateDisparity(
        threshold: Threshold,
        unused1: Sample?,
        unused2: Sample?,
    ): Measurement {
        val unit = threshold.value.unit
        if (unit != DisparityUnit && unit != NumberUnit) {
            throw IllegalArgumentException("Disparity threshold must have Disparity or dimensionless unit, got ${unit.id}")
        }
        return Measurement(threshold.value.value, DisparityUnit)
    }

    fun compare1(
        x: Sample,
        thresholds: List<Threshold>,
        seed: String? = null,
    ): List<Projection> {
        Sample.checkNonWeighted("x", x)
        require(thresholds.isNotEmpty()) {
            "thresholds list cannot be empty"
        }

        for (threshold in thresholds) {
            require(threshold.metric == Metric.Center || threshold.metric == Metric.Spread) {
                "Metric ${threshold.metric} is not supported by Compare1. Use Compare2 instead."
            }
        }

        val normalizedValues =
            thresholds.map { threshold ->
                val spec =
                    compare1Specs.find { it.metric == threshold.metric }
                        ?: throw IllegalStateException("No spec found for metric ${threshold.metric}")
                spec.validateAndNormalize(threshold, x, null)
            }

        return execute(compare1Specs, x, null, thresholds, normalizedValues, seed)
    }

    fun compare2(
        x: Sample,
        y: Sample,
        thresholds: List<Threshold>,
        seed: String? = null,
    ): List<Projection> {
        Sample.checkNonWeighted("x", x)
        Sample.checkNonWeighted("y", y)
        Sample.checkCompatibleUnits(x, y)
        require(thresholds.isNotEmpty()) {
            "thresholds list cannot be empty"
        }

        for (threshold in thresholds) {
            require(threshold.metric == Metric.Shift || threshold.metric == Metric.Ratio || threshold.metric == Metric.Disparity) {
                "Metric ${threshold.metric} is not supported by Compare2. Use Compare1 instead."
            }
        }

        val normalizedValues =
            thresholds.map { threshold ->
                val spec =
                    compare2Specs.find { it.metric == threshold.metric }
                        ?: throw IllegalStateException("No spec found for metric ${threshold.metric}")
                spec.validateAndNormalize(threshold, x, y)
            }

        return execute(compare2Specs, x, y, thresholds, normalizedValues, seed)
    }

    private fun execute(
        specs: List<MetricSpec>,
        x: Sample,
        y: Sample?,
        thresholds: List<Threshold>,
        normalizedValues: List<Measurement>,
        seed: String?,
    ): List<Projection> {
        val results = arrayOfNulls<Projection>(thresholds.size)

        val byMetric =
            thresholds
                .mapIndexed { index, threshold ->
                    Triple(threshold, index, normalizedValues[index])
                }
                .groupBy { it.first.metric }

        for (spec in specs) {
            val entries = byMetric[spec.metric] ?: continue
            val estimate = spec.estimate(x, y)

            for ((threshold, inputIndex, normalizedValue) in entries) {
                val bounds =
                    if (seed != null && spec.seededBounds != null) {
                        spec.seededBounds(x, y, threshold.misrate, seed)
                    } else {
                        spec.bounds(x, y, threshold.misrate)
                    }
                val verdict = computeVerdict(bounds, normalizedValue)
                results[inputIndex] = Projection(threshold, estimate, bounds, verdict)
            }
        }

        @Suppress("UNCHECKED_CAST")
        return results.toList() as List<Projection>
    }

    private fun computeVerdict(
        bounds: Bounds,
        normalizedThreshold: Measurement,
    ): ComparisonVerdict {
        val t = normalizedThreshold.value
        return when {
            bounds.lower > t -> ComparisonVerdict.Greater
            bounds.upper < t -> ComparisonVerdict.Less
            else -> ComparisonVerdict.Inconclusive
        }
    }
}

/**
 * One-sample confirmatory analysis: compares Center/Spread against practical thresholds.
 *
 * @param x The sample to analyze
 * @param thresholds List of thresholds to compare against
 * @return List of projections in the same order as the input thresholds
 * @throws IllegalArgumentException if thresholds is empty or contains unsupported metrics
 */
fun compare1(
    x: Sample,
    thresholds: List<Threshold>,
): List<Projection> = CompareEngine.compare1(x, thresholds, null)

/**
 * One-sample confirmatory analysis with seed for reproducibility.
 *
 * The seed is used for randomized bounds (Spread bounds only).
 *
 * @param x The sample to analyze
 * @param thresholds List of thresholds to compare against
 * @param seed Seed string for reproducible randomization
 * @return List of projections in the same order as the input thresholds
 */
fun compare1(
    x: Sample,
    thresholds: List<Threshold>,
    seed: String,
): List<Projection> = CompareEngine.compare1(x, thresholds, seed)

/**
 * Two-sample confirmatory analysis: compares Shift/Ratio/Disparity against practical thresholds.
 *
 * @param x The first sample
 * @param y The second sample
 * @param thresholds List of thresholds to compare against
 * @return List of projections in the same order as the input thresholds
 * @throws IllegalArgumentException if thresholds is empty or contains unsupported metrics
 */
fun compare2(
    x: Sample,
    y: Sample,
    thresholds: List<Threshold>,
): List<Projection> = CompareEngine.compare2(x, y, thresholds, null)

/**
 * Two-sample confirmatory analysis with seed for reproducibility.
 *
 * The seed is used for randomized bounds (Disparity bounds only).
 *
 * @param x The first sample
 * @param y The second sample
 * @param thresholds List of thresholds to compare against
 * @param seed Seed string for reproducible randomization
 * @return List of projections in the same order as the input thresholds
 */
fun compare2(
    x: Sample,
    y: Sample,
    thresholds: List<Threshold>,
    seed: String,
): List<Projection> = CompareEngine.compare2(x, y, thresholds, seed)
