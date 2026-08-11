// Package pragmastat provides robust statistical estimators for real-world data analysis.
package pragmastat

import (
	"fmt"
	"math"
)

// AssumptionID identifies an assumption in canonical priority order.
type AssumptionID string

// AssumptionSparity and AssumptionDistinctness share a priority level: no estimator checks both.
const (
	AssumptionValidity     AssumptionID = "validity"
	AssumptionDomain       AssumptionID = "domain"
	AssumptionPositivity   AssumptionID = "positivity"
	AssumptionSparity      AssumptionID = "sparity"
	AssumptionDistinctness AssumptionID = "distinctness"
	AssumptionCumulance    AssumptionID = "cumulance"
)

// Subject identifies which sample caused the violation in two-sample functions.
type Subject string

const (
	SubjectX       Subject = "x"
	SubjectY       Subject = "y"
	SubjectMisrate Subject = "misrate"
)

// DefaultMisrate is the default misclassification rate for bounds estimators.
const DefaultMisrate = 1e-3

// Violation represents a specific assumption violation.
type Violation struct {
	ID      AssumptionID
	Subject Subject
}

func (v Violation) String() string {
	return fmt.Sprintf("%s(%s)", v.ID, v.Subject)
}

// AssumptionError is the error type for assumption violations.
type AssumptionError struct {
	Violation Violation
}

func (e *AssumptionError) Error() string {
	return e.Violation.String()
}

func NewValidityError(subject Subject) *AssumptionError {
	return &AssumptionError{Violation: Violation{ID: AssumptionValidity, Subject: subject}}
}

func NewPositivityError(subject Subject) *AssumptionError {
	return &AssumptionError{Violation: Violation{ID: AssumptionPositivity, Subject: subject}}
}

func NewSparityError(subject Subject) *AssumptionError {
	return &AssumptionError{Violation: Violation{ID: AssumptionSparity, Subject: subject}}
}

func NewDomainError(subject Subject) *AssumptionError {
	return &AssumptionError{Violation: Violation{ID: AssumptionDomain, Subject: subject}}
}

func NewDistinctnessError(subject Subject) *AssumptionError {
	return &AssumptionError{Violation: Violation{ID: AssumptionDistinctness, Subject: subject}}
}

func NewCumulanceError(subject Subject) *AssumptionError {
	return &AssumptionError{Violation: Violation{ID: AssumptionCumulance, Subject: subject}}
}

// checkQueryValidity returns a validity error if any element of a Distribution Estimation query
// vector is NaN or infinite.
//
// A query vector is a list of questions asked of a model, not a sample, so an empty vector
// satisfies the assumption vacuously.
func checkQueryValidity(y []float64) error {
	for _, v := range y {
		if math.IsNaN(v) || math.IsInf(v, 0) {
			return NewValidityError(SubjectY)
		}
	}
	return nil
}

// checkCumulance returns a cumulance error unless every queried cumulance lies in the closed
// unit interval.
func checkCumulance(y []float64) error {
	for _, v := range y {
		if !(v >= 0.0 && v <= 1.0) {
			return NewCumulanceError(SubjectY)
		}
	}
	return nil
}

// Log log-transforms a slice. Returns error if any value is non-positive.
func Log[T Number](values []T, subject Subject) ([]float64, error) {
	result := make([]float64, len(values))
	for i, v := range values {
		fv := float64(v)
		if fv <= 0 {
			return nil, NewPositivityError(subject)
		}
		result[i] = logFunction(fv)
	}
	return result, nil
}
