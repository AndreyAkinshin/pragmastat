package dev.pragmastat

import com.fasterxml.jackson.annotation.JsonIgnoreProperties
import com.fasterxml.jackson.annotation.JsonProperty
import com.fasterxml.jackson.databind.ObjectMapper
import com.fasterxml.jackson.module.kotlin.KotlinModule
import com.fasterxml.jackson.module.kotlin.readValue
import dev.pragmastat.distributions.*
import org.junit.jupiter.api.Assumptions
import org.junit.jupiter.api.DynamicTest
import org.junit.jupiter.api.Test
import org.junit.jupiter.api.TestFactory
import org.junit.jupiter.api.assertThrows
import java.io.File
import kotlin.math.abs
import kotlin.test.assertEquals
import kotlin.test.assertTrue

data class OneSampleInput(
    val x: List<Double>,
)

data class TwoSampleInput(
    val x: List<Double>,
    val y: List<Double>,
)

@JsonIgnoreProperties(ignoreUnknown = true)
data class OneSampleTestData(
    val input: OneSampleInput,
    val output: Double? = null,
    @JsonProperty("expected_error") val expectedError: Map<String, String>? = null,
)

@JsonIgnoreProperties(ignoreUnknown = true)
data class TwoSampleTestData(
    val input: TwoSampleInput,
    val output: Double? = null,
    @JsonProperty("expected_error") val expectedError: Map<String, String>? = null,
)

data class PairwiseMarginInput(
    val n: Int,
    val m: Int,
    val misrate: Double,
)

@JsonIgnoreProperties(ignoreUnknown = true)
data class PairwiseMarginTestData(
    val input: PairwiseMarginInput,
    val output: Int? = null,
    @JsonProperty("expected_error") val expectedError: Map<String, String>? = null,
)

data class ShiftBoundsInput(
    val x: List<Double>,
    val y: List<Double>,
    val misrate: Double,
)

data class BoundsOutput(
    val lower: Double,
    val upper: Double,
)

@JsonIgnoreProperties(ignoreUnknown = true)
data class ShiftBoundsTestData(
    val input: ShiftBoundsInput,
    val output: BoundsOutput? = null,
    @JsonProperty("expected_error") val expectedError: Map<String, String>? = null,
)

data class RatioBoundsInput(
    val x: List<Double>,
    val y: List<Double>,
    val misrate: Double,
)

@JsonIgnoreProperties(ignoreUnknown = true)
data class RatioBoundsTestData(
    val input: RatioBoundsInput,
    val output: BoundsOutput? = null,
    @JsonProperty("expected_error") val expectedError: Map<String, String>? = null,
)

class ReferenceTest {
    private val mapper = ObjectMapper().registerModule(KotlinModule.Builder().build())
    private val epsilon = 1e-9

    /** Create a y-sample with Subject.Y so validation errors report the correct subject. */
    private fun sampleY(values: List<Double>): Sample = Sample.create(values, null, NumberUnit, Subject.Y)

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

    @TestFactory
    fun testOneSampleEstimators(): List<DynamicTest> {
        val estimators =
            mapOf<String, (List<Double>) -> Double>(
                "center" to { x -> center(Sample.of(x)).value },
                "spread" to { x -> spread(Sample.of(x)).value },
            )

        val tests = mutableListOf<DynamicTest>()

        for ((estimatorName, estimatorFunc) in estimators) {
            val testDir = File("../tests/$estimatorName")
            if (!testDir.exists() || !testDir.isDirectory) {
                tests.add(
                    DynamicTest.dynamicTest("$estimatorName/skip-missing-directory") {
                        Assumptions.assumeTrue(false, "Skipping $estimatorName tests: directory not found")
                    },
                )
                continue
            }

            testDir.listFiles { _, name -> name.endsWith(".json") }?.forEach { file ->
                val testName = "$estimatorName/${file.nameWithoutExtension}"
                tests.add(
                    DynamicTest.dynamicTest(testName) {
                        val testData = mapper.readValue<OneSampleTestData>(file)

                        // Handle error test cases
                        if (testData.expectedError != null) {
                            val exception =
                                assertThrows<AssumptionException> {
                                    estimatorFunc(testData.input.x)
                                }
                            assertEquals(
                                testData.expectedError["id"],
                                exception.violation!!.id.id,
                                "Expected error id ${testData.expectedError["id"]}, got ${exception.violation!!.id.id}",
                            )
                            if (testData.expectedError.containsKey("subject")) {
                                assertEquals(
                                    testData.expectedError["subject"],
                                    exception.violation!!.subject.id,
                                    "Expected error subject ${testData.expectedError["subject"]}, got ${exception.violation!!.subject.id}",
                                )
                            }
                            return@dynamicTest
                        }

                        val result = estimatorFunc(testData.input.x)
                        assertClose(testData.output!!, result)
                    },
                )
            }
        }

        return tests
    }

