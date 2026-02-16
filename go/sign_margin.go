package pragmastat

import "math"

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

	rLow, logCdfLow, logPmfHigh := binomCdfSplit(n, target)
	logTarget := math.Log(target)

	var logNum float64
	if logTarget > logCdfLow {
		logNum = logSubExp(logTarget, logCdfLow)
	} else {
		logNum = math.Inf(-1)
	}

	var p float64
	if isFinite(logPmfHigh) && isFinite(logNum) {
		p = math.Exp(logNum - logPmfHigh)
	}

	// Clamp p to [0, 1]
	if p < 0 {
		p = 0
	}
	if p > 1 {
		p = 1
	}

	u := rng.UniformFloat64()
	r := rLow
	if u < p {
		r = rLow + 1
	}

	return r * 2, nil
}

// binomCdfSplit finds the split point in the Binomial(n, 0.5) CDF at target.
// Returns (rLow, logCdfLow, logPmfHigh) where rLow is the largest k such that
// CDF(k) <= target.
func binomCdfSplit(n int, target float64) (int, float64, float64) {
	logTarget := math.Log(target)
	logPmf := -float64(n) * math.Ln2
	logCdf := logPmf
	rLow := 0

	if logCdf > logTarget {
		return 0, logCdf, logPmf
	}

	for k := 1; k <= n; k++ {
		logPmfNext := logPmf + math.Log(float64(n-k+1)) - math.Log(float64(k))
		logCdfNext := logAddExp(logCdf, logPmfNext)
		if logCdfNext > logTarget {
			return rLow, logCdf, logPmfNext
		}
		rLow = k
		logPmf = logPmfNext
		logCdf = logCdfNext
	}

	return rLow, logCdf, math.Inf(-1)
}

// logAddExp computes log(exp(a) + exp(b)) in a numerically stable way.
func logAddExp(a, b float64) float64 {
	if math.IsInf(a, -1) {
		return b
	}
	if math.IsInf(b, -1) {
		return a
	}
	m := math.Max(a, b)
	return m + math.Log(math.Exp(a-m)+math.Exp(b-m))
}

// logSubExp computes log(exp(a) - exp(b)) in a numerically stable way.
// Assumes a >= b. Returns -Inf if exp(b-a) >= 1.
func logSubExp(a, b float64) float64 {
	if math.IsInf(b, -1) {
		return a
	}
	diff := math.Exp(b - a)
	if diff >= 1 {
		return math.Inf(-1)
	}
	return a + math.Log(1-diff)
}

// isFinite returns true if v is neither NaN nor infinite.
func isFinite(v float64) bool {
	return !math.IsNaN(v) && !math.IsInf(v, 0)
}
