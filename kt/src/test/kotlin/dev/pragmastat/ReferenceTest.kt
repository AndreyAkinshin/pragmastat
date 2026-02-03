package dev.pragmastat

import com.fasterxml.jackson.databind.ObjectMapper
import com.fasterxml.jackson.module.kotlin.KotlinModule
import com.fasterxml.jackson.module.kotlin.readValue
import dev.pragmastat.distributions.*
import org.junit.jupiter.api.Test
import org.junit.jupiter.api.DynamicTest
import org.junit.jupiter.api.TestFactory
import org.junit.jupiter.api.assertThrows
import java.io.File
import kotlin.test.assertTrue
import kotlin.math.abs

data class TestData(
    val input: Any,
    val output: Double
)

data class OneSampleInput(
    val x: List<Double>
)

data class TwoSampleInput(
    val x: List<Double>,
    val y: List<Double>
)

data class PairwiseMarginInput(
    val n: Int,
    val m: Int,
    val misrate: Double
)

data class PairwiseMarginTestData(
    val input: PairwiseMarginInput,
    val output: Int
)

data class ShiftBoundsInput(
    val x: List<Double>,
    val y: List<Double>,
    val misrate: Double
)

data class BoundsOutput(
    val lower: Double,
    val upper: Double
)

data class ShiftBoundsTestData(
    val input: ShiftBoundsInput,
    val output: BoundsOutput
)

data class RatioBoundsInput(
    val x: List<Double>,
    val y: List<Double>,
    val misrate: Double
)

data class RatioBoundsTestData(
    val input: RatioBoundsInput,
    val output: BoundsOutput
)

class ReferenceTest {
    
    private val mapper = ObjectMapper().registerModule(KotlinModule.Builder().build())
    private val epsilon = 1e-9
    
    private fun assertClose(expected: Double, actual: Double, tolerance: Double = epsilon) {
        assertTrue(abs(expected - actual) < tolerance, 
            "Expected $expected but got $actual (difference: ${abs(expected - actual)})")
    }
    
    @TestFactory
    fun testOneSampleEstimators(): List<DynamicTest> {
        val estimators = mapOf(
            "center" to ::center,
            "spread" to ::spread,
            "rel-spread" to ::relSpread
        )
        
        val tests = mutableListOf<DynamicTest>()
        
        for ((estimatorName, estimatorFunc) in estimators) {
            val testDir = File("../tests/$estimatorName")
            if (!testDir.exists() || !testDir.isDirectory) {
                println("Skipping $estimatorName tests: directory not found")
                continue
            }
            
            testDir.listFiles { _, name -> name.endsWith(".json") }?.forEach { file ->
                val testName = "${estimatorName}/${file.nameWithoutExtension}"
                tests.add(DynamicTest.dynamicTest(testName) {
                    val testData = mapper.readValue<TestData>(file)
                    
                    // Try to parse input as OneSampleInput
                    val input = when (val rawInput = testData.input) {
                        is Map<*, *> -> {
                            if (rawInput.containsKey("x")) {
                                @Suppress("UNCHECKED_CAST")
                                val rawList = rawInput["x"] as List<*>
                                rawList.map { (it as Number).toDouble() }
                            } else {
                                throw IllegalArgumentException("Invalid input format")
                            }
                        }
                        is List<*> -> {
                            rawInput.map { (it as Number).toDouble() }
                        }
                        else -> throw IllegalArgumentException("Invalid input format")
                    }
                    
                    try {
                        val result = estimatorFunc(input)
                        assertClose(testData.output, result)
                    } catch (e: AssumptionException) {
                        // Skip cases that violate assumptions - tested separately
                    }
                })
            }
        }

        return tests
    }

