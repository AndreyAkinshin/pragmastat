package dev.pragmastat

import com.fasterxml.jackson.databind.ObjectMapper
import com.fasterxml.jackson.module.kotlin.KotlinModule
import com.fasterxml.jackson.module.kotlin.readValue
import org.junit.jupiter.api.Test
import org.junit.jupiter.api.DynamicTest
import org.junit.jupiter.api.TestFactory
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
                    
                    val result = estimatorFunc(input)
                    assertClose(testData.output, result)
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
                    
                    val result = estimatorFunc(input.first, input.second)
                    assertClose(testData.output, result)
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
}