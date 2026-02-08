package pragmastat

import (
	"errors"
	"math"
)

const (
	maxExactSize        = 400
	maxAcceptableBinomN = 65
)

// PairwiseMargin determines how many extreme pairwise differences to exclude
// when constructing bounds based on the distribution of dominance statistics.
// Uses exact calculation for small samples (n+m <= 400) and Edgeworth
// approximation for larger samples.
//
// Returns an error if n <= 0, m <= 0, or misrate is outside [0, 1] or NaN.
func PairwiseMargin(n, m int, misrate float64) (int, error) {
	if n <= 0 {
		return 0, NewDomainError(SubjectX)
	}
	if m <= 0 {
		return 0, NewDomainError(SubjectY)
	}
	if math.IsNaN(misrate) || misrate < 0 || misrate > 1 {
		return 0, NewDomainError(SubjectMisrate)
	}

	minMisrate, err := minAchievableMisrateTwoSample(n, m)
	if err != nil {
		return 0, err
	}
	if misrate < minMisrate {
		return 0, NewDomainError(SubjectMisrate)
	}

	// Use exact method for small to medium samples
	if n+m <= maxExactSize {
		return pairwiseMarginExact(n, m, misrate), nil
	}
	return pairwiseMarginApprox(n, m, misrate)
}

// pairwiseMarginExact uses the exact distribution based on Loeffler's recurrence.
func pairwiseMarginExact(n, m int, misrate float64) int {
	return pairwiseMarginExactRaw(n, m, misrate/2) * 2
}

// pairwiseMarginApprox uses Edgeworth approximation for large samples.
func pairwiseMarginApprox(n, m int, misrate float64) (int, error) {
	raw, err := pairwiseMarginApproxRaw(n, m, misrate/2)
	if err != nil {
		return 0, err
	}
	return raw * 2, nil
}

// pairwiseMarginExactRaw implements the inversed Loeffler (1982) algorithm.
// Reference: "Über eine Partition der nat. Zahlen und ihre Anwendung beim U-Test"
func pairwiseMarginExactRaw(n, m int, p float64) int {
	var total float64
	if n+m < maxAcceptableBinomN {
		total = float64(binomialCoefficient(n+m, m))
	} else {
		total = binomialCoefficientFloat(float64(n+m), float64(m))
	}

	pmf := []float64{1}   // pmf[0] = 1
	sigma := []float64{0} // sigma[0] is unused

	u := 0
	cdf := 1.0 / total

	if cdf >= p {
		return 0
	}

	for {
		u++
		// Ensure sigma has entry for u
		if len(sigma) <= u {
			value := 0
			for d := 1; d <= n; d++ {
				if u%d == 0 && u >= d {
					value += d
				}
			}
			for d := m + 1; d <= m+n; d++ {
				if u%d == 0 && u >= d {
					value -= d
				}
			}
			sigma = append(sigma, float64(value))
		}

		// Compute pmf[u] using Loeffler recurrence
		sum := 0.0
		for i := 0; i < u; i++ {
			sum += pmf[i] * sigma[u-i]
		}
		sum /= float64(u)
		pmf = append(pmf, sum)

		cdf += sum / total
		if cdf >= p {
			return u
		}
		if sum == 0 {
			break
		}
	}

	return len(pmf) - 1
}

// pairwiseMarginApproxRaw uses inverse Edgeworth approximation.
func pairwiseMarginApproxRaw(n, m int, misrate float64) (int, error) {
	a := int64(0)
	b := int64(n) * int64(m)
	for a < b-1 {
		c := (a + b) / 2
		p := edgeworthCdf(n, m, c)
		if p < misrate {
			a = c
		} else {
			b = c
		}
	}

	var result int64
	if edgeworthCdf(n, m, b) < misrate {
		result = b
	} else {
		result = a
	}

	if result > int64(^uint(0)>>1) {
		return 0, errors.New("pairwise margin exceeds int range")
	}
	return int(result), nil
}

