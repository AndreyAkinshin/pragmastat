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
//
//nolint:unused // kept for parity with other languages; will be used by two-sample bounds
func minAchievableMisrateTwoSample(n, m int) (float64, error) {
	if n <= 0 {
		return 0, NewDomainError(SubjectX)
	}
	if m <= 0 {
		return 0, NewDomainError(SubjectY)
	}
	return 2.0 / float64(binomialCoefficient(n+m, n)), nil
}