    @TestFactory
    fun testTwoSampleEstimators(): List<DynamicTest> {
        val estimators = mapOf(
            "shift" to ::shift,
            "ratio" to ::ratio,
            "avg-spread" to ::avgSpread,
            "disparity" to ::disparity
        )
        
        val tests = mutableListOf<DynamicTest>()
        
        for ((estimatorName, estimatorFunc) in estimators) {
            val testDir = File("../tests/$estimatorName")
            if (!testDir.exists() || !testDir.isDirectory) {
                println("Skipping $estimatorName tests: directory not found")
                continue
            }
            
            testDir.listFiles { _, name -> name.endsWith(".json") }?.forEach { file ->
                val testName = "${estimatorName}/${file.nameWithoutExtension}"
                tests.add(DynamicTest.dynamicTest(testName) {
                    val testData = mapper.readValue<TestData>(file)
                    
                    // Parse as TwoSampleInput
                    val input = when (val rawInput = testData.input) {
                        is Map<*, *> -> {
                            @Suppress("UNCHECKED_CAST")
                            val rawX = rawInput["x"] as List<*>
                            val x = rawX.map { (it as Number).toDouble() }
                            @Suppress("UNCHECKED_CAST")
                            val rawY = rawInput["y"] as List<*>
                            val y = rawY.map { (it as Number).toDouble() }
                            Pair(x, y)
                        }
                        else -> throw IllegalArgumentException("Invalid input format for two-sample test")
                    }
                    
                    try {
                        val result = estimatorFunc(input.first, input.second)
                        assertClose(testData.output, result)
                    } catch (e: AssumptionException) {
                        // Skip cases that violate assumptions - tested separately
                    }
                })
            }
        }

        return tests
    }

    @TestFactory
    fun testPairwiseMargin(): List<DynamicTest> {
        val tests = mutableListOf<DynamicTest>()
        val testDir = File("../tests/pairwise-margin")

        if (!testDir.exists() || !testDir.isDirectory) {
            println("Skipping pairwise-margin tests: directory not found")
            return tests
        }

        testDir.listFiles { _, name -> name.endsWith(".json") }?.forEach { file ->
            val testName = "pairwise-margin/${file.nameWithoutExtension}"
            tests.add(DynamicTest.dynamicTest(testName) {
                val testData = mapper.readValue<PairwiseMarginTestData>(file)
                val result = pairwiseMargin(
                    testData.input.n,
                    testData.input.m,
                    testData.input.misrate
                )
                assertTrue(result == testData.output,
                    "Expected ${testData.output} but got $result")
            })
        }

        return tests
    }

    @TestFactory
    fun testShiftBounds(): List<DynamicTest> {
        val tests = mutableListOf<DynamicTest>()
        val testDir = File("../tests/shift-bounds")

        if (!testDir.exists() || !testDir.isDirectory) {
            println("Skipping shift-bounds tests: directory not found")
            return tests
        }

        testDir.listFiles { _, name -> name.endsWith(".json") }?.forEach { file ->
            val testName = "shift-bounds/${file.nameWithoutExtension}"
            tests.add(DynamicTest.dynamicTest(testName) {
                val testData = mapper.readValue<ShiftBoundsTestData>(file)
                val result = shiftBounds(
                    testData.input.x,
                    testData.input.y,
                    testData.input.misrate
                )
                assertClose(testData.output.lower, result.lower)
                assertClose(testData.output.upper, result.upper)
            })
        }

        return tests
    }