    @TestFactory
    fun testTwoSampleEstimators(): List<DynamicTest> {
        val estimators =
            mapOf<String, (List<Double>, List<Double>) -> Double>(
                "shift" to { x, y -> shift(Sample.of(x), sampleY(y)).value },
                "ratio" to { x, y -> ratio(Sample.of(x), sampleY(y)).value },
                "avg-spread" to ::avgSpread,
                "disparity" to { x, y -> disparity(Sample.of(x), sampleY(y)).value },
            )

        val tests = mutableListOf<DynamicTest>()

        for ((estimatorName, estimatorFunc) in estimators) {
            val testDir = File("../tests/$estimatorName")
            if (!testDir.exists() || !testDir.isDirectory) {
                tests.add(
                    DynamicTest.dynamicTest("$estimatorName/skip-missing-directory") {
                        Assumptions.assumeTrue(false, "Skipping $estimatorName tests: directory not found")
                    },
                )
                continue
            }

            testDir.listFiles { _, name -> name.endsWith(".json") }?.forEach { file ->
                val testName = "$estimatorName/${file.nameWithoutExtension}"
                tests.add(
                    DynamicTest.dynamicTest(testName) {
                        val testData = mapper.readValue<TwoSampleTestData>(file)

                        // Handle error test cases
                        if (testData.expectedError != null) {
                            val exception =
                                assertThrows<AssumptionException> {
                                    estimatorFunc(testData.input.x, testData.input.y)
                                }
                            assertEquals(
                                testData.expectedError["id"],
                                exception.violation!!.id.id,
                                "Expected error id ${testData.expectedError["id"]}, got ${exception.violation!!.id.id}",
                            )
                            if (testData.expectedError.containsKey("subject")) {
                                assertEquals(
                                    testData.expectedError["subject"],
                                    exception.violation!!.subject.id,
                                    "Expected error subject ${testData.expectedError["subject"]}, got ${exception.violation!!.subject.id}",
                                )
                            }
                            return@dynamicTest
                        }

                        val result = estimatorFunc(testData.input.x, testData.input.y)
                        assertClose(testData.output!!, result)
                    },
                )
            }
        }

        return tests
    }

    @TestFactory
    fun testPairwiseMargin(): List<DynamicTest> {
        val tests = mutableListOf<DynamicTest>()
        val testDir = File("../tests/pairwise-margin")

        if (!testDir.exists() || !testDir.isDirectory) {
            Assumptions.assumeTrue(false, "Skipping pairwise-margin tests: directory not found")
            return tests
        }

        testDir.listFiles { _, name -> name.endsWith(".json") }?.forEach { file ->
            val testName = "pairwise-margin/${file.nameWithoutExtension}"
            tests.add(
                DynamicTest.dynamicTest(testName) {
                    val testData = mapper.readValue<PairwiseMarginTestData>(file)

                    // Handle error test cases
                    if (testData.expectedError != null) {
                        val exception =
                            assertThrows<AssumptionException> {
                                pairwiseMargin(testData.input.n, testData.input.m, testData.input.misrate)
                            }
                        kotlin.test.assertEquals(
                            testData.expectedError["id"],
                            exception.violation!!.id.id,
                            "Expected error id ${testData.expectedError["id"]}, got ${exception.violation!!.id.id}",
                        )
                        if (testData.expectedError.containsKey("subject")) {
                            kotlin.test.assertEquals(
                                testData.expectedError["subject"],
                                exception.violation!!.subject.id,
                                "Expected error subject ${testData.expectedError["subject"]}, got ${exception.violation!!.subject.id}",
                            )
                        }
                        return@dynamicTest
                    }

                    val result =
                        pairwiseMargin(
                            testData.input.n,
                            testData.input.m,
                            testData.input.misrate,
                        )
                    assertTrue(
                        result == testData.output,
                        "Expected ${testData.output} but got $result",
                    )
                },
            )
        }

        return tests
    }

    @TestFactory
    fun testShiftBounds(): List<DynamicTest> {
        val tests = mutableListOf<DynamicTest>()
        val testDir = File("../tests/shift-bounds")

        if (!testDir.exists() || !testDir.isDirectory) {
            Assumptions.assumeTrue(false, "Skipping shift-bounds tests: directory not found")
            return tests
        }

        testDir.listFiles { _, name -> name.endsWith(".json") }?.forEach { file ->
            val testName = "shift-bounds/${file.nameWithoutExtension}"
            tests.add(
                DynamicTest.dynamicTest(testName) {
                    val testData = mapper.readValue<ShiftBoundsTestData>(file)

                    // Handle error test cases
                    if (testData.expectedError != null) {
                        val exception =
                            assertThrows<AssumptionException> {
                                shiftBounds(Sample.of(testData.input.x), sampleY(testData.input.y), testData.input.misrate)
                            }
                        kotlin.test.assertEquals(
                            testData.expectedError["id"],
                            exception.violation!!.id.id,
                            "Expected error id ${testData.expectedError["id"]}, got ${exception.violation!!.id.id}",
                        )
                        if (testData.expectedError.containsKey("subject")) {
                            kotlin.test.assertEquals(
                                testData.expectedError["subject"],
                                exception.violation!!.subject.id,
                                "Expected error subject ${testData.expectedError["subject"]}, got ${exception.violation!!.subject.id}",
                            )
                        }
                        return@dynamicTest
                    }

                    val result =
                        shiftBounds(
                            Sample.of(testData.input.x),
                            sampleY(testData.input.y),
                            testData.input.misrate,
                        )
                    assertClose(testData.output!!.lower, result.lower)
                    assertClose(testData.output!!.upper, result.upper)
                },
            )
        }

        return tests
    }

