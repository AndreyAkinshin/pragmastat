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
	Positivity AssumptionID = "positivity"
	Sparity    AssumptionID = "sparity"
)

// Subject identifies which sample caused the violation in two-sample functions.
type Subject string

const (
	SubjectX Subject = "x"
	SubjectY Subject = "y"
)

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

func NewValidityError(functionName string, subject Subject) *AssumptionError {
	return &AssumptionError{Violation: Violation{ID: Validity, Subject: subject}}
}

func NewPositivityError(functionName string, subject Subject) *AssumptionError {
	return &AssumptionError{Violation: Violation{ID: Positivity, Subject: subject}}
}

func NewSparityError(functionName string, subject Subject) *AssumptionError {
	return &AssumptionError{Violation: Violation{ID: Sparity, Subject: subject}}
}

func checkValidity[T Number](values []T, subject Subject, functionName string) error {
	if len(values) == 0 {
		return NewValidityError(functionName, subject)
	}
	for _, v := range values {
		fv := float64(v)
		if math.IsNaN(fv) || math.IsInf(fv, 0) {
			return NewValidityError(functionName, subject)
		}
	}
	return nil
}

func checkPositivity[T Number](values []T, subject Subject, functionName string) error {
	for _, v := range values {
		if float64(v) <= 0 {
			return NewPositivityError(functionName, subject)
		}
	}
	return nil
}

func checkSparity[T Number](values []T, subject Subject, functionName string) error {
	if len(values) < 2 {
		return NewSparityError(functionName, subject)
	}
	spread, err := fastSpread(values)
	if err != nil {
		return NewValidityError(functionName, subject)
	}
	if spread <= 0 {
		return NewSparityError(functionName, subject)
	}
	return nil
}

// Log log-transforms a slice. Returns error if any value is non-positive.
func Log[T Number](values []T, subject Subject, functionName string) ([]float64, error) {
	result := make([]float64, len(values))
	for i, v := range values {
		fv := float64(v)
		if fv <= 0 {
			return nil, NewPositivityError(functionName, subject)
		}
		result[i] = math.Log(fv)
	}
	return result, nil
}
