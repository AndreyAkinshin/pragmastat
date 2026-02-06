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
	X []interface{} `json:"x,omitempty"`
	Y []interface{} `json:"y,omitempty"`
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

// callFunction dispatches to the appropriate estimator function.
func callFunction(funcName string, x, y []float64) (float64, error) {
	switch funcName {
	case "Center":
		return Center(x)
	case "Ratio":
		return Ratio(x, y)
	case "RelSpread":
		return RelSpread(x)
	case "Spread":
		return Spread(x)
	case "Shift":
		return Shift(x, y)
	case "AvgSpread":
		return AvgSpread(x, y)
	case "Disparity":
		return Disparity(x, y)
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

	totalTests := 0
	passedTests := 0

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
			totalTests++

			t.Run(testName, func(t *testing.T) {
				x := parseArray(testCase.Inputs.X)
				y := parseArray(testCase.Inputs.Y)

				expectedID := AssumptionID(testCase.ExpectedViolation.ID)

				_, err := callFunction(testCase.Function, x, y)

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

				passedTests++
			})
		}
	}

	t.Logf("Assumption Tests: %d/%d passed", passedTests, totalTests)
}
