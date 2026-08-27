package dev.pragmastat

import com.fasterxml.jackson.annotation.JsonIgnoreProperties
import com.fasterxml.jackson.annotation.JsonProperty
import com.fasterxml.jackson.databind.ObjectMapper
import com.fasterxml.jackson.module.kotlin.readValue
import com.fasterxml.jackson.module.kotlin.registerKotlinModule
import dev.pragmastat.distributions.Distribution
import dev.pragmastat.distributions.LinearDistModel
import dev.pragmastat.distributions.NeighborDistModel
import org.junit.jupiter.api.DynamicTest
import org.junit.jupiter.api.TestFactory
import org.junit.jupiter.api.assertThrows
import java.io.File
import kotlin.test.assertEquals

/**
 * LinearDistModel and NeighborDistModel against their shared dist-linear/dist-neighbor fixtures.
 *
 * Unlike the five parametric families, this suite's oracle is not one of the seven ports: it is
 * an independent binary64 + exact-Fraction reimplementation (`tests/oracles/dist_reference.py`),
 * written directly from the normative algorithm so a bug shared by every language cannot pass
 * itself. Every case is compared by binary64 payload, matching the suite's declared exact class
 * in `tests/manifest.json`.
 *
 * Both models are exercised through the raw vector path only (`List<Double>` plus an
 * `assumeSorted` flag): the measured, `Sample`-wrapped facade is a separate surface with its own
 * coverage and is not touched here.
 */
class DistReferenceTest {
    private data class DistQueryInput(
        val x: List<Any?> = emptyList(),
        val y: List<Any?> = emptyList(),
        @JsonProperty("assume_sorted") val assumeSorted: Boolean = false,
    )

    @JsonIgnoreProperties(ignoreUnknown = true)
    private data class DistQueryTestData(
        val input: DistQueryInput = DistQueryInput(),
        val output: List<Any?>? = null,
        @JsonProperty("expected_error") val expectedError: Map<String, String>? = null,
    )

    private data class DistSampleInput(
        val x: List<Any?> = emptyList(),
        val seed: String = "",
        val count: Int = 0,
    )

    @JsonIgnoreProperties(ignoreUnknown = true)
    private data class DistSampleTestData(
        val input: DistSampleInput = DistSampleInput(),
        val output: List<Any?>? = null,
    )

    private data class DistLandscapeInput(
        val x: List<Any?> = emptyList(),
    )

    private data class DistLandscapeOutput(
        val k0: Int = 0,
        val radius: Int = 0,
        val boundaries: List<Any?> = emptyList(),
        val cumulative: List<Any?> = emptyList(),
        val masses: List<Any?> = emptyList(),
        val densities: List<Any?> = emptyList(),
        val sources: List<Int> = emptyList(),
    )

    @JsonIgnoreProperties(ignoreUnknown = true)
    private data class DistLandscapeTestData(
        val input: DistLandscapeInput = DistLandscapeInput(),
        val output: DistLandscapeOutput = DistLandscapeOutput(),
    )

    /** The surface every dist-* query fixture reaches, shared by both reconstructions so the
     * loader below does not need to know which one it is holding. */
    private interface DistQueryModel {
        fun quantile(cumulances: List<Double>): List<Double>

        fun cumulative(values: List<Double>): List<Double>

        fun density(values: List<Double>): List<Double>
    }

    private companion object {
        /**
         * Total fixtures the two suites hold across every op: 236, pinned rather than counted
         * from the directories, because counting the directories and asserting the count matches
         * them asserts nothing.
         */
        const val DIST_FIXTURE_COUNT = 236

        val SUB_SUITES =
            listOf(
                "dist-linear/quantile",
                "dist-linear/cumulative",
                "dist-linear/density",
                "dist-linear/sample",
                "dist-neighbor/quantile",
                "dist-neighbor/cumulative",
                "dist-neighbor/density",
                "dist-neighbor/sample",
                "dist-neighbor/landscape",
            )
    }

    private val mapper = ObjectMapper().registerKotlinModule()

