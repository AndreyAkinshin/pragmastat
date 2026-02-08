package pragmastat

import "math"

// minAchievableMisrateOneSample computes the minimum achievable misrate
// for one-sample signed-rank based bounds.
// Returns 2^(1-n) which is the smallest possible misrate for a sample of size n.
func minAchievableMisrateOneSample(n int) (float64, error) {
	if n <= 0 {
		return 0, &AssumptionError{Violation: Violation{ID: Domain, Subject: SubjectX}}
	}
	return math.Pow(2, float64(1-n)), nil
}

// minAchievableMisrateTwoSample computes the minimum achievable misrate
// for two-sample Mann-Whitney based bounds.
func minAchievableMisrateTwoSample(n, m int) (float64, error) {
	if n <= 0 {
		return 0, NewDomainError(SubjectX)
	}
	if m <= 0 {
		return 0, NewDomainError(SubjectY)
	}
	var binom float64
	if n+m < maxAcceptableBinomN {
		binom = float64(binomialCoefficient(n+m, n))
	} else {
		binom = binomialCoefficientFloat(float64(n+m), float64(n))
	}
	return 2.0 / binom, nil
}
