package com.pragmastat.example

import com.pragmastat.*

fun main() {
    // One-sample analysis
    println("=== One-Sample Analysis ===")
    val x = listOf(1.2, 3.4, 2.5, 4.1, 2.8)
    
    println("Sample: $x")
    println("Center: %.4f".format(center(x)))
    println("Spread: %.4f".format(spread(x)))
    println("RelSpread: %.2f%%".format(relSpread(x) * 100))
    
    // Two-sample comparison
    println("\n=== Two-Sample Comparison ===")
    val y = listOf(2.1, 4.3, 3.2, 5.0, 3.7)
    
    println("Sample X: $x")
    println("Sample Y: $y")
    val shiftVal = shift(x, y)
    println("Shift: %.4f (X is typically %.4f units larger than Y)".format(shiftVal, shiftVal))
    val ratioVal = ratio(x, y)
    println("Ratio: %.4f (X is typically %.1f%% of Y)".format(ratioVal, ratioVal * 100))
    println("AvgSpread: %.4f".format(avgSpread(x, y)))
    println("Disparity: %.4f".format(disparity(x, y)))
    
    // Demonstrating robustness with outliers
    println("\n=== Robustness Demonstration ===")
    val normal = listOf(1.0, 2.0, 3.0, 4.0, 5.0)
    val withOutlier = listOf(1.0, 2.0, 3.0, 4.0, 100.0)
    
    println("Normal sample: $normal")
    println("  Center: %.2f".format(center(normal)))
    println("  Spread: %.2f".format(spread(normal)))
    
    println("Sample with outlier: $withOutlier")
    println("  Center: %.2f (robust)".format(center(withOutlier)))
    println("  Spread: %.2f (robust)".format(spread(withOutlier)))
    
    // Traditional mean for comparison
    fun mean(values: List<Double>): Double = values.sum() / values.size
    
    println("\nComparison with traditional mean:")
    println("  Mean of normal: %.2f".format(mean(normal)))
    println("  Mean with outlier: %.2f (affected by outlier)".format(mean(withOutlier)))
}