    @TestFactory
    fun testRatioBounds(): List<DynamicTest> {
        val tests = mutableListOf<DynamicTest>()
        val testDir = File("../tests/ratio-bounds")

        if (!testDir.exists() || !testDir.isDirectory) {
            Assumptions.assumeTrue(false, "Skipping ratio-bounds tests: directory not found")
            return tests
        }

        testDir.listFiles { _, name -> name.endsWith(".json") }?.forEach { file ->
            val testName = "ratio-bounds/${file.nameWithoutExtension}"
            tests.add(
                DynamicTest.dynamicTest(testName) {
                    val testData = mapper.readValue<RatioBoundsTestData>(file)

                    // Handle error test cases
                    if (testData.expectedError != null) {
                        val exception =
                            assertThrows<AssumptionException> {
                                ratioBounds(Sample.of(testData.input.x), sampleY(testData.input.y), testData.input.misrate)
                            }
                        kotlin.test.assertEquals(
                            testData.expectedError["id"],
                            exception.violation!!.id.id,
                            "Expected error id ${testData.expectedError["id"]}, got ${exception.violation!!.id.id}",
                        )
                        if (testData.expectedError.containsKey("subject")) {
                            kotlin.test.assertEquals(
                                testData.expectedError["subject"],
                                exception.violation!!.subject.id,
                                "Expected error subject ${testData.expectedError["subject"]}, got ${exception.violation!!.subject.id}",
                            )
                        }
                        return@dynamicTest
                    }

                    val result =
                        ratioBounds(
                            Sample.of(testData.input.x),
                            sampleY(testData.input.y),
                            testData.input.misrate,
                        )
                    assertClose(testData.output!!.lower, result.lower)
                    assertClose(testData.output!!.upper, result.upper)
                },
            )
        }

        return tests
    }

    // Rng reference tests

    data class UniformInput(val seed: Long, val count: Int)

    data class UniformTestData(val input: UniformInput, val output: List<Double>)

    data class UniformIntInput(val seed: Long, val min: Long, val max: Long, val count: Int)

    data class UniformIntTestData(val input: UniformIntInput, val output: List<Long>)

    data class StringSeedInput(val seed: String, val count: Int)

    data class StringSeedTestData(val input: StringSeedInput, val output: List<Double>)

    data class ShuffleInput(val seed: Long, val x: List<Double>)

    data class ShuffleTestData(val input: ShuffleInput, val output: List<Double>)

    data class SampleInput(val seed: Long, val x: List<Double>, val k: Int)

    data class SampleTestData(val input: SampleInput, val output: List<Double>)

    data class ResampleTestData(val input: SampleInput, val output: List<Double>)

    // Distribution reference tests

    data class UniformDistInput(val seed: Long, val min: Double, val max: Double, val count: Int)

    data class UniformDistTestData(val input: UniformDistInput, val output: List<Double>)

    data class AdditiveDistInput(
        val seed: Long,
        val mean: Double,
        val stdDev: Double,
        val count: Int,
    )

    data class AdditiveDistTestData(val input: AdditiveDistInput, val output: List<Double>)

    data class MultiplicDistInput(
        val seed: Long,
        val logMean: Double,
        val logStdDev: Double,
        val count: Int,
    )

    data class MultiplicDistTestData(val input: MultiplicDistInput, val output: List<Double>)

    data class ExpDistInput(val seed: Long, val rate: Double, val count: Int)

    data class ExpDistTestData(val input: ExpDistInput, val output: List<Double>)

    data class PowerDistInput(val seed: Long, val min: Double, val shape: Double, val count: Int)

    data class PowerDistTestData(val input: PowerDistInput, val output: List<Double>)

    // New Rng test data classes
    data class UniformRangeInput(val seed: Long, val min: Double, val max: Double, val count: Int)

    data class UniformRangeTestData(val input: UniformRangeInput, val output: List<Double>)

    data class UniformF32Input(val seed: Long, val count: Int)

    data class UniformF32TestData(val input: UniformF32Input, val output: List<Float>)

    data class UniformI32Input(val seed: Long, val min: Int, val max: Int, val count: Int)

    data class UniformI32TestData(val input: UniformI32Input, val output: List<Int>)

    data class UniformBoolInput(val seed: Long, val count: Int)

    data class UniformBoolTestData(val input: UniformBoolInput, val output: List<Boolean>)