    @TestFactory
    fun testRatioBounds(): List<DynamicTest> {
        val tests = mutableListOf<DynamicTest>()
        val testDir = File("../tests/ratio-bounds")

        if (!testDir.exists() || !testDir.isDirectory) {
            println("Skipping ratio-bounds tests: directory not found")
            return tests
        }

        testDir.listFiles { _, name -> name.endsWith(".json") }?.forEach { file ->
            val testName = "ratio-bounds/${file.nameWithoutExtension}"
            tests.add(DynamicTest.dynamicTest(testName) {
                val testData = mapper.readValue<RatioBoundsTestData>(file)
                val result = ratioBounds(
                    testData.input.x,
                    testData.input.y,
                    testData.input.misrate
                )
                assertClose(testData.output.lower, result.lower)
                assertClose(testData.output.upper, result.upper)
            })
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

    // Distribution reference tests

    data class UniformDistInput(val seed: Long, val min: Double, val max: Double, val count: Int)
    data class UniformDistTestData(val input: UniformDistInput, val output: List<Double>)

    data class AdditiveDistInput(
        val seed: Long,
        val mean: Double,
        val stdDev: Double,
        val count: Int
    )
    data class AdditiveDistTestData(val input: AdditiveDistInput, val output: List<Double>)

    data class MultiplicDistInput(
        val seed: Long,
        val logMean: Double,
        val logStdDev: Double,
        val count: Int
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
            println("Skipping rng uniform tests: directory not found")
            return tests
        }

        testDir.listFiles { _, name -> name.startsWith("uniform-seed-") && name.endsWith(".json") }?.forEach { file ->
            val testName = "rng/${file.nameWithoutExtension}"
            tests.add(DynamicTest.dynamicTest(testName) {
                val testData = mapper.readValue<UniformTestData>(file)
                val rng = Rng(testData.input.seed)
                for (i in 0 until testData.input.count) {
                    val actual = rng.uniform()
                    val expected = testData.output[i]
                    assertClose(expected, actual, 1e-15)
                }
            })
        }

        return tests
    }

    @TestFactory
    fun testRngUniformInt(): List<DynamicTest> {
        val tests = mutableListOf<DynamicTest>()
        val testDir = File("../tests/rng")

        if (!testDir.exists() || !testDir.isDirectory) {
            println("Skipping rng uniform int tests: directory not found")
            return tests
        }

        testDir.listFiles { _, name -> name.startsWith("uniform-int-") && name.endsWith(".json") }?.forEach { file ->
            val testName = "rng/${file.nameWithoutExtension}"
            tests.add(DynamicTest.dynamicTest(testName) {
                val testData = mapper.readValue<UniformIntTestData>(file)
                val rng = Rng(testData.input.seed)
                for (i in 0 until testData.input.count) {
                    val actual = rng.uniformLong(testData.input.min, testData.input.max)
                    val expected = testData.output[i]
                    assertTrue(actual == expected, "Expected $expected but got $actual at index $i")
                }
            })
        }

        return tests
    }

    @TestFactory
    fun testRngStringSeed(): List<DynamicTest> {
        val tests = mutableListOf<DynamicTest>()
        val testDir = File("../tests/rng")

        if (!testDir.exists() || !testDir.isDirectory) {
            println("Skipping rng string seed tests: directory not found")
            return tests
        }

        testDir.listFiles { _, name -> name.startsWith("uniform-string-") && name.endsWith(".json") }?.forEach { file ->
            val testName = "rng/${file.nameWithoutExtension}"
            tests.add(DynamicTest.dynamicTest(testName) {
                val testData = mapper.readValue<StringSeedTestData>(file)
                val rng = Rng(testData.input.seed)
                for (i in 0 until testData.input.count) {
                    val actual = rng.uniform()
                    val expected = testData.output[i]
                    assertClose(expected, actual, 1e-15)
                }
            })
        }

        return tests
    }

    @TestFactory
    fun testRngUniformRange(): List<DynamicTest> {
        val tests = mutableListOf<DynamicTest>()
        val testDir = File("../tests/rng")

        if (!testDir.exists() || !testDir.isDirectory) {
            println("Skipping rng uniform range tests: directory not found")
            return tests
        }

        testDir.listFiles { _, name -> name.startsWith("uniform-range-") && name.endsWith(".json") }?.forEach { file ->
            val testName = "rng/${file.nameWithoutExtension}"
            tests.add(DynamicTest.dynamicTest(testName) {
                val testData = mapper.readValue<UniformRangeTestData>(file)
                val rng = Rng(testData.input.seed)
                for (i in 0 until testData.input.count) {
                    val actual = rng.uniform(testData.input.min, testData.input.max)
                    val expected = testData.output[i]
                    assertClose(expected, actual, 1e-12)
                }
            })
        }

        return tests
    }

    @TestFactory
    fun testRngUniformFloat(): List<DynamicTest> {
        val tests = mutableListOf<DynamicTest>()
        val testDir = File("../tests/rng")

        if (!testDir.exists() || !testDir.isDirectory) {
            println("Skipping rng uniform float tests: directory not found")
            return tests
        }

        testDir.listFiles { _, name -> name.startsWith("uniform-f32-") && name.endsWith(".json") }?.forEach { file ->
            val testName = "rng/${file.nameWithoutExtension}"
            tests.add(DynamicTest.dynamicTest(testName) {
                val testData = mapper.readValue<UniformF32TestData>(file)
                val rng = Rng(testData.input.seed)
                for (i in 0 until testData.input.count) {
                    val actual = rng.uniformFloat()
                    val expected = testData.output[i]
                    assertTrue(actual == expected, "Expected $expected but got $actual at index $i")
                }
            })
        }

        return tests
    }

    @TestFactory
    fun testRngUniformI32(): List<DynamicTest> {
        val tests = mutableListOf<DynamicTest>()
        val testDir = File("../tests/rng")

        if (!testDir.exists() || !testDir.isDirectory) {
            println("Skipping rng uniform i32 tests: directory not found")
            return tests
        }

        testDir.listFiles { _, name -> name.startsWith("uniform-i32-") && name.endsWith(".json") }?.forEach { file ->
            val testName = "rng/${file.nameWithoutExtension}"
            tests.add(DynamicTest.dynamicTest(testName) {
                val testData = mapper.readValue<UniformI32TestData>(file)
                val rng = Rng(testData.input.seed)
                for (i in 0 until testData.input.count) {
                    val actual = rng.uniformInt(testData.input.min, testData.input.max)
                    val expected = testData.output[i]
                    assertTrue(actual == expected, "Expected $expected but got $actual at index $i")
                }
            })
        }

        return tests
    }

    @TestFactory
    fun testRngUniformBool(): List<DynamicTest> {
        val tests = mutableListOf<DynamicTest>()
        val testDir = File("../tests/rng")

        if (!testDir.exists() || !testDir.isDirectory) {
            println("Skipping rng uniform bool tests: directory not found")
            return tests
        }

        testDir.listFiles { _, name -> name.startsWith("uniform-bool-seed-") && name.endsWith(".json") }?.forEach { file ->
            val testName = "rng/${file.nameWithoutExtension}"
            tests.add(DynamicTest.dynamicTest(testName) {
                val testData = mapper.readValue<UniformBoolTestData>(file)
                val rng = Rng(testData.input.seed)
                for (i in 0 until testData.input.count) {
                    val actual = rng.uniformBool()
                    val expected = testData.output[i]
                    assertTrue(actual == expected, "Expected $expected but got $actual at index $i")
                }
            })
        }

        return tests
    }

    @TestFactory
    fun testShuffle(): List<DynamicTest> {
        val tests = mutableListOf<DynamicTest>()
        val testDir = File("../tests/shuffle")

        if (!testDir.exists() || !testDir.isDirectory) {
            println("Skipping shuffle tests: directory not found")
            return tests
        }

        testDir.listFiles { _, name -> name.endsWith(".json") }?.forEach { file ->
            val testName = "shuffle/${file.nameWithoutExtension}"
            tests.add(DynamicTest.dynamicTest(testName) {
                val testData = mapper.readValue<ShuffleTestData>(file)
                val rng = Rng(testData.input.seed)
                val actual = rng.shuffle(testData.input.x)
                for (i in actual.indices) {
                    assertClose(testData.output[i], actual[i], 1e-15)
                }
            })
        }

        return tests
    }

    @TestFactory
    fun testSample(): List<DynamicTest> {
        val tests = mutableListOf<DynamicTest>()
        val testDir = File("../tests/sample")

        if (!testDir.exists() || !testDir.isDirectory) {
            println("Skipping sample tests: directory not found")
            return tests
        }

        testDir.listFiles { _, name -> name.endsWith(".json") }?.forEach { file ->
            val testName = "sample/${file.nameWithoutExtension}"
            tests.add(DynamicTest.dynamicTest(testName) {
                val testData = mapper.readValue<SampleTestData>(file)
                val rng = Rng(testData.input.seed)
                val actual = rng.sample(testData.input.x, testData.input.k)
                for (i in actual.indices) {
                    assertClose(testData.output[i], actual[i], 1e-15)
                }
            })
        }

        return tests
    }

    @TestFactory
    fun testUniformDistribution(): List<DynamicTest> {
        val tests = mutableListOf<DynamicTest>()
        val testDir = File("../tests/distributions/uniform")

        if (!testDir.exists() || !testDir.isDirectory) {
            println("Skipping uniform distribution tests: directory not found")
            return tests
        }

        testDir.listFiles { _, name -> name.endsWith(".json") }?.forEach { file ->
            val testName = "distributions/uniform/${file.nameWithoutExtension}"
            tests.add(DynamicTest.dynamicTest(testName) {
                val testData = mapper.readValue<UniformDistTestData>(file)
                val rng = Rng(testData.input.seed)
                val dist = Uniform(testData.input.min, testData.input.max)
                for (i in 0 until testData.input.count) {
                    val actual = dist.sample(rng)
                    val expected = testData.output[i]
                    assertClose(expected, actual, 1e-12)
                }
            })
        }

        return tests
    }

    @TestFactory
    fun testAdditiveDistribution(): List<DynamicTest> {
        val tests = mutableListOf<DynamicTest>()
        val testDir = File("../tests/distributions/additive")

        if (!testDir.exists() || !testDir.isDirectory) {
            println("Skipping additive distribution tests: directory not found")
            return tests
        }

        testDir.listFiles { _, name -> name.endsWith(".json") }?.forEach { file ->
            val testName = "distributions/additive/${file.nameWithoutExtension}"
            tests.add(DynamicTest.dynamicTest(testName) {
                val testData = mapper.readValue<AdditiveDistTestData>(file)
                val rng = Rng(testData.input.seed)
                val dist = Additive(testData.input.mean, testData.input.stdDev)
                for (i in 0 until testData.input.count) {
                    val actual = dist.sample(rng)
                    val expected = testData.output[i]
                    assertClose(expected, actual, 1e-12)
                }
            })
        }

        return tests
    }

    @TestFactory
    fun testMultiplicDistribution(): List<DynamicTest> {
        val tests = mutableListOf<DynamicTest>()
        val testDir = File("../tests/distributions/multiplic")

        if (!testDir.exists() || !testDir.isDirectory) {
            println("Skipping multiplic distribution tests: directory not found")
            return tests
        }

        testDir.listFiles { _, name -> name.endsWith(".json") }?.forEach { file ->
            val testName = "distributions/multiplic/${file.nameWithoutExtension}"
            tests.add(DynamicTest.dynamicTest(testName) {
                val testData = mapper.readValue<MultiplicDistTestData>(file)
                val rng = Rng(testData.input.seed)
                val dist = Multiplic(testData.input.logMean, testData.input.logStdDev)
                for (i in 0 until testData.input.count) {
                    val actual = dist.sample(rng)
                    val expected = testData.output[i]
                    assertClose(expected, actual, 1e-12)
                }
            })
        }

        return tests
    }

    @TestFactory
    fun testExpDistribution(): List<DynamicTest> {
        val tests = mutableListOf<DynamicTest>()
        val testDir = File("../tests/distributions/exp")

        if (!testDir.exists() || !testDir.isDirectory) {
            println("Skipping exp distribution tests: directory not found")
            return tests
        }

        testDir.listFiles { _, name -> name.endsWith(".json") }?.forEach { file ->
            val testName = "distributions/exp/${file.nameWithoutExtension}"
            tests.add(DynamicTest.dynamicTest(testName) {
                val testData = mapper.readValue<ExpDistTestData>(file)
                val rng = Rng(testData.input.seed)
                val dist = Exp(testData.input.rate)
                for (i in 0 until testData.input.count) {
                    val actual = dist.sample(rng)
                    val expected = testData.output[i]
                    assertClose(expected, actual, 1e-12)
                }
            })
        }

        return tests
    }

    @TestFactory
    fun testPowerDistribution(): List<DynamicTest> {
        val tests = mutableListOf<DynamicTest>()
        val testDir = File("../tests/distributions/power")

        if (!testDir.exists() || !testDir.isDirectory) {
            println("Skipping power distribution tests: directory not found")
            return tests
        }

        testDir.listFiles { _, name -> name.endsWith(".json") }?.forEach { file ->
            val testName = "distributions/power/${file.nameWithoutExtension}"
            tests.add(DynamicTest.dynamicTest(testName) {
                val testData = mapper.readValue<PowerDistTestData>(file)
                val rng = Rng(testData.input.seed)
                val dist = Power(testData.input.min, testData.input.shape)
                for (i in 0 until testData.input.count) {
                    val actual = dist.sample(rng)
                    val expected = testData.output[i]
                    assertClose(expected, actual, 1e-12)
                }
            })
        }

        return tests
    }

    @Test
    fun `sample with negative k throws IllegalArgumentException`() {
        val rng = Rng(42)
        assertThrows<IllegalArgumentException> {
            rng.sample(listOf(1, 2, 3), -1)
        }
    }
}
