package pragmastat

import (
	"encoding/json"
	"os"
	"path/filepath"
	"strings"
	"testing"
)

// This file provides the shared, parameterized dual-path harness used by the
// reference-fixture tests. Every public estimator fixture runs through BOTH
// entry points:
//
//   1. the raw native-slice API (with assumeSorted=false), and
//   2. the Sample API (the thin unit-aware adapter).
//
// Both must match the fixture's expected output / expected error. Running each
// fixture through both paths is what catches Sample-adapter bugs (a past
// critical bug shipped because fixtures only ran through the raw path).

// expectedError is the shape of the "expected_error" object in fixtures.
type expectedError struct {
	ID      string `json:"id"`
	Subject string `json:"subject"`
}

// parseExpectedError decodes the fixture's expected_error object (id + optional subject).
func parseExpectedError(raw json.RawMessage) (expectedError, bool) {
	var ee expectedError
	if err := json.Unmarshal(raw, &ee); err != nil {
		return expectedError{}, false
	}
	return ee, true
}

// assertErrorMatches asserts that err is the AssumptionError the fixture
// expects. The id is always checked. The subject is checked only when
// checkSubject is true and the fixture provides one — this lets the Sample path
// skip the subject check for sample-construction validity errors that the
// fixture attributes to subject "y" (Sample construction reports a fixed
// subject, so a y-argument validity error surfaces as subject "x").
func assertErrorMatches(t *testing.T, raw json.RawMessage, err error, checkSubject bool) {
	t.Helper()
	if err == nil {
		t.Errorf("expected error but got none")
		return
	}
	ee, ok := parseExpectedError(raw)
	if !ok {
		return
	}
	ae, ok := err.(*AssumptionError)
	if !ok {
		t.Errorf("expected *AssumptionError but got %T: %v", err, err)
		return
	}
	if string(ae.Violation.ID) != ee.ID {
		t.Errorf("expected error id %q, got %q", ee.ID, ae.Violation.ID)
	}
	if checkSubject && ee.Subject != "" {
		if string(ae.Violation.Subject) != ee.Subject {
			t.Errorf("expected error subject %q, got %q", ee.Subject, ae.Violation.Subject)
		}
	}
}

// forEachFixture iterates the *.json fixtures under tests/<dir>, decoding each
// into a TestData plus the typed input, and invokes body for each.
func forEachFixture[I any](t *testing.T, dir string, body func(t *testing.T, td TestData, input I)) {
	t.Helper()
	dirPath := filepath.Join("../tests", dir)
	files, err := os.ReadDir(dirPath)
	if err != nil {
		t.Fatalf("Test data directory not found for %s: %v", dir, err)
	}

	jsonCount := 0
	for _, file := range files {
		if strings.HasSuffix(file.Name(), ".json") {
			jsonCount++
		}
	}
	if jsonCount == 0 {
		t.Errorf("No JSON test files found for %s in %s", dir, dirPath)
		return
	}

	for _, file := range files {
		if !strings.HasSuffix(file.Name(), ".json") {
			continue
		}
		testName := strings.TrimSuffix(file.Name(), ".json")
		t.Run(testName, func(t *testing.T) {
			data, err := os.ReadFile(filepath.Join(dirPath, file.Name()))
			if err != nil {
				t.Fatalf("Failed to read test file: %v", err)
			}
			var td TestData
			if err := json.Unmarshal(data, &td); err != nil {
				t.Fatalf("Failed to parse test data: %v", err)
			}
			var input I
			if err := json.Unmarshal(td.Input, &input); err != nil {
				t.Fatalf("Failed to parse input data: %v", err)
			}
			body(t, td, input)
		})
	}
}

