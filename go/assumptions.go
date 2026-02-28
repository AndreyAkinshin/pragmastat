// Package pragmastat provides robust statistical estimators for real-world data analysis.
package pragmastat

import (
	"fmt"
	"math"
)

// AssumptionID identifies an assumption in canonical priority order.
type AssumptionID string

const (
	Validity   AssumptionID = "validity"
	Domain     AssumptionID = "domain"
	Positivity AssumptionID = "positivity"
	Sparity    AssumptionID = "sparity"
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
	return &AssumptionError{Violation: Violation{ID: Validity, Subject: subject}}
}

func NewPositivityError(subject Subject) *AssumptionError {
	return &AssumptionError{Violation: Violation{ID: Positivity, Subject: subject}}
}

func NewSparityError(subject Subject) *AssumptionError {
	return &AssumptionError{Violation: Violation{ID: Sparity, Subject: subject}}
}

func NewDomainError(subject Subject) *AssumptionError {
	return &AssumptionError{Violation: Violation{ID: Domain, Subject: subject}}
}

// Log log-transforms a slice. Returns error if any value is non-positive.
func Log[T Number](values []T, subject Subject) ([]float64, error) {
	result := make([]float64, len(values))
	for i, v := range values {
		fv := float64(v)
		if fv <= 0 {
			return nil, NewPositivityError(subject)
		}
		result[i] = math.Log(fv)
	}
	return result, nil
}