    @TestFactory
    fun testRngUniform(): List<DynamicTest> {
        val tests = mutableListOf<DynamicTest>()
        val testDir = File("../tests/rng")

        if (!testDir.exists() || !testDir.isDirectory) {
            Assumptions.assumeTrue(false, "Skipping rng uniform tests: directory not found")
            return tests
        }

        testDir.listFiles { _, name -> name.startsWith("uniform-seed-") && name.endsWith(".json") }?.forEach { file ->
            val testName = "rng/${file.nameWithoutExtension}"
            tests.add(
                DynamicTest.dynamicTest(testName) {
                    val testData = mapper.readValue<UniformTestData>(file)
                    val rng = Rng(testData.input.seed)
                    for (i in 0 until testData.input.count) {
                        val actual = rng.uniformDouble()
                        val expected = testData.output[i]
                        assertClose(expected, actual, 1e-15)
                    }
                },
            )
        }

        return tests
    }

    @TestFactory
    fun testRngUniformInt(): List<DynamicTest> {
        val tests = mutableListOf<DynamicTest>()
        val testDir = File("../tests/rng")

        if (!testDir.exists() || !testDir.isDirectory) {
            Assumptions.assumeTrue(false, "Skipping rng uniform int tests: directory not found")
            return tests
        }

        testDir.listFiles { _, name -> name.startsWith("uniform-int-") && name.endsWith(".json") }?.forEach { file ->
            val testName = "rng/${file.nameWithoutExtension}"
            tests.add(
                DynamicTest.dynamicTest(testName) {
                    val testData = mapper.readValue<UniformIntTestData>(file)
                    val rng = Rng(testData.input.seed)
                    for (i in 0 until testData.input.count) {
                        val actual = rng.uniformLong(testData.input.min, testData.input.max)
                        val expected = testData.output[i]
                        assertTrue(actual == expected, "Expected $expected but got $actual at index $i")
                    }
                },
            )
        }

        return tests
    }

    @TestFactory
    fun testRngStringSeed(): List<DynamicTest> {
        val tests = mutableListOf<DynamicTest>()
        val testDir = File("../tests/rng")

        if (!testDir.exists() || !testDir.isDirectory) {
            Assumptions.assumeTrue(false, "Skipping rng string seed tests: directory not found")
            return tests
        }

        testDir.listFiles { _, name -> name.startsWith("uniform-string-") && name.endsWith(".json") }?.forEach { file ->
            val testName = "rng/${file.nameWithoutExtension}"
            tests.add(
                DynamicTest.dynamicTest(testName) {
                    val testData = mapper.readValue<StringSeedTestData>(file)
                    val rng = Rng(testData.input.seed)
                    for (i in 0 until testData.input.count) {
                        val actual = rng.uniformDouble()
                        val expected = testData.output[i]
                        assertClose(expected, actual, 1e-15)
                    }
                },
            )
        }

        return tests
    }

    @TestFactory
    fun testRngUniformRange(): List<DynamicTest> {
        val tests = mutableListOf<DynamicTest>()
        val testDir = File("../tests/rng")

        if (!testDir.exists() || !testDir.isDirectory) {
            Assumptions.assumeTrue(false, "Skipping rng uniform range tests: directory not found")
            return tests
        }

        testDir.listFiles { _, name -> name.startsWith("uniform-range-") && name.endsWith(".json") }?.forEach { file ->
            val testName = "rng/${file.nameWithoutExtension}"
            tests.add(
                DynamicTest.dynamicTest(testName) {
                    val testData = mapper.readValue<UniformRangeTestData>(file)
                    val rng = Rng(testData.input.seed)
                    for (i in 0 until testData.input.count) {
                        val actual = rng.uniformDouble(testData.input.min, testData.input.max)
                        val expected = testData.output[i]
                        assertClose(expected, actual, 1e-12)
                    }
                },
            )
        }

        return tests
    }

    @TestFactory
    fun testRngUniformFloat(): List<DynamicTest> {
        val tests = mutableListOf<DynamicTest>()
        val testDir = File("../tests/rng")

        if (!testDir.exists() || !testDir.isDirectory) {
            Assumptions.assumeTrue(false, "Skipping rng uniform float tests: directory not found")
            return tests
        }

        testDir.listFiles { _, name -> name.startsWith("uniform-f32-") && name.endsWith(".json") }?.forEach { file ->
            val testName = "rng/${file.nameWithoutExtension}"
            tests.add(
                DynamicTest.dynamicTest(testName) {
                    val testData = mapper.readValue<UniformF32TestData>(file)
                    val rng = Rng(testData.input.seed)
                    for (i in 0 until testData.input.count) {
                        val actual = rng.uniformFloat()
                        val expected = testData.output[i]
                        assertTrue(actual == expected, "Expected $expected but got $actual at index $i")
                    }
                },
            )
        }

        return tests
    }

