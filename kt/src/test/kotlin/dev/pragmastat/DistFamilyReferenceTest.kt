package dev.pragmastat

import com.fasterxml.jackson.databind.ObjectMapper
import com.fasterxml.jackson.module.kotlin.readValue
import com.fasterxml.jackson.module.kotlin.registerKotlinModule
import dev.pragmastat.distributions.Additive
import dev.pragmastat.distributions.Distribution
import dev.pragmastat.distributions.Exp
import dev.pragmastat.distributions.Multiplic
import dev.pragmastat.distributions.Power
import dev.pragmastat.distributions.Uniform
import org.junit.jupiter.api.DynamicTest
import org.junit.jupiter.api.Test
import org.junit.jupiter.api.TestFactory
import org.junit.jupiter.api.assertAll
import java.io.File
import kotlin.test.assertEquals
import kotlin.test.assertTrue

/**
 * The five parametric families against their shared `dist-*` fixtures.
 *
 * Each family answers three questions about itself, and each answer is assembled out of the
 * toolkit's own exponential, logarithm, normal cumulative and normal quantile. Nothing on these
 * paths reaches a platform transcendental, so the seven ports owe each other identical bits and
 * the suites are declared `exact` in `tests/manifest.json`: the comparison below is by binary64
 * payload. A tolerance would pass on exactly the divergence these suites exist to catch.
 *
 * The sibling `distributions` suite stays approximate, because drawing a sample still goes through
 * the platform's log, exp, cos and pow. That is a separate change with its own effect on every
 * seeded stream.
 */
class DistFamilyReferenceTest {
    private data class FamilyInput(
        val parameters: Map<String, Double> = emptyMap(),
        val x: List<Double> = emptyList(),
        val p: List<Double> = emptyList(),
    )

    private data class FamilyOutput(
        val density: List<Double> = emptyList(),
        val cumulative: List<Double> = emptyList(),
        val quantile: List<Double> = emptyList(),
    )

    private data class FamilyTestData(
        val input: FamilyInput = FamilyInput(),
        val output: FamilyOutput = FamilyOutput(),
    )

    private companion object {
        /**
         * How many values the five suites hold between them: for each family, a density and a
         * cumulative at every x and a quantile at every p. Pinned rather than derived, because a
         * loader that finds four suites out of five satisfies every other assertion here by having
         * less to compare.
         */
        const val DIST_FAMILY_VALUE_COUNT = 7800

        val SUITES = listOf("dist-uniform", "dist-exp", "dist-additive", "dist-multiplic", "dist-power")
    }

    private val mapper = ObjectMapper().registerKotlinModule()

    /**
     * Builds the family the suite names, rather than letting the parameter names decide: two
     * families could come to share a spelling, and a fixture read into the wrong family would
     * still produce numbers and still be compared.
     */
    private fun build(
        suite: String,
        parameters: Map<String, Double>,
    ): Distribution {
        fun need(name: String): Double = requireNotNull(parameters[name]) { "$suite: fixture has no parameter named $name" }
        return when (suite) {
            "dist-uniform" -> Uniform(need("min"), need("max"))
            "dist-exp" -> Exp(need("rate"))
            "dist-additive" -> Additive(need("mean"), need("stdDev"))
            "dist-multiplic" -> Multiplic(need("logMean"), need("logStdDev"))
            "dist-power" -> Power(need("min"), need("shape"))
            else -> error("Unknown suite $suite")
        }
    }