// edgeworthCdf computes the CDF using Edgeworth expansion.
func edgeworthCdf(n, m int, u int64) float64 {
	nm := float64(n) * float64(m)
	mu := nm / 2.0
	su := math.Sqrt(nm * float64(n+m+1) / 12.0)
	// -0.5 continuity correction: computing P(U ≥ u) for a right-tail discrete CDF
	z := (float64(u) - mu - 0.5) / su
	phi := math.Exp(-z*z/2) / math.Sqrt(2*math.Pi)
	Phi := gaussCdf(z)

	// Pre-compute powers of n and m as float64 (avoids int64 overflow for large n, m)
	nf := float64(n)
	mf := float64(m)
	n2 := nf * nf
	n3 := n2 * nf
	n4 := n2 * n2
	m2 := mf * mf
	m3 := m2 * mf
	m4 := m2 * m2

	// Compute moments using float64 arithmetic
	mu2 := (nf * mf * (nf + mf + 1)) / 12.0
	mu4 := (nf * mf * (nf + mf + 1) *
		(5*mf*nf*(mf+nf) -
			2*(m2+n2) +
			3*mf*nf -
			2*(nf+mf))) / 240.0

	mu6 := (nf * mf * (nf + mf + 1) *
		(35*m2*n2*(m2+n2) +
			70*m3*n3 -
			42*mf*nf*(m3+n3) -
			14*m2*n2*(nf+mf) +
			16*(n4+m4) -
			52*nf*mf*(n2+m2) -
			43*n2*m2 +
			32*(m3+n3) +
			14*mf*nf*(nf+mf) +
			8*(n2+m2) +
			16*nf*mf -
			8*(nf+mf))) / 4032.0

	// Pre-compute powers of mu2 and related terms
	mu2_2 := mu2 * mu2
	mu2_3 := mu2_2 * mu2
	mu4_mu2_2 := mu4 / mu2_2

	// Factorial constants: 4! = 24, 6! = 720, 8! = 40320
	e3 := (mu4_mu2_2 - 3) / 24.0
	e5 := (mu6/mu2_3 - 15*mu4_mu2_2 + 30) / 720.0
	e7 := 35 * (mu4_mu2_2 - 3) * (mu4_mu2_2 - 3) / 40320.0

	// Pre-compute powers of z for Hermite polynomials
	z2 := z * z
	z3 := z2 * z
	z5 := z3 * z2
	z7 := z5 * z2

	f3 := -phi * (z3 - 3*z)
	f5 := -phi * (z5 - 10*z3 + 15*z)
	f7 := -phi * (z7 - 21*z5 + 105*z3 - 105*z)

	edgeworth := Phi + e3*f3 + e5*f5 + e7*f7
	return math.Max(0, math.Min(edgeworth, 1))
}

// binomialCoefficient computes C(n, k) for small values using Pascal's triangle.
func binomialCoefficient(n, k int) int64 {
	if k < 0 || k > n {
		return 0
	}
	if k > n-k {
		k = n - k
	}

	result := int64(1)
	for i := 0; i < k; i++ {
		result = result * int64(n-i) / int64(i+1)
	}
	return result
}

// binomialCoefficientFloat computes C(n, k) for large values using logarithms.
func binomialCoefficientFloat(n, k float64) float64 {
	return math.Exp(logBinomialCoefficient(n, k))
}

// logBinomialCoefficient computes log(C(n, k)).
func logBinomialCoefficient(n, k float64) float64 {
	return logFactorial(n) - logFactorial(k) - logFactorial(n-k)
}

// logFactorial computes log(n!) using Stirling's approximation for large n.
// Since n! = Gamma(n+1), we compute log(Gamma(n+1)) using stirlingApproxLog.
func logFactorial(n float64) float64 {
	if n < 1e-5 {
		return 0
	}

	// n! = Gamma(n+1), so work with x = n+1
	x := n + 1

	// Numerical stability note: Stirling's approximation is inaccurate for small x.
	// Use Gamma recurrence: Gamma(x) = Gamma(x+k) / (x*(x+1)*...*(x+k-1))
	// These branches handle small arguments to maintain precision.
	// Currently unreachable (n+m >= 65), but retained for standalone correctness.
	if x < 1 {
		return stirlingApproxLog(x+3) - math.Log(x*(x+1)*(x+2))
	}
	if x < 2 {
		return stirlingApproxLog(x+2) - math.Log(x*(x+1))
	}
	if x < 3 {
		return stirlingApproxLog(x+1) - math.Log(x)
	}

	return stirlingApproxLog(x)
}

// stirlingApproxLog computes Stirling's approximation with Bernoulli correction.
func stirlingApproxLog(x float64) float64 {
	result := x*math.Log(x) - x + math.Log(2*math.Pi/x)/2

	// Add Bernoulli correction series
	// Bernoulli numbers: B2 = 1/6, B4 = -1/30, B6 = 1/42, B8 = -1/30, B10 = 5/66
	const b2 = 1.0 / 6
	const b4 = -1.0 / 30
	const b6 = 1.0 / 42
	const b8 = -1.0 / 30
	const b10 = 5.0 / 66

	x2 := x * x
	x3 := x2 * x
	x5 := x3 * x2
	x7 := x5 * x2
	x9 := x7 * x2

	result += b2/(2*x) +
		b4/(12*x3) +
		b6/(30*x5) +
		b8/(56*x7) +
		b10/(90*x9)

	return result
}