// scalarEntry is one entry point for a scalar-valued (Measurement / float64)
// estimator. raw evaluates the raw slice API; sample evaluates the Sample API.
// sampleCreation reports whether a returned error originated from Sample
// construction (so the subject check is relaxed on a y-validity fixture).
type scalarEntry struct {
	name string
	// run returns the scalar value, an error, and whether the error (if any)
	// came from Sample construction.
	run func(t *testing.T) (value float64, err error, sampleCreation bool)
}

// boundsEntry is one entry point for a bounds-valued estimator.
type boundsEntry struct {
	name string
	run  func(t *testing.T) (b Bounds, err error, sampleCreation bool)
}

// runScalarDualPath runs both entry points for a scalar estimator against the
// fixture's expected output or expected error.
func runScalarDualPath(t *testing.T, td TestData, entries []scalarEntry) {
	t.Helper()
	if len(td.ExpectedError) > 0 {
		for _, e := range entries {
			e := e
			t.Run(e.name, func(t *testing.T) {
				value, err, sampleCreation := e.run(t)
				_ = value
				// For sample-construction validity errors attributed to
				// subject "y", the Sample path reports subject "x" (fixed
				// construction subject); skip the subject check there.
				checkSubject := true
				if sampleCreation {
					if ee, ok := parseExpectedError(td.ExpectedError); ok && ee.Subject == "y" {
						checkSubject = false
					}
				}
				assertErrorMatches(t, td.ExpectedError, err, checkSubject)
			})
		}
		return
	}

	var expected float64
	if err := json.Unmarshal(td.Output, &expected); err != nil {
		t.Fatalf("Failed to parse output data: %v", err)
	}
	for _, e := range entries {
		e := e
		t.Run(e.name, func(t *testing.T) {
			value, err, _ := e.run(t)
			if err != nil {
				t.Fatalf("%s entry returned error: %v", e.name, err)
			}
			if !floatEquals(value, expected, 1e-9) {
				t.Errorf("%s = %v, want %v", e.name, value, expected)
			}
		})
	}
}

// runBoundsDualPath runs both entry points for a bounds estimator against the
// fixture's expected output or expected error.
func runBoundsDualPath(t *testing.T, td TestData, entries []boundsEntry) {
	t.Helper()
	if len(td.ExpectedError) > 0 {
		for _, e := range entries {
			e := e
			t.Run(e.name, func(t *testing.T) {
				_, err, sampleCreation := e.run(t)
				checkSubject := true
				if sampleCreation {
					if ee, ok := parseExpectedError(td.ExpectedError); ok && ee.Subject == "y" {
						checkSubject = false
					}
				}
				assertErrorMatches(t, td.ExpectedError, err, checkSubject)
			})
		}
		return
	}

	var expected BoundsOutput
	if err := json.Unmarshal(td.Output, &expected); err != nil {
		t.Fatalf("Failed to parse output data: %v", err)
	}
	for _, e := range entries {
		e := e
		t.Run(e.name, func(t *testing.T) {
			b, err, _ := e.run(t)
			if err != nil {
				t.Fatalf("%s entry returned error: %v", e.name, err)
			}
			if !floatEquals(b.Lower, expected.Lower, 1e-9) || !floatEquals(b.Upper, expected.Upper, 1e-9) {
				t.Errorf("%s = [%v, %v], want [%v, %v]",
					e.name, b.Lower, b.Upper, expected.Lower, expected.Upper)
			}
		})
	}
}

// --- Sample-construction helpers for the Sample entry points ---
//
// These build x (subject x) and, for two-sample estimators, y (subject y). A
// construction error short-circuits: it is returned as the estimator error with
// sampleCreation=true so the caller can relax the y-subject check.

func sampleX(values []float64) (*Sample, error) {
	return NewSample(values)
}

func sampleY(values []float64) (*Sample, error) {
	// Sample carries no subject; the y argument is an ordinary Sample. A y-side
	// validity error therefore surfaces from construction with subject "x", which
	// is why runScalarDualPath/runBoundsDualPath relax the subject check for
	// sample-construction validity errors whose fixture expects "y".
	return NewSample(values)
}
