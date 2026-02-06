package pragmastat

import (
	"math"
)

const (
	// signedRankMaxExactSize is the maximum n for exact signed-rank computation.
	// Limited to 63 because 2^n must fit in a 64-bit integer for exact computation.
	signedRankMaxExactSize = 63
)

// SignedRankMargin computes the margin for one-sample signed-rank bounds.
// Uses Wilcoxon signed-rank distribution to determine the margin that achieves
// the specified misrate.
func SignedRankMargin(n int, misrate float64) (int, error) {
	if n <= 0 {
		return 0, NewDomainError(SubjectX)
	}
	if math.IsNaN(misrate) || misrate < 0 || misrate > 1 {
		return 0, NewDomainError(SubjectMisrate)
	}

	minMisrate, err := minAchievableMisrateOneSample(n)
	if err != nil {
		return 0, err
	}
	if misrate < minMisrate {
		return 0, NewDomainError(SubjectMisrate)
	}

	if n <= signedRankMaxExactSize {
		return signedRankMarginExact(n, misrate), nil
	}
	return signedRankMarginApprox(n, misrate)
}

// signedRankMarginExact computes one-sided margin using exact Wilcoxon signed-rank distribution.
// Uses dynamic programming to compute the CDF.
func signedRankMarginExact(n int, misrate float64) int {
	raw := signedRankMarginExactRaw(n, misrate/2)
	return raw * 2
}

func signedRankMarginExactRaw(n int, p float64) int {
	total := uint64(1) << n
	maxW := int64(n) * int64(n+1) / 2

	count := make([]uint64, maxW+1)
	count[0] = 1

	for i := 1; i <= n; i++ {
		maxWi := int64(i) * int64(i+1) / 2
		if maxWi > maxW {
			maxWi = maxW
		}
		for w := maxWi; w >= int64(i); w-- {
			count[w] += count[w-int64(i)]
		}
	}

	var cumulative uint64
	for w := int64(0); w <= maxW; w++ {
		cumulative += count[w]
		cdf := float64(cumulative) / float64(total)
		if cdf >= p {
			return int(w)
		}
	}

	return int(maxW)
}

// signedRankMarginApprox computes one-sided margin using Edgeworth approximation for large n.
func signedRankMarginApprox(n int, misrate float64) (int, error) {
	raw := signedRankMarginApproxRaw(n, misrate/2)
	margin := raw * 2
	if margin > int64(math.MaxInt32) {
		return 0, NewDomainError(SubjectX)
	}
	return int(margin), nil
}

func signedRankMarginApproxRaw(n int, misrate float64) int64 {
	maxW := int64(n) * int64(n+1) / 2
	a := int64(0)
	b := maxW

	for a < b-1 {
		c := (a + b) / 2
		cdf := signedRankEdgeworthCdf(n, c)
		if cdf < misrate {
			a = c
		} else {
			b = c
		}
	}

	if signedRankEdgeworthCdf(n, b) < misrate {
		return b
	}
	return a
}

// signedRankEdgeworthCdf computes Edgeworth expansion for Wilcoxon signed-rank distribution CDF.
func signedRankEdgeworthCdf(n int, w int64) float64 {
	mu := float64(n) * float64(n+1) / 4.0
	sigma2 := float64(n) * float64(n+1) * float64(2*n+1) / 24.0
	sigma := math.Sqrt(sigma2)

	// +0.5 continuity correction: computing P(W ≤ w) for a left-tail discrete CDF
	z := (float64(w) - mu + 0.5) / sigma
	phi := math.Exp(-z*z/2) / math.Sqrt(2*math.Pi)
	Phi := gaussCdf(z)

	mu4 := centralMoment4(n)
	kappa4 := mu4 - 3*sigma2*sigma2

	e3 := kappa4 / (24 * sigma2 * sigma2)

	z2 := z * z
	z3 := z2 * z
	f3 := -phi * (z3 - 3*z)

	edgeworth := Phi + e3*f3
	return math.Min(math.Max(edgeworth, 0), 1)
}

// centralMoment4 computes the 4th central moment of signed-rank distribution.
// E[(W - mu)^4] where W is the Wilcoxon signed-rank statistic.
func centralMoment4(n int) float64 {
	n2 := float64(n) * float64(n)
	n3 := n2 * float64(n)
	n4 := n2 * n2
	n5 := n4 * float64(n)

	return (9*n5 + 45*n4 + 65*n3 + 15*n2 - 14*float64(n)) / 480.0
}