    @TestFactory
    fun testRngUniformI32(): List<DynamicTest> {
        val tests = mutableListOf<DynamicTest>()
        val testDir = File("../tests/rng")

        if (!testDir.exists() || !testDir.isDirectory) {
            Assumptions.assumeTrue(false, "Skipping rng uniform i32 tests: directory not found")
            return tests
        }

        testDir.listFiles { _, name -> name.startsWith("uniform-i32-") && name.endsWith(".json") }?.forEach { file ->
            val testName = "rng/${file.nameWithoutExtension}"
            tests.add(
                DynamicTest.dynamicTest(testName) {
                    val testData = mapper.readValue<UniformI32TestData>(file)
                    val rng = Rng(testData.input.seed)
                    for (i in 0 until testData.input.count) {
                        val actual = rng.uniformInt(testData.input.min, testData.input.max)
                        val expected = testData.output[i]
                        assertTrue(actual == expected, "Expected $expected but got $actual at index $i")
                    }
                },
            )
        }

        return tests
    }

    @TestFactory
    fun testRngUniformBool(): List<DynamicTest> {
        val tests = mutableListOf<DynamicTest>()
        val testDir = File("../tests/rng")

        if (!testDir.exists() || !testDir.isDirectory) {
            Assumptions.assumeTrue(false, "Skipping rng uniform bool tests: directory not found")
            return tests
        }

        testDir.listFiles { _, name -> name.startsWith("uniform-bool-seed-") && name.endsWith(".json") }?.forEach { file ->
            val testName = "rng/${file.nameWithoutExtension}"
            tests.add(
                DynamicTest.dynamicTest(testName) {
                    val testData = mapper.readValue<UniformBoolTestData>(file)
                    val rng = Rng(testData.input.seed)
                    for (i in 0 until testData.input.count) {
                        val actual = rng.uniformBool()
                        val expected = testData.output[i]
                        assertTrue(actual == expected, "Expected $expected but got $actual at index $i")
                    }
                },
            )
        }

        return tests
    }

    @TestFactory
    fun testShuffle(): List<DynamicTest> {
        val tests = mutableListOf<DynamicTest>()
        val testDir = File("../tests/shuffle")

        if (!testDir.exists() || !testDir.isDirectory) {
            Assumptions.assumeTrue(false, "Skipping shuffle tests: directory not found")
            return tests
        }

        testDir.listFiles { _, name -> name.endsWith(".json") }?.forEach { file ->
            val testName = "shuffle/${file.nameWithoutExtension}"
            tests.add(
                DynamicTest.dynamicTest(testName) {
                    val testData = mapper.readValue<ShuffleTestData>(file)
                    val rng = Rng(testData.input.seed)
                    val actual = rng.shuffle(testData.input.x)
                    for (i in actual.indices) {
                        assertClose(testData.output[i], actual[i], 1e-15)
                    }
                },
            )
        }

        return tests
    }

    @TestFactory
    fun testSample(): List<DynamicTest> {
        val tests = mutableListOf<DynamicTest>()
        val testDir = File("../tests/sample")

        if (!testDir.exists() || !testDir.isDirectory) {
            Assumptions.assumeTrue(false, "Skipping sample tests: directory not found")
            return tests
        }

        testDir.listFiles { _, name -> name.endsWith(".json") }?.forEach { file ->
            val testName = "sample/${file.nameWithoutExtension}"
            tests.add(
                DynamicTest.dynamicTest(testName) {
                    val testData = mapper.readValue<SampleTestData>(file)
                    val rng = Rng(testData.input.seed)
                    val actual = rng.sample(testData.input.x, testData.input.k)
                    for (i in actual.indices) {
                        assertClose(testData.output[i], actual[i], 1e-15)
                    }
                },
            )
        }

        return tests
    }

    @TestFactory
    fun testResample(): List<DynamicTest> {
        val tests = mutableListOf<DynamicTest>()
        val testDir = File("../tests/resample")

        if (!testDir.exists() || !testDir.isDirectory) {
            Assumptions.assumeTrue(false, "Skipping resample tests: directory not found")
            return tests
        }

        testDir.listFiles { _, name -> name.endsWith(".json") }?.forEach { file ->
            val testName = "resample/${file.nameWithoutExtension}"
            tests.add(
                DynamicTest.dynamicTest(testName) {
                    val testData = mapper.readValue<ResampleTestData>(file)
                    val rng = Rng(testData.input.seed)
                    val actual = rng.resample(testData.input.x, testData.input.k)
                    for (i in actual.indices) {
                        assertClose(testData.output[i], actual[i], 1e-15)
                    }
                },
            )
        }

        return tests
    }

    @TestFactory
    fun testUniformDistribution(): List<DynamicTest> {
        val tests = mutableListOf<DynamicTest>()
        val testDir = File("../tests/distributions/uniform")

        if (!testDir.exists() || !testDir.isDirectory) {
            Assumptions.assumeTrue(false, "Skipping uniform distribution tests: directory not found")
            return tests
        }

        testDir.listFiles { _, name -> name.endsWith(".json") }?.forEach { file ->
            val testName = "distributions/uniform/${file.nameWithoutExtension}"
            tests.add(
                DynamicTest.dynamicTest(testName) {
                    val testData = mapper.readValue<UniformDistTestData>(file)
                    val rng = Rng(testData.input.seed)
                    val dist = Uniform(testData.input.min, testData.input.max)
                    for (i in 0 until testData.input.count) {
                        val actual = dist.sample(rng)
                        val expected = testData.output[i]
                        assertClose(expected, actual, 1e-12)
                    }
                },
            )
        }

        return tests
    }

