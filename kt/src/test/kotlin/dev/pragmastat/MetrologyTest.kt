package dev.pragmastat

import com.fasterxml.jackson.databind.JsonNode
import com.fasterxml.jackson.databind.ObjectMapper
import com.fasterxml.jackson.module.kotlin.KotlinModule
import org.junit.jupiter.api.DynamicTest
import org.junit.jupiter.api.TestFactory
import java.io.File
import kotlin.math.abs
import kotlin.test.assertEquals
import kotlin.test.assertTrue

class MetrologyTest {
    private val mapper = ObjectMapper().registerModule(KotlinModule.Builder().build())
    private val epsilon = 1e-9

    private fun assertClose(
        expected: Double,
        actual: Double,
        tolerance: Double = epsilon,
    ) {
        assertTrue(
            abs(expected - actual) < tolerance,
            "Expected $expected but got $actual (difference: ${abs(expected - actual)})",
        )
    }

    /** Parse a JSON value that may be a number or a special float string. */
    private fun parseFloat(node: JsonNode): Double =
        when {
            node.isNumber -> node.asDouble()
            node.isTextual ->
                when (node.asText()) {
                    "NaN" -> Double.NaN
                    "Infinity" -> Double.POSITIVE_INFINITY
                    "-Infinity" -> Double.NEGATIVE_INFINITY
                    else -> throw IllegalArgumentException("unexpected string value: ${node.asText()}")
                }
            else -> throw IllegalArgumentException("unexpected JSON value type: $node")
        }

    private fun parseFloatList(node: JsonNode): List<Double> = node.map { parseFloat(it) }

    // =========================================================================
    // Sample construction tests
    // =========================================================================

    @TestFactory
    fun testSampleConstruction(): List<DynamicTest> {
        val tests = mutableListOf<DynamicTest>()
        val testDir = File("../tests/sample-construction")

        if (!testDir.exists() || !testDir.isDirectory) {
            return tests
        }

        testDir.listFiles { _, name -> name.endsWith(".json") }?.forEach { file ->
            val testName = "sample-construction/${file.nameWithoutExtension}"
            tests.add(
                DynamicTest.dynamicTest(testName) {
                    val root = mapper.readTree(file)
                    val input = root["input"]
                    val values = parseFloatList(input["values"])
                    val weights: List<Double>? =
                        input["weights"]?.let { w ->
                            w.map { it.asDouble() }
                        }

                    if (root.has("expected_error")) {
                        val threw =
                            try {
                                if (weights != null) {
                                    Sample.weighted(values, weights)
                                } else {
                                    Sample.of(values)
                                }
                                false
                            } catch (_: AssumptionException) {
                                true
                            } catch (_: IllegalArgumentException) {
                                true
                            }
                        assertTrue(threw, "Expected error but sample was created successfully")
                        return@dynamicTest
                    }

                    val output = root["output"]
                    val expectedSize = output["size"].asInt()
                    val expectedIsWeighted = output["is_weighted"].asBoolean()

                    val sample =
                        if (weights != null) {
                            Sample.weighted(values, weights)
                        } else {
                            Sample.of(values)
                        }

                    assertEquals(expectedSize, sample.size, "size mismatch")
                    assertEquals(expectedIsWeighted, sample.isWeighted, "isWeighted mismatch")
                },
            )
        }

        return tests
    }

    // =========================================================================
    // Unit propagation tests
    // =========================================================================

    @TestFactory
    fun testUnitPropagation(): List<DynamicTest> {
        val tests = mutableListOf<DynamicTest>()
        val testDir = File("../tests/unit-propagation")
        val registry = UnitRegistry.standard()

        if (!testDir.exists() || !testDir.isDirectory) {
            return tests
        }

        testDir.listFiles { _, name -> name.endsWith(".json") }?.forEach { file ->
            val testName = "unit-propagation/${file.nameWithoutExtension}"
            tests.add(
                DynamicTest.dynamicTest(testName) {
                    val root = mapper.readTree(file)
                    val input = root["input"]

                    // Handle expected_error case (weighted-rejected)
                    if (root.has("expected_error")) {
                        val estimator = input["estimator"].asText()
                        val xValues = input["x"].map { it.asDouble() }
                        val xWeights = input["x_weights"].map { it.asDouble() }
                        val sx = Sample.weighted(xValues, xWeights)
                        val threw =
                            try {
                                when (estimator) {
                                    "center" -> center(sx)
                                    else -> error("unknown estimator for error case: $estimator")
                                }
                                false
                            } catch (_: AssumptionException) {
                                true
                            }
                        assertTrue(threw, "Expected error for weighted sample, got none")
                        return@dynamicTest
                    }

                    val estimator = input["estimator"].asText()
                    val xValues = input["x"].map { it.asDouble() }
                    val xUnitId = input["x_unit"].asText()
                    val xUnit = registry.resolve(xUnitId)

                    val sx = Sample.of(xValues, xUnit)

                    val output = root["output"]
                    val expectedUnit = output["unit"].asText()
                    val expectedValue: Double? = output["value"]?.takeIf { !it.isNull }?.asDouble()

                    when (estimator) {
                        "center" -> {
                            val m = center(sx)
                            assertEquals(expectedUnit, m.unit.id, "unit mismatch")
                            if (expectedValue != null) {
                                assertClose(expectedValue, m.value)
                            }
                        }

                        "spread" -> {
                            val m = spread(sx)
                            assertEquals(expectedUnit, m.unit.id, "unit mismatch")
                        }

                        "shift" -> {
                            val yValues = input["y"].map { it.asDouble() }
                            val yUnitId = input["y_unit"].asText()
                            val yUnit = registry.resolve(yUnitId)
                            val sy = Sample.of(yValues, yUnit)
                            val m = shift(sx, sy)
                            assertEquals(expectedUnit, m.unit.id, "unit mismatch")
                        }

                        "ratio" -> {
                            val yValues = input["y"].map { it.asDouble() }
                            val yUnitId = input["y_unit"].asText()
                            val yUnit = registry.resolve(yUnitId)
                            val sy = Sample.of(yValues, yUnit)
                            val m = ratio(sx, sy)
                            assertEquals(expectedUnit, m.unit.id, "unit mismatch")
                        }

                        "disparity" -> {
                            val yValues = input["y"].map { it.asDouble() }
                            val yUnitId = input["y_unit"].asText()
                            val yUnit = registry.resolve(yUnitId)
                            val sy = Sample.of(yValues, yUnit)
                            val m = disparity(sx, sy)
                            assertEquals(expectedUnit, m.unit.id, "unit mismatch")
                        }

                        else -> throw IllegalArgumentException("unknown estimator: $estimator")
                    }
                },
            )
        }

        return tests
    }
}
