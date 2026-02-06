package pragmastat

import (
	"encoding/json"
	"math"
	"os"
	"path/filepath"
	"testing"
)

// AssumptionTestInputs represents input data for assumption tests.
type AssumptionTestInputs struct {
	X       []interface{} `json:"x,omitempty"`
	Y       []interface{} `json:"y,omitempty"`
	Misrate any           `json:"misrate,omitempty"`
	N       *int          `json:"n,omitempty"`
	Seed    *string       `json:"seed,omitempty"`
}

// ExpectedViolation represents the expected violation.
type ExpectedViolation struct {
	ID string `json:"id"`
}

// AssumptionTestCase represents a single test case.
type AssumptionTestCase struct {
	Name              string               `json:"name"`
	Function          string               `json:"function"`
	Inputs            AssumptionTestInputs `json:"inputs"`
	ExpectedViolation ExpectedViolation    `json:"expected_violation"`
}

// AssumptionTestSuite represents a test suite.
type AssumptionTestSuite struct {
	Suite       string               `json:"suite"`
	Description string               `json:"description"`
	Cases       []AssumptionTestCase `json:"cases"`
}

// SuiteEntry represents an entry in the manifest.
type SuiteEntry struct {
	Name        string `json:"name"`
	File        string `json:"file"`
	Description string `json:"description"`
}

// AssumptionManifest represents the manifest of test suites.
type AssumptionManifest struct {
	Name        string       `json:"name"`
	Description string       `json:"description"`
	Suites      []SuiteEntry `json:"suites"`
}

// parseValue converts a JSON value to float64, handling special values.
func parseValue(v interface{}) float64 {
	switch val := v.(type) {
	case float64:
		return val
	case int:
		return float64(val)
	case string:
		switch val {
		case "NaN":
			return math.NaN()
		case "Infinity":
			return math.Inf(1)
		case "-Infinity":
			return math.Inf(-1)
		default:
			panic("Unknown string value: " + val)
		}
	default:
		panic("Unexpected value type")
	}
}

// parseArray converts a slice of interface{} to []float64.
func parseArray(arr []interface{}) []float64 {
	if arr == nil {
		return []float64{}
	}
	result := make([]float64, len(arr))
	for i, v := range arr {
		result[i] = parseValue(v)
	}
	return result
}

// callAssumptionFunction dispatches to the appropriate estimator function.
func callAssumptionFunction(funcName string, inputs AssumptionTestInputs) error {
	x := parseArray(inputs.X)
	y := parseArray(inputs.Y)

	switch funcName {
	case "Center":
		_, err := Center(x)
		return err
	case "Ratio":
		_, err := Ratio(x, y)
		return err
	case "RelSpread":
		_, err := RelSpread(x)
		return err
	case "Spread":
		_, err := Spread(x)
		return err
	case "Shift":
		_, err := Shift(x, y)
		return err
	case "AvgSpread":
		_, err := AvgSpread(x, y)
		return err
	case "Disparity":
		_, err := Disparity(x, y)
		return err
	case "MedianBounds":
		_, err := MedianBounds(x, parseValue(inputs.Misrate))
		return err
	case "CenterBounds":
		_, err := CenterBounds(x, parseValue(inputs.Misrate))
		return err
	case "CenterBoundsApprox":
		_, err := CenterBoundsApproxWithSeed(x, parseValue(inputs.Misrate), inputs.Seed)
		return err
	case "SignedRankMargin":
		_, err := SignedRankMargin(*inputs.N, parseValue(inputs.Misrate))
		return err
	default:
		panic("Unknown function: " + funcName)
	}
}

func TestAssumptionViolations(t *testing.T) {
	assumptionsDir := filepath.Join("..", "tests", "assumptions")

	// Load manifest
	manifestPath := filepath.Join(assumptionsDir, "manifest.json")
	manifestData, err := os.ReadFile(manifestPath)
	if err != nil {
		t.Fatalf("Failed to read manifest: %v", err)
	}

	var manifest AssumptionManifest
	if err := json.Unmarshal(manifestData, &manifest); err != nil {
		t.Fatalf("Failed to parse manifest: %v", err)
	}

	// Run each suite
	for _, suiteEntry := range manifest.Suites {
		suitePath := filepath.Join(assumptionsDir, suiteEntry.File)
		suiteData, err := os.ReadFile(suitePath)
		if err != nil {
			t.Fatalf("Failed to read suite %s: %v", suiteEntry.Name, err)
		}

		var suite AssumptionTestSuite
		if err := json.Unmarshal(suiteData, &suite); err != nil {
			t.Fatalf("Failed to parse suite %s: %v", suiteEntry.Name, err)
		}

		for _, testCase := range suite.Cases {
			testName := suite.Suite + "/" + testCase.Name

			t.Run(testName, func(t *testing.T) {
				expectedID := AssumptionID(testCase.ExpectedViolation.ID)

				err := callAssumptionFunction(testCase.Function, testCase.Inputs)

				if err == nil {
					t.Errorf("Expected violation %s but got success", expectedID)
					return
				}

				assumptionErr, ok := err.(*AssumptionError)
				if !ok {
					t.Errorf("Expected AssumptionError but got %T: %v", err, err)
					return
				}

				if assumptionErr.Violation.ID != expectedID {
					t.Errorf("Expected id=%s, got %s",
						expectedID, assumptionErr.Violation.ID)
					return
				}
			})
		}
	}
}