    @TestFactory
    fun testAdditiveDistribution(): List<DynamicTest> {
        val tests = mutableListOf<DynamicTest>()
        val testDir = File("../tests/distributions/additive")

        if (!testDir.exists() || !testDir.isDirectory) {
            Assumptions.assumeTrue(false, "Skipping additive distribution tests: directory not found")
            return tests
        }

        testDir.listFiles { _, name -> name.endsWith(".json") }?.forEach { file ->
            val testName = "distributions/additive/${file.nameWithoutExtension}"
            tests.add(
                DynamicTest.dynamicTest(testName) {
                    val testData = mapper.readValue<AdditiveDistTestData>(file)
                    val rng = Rng(testData.input.seed)
                    val dist = Additive(testData.input.mean, testData.input.stdDev)
                    for (i in 0 until testData.input.count) {
                        val actual = dist.sample(rng)
                        val expected = testData.output[i]
                        assertClose(expected, actual, 1e-12)
                    }
                },
            )
        }

        return tests
    }

    @TestFactory
    fun testMultiplicDistribution(): List<DynamicTest> {
        val tests = mutableListOf<DynamicTest>()
        val testDir = File("../tests/distributions/multiplic")

        if (!testDir.exists() || !testDir.isDirectory) {
            Assumptions.assumeTrue(false, "Skipping multiplic distribution tests: directory not found")
            return tests
        }

        testDir.listFiles { _, name -> name.endsWith(".json") }?.forEach { file ->
            val testName = "distributions/multiplic/${file.nameWithoutExtension}"
            tests.add(
                DynamicTest.dynamicTest(testName) {
                    val testData = mapper.readValue<MultiplicDistTestData>(file)
                    val rng = Rng(testData.input.seed)
                    val dist = Multiplic(testData.input.logMean, testData.input.logStdDev)
                    for (i in 0 until testData.input.count) {
                        val actual = dist.sample(rng)
                        val expected = testData.output[i]
                        assertClose(expected, actual, 1e-12)
                    }
                },
            )
        }

        return tests
    }

    @TestFactory
    fun testExpDistribution(): List<DynamicTest> {
        val tests = mutableListOf<DynamicTest>()
        val testDir = File("../tests/distributions/exp")

        if (!testDir.exists() || !testDir.isDirectory) {
            Assumptions.assumeTrue(false, "Skipping exp distribution tests: directory not found")
            return tests
        }

        testDir.listFiles { _, name -> name.endsWith(".json") }?.forEach { file ->
            val testName = "distributions/exp/${file.nameWithoutExtension}"
            tests.add(
                DynamicTest.dynamicTest(testName) {
                    val testData = mapper.readValue<ExpDistTestData>(file)
                    val rng = Rng(testData.input.seed)
                    val dist = Exp(testData.input.rate)
                    for (i in 0 until testData.input.count) {
                        val actual = dist.sample(rng)
                        val expected = testData.output[i]
                        assertClose(expected, actual, 1e-12)
                    }
                },
            )
        }

        return tests
    }

    @TestFactory
    fun testPowerDistribution(): List<DynamicTest> {
        val tests = mutableListOf<DynamicTest>()
        val testDir = File("../tests/distributions/power")

        if (!testDir.exists() || !testDir.isDirectory) {
            Assumptions.assumeTrue(false, "Skipping power distribution tests: directory not found")
            return tests
        }

        testDir.listFiles { _, name -> name.endsWith(".json") }?.forEach { file ->
            val testName = "distributions/power/${file.nameWithoutExtension}"
            tests.add(
                DynamicTest.dynamicTest(testName) {
                    val testData = mapper.readValue<PowerDistTestData>(file)
                    val rng = Rng(testData.input.seed)
                    val dist = Power(testData.input.min, testData.input.shape)
                    for (i in 0 until testData.input.count) {
                        val actual = dist.sample(rng)
                        val expected = testData.output[i]
                        assertClose(expected, actual, 1e-12)
                    }
                },
            )
        }

        return tests
    }

    @Test
    fun `sample with negative k throws IllegalArgumentException`() {
        val rng = Rng("test-sample-validation")
        assertThrows<IllegalArgumentException> {
            rng.sample(listOf(1, 2, 3), -1)
        }
    }

    // One-sample bounds reference tests

    data class SignedRankMarginInput(val n: Int, val misrate: Double)

    @JsonIgnoreProperties(ignoreUnknown = true)
    data class SignedRankMarginTestData(
        val input: SignedRankMarginInput,
        val output: Int? = null,
        @JsonProperty("expected_error") val expectedError: Map<String, String>? = null,
    )

    data class CenterBoundsInput(val x: List<Double>, val misrate: Double)