    @TestFactory
    fun distFamilyReference(): List<DynamicTest> {
        val tests = mutableListOf<DynamicTest>()
        var total = 0

        for (suite in SUITES) {
            val testDir = File("../tests/$suite")
            val loaded =
                testDir
                    .listFiles { _, name -> name.endsWith(".json") }
                    ?.sortedBy { it.name }
                    .orEmpty()
                    .map { file -> file.nameWithoutExtension to mapper.readValue<FamilyTestData>(file) }

            total += loaded.sumOf { (_, data) -> 2 * data.input.x.size + data.input.p.size }

            for ((name, data) in loaded) {
                tests.add(
                    DynamicTest.dynamicTest("$suite/$name") {
                        // A file that parses into empty lists passes every comparison below, which
                        // is the one way this test could report success while checking nothing.
                        assertTrue(data.input.x.isNotEmpty(), "$name: no x values")
                        assertTrue(data.input.p.isNotEmpty(), "$name: no p values")
                        assertEquals(data.input.x.size, data.output.density.size, "$name: densities")
                        assertEquals(data.input.x.size, data.output.cumulative.size, "$name: cumulatives")
                        assertEquals(data.input.p.size, data.output.quantile.size, "$name: quantiles")

                        val d = build(suite, data.input.parameters)
                        assertAll(
                            data.input.x.indices.map { i ->
                                {
                                    assertBitwise(data.output.density[i], d.densityAt(data.input.x[i]), "densityAt(${data.input.x[i]})")
                                    assertBitwise(
                                        data.output.cumulative[i],
                                        d.cumulativeAt(data.input.x[i]),
                                        "cumulativeAt(${data.input.x[i]})",
                                    )
                                }
                            } +
                                data.input.p.indices.map { i ->
                                    {
                                        assertBitwise(
                                            data.output.quantile[i],
                                            d.quantileAt(data.input.p[i]),
                                            "quantileAt(${data.input.p[i]})",
                                        )
                                    }
                                },
                        )
                    },
                )
            }
        }

        val loadedTotal = total
        tests.add(
            DynamicTest.dynamicTest("dist-family/coverage: $loadedTotal values over ${SUITES.size} suites") {
                assertEquals(
                    DIST_FAMILY_VALUE_COUNT,
                    loadedTotal,
                    "dist-family: loaded $loadedTotal values, expected $DIST_FAMILY_VALUE_COUNT",
                )
            },
        )
        return tests
    }

    /**
     * The arguments the fixtures cannot carry, because JSON writes neither a NaN nor an infinity.
     *
     * Without this table the fixtures are the only check the trio has, and they sample the
     * interior of each support through the family's own quantile. That left two defects alive:
     * Uniform reported a perfectly ordinary density for NaN in six ports, since neither half of
     * `x < min || x >= max` holds for a NaN and control fell straight through, and R raised an
     * error from thirteen of the fifteen functions. Neither is reachable from a stored fixture.
     */
    private fun families(): List<Quadruple> =
        listOf(
            Quadruple("dist-uniform", Uniform(0.0, 1.0), 0.0, 1.0),
            Quadruple("dist-exp", Exp(1.0), 0.0, Double.POSITIVE_INFINITY),
            Quadruple("dist-additive", Additive(0.0, 1.0), Double.NEGATIVE_INFINITY, Double.POSITIVE_INFINITY),
            Quadruple("dist-multiplic", Multiplic(0.0, 1.0), 0.0, Double.POSITIVE_INFINITY),
            Quadruple("dist-power", Power(1.0, 2.0), 1.0, Double.POSITIVE_INFINITY),
        )

    private data class Quadruple(
        val name: String,
        val distribution: Distribution,
        val lower: Double,
        val upper: Double,
    )

    @Test
    fun nanPropagatesThroughAllThree() {
        for ((name, d, _, _) in families()) {
            assertTrue(d.densityAt(Double.NaN).isNaN(), "$name: densityAt(NaN)")
            assertTrue(d.cumulativeAt(Double.NaN).isNaN(), "$name: cumulativeAt(NaN)")
            assertTrue(d.quantileAt(Double.NaN).isNaN(), "$name: quantileAt(NaN)")
        }
    }

    @Test
    fun theInfinitiesLieOutsideEverySupport() {
        for ((name, d, _, _) in families()) {
            assertEquals(0.0, d.densityAt(Double.NEGATIVE_INFINITY), name)
            assertEquals(0.0, d.densityAt(Double.POSITIVE_INFINITY), name)
            assertEquals(0.0, d.cumulativeAt(Double.NEGATIVE_INFINITY), name)
            assertEquals(1.0, d.cumulativeAt(Double.POSITIVE_INFINITY), name)
        }
    }

    @Test
    fun aProbabilityOutsideTheUnitIntervalReturnsTheEndpoints() {
        for ((name, d, lower, upper) in families()) {
            for (p in listOf(0.0, -0.1, Double.NEGATIVE_INFINITY)) {
                assertEquals(lower, d.quantileAt(p), "$name: quantileAt($p)")
            }
            for (p in listOf(1.0, 1.1, Double.POSITIVE_INFINITY)) {
                assertEquals(upper, d.quantileAt(p), "$name: quantileAt($p)")
            }
        }
    }
}
