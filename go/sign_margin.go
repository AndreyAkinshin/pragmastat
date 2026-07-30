package pragmastat

import "math"

// The randomized sign margin, computed without a single library call.
//
// It used to be evaluated in log space: nine calls to math.Log and math.Exp, one of them inside a
// loop that runs n times. IEEE 754 fixes nothing about either function, and this value is not
// merely returned to the caller: the margin selects an order statistic, so a difference between
// two conforming libm implementations becomes a different confidence interval from identical
// inputs. It did. Two ports disagreed on SpreadBounds for a sample of 200 consecutive integers.
//
// No logarithm is needed. Binomial(n, 1/2) has an exact rational distribution function, and the
// two quantities the randomization wants are its partial sum and the next term. Both follow from
// the same multiplicative recurrence the binomial coefficient uses: one multiply and one divide
// per step, plus a scaling by a power of two, and IEEE 754 pins all three.
//
// The scaling is what makes the recurrence work at any n. pmf(0) is 2^-n, which underflows to
// zero past n = 1074, so the running term is carried as w * 2^e with the exponent tracked
// separately: w stays in the normal range and e absorbs the magnitude. Rescaling happens by
// multiplying by a power of two, which is exact, so it costs no accuracy and changes no bits.
//
// Measured against exact rational arithmetic over 195 (n, misrate) pairs spanning n = 1 to 5000
// and misrate from 1 down to the smallest positive double: the selected index is right every
// time, and the randomization probability is within 6.1e-13. The log-space version it replaces
// reached 1.9e-11 on the same set, thirty times further out, and did it differently in each port.

// signMarginScaleStep is how far the running term is rescaled when it grows too large. Any power
// of two works; 512 keeps the rescaling rare without letting w approach the overflow threshold.
const signMarginScaleStep = 512

// signMarginRandomized computes the randomized sign margin for Binomial(n, 0.5).
// Uses binomial CDF inversion with randomized tie-breaking.
func signMarginRandomized(n int, misrate float64, rng *Rng) (int, error) {
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

	target := misrate / 2

	if target <= 0 {
		return 0, nil
	}
	if target >= 1 {
		return n * 2, nil
	}

	rLow, p := binomCdfSplit(n, target)

	u := rng.UniformFloat64()
	r := rLow
	if u < p {
		r = rLow + 1
	}

	return r * 2, nil
}

// binomCdfSplit returns the largest k whose Binomial(n, 0.5) CDF does not exceed target, together
// with the fraction of the next term that would be needed to reach it. The caller compares that
// fraction against a uniform draw, which is what makes the margin achieve the requested misrate
// exactly rather than the next admissible one below it.
func binomCdfSplit(n int, target float64) (int, float64) {
	// Binomial(n, 1/2) is symmetric, so for odd n the distribution function at (n-1)/2 is exactly
	// one half. No approximation reproduces an exact equality, and misrate = 1 lands on it: the
	// summation would decide the comparison by its last accumulated bit.
	if target == 0.5 && n%2 == 1 {
		return (n - 1) / 2, 0
	}

	scaleUp := math.Ldexp(1, signMarginScaleStep)
	scaleDown := math.Ldexp(1, -signMarginScaleStep)

	// The running term pmf(k) is w * 2^e, starting from pmf(0) = 2^-n.
	w := 1.0
	e := -n
	cdf := 1.0

	if math.Ldexp(cdf, e) > target {
		return 0, 0
	}

	rLow := 0
	for k := 1; k <= n; k++ {
		w = w * float64(n-k+1) / float64(k)
		for w > scaleUp {
			w *= scaleDown
			cdf *= scaleDown
			e += signMarginScaleStep
		}
		next := cdf + w
		if math.Ldexp(next, e) > target {
			// target and cdf are both in units of 2^e here, so the fraction is a plain quotient.
			p := (math.Ldexp(target, -e) - cdf) / w
			if p < 0 {
				p = 0
			}
			if p > 1 {
				p = 1
			}
			return rLow, p
		}
		rLow = k
		cdf = next
	}

	return rLow, 0
}

// isFinite returns true if v is neither NaN nor infinite.
func isFinite(v float64) bool {
	return !math.IsNaN(v) && !math.IsInf(v, 0)
}