    @JsonIgnoreProperties(ignoreUnknown = true)
    data class CenterBoundsTestData(
        val input: CenterBoundsInput,
        val output: BoundsOutput? = null,
        @JsonProperty("expected_error") val expectedError: Map<String, String>? = null,
    )

    data class SpreadBoundsInput(val x: List<Double>, val misrate: Double, val seed: String)

    @JsonIgnoreProperties(ignoreUnknown = true)
    data class SpreadBoundsTestData(
        val input: SpreadBoundsInput,
        val output: BoundsOutput? = null,
        @JsonProperty("expected_error") val expectedError: Map<String, String>? = null,
    )

    data class AvgSpreadBoundsInput(val x: List<Double>, val y: List<Double>, val misrate: Double, val seed: String)

    @JsonIgnoreProperties(ignoreUnknown = true)
    data class AvgSpreadBoundsTestData(
        val input: AvgSpreadBoundsInput,
        val output: BoundsOutput? = null,
        @JsonProperty("expected_error") val expectedError: Map<String, String>? = null,
    )

    data class DisparityBoundsInput(val x: List<Double>, val y: List<Double>, val misrate: Double, val seed: String)

    @JsonIgnoreProperties(ignoreUnknown = true)
    data class DisparityBoundsTestData(
        val input: DisparityBoundsInput,
        val output: BoundsOutput? = null,
        @JsonProperty("expected_error") val expectedError: Map<String, String>? = null,
    )

    @TestFactory
    fun testSignedRankMargin(): List<DynamicTest> {
        val tests = mutableListOf<DynamicTest>()
        val testDir = File("../tests/signed-rank-margin")

        if (!testDir.exists() || !testDir.isDirectory) {
            Assumptions.assumeTrue(false, "Skipping signed-rank-margin tests: directory not found")
            return tests
        }

        testDir.listFiles { _, name -> name.endsWith(".json") }?.forEach { file ->
            val testName = "signed-rank-margin/${file.nameWithoutExtension}"
            tests.add(
                DynamicTest.dynamicTest(testName) {
                    val testData = mapper.readValue<SignedRankMarginTestData>(file)

                    // Handle error test cases
                    if (testData.expectedError != null) {
                        val exception =
                            assertThrows<AssumptionException> {
                                signedRankMargin(testData.input.n, testData.input.misrate)
                            }
                        kotlin.test.assertEquals(
                            testData.expectedError["id"],
                            exception.violation!!.id.id,
                            "Expected error id ${testData.expectedError["id"]}, got ${exception.violation!!.id.id}",
                        )
                        if (testData.expectedError.containsKey("subject")) {
                            kotlin.test.assertEquals(
                                testData.expectedError["subject"],
                                exception.violation!!.subject.id,
                                "Expected error subject ${testData.expectedError["subject"]}, got ${exception.violation!!.subject.id}",
                            )
                        }
                        return@dynamicTest
                    }

                    val result =
                        signedRankMargin(
                            testData.input.n,
                            testData.input.misrate,
                        )
                    assertTrue(
                        result == testData.output,
                        "Expected ${testData.output} but got $result",
                    )
                },
            )
        }

        return tests
    }

    @TestFactory
    fun testCenterBounds(): List<DynamicTest> {
        val tests = mutableListOf<DynamicTest>()
        val testDir = File("../tests/center-bounds")

        if (!testDir.exists() || !testDir.isDirectory) {
            Assumptions.assumeTrue(false, "Skipping center-bounds tests: directory not found")
            return tests
        }

        testDir.listFiles { _, name -> name.endsWith(".json") }?.forEach { file ->
            val testName = "center-bounds/${file.nameWithoutExtension}"
            tests.add(
                DynamicTest.dynamicTest(testName) {
                    val testData = mapper.readValue<CenterBoundsTestData>(file)

                    // Handle error test cases
                    if (testData.expectedError != null) {
                        val exception =
                            assertThrows<AssumptionException> {
                                centerBounds(Sample.of(testData.input.x), testData.input.misrate)
                            }
                        assertEquals(
                            testData.expectedError["id"],
                            exception.violation!!.id.id,
                            "Expected error id ${testData.expectedError["id"]}, got ${exception.violation!!.id.id}",
                        )
                        if (testData.expectedError.containsKey("subject")) {
                            assertEquals(
                                testData.expectedError["subject"],
                                exception.violation!!.subject.id,
                                "Expected error subject ${testData.expectedError["subject"]}, got ${exception.violation!!.subject.id}",
                            )
                        }
                        return@dynamicTest
                    }

                    val result =
                        centerBounds(
                            Sample.of(testData.input.x),
                            testData.input.misrate,
                        )
                    assertClose(testData.output!!.lower, result.lower)
                    assertClose(testData.output!!.upper, result.upper)
                },
            )
        }

        return tests
    }