    /**
     * Reads x/y and the query outputs. JSON has no literal for NaN or an infinity, so the
     * generator spells them as the strings "NaN"/"Infinity"/"-Infinity" wherever one is needed;
     * everything else arrives as a plain JSON number.
     */
    private fun parseSpecialFloats(raw: List<Any?>): List<Double> =
        raw.map { v ->
            when (v) {
                is String ->
                    when (v) {
                        "NaN" -> Double.NaN
                        "Infinity" -> Double.POSITIVE_INFINITY
                        "-Infinity" -> Double.NEGATIVE_INFINITY
                        else -> error("unexpected string in numeric fixture field: $v")
                    }
                is Number -> v.toDouble()
                else -> error("unexpected value in numeric fixture field: $v")
            }
        }

    /**
     * A missing directory already fails below (`listFiles` returns null, `?.sortedBy` short-circuits to
     * `orEmpty()`, same outcome as an existing-but-empty one) -- so both cases need the same explicit
     * check. Without it, a filtered run of a single sub-suite (`--tests '*.distLinearReference'`, or an
     * IDE single-test run) would silently contribute zero DynamicTests and report a pass while checking
     * nothing; only the separate cross-suite [distReferenceCoverage] total would ever catch it, and only
     * when the whole file runs. Every sibling language's loader for this suite asserts non-emptiness at
     * load time for the same reason.
     */
    private inline fun <reified T> loadFixtures(dir: String): List<Pair<String, T>> {
        val testDir = File("../tests/$dir")
        val files =
            testDir
                .listFiles { _, name -> name.endsWith(".json") }
                ?.sortedBy { it.name }
                .orEmpty()
        check(files.isNotEmpty()) { "No JSON test files found for '$dir' in ${testDir.path}" }
        return files.map { file -> file.nameWithoutExtension to mapper.readValue<T>(file) }
    }

    /**
     * Builds the model the fixture directory names, rather than letting the constructor
     * signatures decide: both take a `List<Double>` and an `assumeSorted` flag, so a fixture read
     * into the wrong model would still produce numbers and still be compared.
     */
    private fun buildDistModel(
        model: String,
        x: List<Double>,
        assumeSorted: Boolean,
    ): DistQueryModel =
        when (model) {
            "linear" -> {
                val m = LinearDistModel(x, assumeSorted)
                object : DistQueryModel {
                    override fun quantile(cumulances: List<Double>) = m.quantile(cumulances)

                    override fun cumulative(values: List<Double>) = m.cumulative(values)

                    override fun density(values: List<Double>) = m.density(values)
                }
            }
            "neighbor" -> {
                val m = NeighborDistModel(x, assumeSorted)
                object : DistQueryModel {
                    override fun quantile(cumulances: List<Double>) = m.quantile(cumulances)

                    override fun cumulative(values: List<Double>) = m.cumulative(values)

                    override fun density(values: List<Double>) = m.density(values)
                }
            }
            else -> error("unknown dist model $model")
        }

    private fun runDistOp(
        d: DistQueryModel,
        op: String,
        y: List<Double>,
    ): List<Double> =
        when (op) {
            "quantile" -> d.quantile(y)
            "cumulative" -> d.cumulative(y)
            "density" -> d.density(y)
            else -> error("unknown dist operation $op")
        }

    /**
     * Every fixture under dist-<model>/<op>. Whether a case declares `expected_error` is decided
     * BEFORE the model is built or queried: only once that is known does the test attempt the
     * operation, either asserting it fails with the declared id/subject or asserting it succeeds
     * and comparing the payload. Trying the operation first and only afterwards checking for
     * `expected_error` would let a real bug (an exception the fixture never asked for) pass
     * silently, since a caught-and-then-unchecked exception looks the same as the one the test
     * wanted.
     */
    private fun distQueryTests(
        model: String,
        op: String,
    ): List<DynamicTest> =
        loadFixtures<DistQueryTestData>("dist-$model/$op").map { (name, data) ->
            DynamicTest.dynamicTest("dist-$model/$op/$name") {
                val x = parseSpecialFloats(data.input.x)
                val y = parseSpecialFloats(data.input.y)

                if (data.expectedError != null) {
                    val exception =
                        assertThrows<AssumptionException> {
                            val d = buildDistModel(model, x, data.input.assumeSorted)
                            runDistOp(d, op, y)
                        }
                    assertEquals(
                        data.expectedError["id"],
                        exception.violation!!.id.id,
                        "$name: expected error id ${data.expectedError["id"]}, got ${exception.violation!!.id.id}",
                    )
                    assertEquals(
                        data.expectedError["subject"],
                        exception.violation!!.subject.id,
                        "$name: expected error subject ${data.expectedError["subject"]}, got ${exception.violation!!.subject.id}",
                    )
                    return@dynamicTest
                }

                val d = buildDistModel(model, x, data.input.assumeSorted)
                val actual = runDistOp(d, op, y)
                assertBitwise(parseSpecialFloats(data.output!!), actual, "$name: $op")
            }
        }

