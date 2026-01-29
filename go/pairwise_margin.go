package pragmastat

import (
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
		return 0, errNMustBePositive
	}
	if m <= 0 {
		return 0, errMMustBePositive
	}
	if math.IsNaN(misrate) || misrate < 0 || misrate > 1 {
		return 0, errMisrateOutOfRange
	}

	// Use exact method for small to medium samples
	if n+m <= maxExactSize {
		return pairwiseMarginExact(n, m, misrate), nil
	}
	return pairwiseMarginApprox(n, m, misrate), nil
}

// pairwiseMarginExact uses the exact distribution based on Loeffler's recurrence.
func pairwiseMarginExact(n, m int, misrate float64) int {
	return pairwiseMarginExactRaw(n, m, misrate/2) * 2
}

// pairwiseMarginApprox uses Edgeworth approximation for large samples.
func pairwiseMarginApprox(n, m int, misrate float64) int {
	return pairwiseMarginApproxRaw(n, m, misrate/2) * 2
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
func pairwiseMarginApproxRaw(n, m int, misrate float64) int {
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
		panic("pairwise margin exceeds int range")
	}
	return int(result)
}

// edgeworthCdf computes the CDF using Edgeworth expansion.
func edgeworthCdf(n, m int, u int64) float64 {
	nm := float64(n) * float64(m)
	mu := nm / 2.0
	su := math.Sqrt(nm * float64(n+m+1) / 12.0)
	z := (float64(u) - mu - 0.5) / su
	phi := math.Exp(-z*z/2) / math.Sqrt(2*math.Pi)
	Phi := gaussCdf(z)

	// Pre-compute powers of n and m for efficiency (as integers for precision)
	n2 := n * n
	n3 := n2 * n
	n4 := n2 * n2
	m2 := m * m
	m3 := m2 * m
	m4 := m2 * m2

	// Use integer arithmetic first for precision, then convert to float
	mu2 := float64(n*m*(n+m+1)) / 12.0
	mu4 := float64(n*m*(n+m+1)) *
		float64(5*m*n*(m+n)-
			2*(m2+n2)+
			3*m*n-
			2*(n+m)) / 240.0

	mu6 := float64(n*m*(n+m+1)) *
		float64(35*m2*n2*(m2+n2)+
			70*m3*n3-
			42*m*n*(m3+n3)-
			14*m2*n2*(n+m)+
			16*(n4+m4)-
			52*n*m*(n2+m2)-
			43*n2*m2+
			32*(m3+n3)+
			14*m*n*(n+m)+
			8*(n2+m2)+
			16*n*m-
			8*(n+m)) / 4032.0

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

	// DONT TOUCH: Stirling's approximation is inaccurate for small x.
	// Use Gamma recurrence: Gamma(x) = Gamma(x+k) / (x*(x+1)*...*(x+k-1))
	// These branches appear unreachable in current usage (n+m >= 65), but
	// are retained for correctness if the function is used in other contexts.
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

// gaussCdf computes the standard normal CDF using ACM Algorithm 209.
// Calculates (1/sqrt(2*pi)) * integral from -infinity to x of e^(-u^2/2) du
func gaussCdf(x float64) float64 {
	var z float64
	if math.Abs(x) < 1e-9 {
		z = 0.0
	} else {
		y := math.Abs(x) / 2
		if y >= 3.0 {
			z = 1.0
		} else if y < 1.0 {
			w := y * y
			z = ((((((((0.000124818987*w-0.001075204047)*w+
				0.005198775019)*w-0.019198292004)*w+
				0.059054035642)*w-0.151968751364)*w+
				0.319152932694)*w-0.531923007300)*w +
				0.797884560593) * y * 2.0
		} else {
			y = y - 2.0
			z = (((((((((((((-0.000045255659*y+0.000152529290)*y-
				0.000019538132)*y-0.000676904986)*y+
				0.001390604284)*y-0.000794620820)*y-
				0.002034254874)*y+0.006549791214)*y-
				0.010557625006)*y+0.011630447319)*y-
				0.009279453341)*y+0.005353579108)*y-
				0.002141268741)*y+0.000535310849)*y +
				0.999936657524
		}
	}

	if x > 0.0 {
		return (z + 1.0) / 2
	}
	return (1.0 - z) / 2
}