    @TestFactory
    fun testSpreadBounds(): List<DynamicTest> {
        val tests = mutableListOf<DynamicTest>()
        val testDir = File("../tests/spread-bounds")

        if (!testDir.exists() || !testDir.isDirectory) {
            Assumptions.assumeTrue(false, "Skipping spread-bounds tests: directory not found")
            return tests
        }

        testDir.listFiles { _, name -> name.endsWith(".json") }?.forEach { file ->
            val testName = "spread-bounds/${file.nameWithoutExtension}"
            tests.add(
                DynamicTest.dynamicTest(testName) {
                    val testData = mapper.readValue<SpreadBoundsTestData>(file)

                    if (testData.expectedError != null) {
                        val exception =
                            assertThrows<AssumptionException> {
                                spreadBounds(Sample.of(testData.input.x), testData.input.misrate, testData.input.seed)
                            }
                        assertEquals(
                            testData.expectedError["id"],
                            exception.violation!!.id.id,
                            "Expected error id ${testData.expectedError["id"]}, got ${exception.violation!!.id.id}",
                        )
                        if (testData.expectedError.containsKey("subject")) {
                            assertEquals(
                                testData.expectedError["subject"],
                                exception.violation!!.subject.id,
                                "Expected error subject ${testData.expectedError["subject"]}, got ${exception.violation!!.subject.id}",
                            )
                        }
                        return@dynamicTest
                    }

                    val result =
                        spreadBounds(
                            Sample.of(testData.input.x),
                            testData.input.misrate,
                            testData.input.seed,
                        )
                    assertClose(testData.output!!.lower, result.lower)
                    assertClose(testData.output!!.upper, result.upper)
                },
            )
        }

        return tests
    }

    @TestFactory
    fun testAvgSpreadBounds(): List<DynamicTest> {
        val tests = mutableListOf<DynamicTest>()
        val testDir = File("../tests/avg-spread-bounds")

        if (!testDir.exists() || !testDir.isDirectory) {
            Assumptions.assumeTrue(false, "Skipping avg-spread-bounds tests: directory not found")
            return tests
        }

        testDir.listFiles { _, name -> name.endsWith(".json") }?.forEach { file ->
            val testName = "avg-spread-bounds/${file.nameWithoutExtension}"
            tests.add(
                DynamicTest.dynamicTest(testName) {
                    val testData = mapper.readValue<AvgSpreadBoundsTestData>(file)

                    if (testData.expectedError != null) {
                        val exception =
                            assertThrows<AssumptionException> {
                                avgSpreadBounds(testData.input.x, testData.input.y, testData.input.misrate, testData.input.seed)
                            }
                        assertEquals(
                            testData.expectedError["id"],
                            exception.violation!!.id.id,
                            "Expected error id ${testData.expectedError["id"]}, got ${exception.violation!!.id.id}",
                        )
                        if (testData.expectedError.containsKey("subject")) {
                            assertEquals(
                                testData.expectedError["subject"],
                                exception.violation!!.subject.id,
                                "Expected error subject ${testData.expectedError["subject"]}, got ${exception.violation!!.subject.id}",
                            )
                        }
                        return@dynamicTest
                    }

                    val result =
                        avgSpreadBounds(
                            testData.input.x,
                            testData.input.y,
                            testData.input.misrate,
                            testData.input.seed,
                        )
                    assertClose(testData.output!!.lower, result.lower)
                    assertClose(testData.output!!.upper, result.upper)
                },
            )
        }

        return tests
    }

    @TestFactory
    fun testDisparityBounds(): List<DynamicTest> {
        val tests = mutableListOf<DynamicTest>()
        val testDir = File("../tests/disparity-bounds")

        if (!testDir.exists() || !testDir.isDirectory) {
            Assumptions.assumeTrue(false, "Skipping disparity-bounds tests: directory not found")
            return tests
        }

        testDir.listFiles { _, name -> name.endsWith(".json") }?.forEach { file ->
            val testName = "disparity-bounds/${file.nameWithoutExtension}"
            tests.add(
                DynamicTest.dynamicTest(testName) {
                    val testData = mapper.readValue<DisparityBoundsTestData>(file)

                    if (testData.expectedError != null) {
                        val exception =
                            assertThrows<AssumptionException> {
                                disparityBounds(
                                    Sample.of(testData.input.x),
                                    sampleY(testData.input.y),
                                    testData.input.misrate,
                                    testData.input.seed,
                                )
                            }
                        assertEquals(
                            testData.expectedError["id"],
                            exception.violation!!.id.id,
                            "Expected error id ${testData.expectedError["id"]}, got ${exception.violation!!.id.id}",
                        )
                        if (testData.expectedError.containsKey("subject")) {
                            assertEquals(
                                testData.expectedError["subject"],
                                exception.violation!!.subject.id,
                                "Expected error subject ${testData.expectedError["subject"]}, got ${exception.violation!!.subject.id}",
                            )
                        }
                        return@dynamicTest
                    }

                    val result =
                        disparityBounds(
                            Sample.of(testData.input.x),
                            sampleY(testData.input.y),
                            testData.input.misrate,
                            testData.input.seed,
                        )
                    assertClose(testData.output!!.lower, result.lower)
                    assertClose(testData.output!!.upper, result.upper)
                },
            )
        }

        return tests
    }
}
