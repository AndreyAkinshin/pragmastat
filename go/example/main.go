package main

import (
	"fmt"
	"log"

	"github.com/AndreyAkinshin/pragmastat"
)

func main() {
	// One-sample analysis
	fmt.Println("=== One-Sample Analysis ===")
	x := []float64{1.2, 3.4, 2.5, 4.1, 2.8}

	fmt.Printf("Sample: %v\n", x)
	center, err := pragmastat.Center(x)
	if err != nil {
		log.Fatal(err)
	}
	fmt.Printf("Center: %.4f\n", center)

	spread, err := pragmastat.Spread(x)
	if err != nil {
		log.Fatal(err)
	}
	fmt.Printf("Spread: %.4f\n", spread)

	volatility, err := pragmastat.Volatility(x)
	if err != nil {
		log.Fatal(err)
	}
	fmt.Printf("Volatility: %.2f%%\n", volatility*100)

	precision, err := pragmastat.Precision(x)
	if err != nil {
		log.Fatal(err)
	}
	fmt.Printf("Precision: %.4f\n", precision)

	// Two-sample comparison
	fmt.Println("\n=== Two-Sample Comparison ===")
	y := []float64{2.1, 4.3, 3.2, 5.0, 3.7}

	fmt.Printf("Sample X: %v\n", x)
	fmt.Printf("Sample Y: %v\n", y)

	medShift, err := pragmastat.MedShift(x, y)
	if err != nil {
		log.Fatal(err)
	}
	fmt.Printf("MedShift: %.4f (X is typically %.4f units larger than Y)\n", medShift, medShift)

	medRatio, err := pragmastat.MedRatio(x, y)
	if err != nil {
		log.Fatal(err)
	}
	fmt.Printf("MedRatio: %.4f (X is typically %.1f%% of Y)\n", medRatio, medRatio*100)

	medSpread, err := pragmastat.MedSpread(x, y)
	if err != nil {
		log.Fatal(err)
	}
	fmt.Printf("MedSpread: %.4f\n", medSpread)

	medDisparity, err := pragmastat.MedDisparity(x, y)
	if err != nil {
		log.Fatal(err)
	}
	fmt.Printf("MedDisparity: %.4f\n", medDisparity)

	// Demonstrating robustness with outliers
	fmt.Println("\n=== Robustness Demonstration ===")
	normal := []float64{1, 2, 3, 4, 5}
	withOutlier := []float64{1, 2, 3, 4, 100}

	fmt.Printf("Normal sample: %v\n", normal)
	centerNormal, err := pragmastat.Center(normal)
	if err != nil {
		log.Fatal(err)
	}
	fmt.Printf("  Center: %.2f\n", centerNormal)

	spreadNormal, err := pragmastat.Spread(normal)
	if err != nil {
		log.Fatal(err)
	}
	fmt.Printf("  Spread: %.2f\n", spreadNormal)

	fmt.Printf("Sample with outlier: %v\n", withOutlier)
	centerOutlier, err := pragmastat.Center(withOutlier)
	if err != nil {
		log.Fatal(err)
	}
	fmt.Printf("  Center: %.2f (robust)\n", centerOutlier)

	spreadOutlier, err := pragmastat.Spread(withOutlier)
	if err != nil {
		log.Fatal(err)
	}
	fmt.Printf("  Spread: %.2f (robust)\n", spreadOutlier)

	// Traditional mean for comparison
	mean := func(x []float64) float64 {
		sum := 0.0
		for _, v := range x {
			sum += v
		}
		return sum / float64(len(x))
	}

	fmt.Printf("\nComparison with traditional mean:\n")
	fmt.Printf("  Mean of normal: %.2f\n", mean(normal))
	fmt.Printf("  Mean with outlier: %.2f (affected by outlier)\n", mean(withOutlier))
}
