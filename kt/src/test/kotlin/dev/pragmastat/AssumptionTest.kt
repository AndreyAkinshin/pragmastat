package dev.pragmastat

import com.fasterxml.jackson.annotation.JsonIgnoreProperties
import com.fasterxml.jackson.annotation.JsonProperty
import com.fasterxml.jackson.databind.DeserializationFeature
import com.fasterxml.jackson.databind.JsonNode
import com.fasterxml.jackson.databind.ObjectMapper
import com.fasterxml.jackson.module.kotlin.KotlinModule
import com.fasterxml.jackson.module.kotlin.readValue
import org.junit.jupiter.api.DynamicTest
import org.junit.jupiter.api.TestFactory
import java.io.File
import kotlin.test.assertEquals
import kotlin.test.assertFailsWith

/**
 * Assumption violation conformance tests.
 * These tests verify that assumption violations are reported correctly and
 * consistently across all languages.
 */
class AssumptionTest {

    private val mapper = ObjectMapper()
        .registerModule(KotlinModule.Builder().build())
        .configure(DeserializationFeature.FAIL_ON_UNKNOWN_PROPERTIES, false)

    data class ExpectedViolation(
        val id: String
    )

    data class TestInputs(
        val x: List<JsonNode>? = null,
        val y: List<JsonNode>? = null,
        val misrate: JsonNode? = null,
        val n: Int? = null,
        val seed: String? = null
    )

    data class AssumptionTestCase(
        val name: String,
        val function: String,
        val inputs: TestInputs,
        @JsonProperty("expected_violation")
        val expectedViolation: ExpectedViolation
    )

    data class AssumptionTestSuite(
        val suite: String,
        val description: String,
        val cases: List<AssumptionTestCase>
    )

    data class SuiteEntry(
        val name: String,
        val file: String,
        val description: String
    )

    data class Manifest(
        val name: String,
        val description: String,
        val suites: List<SuiteEntry>
    )

    private fun findRepoRoot(): File {
        var current = File(System.getProperty("user.dir"))
        while (!File(current, "CITATION.cff").exists()) {
            current = current.parentFile ?: throw IllegalStateException(
                "Could not find repository root (CITATION.cff not found)"
            )
        }
        return current
    }

    private fun parseValue(node: JsonNode): Double {
        return when {
            node.isNumber -> node.asDouble()
            node.isTextual -> when (node.asText()) {
                "NaN" -> Double.NaN
                "Infinity" -> Double.POSITIVE_INFINITY
                "-Infinity" -> Double.NEGATIVE_INFINITY
                else -> throw IllegalArgumentException("Unknown string value: ${node.asText()}")
            }
            else -> throw IllegalArgumentException("Unexpected node type: ${node.nodeType}")
        }
    }

    private fun parseArray(arr: List<JsonNode>?): List<Double> {
        return arr?.map { parseValue(it) } ?: emptyList()
    }

    private fun callFunction(funcName: String, inputs: TestInputs) {
        val x = parseArray(inputs.x)
        val y = parseArray(inputs.y)
        when (funcName) {
            "Center" -> center(x)
            "Ratio" -> ratio(x, y)
            "RelSpread" -> relSpread(x)
            "Spread" -> spread(x)
            "Shift" -> shift(x, y)
            "AvgSpread" -> avgSpread(x, y)
            "Disparity" -> disparity(x, y)
            "MedianBounds" -> medianBounds(x, parseValue(inputs.misrate!!))
            "CenterBounds" -> centerBounds(x, parseValue(inputs.misrate!!))
            "SignedRankMargin" -> signedRankMargin(inputs.n!!, parseValue(inputs.misrate!!))
            else -> throw IllegalArgumentException("Unknown function: $funcName")
        }
    }

    @TestFactory
    fun testAssumptionViolations(): List<DynamicTest> {
        val repoRoot = findRepoRoot()
        val assumptionsDir = File(repoRoot, "tests/assumptions")

        val manifestFile = File(assumptionsDir, "manifest.json")
        val manifest: Manifest = mapper.readValue(manifestFile)

        val tests = mutableListOf<DynamicTest>()

        for (suiteEntry in manifest.suites) {
            val suiteFile = File(assumptionsDir, suiteEntry.file)
            val suite: AssumptionTestSuite = mapper.readValue(suiteFile)

            for (testCase in suite.cases) {
                val testName = "${suite.suite}/${testCase.name}"
                tests.add(DynamicTest.dynamicTest(testName) {
                    val expectedId = AssumptionId.entries.find { it.id == testCase.expectedViolation.id }
                        ?: throw IllegalArgumentException("Unknown assumption ID: ${testCase.expectedViolation.id}")

                    val exception = assertFailsWith<AssumptionException> {
                        callFunction(testCase.function, testCase.inputs)
                    }

                    assertEquals(expectedId, exception.violation.id,
                        "Expected id=${expectedId.id}, got ${exception.violation.id.id}")
                })
            }
        }

        return tests
    }
}
