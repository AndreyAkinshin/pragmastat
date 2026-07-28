package pragmastat

import (
	"errors"
	"math"
)

const (
	maxExactSize        = 400
	maxAcceptableBinomN = 62
)

// Every product in this file that feeds an addition or a subtraction is wrapped
// in an explicit float64() conversion, as is every named power such a product is
// built from. The conversions are not redundant. The Go specification lets an
// implementation fuse a multiply and an add into a single rounding, and gc does
// it on arm64, ppc64le, s390x, riscv64 and loong64, never on amd64. None of the
// other six languages fuse, so a fused result here would differ in the last bit
// from every one of them. An explicit conversion is the only way the language
// offers to pin an intermediate rounding.
//
// One bit matters more here than it usually does. pairwiseMargin returns an
// integer that selects an order statistic, so a single-ULP difference in the
// Edgeworth CDF or in Stirling's log-gamma can move the returned index and shift
// the reported bounds by far more than the 1e-9 conformance tolerance. Measured
// under qemu-aarch64, the unpinned code returned a different integer on 8268 of
// 79797 (n, m) pairs at misrate == minAchievableMisrateTwoSample(n, m).
//
// A few of the wrapped products only scale by a power of two, where halving and
// doubling are exact and fusion is provably bit-exact. They are wrapped anyway:
// an exemption every future reader has to re-derive is worth less than a
// conversion.

// pairwiseMargin determines how many extreme pairwise differences to exclude
// when constructing bounds based on the distribution of dominance statistics.
// Uses exact calculation for small samples (n+m <= 400) and Edgeworth
// approximation for larger samples.
//
// Returns an error if n <= 0, m <= 0, or misrate is outside [0, 1] or NaN.
func pairwiseMargin(n, m int, misrate float64) (int, error) {
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
		for i := range u {
			sum += float64(pmf[i] * sigma[u-i])
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
	// gc strength-reduces /2.0 to *0.5 and then fuses the halving into the
	// subtraction below; the conversion keeps the two roundings apart.
	mu := float64(nm / 2.0)
	su := math.Sqrt(nm * float64(n+m+1) / 12.0)
	// -0.5 continuity correction: computing P(U ≥ u) for a right-tail discrete CDF
	z := (float64(u) - mu - 0.5) / su
	phi := portableExp(-z*z/2) / math.Sqrt(2*math.Pi)
	Phi := gaussCdf(z)

	// Pre-compute powers of n and m as float64 (avoids int64 overflow for large n, m)
	nf := float64(n)
	mf := float64(m)
	n2 := float64(nf * nf)
	n3 := float64(n2 * nf)
	n4 := float64(n2 * n2)
	m2 := float64(mf * mf)
	m3 := float64(m2 * mf)
	m4 := float64(m2 * m2)

	// Compute moments using float64 arithmetic
	mu2 := (nf * mf * (nf + mf + 1)) / 12.0
	mu4 := (nf * mf * (nf + mf + 1) *
		(float64(5*mf*nf*(mf+nf)) -
			float64(2*(m2+n2)) +
			float64(3*mf*nf) -
			float64(2*(nf+mf)))) / 240.0

	mu6 := (nf * mf * (nf + mf + 1) *
		(float64(35*m2*n2*(m2+n2)) +
			float64(70*m3*n3) -
			float64(42*mf*nf*(m3+n3)) -
			float64(14*m2*n2*(nf+mf)) +
			float64(16*(n4+m4)) -
			float64(52*nf*mf*(n2+m2)) -
			float64(43*n2*m2) +
			float64(32*(m3+n3)) +
			float64(14*mf*nf*(nf+mf)) +
			float64(8*(n2+m2)) +
			float64(16*nf*mf) -
			float64(8*(nf+mf)))) / 4032.0

	// Pre-compute powers of mu2 and related terms
	mu2_2 := mu2 * mu2
	mu2_3 := mu2_2 * mu2
	mu4_mu2_2 := mu4 / mu2_2

	// Factorial constants: 4! = 24, 6! = 720, 8! = 40320
	e3 := (mu4_mu2_2 - 3) / 24.0
	e5 := (mu6/mu2_3 - float64(15*mu4_mu2_2) + 30) / 720.0
	e7 := 35 * (mu4_mu2_2 - 3) * (mu4_mu2_2 - 3) / 40320.0

	// Pre-compute powers of z for Hermite polynomials
	z2 := float64(z * z)
	z3 := float64(z2 * z)
	z5 := float64(z3 * z2)
	z7 := float64(z5 * z2)

	f3 := -phi * (z3 - float64(3*z))
	f5 := -phi * (z5 - float64(10*z3) + float64(15*z))
	f7 := -phi * (z7 - float64(21*z5) + float64(105*z3) - float64(105*z))

	edgeworth := Phi + float64(e3*f3) + float64(e5*f5) + float64(e7*f7)
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
	for i := range k {
		result = result * int64(n-i) / int64(i+1)
	}
	return result
}

// binomialCoefficientFloat computes C(n, k) in binary64 by the multiplicative recurrence
// C(n, k) = prod_{i=1..k} (n-k+i)/i.
//
// It replaced an exp-of-Stirling formulation, for two reasons. The specification defines the
// admissible misrate as `misrate >= 2 / C(n+m, n)`, an exact integer quantity; Stirling
// approximated it to a worst relative error of 8.1e-9 over 4 <= n+m <= 400, while this
// recurrence is within 2.2e-15. And Stirling reached the answer through math.Log and math.Exp,
// which every language takes from a different libm: perturbing those by a single ulp moved the
// computed misrate floor on 75579 of 79797 sample-size pairs. This form calls nothing, so the
// same perturbation moves none of them.
//
// Normalizing k to the smaller half also makes the function symmetric by construction, which
// matters because the two call sites ask for C(n+m, n) and C(n+m, m). Those are the same
// number, and now they are also the same bits, so the comparison at the misrate floor cannot
// be decided by which of the two rounded higher.
func binomialCoefficientFloat(n, k float64) float64 {
	nn, kk := int(n), int(k)
	if kk > nn-kk {
		kk = nn - kk
	}
	acc := 1.0
	for i := 1; i <= kk; i++ {
		acc = acc * float64(nn-kk+i) / float64(i)
	}
	return acc
}
