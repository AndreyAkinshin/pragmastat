package com.pragmastat

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

class ReferenceTest {
    
    private val mapper = ObjectMapper().registerModule(KotlinModule())
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
                                (rawInput["x"] as List<Double>)
                            } else {
                                throw IllegalArgumentException("Invalid input format")
                            }
                        }
                        is List<*> -> {
                            @Suppress("UNCHECKED_CAST")
                            rawInput as List<Double>
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
                            val x = rawInput["x"] as List<Double>
                            @Suppress("UNCHECKED_CAST")
                            val y = rawInput["y"] as List<Double>
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
}