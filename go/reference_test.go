package pragmastat

import (
	"encoding/json"
	"fmt"
	"io/ioutil"
	"path/filepath"
	"strings"
	"testing"
)

// TestData represents the structure of test JSON files
type TestData struct {
	Input  json.RawMessage `json:"input"`
	Output float64         `json:"output"`
}

// OneSampleInput represents input for one-sample tests
type OneSampleInput struct {
	X []float64 `json:"x"`
}

// TwoSampleInput represents input for two-sample tests
type TwoSampleInput struct {
	X []float64 `json:"x"`
	Y []float64 `json:"y"`
}

func TestReferenceData(t *testing.T) {
	// Map estimator names to functions
	oneSampleEstimators := map[string]func([]float64) (float64, error){
		"center":     Center[float64],
		"spread":     Spread[float64],
		"rel-spread": RelSpread[float64],
	}

	twoSampleEstimators := map[string]func([]float64, []float64) (float64, error){
		"shift":      Shift[float64],
		"ratio":      Ratio[float64],
		"avg-spread": AvgSpread[float64],
		"disparity":  Disparity[float64],
	}

	testDataPath := "../tests"

	// Test one-sample estimators
	for estimatorName, estimatorFunc := range oneSampleEstimators {
		dirPath := filepath.Join(testDataPath, estimatorName)
		files, err := ioutil.ReadDir(dirPath)
		if err != nil {
			t.Logf("Skipping %s tests: %v", estimatorName, err)
			continue
		}

		for _, file := range files {
			if !strings.HasSuffix(file.Name(), ".json") {
				continue
			}

			testName := strings.TrimSuffix(file.Name(), ".json")
			t.Run(fmt.Sprintf("%s/%s", estimatorName, testName), func(t *testing.T) {
				filePath := filepath.Join(dirPath, file.Name())
				data, err := ioutil.ReadFile(filePath)
				if err != nil {
					t.Fatalf("Failed to read test file: %v", err)
				}

				var testData TestData
				if err := json.Unmarshal(data, &testData); err != nil {
					t.Fatalf("Failed to parse test data: %v", err)
				}

				// Try to parse as OneSampleInput first
				var input OneSampleInput
				if err := json.Unmarshal(testData.Input, &input); err == nil && input.X != nil {
					result, err := estimatorFunc(input.X)
					if err != nil {
						t.Fatalf("%s(%v) error: %v", estimatorName, input.X, err)
					}
					if !floatEquals(result, testData.Output, 1e-9) {
						t.Errorf("%s(%v) = %v, want %v", estimatorName, input.X, result, testData.Output)
					}
					return
				}

				// Try to parse as direct array
				var directInput []float64
				if err := json.Unmarshal(testData.Input, &directInput); err == nil {
					result, err := estimatorFunc(directInput)
					if err != nil {
						t.Fatalf("%s(%v) error: %v", estimatorName, directInput, err)
					}
					if !floatEquals(result, testData.Output, 1e-9) {
						t.Errorf("%s(%v) = %v, want %v", estimatorName, directInput, result, testData.Output)
					}
					return
				}

				t.Fatalf("Failed to parse input data")
			})
		}
	}

	// Test two-sample estimators
	for estimatorName, estimatorFunc := range twoSampleEstimators {
		dirPath := filepath.Join(testDataPath, estimatorName)
		files, err := ioutil.ReadDir(dirPath)
		if err != nil {
			t.Logf("Skipping %s tests: %v", estimatorName, err)
			continue
		}

		for _, file := range files {
			if !strings.HasSuffix(file.Name(), ".json") {
				continue
			}

			testName := strings.TrimSuffix(file.Name(), ".json")
			t.Run(fmt.Sprintf("%s/%s", estimatorName, testName), func(t *testing.T) {
				filePath := filepath.Join(dirPath, file.Name())
				data, err := ioutil.ReadFile(filePath)
				if err != nil {
					t.Fatalf("Failed to read test file: %v", err)
				}

				var testData TestData
				if err := json.Unmarshal(data, &testData); err != nil {
					t.Fatalf("Failed to parse test data: %v", err)
				}

				var input TwoSampleInput
				if err := json.Unmarshal(testData.Input, &input); err != nil {
					t.Fatalf("Failed to parse input data: %v", err)
				}

				result, err := estimatorFunc(input.X, input.Y)
				if err != nil {
					t.Fatalf("%s(%v, %v) error: %v", estimatorName, input.X, input.Y, err)
				}
				if !floatEquals(result, testData.Output, 1e-9) {
					t.Errorf("%s(%v, %v) = %v, want %v", estimatorName, input.X, input.Y, result, testData.Output)
				}
			})
		}
	}
}
