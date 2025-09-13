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

	relSpread, err := pragmastat.RelSpread(x)
	if err != nil {
		log.Fatal(err)
	}
	fmt.Printf("RelSpread: %.2f%%\n", relSpread*100)

	// Two-sample comparison
	fmt.Println("\n=== Two-Sample Comparison ===")
	y := []float64{2.1, 4.3, 3.2, 5.0, 3.7}

	fmt.Printf("Sample X: %v\n", x)
	fmt.Printf("Sample Y: %v\n", y)

	shift, err := pragmastat.Shift(x, y)
	if err != nil {
		log.Fatal(err)
	}
	fmt.Printf("Shift: %.4f (X is typically %.4f units larger than Y)\n", shift, shift)

	ratio, err := pragmastat.Ratio(x, y)
	if err != nil {
		log.Fatal(err)
	}
	fmt.Printf("Ratio: %.4f (X is typically %.1f%% of Y)\n", ratio, ratio*100)

	avgSpread, err := pragmastat.AvgSpread(x, y)
	if err != nil {
		log.Fatal(err)
	}
	fmt.Printf("AvgSpread: %.4f\n", avgSpread)

	disparity, err := pragmastat.Disparity(x, y)
	if err != nil {
		log.Fatal(err)
	}
	fmt.Printf("Disparity: %.4f\n", disparity)

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