    /**
     * Every fixture under dist-<model>/sample. No case here declares an error: an invalid x would
     * already be covered by the query suites above, and drawing from a built model cannot fail on
     * its own. Both models draw by inverting their own quantile at a uniform cumulance, so the
     * shared [Distribution.samples] default is the one and only sampling route.
     */
    private fun distSampleTests(model: String): List<DynamicTest> =
        loadFixtures<DistSampleTestData>("dist-$model/sample").map { (name, data) ->
            DynamicTest.dynamicTest("dist-$model/sample/$name") {
                val x = parseSpecialFloats(data.input.x)
                val distribution: Distribution =
                    when (model) {
                        "linear" -> LinearDistModel(x, false)
                        "neighbor" -> NeighborDistModel(x, false)
                        else -> error("unknown dist model $model")
                    }

                val rng = Rng(data.input.seed)
                val actual = distribution.samples(rng, data.input.count)
                assertBitwise(parseSpecialFloats(data.output!!), actual, "$name: samples")
            }
        }

    /**
     * Every fixture under dist-neighbor/landscape. Every case here builds successfully by
     * construction: the generator only emits a landscape for a sample that already passed the
     * query suites' validity checks.
     */
    private fun distNeighborLandscapeTests(): List<DynamicTest> =
        loadFixtures<DistLandscapeTestData>("dist-neighbor/landscape").map { (name, data) ->
            DynamicTest.dynamicTest("dist-neighbor/landscape/$name") {
                val x = parseSpecialFloats(data.input.x)
                val model = NeighborDistModel(x, false)
                val landscape = model.getLandscape()

                assertEquals(data.output.k0, landscape.scaleIndex, "$name: k0")
                assertEquals(data.output.radius, landscape.radius, "$name: radius")
                assertBitwise(parseSpecialFloats(data.output.boundaries), landscape.boundaries, "$name: boundaries")
                assertBitwise(parseSpecialFloats(data.output.cumulative), landscape.cumulative, "$name: cumulative")
                assertBitwise(parseSpecialFloats(data.output.masses), landscape.masses, "$name: masses")
                assertBitwise(parseSpecialFloats(data.output.densities), landscape.densities, "$name: densities")
                assertEquals(data.output.sources, landscape.sources, "$name: sources")
            }
        }

    @TestFactory
    fun distLinearReference(): List<DynamicTest> {
        val tests = mutableListOf<DynamicTest>()
        for (op in listOf("quantile", "cumulative", "density")) {
            tests += distQueryTests("linear", op)
        }
        tests += distSampleTests("linear")
        return tests
    }

    @TestFactory
    fun distNeighborReference(): List<DynamicTest> {
        val tests = mutableListOf<DynamicTest>()
        for (op in listOf("quantile", "cumulative", "density")) {
            tests += distQueryTests("neighbor", op)
        }
        tests += distSampleTests("neighbor")
        tests += distNeighborLandscapeTests()
        return tests
    }

    @TestFactory
    fun distReferenceCoverage(): List<DynamicTest> {
        val total =
            SUB_SUITES.sumOf { dir ->
                File("../tests/$dir").listFiles { _, name -> name.endsWith(".json") }?.size ?: 0
            }
        return listOf(
            DynamicTest.dynamicTest("dist-reference/coverage: $total fixtures over ${SUB_SUITES.size} sub-suites") {
                assertEquals(DIST_FIXTURE_COUNT, total, "dist-reference: loaded $total fixtures, expected $DIST_FIXTURE_COUNT")
            },
        )
    }
}
