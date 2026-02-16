package pragmastat

import (
	"encoding/json"
	"fmt"
	"os"
	"path/filepath"
	"strings"
	"testing"
)

// TestData represents the structure of test JSON files
type TestData struct {
	Input         json.RawMessage `json:"input"`
	Output        json.RawMessage `json:"output,omitempty"`
	ExpectedError json.RawMessage `json:"expected_error,omitempty"`
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

// PairwiseMarginInput represents input for pairwise-margin tests
type PairwiseMarginInput struct {
	N       int     `json:"n"`
	M       int     `json:"m"`
	Misrate float64 `json:"misrate"`
}

// ShiftBoundsInput represents input for shift-bounds tests
type ShiftBoundsInput struct {
	X       []float64 `json:"x"`
	Y       []float64 `json:"y"`
	Misrate float64   `json:"misrate"`
}

// RatioBoundsInput represents input for ratio-bounds tests
type RatioBoundsInput struct {
	X       []float64 `json:"x"`
	Y       []float64 `json:"y"`
	Misrate float64   `json:"misrate"`
}

// BoundsOutput represents output for bounds tests
type BoundsOutput struct {
	Lower float64 `json:"lower"`
	Upper float64 `json:"upper"`
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
		"avg-spread": avgSpread[float64],
		"disparity":  Disparity[float64],
	}

	// Special test for pairwise-margin
	t.Run("pairwise-margin", func(t *testing.T) {
		dirPath := filepath.Join("../tests", "pairwise-margin")
		files, err := os.ReadDir(dirPath)
		if err != nil {
			t.Skipf("Skipping pairwise-margin tests: %v", err)
		}

		for _, file := range files {
			if !strings.HasSuffix(file.Name(), ".json") {
				continue
			}

			testName := strings.TrimSuffix(file.Name(), ".json")
			t.Run(testName, func(t *testing.T) {
				filePath := filepath.Join(dirPath, file.Name())
				data, err := os.ReadFile(filePath)
				if err != nil {
					t.Fatalf("Failed to read test file: %v", err)
				}

				var testData TestData
				if err := json.Unmarshal(data, &testData); err != nil {
					t.Fatalf("Failed to parse test data: %v", err)
				}

				var input PairwiseMarginInput
				if err := json.Unmarshal(testData.Input, &input); err != nil {
					t.Fatalf("Failed to parse input data: %v", err)
				}

				// Handle error test cases
				if len(testData.ExpectedError) > 0 {
					_, err := pairwiseMargin(input.N, input.M, input.Misrate)
					if err == nil {
						t.Errorf("Expected error for pairwiseMargin(%d, %d, %v), but got none", input.N, input.M, input.Misrate)
						return
					}
					// Verify error details match expected
					var expectedError map[string]string
					if jsonErr := json.Unmarshal(testData.ExpectedError, &expectedError); jsonErr == nil {
						if ae, ok := err.(*AssumptionError); ok {
							if string(ae.Violation.ID) != expectedError["id"] {
								t.Errorf("Expected error id %q, got %q", expectedError["id"], ae.Violation.ID)
							}
						}
					}
					return
				}

				var expected int
				if err := json.Unmarshal(testData.Output, &expected); err != nil {
					t.Fatalf("Failed to parse output data: %v", err)
				}

				actual, err := pairwiseMargin(input.N, input.M, input.Misrate)
				if err != nil {
					t.Fatalf("PairwiseMargin returned unexpected error: %v", err)
				}
				if actual != expected {
					t.Errorf("pairwiseMargin(%d, %d, %v) = %d, want %d",
						input.N, input.M, input.Misrate, actual, expected)
				}
			})
		}
	})

	// Special test for shift-bounds
	t.Run("shift-bounds", func(t *testing.T) {
		dirPath := filepath.Join("../tests", "shift-bounds")
		files, err := os.ReadDir(dirPath)
		if err != nil {
			t.Skipf("Skipping shift-bounds tests: %v", err)
		}

		for _, file := range files {
			if !strings.HasSuffix(file.Name(), ".json") {
				continue
			}

			testName := strings.TrimSuffix(file.Name(), ".json")
			t.Run(testName, func(t *testing.T) {
				filePath := filepath.Join(dirPath, file.Name())
				data, err := os.ReadFile(filePath)
				if err != nil {
					t.Fatalf("Failed to read test file: %v", err)
				}

				var testData TestData
				if err := json.Unmarshal(data, &testData); err != nil {
					t.Fatalf("Failed to parse test data: %v", err)
				}

				var input ShiftBoundsInput
				if err := json.Unmarshal(testData.Input, &input); err != nil {
					t.Fatalf("Failed to parse input data: %v", err)
				}

				// Handle error test cases
				if len(testData.ExpectedError) > 0 {
					_, err := ShiftBounds[float64](input.X, input.Y, input.Misrate)
					if err == nil {
						t.Errorf("Expected error for ShiftBounds(%v, %v, %v), but got none", input.X, input.Y, input.Misrate)
						return
					}
					var expectedError map[string]string
					if jsonErr := json.Unmarshal(testData.ExpectedError, &expectedError); jsonErr == nil {
						if ae, ok := err.(*AssumptionError); ok {
							if string(ae.Violation.ID) != expectedError["id"] {
								t.Errorf("Expected error id %q, got %q", expectedError["id"], ae.Violation.ID)
							}
						}
					}
					return
				}

				var expected BoundsOutput
				if err := json.Unmarshal(testData.Output, &expected); err != nil {
					t.Fatalf("Failed to parse output data: %v", err)
				}

				actual, err := ShiftBounds[float64](input.X, input.Y, input.Misrate)
				if err != nil {
					t.Fatalf("ShiftBounds(%v, %v, %v) error: %v",
						input.X, input.Y, input.Misrate, err)
				}
				if !floatEquals(actual.Lower, expected.Lower, 1e-9) ||
					!floatEquals(actual.Upper, expected.Upper, 1e-9) {
					t.Errorf("ShiftBounds(%v, %v, %v) = [%v, %v], want [%v, %v]",
						input.X, input.Y, input.Misrate,
						actual.Lower, actual.Upper,
						expected.Lower, expected.Upper)
				}
			})
		}
	})

	// Special test for ratio-bounds
	t.Run("ratio-bounds", func(t *testing.T) {
		dirPath := filepath.Join("../tests", "ratio-bounds")
		files, err := os.ReadDir(dirPath)
		if err != nil {
			t.Skipf("Skipping ratio-bounds tests: %v", err)
		}

		for _, file := range files {
			if !strings.HasSuffix(file.Name(), ".json") {
				continue
			}

			testName := strings.TrimSuffix(file.Name(), ".json")
			t.Run(testName, func(t *testing.T) {
				filePath := filepath.Join(dirPath, file.Name())
				data, err := os.ReadFile(filePath)
				if err != nil {
					t.Fatalf("Failed to read test file: %v", err)
				}

				var testData TestData
				if err := json.Unmarshal(data, &testData); err != nil {
					t.Fatalf("Failed to parse test data: %v", err)
				}

				var input RatioBoundsInput
				if err := json.Unmarshal(testData.Input, &input); err != nil {
					t.Fatalf("Failed to parse input data: %v", err)
				}

				// Handle error test cases
				if len(testData.ExpectedError) > 0 {
					_, err := RatioBounds(input.X, input.Y, input.Misrate)
					if err == nil {
						t.Errorf("Expected error for RatioBounds(%v, %v, %v), but got none", input.X, input.Y, input.Misrate)
						return
					}
					var expectedError map[string]string
					if jsonErr := json.Unmarshal(testData.ExpectedError, &expectedError); jsonErr == nil {
						if ae, ok := err.(*AssumptionError); ok {
							if string(ae.Violation.ID) != expectedError["id"] {
								t.Errorf("Expected error id %q, got %q", expectedError["id"], ae.Violation.ID)
							}
						}
					}
					return
				}

				var expected BoundsOutput
				if err := json.Unmarshal(testData.Output, &expected); err != nil {
					t.Fatalf("Failed to parse output data: %v", err)
				}

				actual, err := RatioBounds(input.X, input.Y, input.Misrate)
				if err != nil {
					t.Fatalf("RatioBounds(%v, %v, %v) error: %v",
						input.X, input.Y, input.Misrate, err)
				}
				if !floatEquals(actual.Lower, expected.Lower, 1e-9) ||
					!floatEquals(actual.Upper, expected.Upper, 1e-9) {
					t.Errorf("RatioBounds(%v, %v, %v) = [%v, %v], want [%v, %v]",
						input.X, input.Y, input.Misrate,
						actual.Lower, actual.Upper,
						expected.Lower, expected.Upper)
				}
			})
		}
	})

	testDataPath := "../tests"

	// Test one-sample estimators
	for estimatorName, estimatorFunc := range oneSampleEstimators {
		dirPath := filepath.Join(testDataPath, estimatorName)
		files, err := os.ReadDir(dirPath)
		if err != nil {
			t.Skipf("Test directory not found for %s: %v", estimatorName, err)
			continue
		}

		jsonFileCount := 0
		for _, file := range files {
			if strings.HasSuffix(file.Name(), ".json") {
				jsonFileCount++
			}
		}
		if jsonFileCount == 0 {
			t.Errorf("No JSON test files found for %s in %s", estimatorName, dirPath)
			continue
		}

		for _, file := range files {
			if !strings.HasSuffix(file.Name(), ".json") {
				continue
			}

			testName := strings.TrimSuffix(file.Name(), ".json")
			t.Run(fmt.Sprintf("%s/%s", estimatorName, testName), func(t *testing.T) {
				filePath := filepath.Join(dirPath, file.Name())
				data, err := os.ReadFile(filePath)
				if err != nil {
					t.Fatalf("Failed to read test file: %v", err)
				}

				var testData TestData
				if err := json.Unmarshal(data, &testData); err != nil {
					t.Fatalf("Failed to parse test data: %v", err)
				}

				var expected float64
				if err := json.Unmarshal(testData.Output, &expected); err != nil {
					t.Fatalf("Failed to parse output data: %v", err)
				}

				// Try to parse as OneSampleInput first
				var input OneSampleInput
				if err := json.Unmarshal(testData.Input, &input); err == nil && input.X != nil {
					result, err := estimatorFunc(input.X)
					if err != nil {
						// Skip cases that violate assumptions - tested separately
						if _, ok := err.(*AssumptionError); ok {
							t.Skipf("skipping due to assumption violation: %v", err)
						}
						t.Fatalf("%s(%v) error: %v", estimatorName, input.X, err)
					}
					if !floatEquals(result, expected, 1e-9) {
						t.Errorf("%s(%v) = %v, want %v", estimatorName, input.X, result, expected)
					}
					return
				}

				// Try to parse as direct array
				var directInput []float64
				if err := json.Unmarshal(testData.Input, &directInput); err == nil {
					result, err := estimatorFunc(directInput)
					if err != nil {
						// Skip cases that violate assumptions - tested separately
						if _, ok := err.(*AssumptionError); ok {
							t.Skipf("skipping due to assumption violation: %v", err)
						}
						t.Fatalf("%s(%v) error: %v", estimatorName, directInput, err)
					}
					if !floatEquals(result, expected, 1e-9) {
						t.Errorf("%s(%v) = %v, want %v", estimatorName, directInput, result, expected)
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
		files, err := os.ReadDir(dirPath)
		if err != nil {
			t.Skipf("Test directory not found for %s: %v", estimatorName, err)
			continue
		}

		jsonFileCount := 0
		for _, file := range files {
			if strings.HasSuffix(file.Name(), ".json") {
				jsonFileCount++
			}
		}
		if jsonFileCount == 0 {
			t.Errorf("No JSON test files found for %s in %s", estimatorName, dirPath)
			continue
		}

		for _, file := range files {
			if !strings.HasSuffix(file.Name(), ".json") {
				continue
			}

			testName := strings.TrimSuffix(file.Name(), ".json")
			t.Run(fmt.Sprintf("%s/%s", estimatorName, testName), func(t *testing.T) {
				filePath := filepath.Join(dirPath, file.Name())
				data, err := os.ReadFile(filePath)
				if err != nil {
					t.Fatalf("Failed to read test file: %v", err)
				}

				var testData TestData
				if err := json.Unmarshal(data, &testData); err != nil {
					t.Fatalf("Failed to parse test data: %v", err)
				}

				var expected float64
				if err := json.Unmarshal(testData.Output, &expected); err != nil {
					t.Fatalf("Failed to parse output data: %v", err)
				}

				var input TwoSampleInput
				if err := json.Unmarshal(testData.Input, &input); err != nil {
					t.Fatalf("Failed to parse input data: %v", err)
				}

				result, err := estimatorFunc(input.X, input.Y)
				if err != nil {
					// Skip cases that violate assumptions - tested separately
					if _, ok := err.(*AssumptionError); ok {
						t.Skipf("skipping due to assumption violation: %v", err)
					}
					t.Fatalf("%s(%v, %v) error: %v", estimatorName, input.X, input.Y, err)
				}
				if !floatEquals(result, expected, 1e-9) {
					t.Errorf("%s(%v, %v) = %v, want %v", estimatorName, input.X, input.Y, result, expected)
				}
			})
		}
	}
}

// Rng reference tests

// UniformInput represents input for uniform tests
type UniformInput struct {
	Seed  int64 `json:"seed"`
	Count int   `json:"count"`
}

// UniformIntInput represents input for uniform int tests
type UniformIntInput struct {
	Seed  int64 `json:"seed"`
	Min   int64 `json:"min"`
	Max   int64 `json:"max"`
	Count int   `json:"count"`
}

// StringSeedInput represents input for string seed tests
type StringSeedInput struct {
	Seed  string `json:"seed"`
	Count int    `json:"count"`
}

// ShuffleInput represents input for shuffle tests
type ShuffleInput struct {
	Seed int64     `json:"seed"`
	X    []float64 `json:"x"`
}

// SampleInput represents input for sample tests
type SampleInput struct {
	Seed int64     `json:"seed"`
	X    []float64 `json:"x"`
	K    int       `json:"k"`
}

// UniformRangeInput represents input for uniform range tests
type UniformRangeInput struct {
	Seed  int64   `json:"seed"`
	Min   float64 `json:"min"`
	Max   float64 `json:"max"`
	Count int     `json:"count"`
}

// UniformF32Input represents input for uniform f32 tests
type UniformF32Input struct {
	Seed  int64 `json:"seed"`
	Count int   `json:"count"`
}

// UniformI32Input represents input for uniform i32 tests
type UniformI32Input struct {
	Seed  int64 `json:"seed"`
	Min   int32 `json:"min"`
	Max   int32 `json:"max"`
	Count int   `json:"count"`
}

// UniformBoolInput represents input for uniform bool tests
type UniformBoolInput struct {
	Seed  int64 `json:"seed"`
	Count int   `json:"count"`
}

// Distribution reference tests

type UniformDistInput struct {
	Seed  int64   `json:"seed"`
	Min   float64 `json:"min"`
	Max   float64 `json:"max"`
	Count int     `json:"count"`
}

type UniformDistTestCase struct {
	Input  UniformDistInput `json:"input"`
	Output []float64        `json:"output"`
}

type AdditiveDistInput struct {
	Seed   int64   `json:"seed"`
	Mean   float64 `json:"mean"`
	StdDev float64 `json:"stdDev"`
	Count  int     `json:"count"`
}

type AdditiveDistTestCase struct {
	Input  AdditiveDistInput `json:"input"`
	Output []float64         `json:"output"`
}

type MultiplicDistInput struct {
	Seed      int64   `json:"seed"`
	LogMean   float64 `json:"logMean"`
	LogStdDev float64 `json:"logStdDev"`
	Count     int     `json:"count"`
}

type MultiplicDistTestCase struct {
	Input  MultiplicDistInput `json:"input"`
	Output []float64          `json:"output"`
}

type ExpDistInput struct {
	Seed  int64   `json:"seed"`
	Rate  float64 `json:"rate"`
	Count int     `json:"count"`
}

type ExpDistTestCase struct {
	Input  ExpDistInput `json:"input"`
	Output []float64    `json:"output"`
}

type PowerDistInput struct {
	Seed  int64   `json:"seed"`
	Min   float64 `json:"min"`
	Shape float64 `json:"shape"`
	Count int     `json:"count"`
}

type PowerDistTestCase struct {
	Input  PowerDistInput `json:"input"`
	Output []float64      `json:"output"`
}

func TestRngUniformReference(t *testing.T) {
	dirPath := "../tests/rng"
	files, err := os.ReadDir(dirPath)
	if err != nil {
		t.Fatalf("Failed to read directory: %v", err)
	}

	for _, file := range files {
		if !strings.HasPrefix(file.Name(), "uniform-seed-") || !strings.HasSuffix(file.Name(), ".json") {
			continue
		}

		testName := strings.TrimSuffix(file.Name(), ".json")
		t.Run(testName, func(t *testing.T) {
			filePath := filepath.Join(dirPath, file.Name())
			data, err := os.ReadFile(filePath)
			if err != nil {
				t.Fatalf("Failed to read test file: %v", err)
			}

			var testData struct {
				Input  UniformInput `json:"input"`
				Output []float64    `json:"output"`
			}
			if err := json.Unmarshal(data, &testData); err != nil {
				t.Fatalf("Failed to parse test data: %v", err)
			}

			rng := NewRngFromSeed(testData.Input.Seed)
			if len(testData.Output) != testData.Input.Count {
				t.Fatalf("Output length %d != count %d", len(testData.Output), testData.Input.Count)
			}
			for i := 0; i < testData.Input.Count; i++ {
				actual := rng.UniformFloat64()
				expected := testData.Output[i]
				if !floatEquals(actual, expected, 1e-15) {
					t.Errorf("UniformFloat64() at index %d = %v, want %v", i, actual, expected)
				}
			}
		})
	}
}

func TestRngUniformIntReference(t *testing.T) {
	dirPath := "../tests/rng"
	files, err := os.ReadDir(dirPath)
	if err != nil {
		t.Fatalf("Failed to read directory: %v", err)
	}

	for _, file := range files {
		if !strings.HasPrefix(file.Name(), "uniform-int-") || !strings.HasSuffix(file.Name(), ".json") {
			continue
		}

		testName := strings.TrimSuffix(file.Name(), ".json")
		t.Run(testName, func(t *testing.T) {
			filePath := filepath.Join(dirPath, file.Name())
			data, err := os.ReadFile(filePath)
			if err != nil {
				t.Fatalf("Failed to read test file: %v", err)
			}

			var testData struct {
				Input  UniformIntInput `json:"input"`
				Output []int64         `json:"output"`
			}
			if err := json.Unmarshal(data, &testData); err != nil {
				t.Fatalf("Failed to parse test data: %v", err)
			}

			rng := NewRngFromSeed(testData.Input.Seed)
			if len(testData.Output) != testData.Input.Count {
				t.Fatalf("Output length %d != count %d", len(testData.Output), testData.Input.Count)
			}
			for i := 0; i < testData.Input.Count; i++ {
				actual := rng.UniformInt64(testData.Input.Min, testData.Input.Max)
				expected := testData.Output[i]
				if actual != expected {
					t.Errorf("UniformInt64(%d, %d) at index %d = %d, want %d",
						testData.Input.Min, testData.Input.Max, i, actual, expected)
				}
			}
		})
	}
}

func TestRngStringSeedReference(t *testing.T) {
	dirPath := "../tests/rng"
	files, err := os.ReadDir(dirPath)
	if err != nil {
		t.Fatalf("Failed to read directory: %v", err)
	}

	for _, file := range files {
		if !strings.HasPrefix(file.Name(), "uniform-string-") || !strings.HasSuffix(file.Name(), ".json") {
			continue
		}

		testName := strings.TrimSuffix(file.Name(), ".json")
		t.Run(testName, func(t *testing.T) {
			filePath := filepath.Join(dirPath, file.Name())
			data, err := os.ReadFile(filePath)
			if err != nil {
				t.Fatalf("Failed to read test file: %v", err)
			}

			var testData struct {
				Input  StringSeedInput `json:"input"`
				Output []float64       `json:"output"`
			}
			if err := json.Unmarshal(data, &testData); err != nil {
				t.Fatalf("Failed to parse test data: %v", err)
			}

			rng := NewRngFromString(testData.Input.Seed)
			if len(testData.Output) != testData.Input.Count {
				t.Fatalf("Output length %d != count %d", len(testData.Output), testData.Input.Count)
			}
			for i := 0; i < testData.Input.Count; i++ {
				actual := rng.UniformFloat64()
				expected := testData.Output[i]
				if !floatEquals(actual, expected, 1e-15) {
					t.Errorf("UniformFloat64() at index %d = %v, want %v", i, actual, expected)
				}
			}
		})
	}
}

func TestRngUniformRangeReference(t *testing.T) {
	dirPath := "../tests/rng"
	files, err := os.ReadDir(dirPath)
	if err != nil {
		t.Fatalf("Failed to read directory: %v", err)
	}

	for _, file := range files {
		if !strings.HasPrefix(file.Name(), "uniform-range-") || !strings.HasSuffix(file.Name(), ".json") {
			continue
		}

		testName := strings.TrimSuffix(file.Name(), ".json")
		t.Run(testName, func(t *testing.T) {
			filePath := filepath.Join(dirPath, file.Name())
			data, err := os.ReadFile(filePath)
			if err != nil {
				t.Fatalf("Failed to read test file: %v", err)
			}

			var testData struct {
				Input  UniformRangeInput `json:"input"`
				Output []float64         `json:"output"`
			}
			if err := json.Unmarshal(data, &testData); err != nil {
				t.Fatalf("Failed to parse test data: %v", err)
			}

			rng := NewRngFromSeed(testData.Input.Seed)
			if len(testData.Output) != testData.Input.Count {
				t.Fatalf("Output length %d != count %d", len(testData.Output), testData.Input.Count)
			}
			for i := 0; i < testData.Input.Count; i++ {
				actual := rng.UniformFloat64Range(testData.Input.Min, testData.Input.Max)
				expected := testData.Output[i]
				if !floatEquals(actual, expected, 1e-12) {
					t.Errorf("UniformFloat64Range(%v, %v) at index %d = %v, want %v",
						testData.Input.Min, testData.Input.Max, i, actual, expected)
				}
			}
		})
	}
}

func TestRngUniformFloat32Reference(t *testing.T) {
	dirPath := "../tests/rng"
	files, err := os.ReadDir(dirPath)
	if err != nil {
		t.Fatalf("Failed to read directory: %v", err)
	}

	for _, file := range files {
		if !strings.HasPrefix(file.Name(), "uniform-f32-") || !strings.HasSuffix(file.Name(), ".json") {
			continue
		}

		testName := strings.TrimSuffix(file.Name(), ".json")
		t.Run(testName, func(t *testing.T) {
			filePath := filepath.Join(dirPath, file.Name())
			data, err := os.ReadFile(filePath)
			if err != nil {
				t.Fatalf("Failed to read test file: %v", err)
			}

			var testData struct {
				Input  UniformF32Input `json:"input"`
				Output []float32       `json:"output"`
			}
			if err := json.Unmarshal(data, &testData); err != nil {
				t.Fatalf("Failed to parse test data: %v", err)
			}

			rng := NewRngFromSeed(testData.Input.Seed)
			if len(testData.Output) != testData.Input.Count {
				t.Fatalf("Output length %d != count %d", len(testData.Output), testData.Input.Count)
			}
			for i := 0; i < testData.Input.Count; i++ {
				actual := rng.UniformFloat32()
				expected := testData.Output[i]
				if actual != expected {
					t.Errorf("UniformFloat32() at index %d = %v, want %v", i, actual, expected)
				}
			}
		})
	}
}

func TestRngUniformInt32Reference(t *testing.T) {
	dirPath := "../tests/rng"
	files, err := os.ReadDir(dirPath)
	if err != nil {
		t.Fatalf("Failed to read directory: %v", err)
	}

	for _, file := range files {
		if !strings.HasPrefix(file.Name(), "uniform-i32-") || !strings.HasSuffix(file.Name(), ".json") {
			continue
		}

		testName := strings.TrimSuffix(file.Name(), ".json")
		t.Run(testName, func(t *testing.T) {
			filePath := filepath.Join(dirPath, file.Name())
			data, err := os.ReadFile(filePath)
			if err != nil {
				t.Fatalf("Failed to read test file: %v", err)
			}

			var testData struct {
				Input  UniformI32Input `json:"input"`
				Output []int32         `json:"output"`
			}
			if err := json.Unmarshal(data, &testData); err != nil {
				t.Fatalf("Failed to parse test data: %v", err)
			}

			rng := NewRngFromSeed(testData.Input.Seed)
			if len(testData.Output) != testData.Input.Count {
				t.Fatalf("Output length %d != count %d", len(testData.Output), testData.Input.Count)
			}
			for i := 0; i < testData.Input.Count; i++ {
				actual := rng.UniformInt32(testData.Input.Min, testData.Input.Max)
				expected := testData.Output[i]
				if actual != expected {
					t.Errorf("UniformInt32(%d, %d) at index %d = %d, want %d",
						testData.Input.Min, testData.Input.Max, i, actual, expected)
				}
			}
		})
	}
}

func TestRngUniformBoolReference(t *testing.T) {
	dirPath := "../tests/rng"
	files, err := os.ReadDir(dirPath)
	if err != nil {
		t.Fatalf("Failed to read directory: %v", err)
	}

	for _, file := range files {
		if !strings.HasPrefix(file.Name(), "uniform-bool-seed-") || !strings.HasSuffix(file.Name(), ".json") {
			continue
		}

		testName := strings.TrimSuffix(file.Name(), ".json")
		t.Run(testName, func(t *testing.T) {
			filePath := filepath.Join(dirPath, file.Name())
			data, err := os.ReadFile(filePath)
			if err != nil {
				t.Fatalf("Failed to read test file: %v", err)
			}

			var testData struct {
				Input  UniformBoolInput `json:"input"`
				Output []bool           `json:"output"`
			}
			if err := json.Unmarshal(data, &testData); err != nil {
				t.Fatalf("Failed to parse test data: %v", err)
			}

			rng := NewRngFromSeed(testData.Input.Seed)
			if len(testData.Output) != testData.Input.Count {
				t.Fatalf("Output length %d != count %d", len(testData.Output), testData.Input.Count)
			}
			for i := 0; i < testData.Input.Count; i++ {
				actual := rng.UniformBool()
				expected := testData.Output[i]
				if actual != expected {
					t.Errorf("UniformBool() at index %d = %v, want %v", i, actual, expected)
				}
			}
		})
	}
}

func TestShuffleReference(t *testing.T) {
	dirPath := "../tests/shuffle"
	files, err := os.ReadDir(dirPath)
	if err != nil {
		t.Fatalf("Failed to read directory: %v", err)
	}

	for _, file := range files {
		if !strings.HasSuffix(file.Name(), ".json") {
			continue
		}

		testName := strings.TrimSuffix(file.Name(), ".json")
		t.Run(testName, func(t *testing.T) {
			filePath := filepath.Join(dirPath, file.Name())
			data, err := os.ReadFile(filePath)
			if err != nil {
				t.Fatalf("Failed to read test file: %v", err)
			}

			var testData struct {
				Input  ShuffleInput `json:"input"`
				Output []float64    `json:"output"`
			}
			if err := json.Unmarshal(data, &testData); err != nil {
				t.Fatalf("Failed to parse test data: %v", err)
			}

			rng := NewRngFromSeed(testData.Input.Seed)
			actual := Shuffle(rng, testData.Input.X)

			if len(actual) != len(testData.Output) {
				t.Fatalf("Shuffle() length = %d, want %d", len(actual), len(testData.Output))
			}
			for i, v := range actual {
				if !floatEquals(v, testData.Output[i], 1e-15) {
					t.Errorf("Shuffle() at index %d = %v, want %v", i, v, testData.Output[i])
				}
			}
		})
	}
}

func TestSampleReference(t *testing.T) {
	dirPath := "../tests/sample"
	files, err := os.ReadDir(dirPath)
	if err != nil {
		t.Fatalf("Failed to read directory: %v", err)
	}

	for _, file := range files {
		if !strings.HasSuffix(file.Name(), ".json") {
			continue
		}

		testName := strings.TrimSuffix(file.Name(), ".json")
		t.Run(testName, func(t *testing.T) {
			filePath := filepath.Join(dirPath, file.Name())
			data, err := os.ReadFile(filePath)
			if err != nil {
				t.Fatalf("Failed to read test file: %v", err)
			}

			var testData struct {
				Input  SampleInput `json:"input"`
				Output []float64   `json:"output"`
			}
			if err := json.Unmarshal(data, &testData); err != nil {
				t.Fatalf("Failed to parse test data: %v", err)
			}

			rng := NewRngFromSeed(testData.Input.Seed)
			actual := Sample(rng, testData.Input.X, testData.Input.K)

			if len(actual) != len(testData.Output) {
				t.Fatalf("Sample() length = %d, want %d", len(actual), len(testData.Output))
			}
			for i, v := range actual {
				if !floatEquals(v, testData.Output[i], 1e-15) {
					t.Errorf("Sample() at index %d = %v, want %v", i, v, testData.Output[i])
				}
			}
		})
	}
}

func TestResampleReference(t *testing.T) {
	dirPath := "../tests/resample"
	files, err := os.ReadDir(dirPath)
	if err != nil {
		t.Fatalf("Failed to read directory: %v", err)
	}

	for _, file := range files {
		if !strings.HasSuffix(file.Name(), ".json") {
			continue
		}

		testName := strings.TrimSuffix(file.Name(), ".json")
		t.Run(testName, func(t *testing.T) {
			filePath := filepath.Join(dirPath, file.Name())
			data, err := os.ReadFile(filePath)
			if err != nil {
				t.Fatalf("Failed to read test file: %v", err)
			}

			var testData struct {
				Input  SampleInput `json:"input"`
				Output []float64   `json:"output"`
			}
			if err := json.Unmarshal(data, &testData); err != nil {
				t.Fatalf("Failed to parse test data: %v", err)
			}

			rng := NewRngFromSeed(testData.Input.Seed)
			actual := Resample(rng, testData.Input.X, testData.Input.K)

			if len(actual) != len(testData.Output) {
				t.Fatalf("Resample() length = %d, want %d", len(actual), len(testData.Output))
			}
			for i, v := range actual {
				if !floatEquals(v, testData.Output[i], 1e-15) {
					t.Errorf("Resample() at index %d = %v, want %v", i, v, testData.Output[i])
				}
			}
		})
	}
}

func TestUniformDistributionReference(t *testing.T) {
	dirPath := filepath.Join("../tests", "distributions", "uniform")
	files, err := os.ReadDir(dirPath)
	if err != nil {
		t.Fatalf("Failed to read directory: %v", err)
	}

	for _, file := range files {
		if !strings.HasSuffix(file.Name(), ".json") {
			continue
		}

		testName := strings.TrimSuffix(file.Name(), ".json")
		t.Run(testName, func(t *testing.T) {
			filePath := filepath.Join(dirPath, file.Name())
			data, err := os.ReadFile(filePath)
			if err != nil {
				t.Fatalf("Failed to read test file: %v", err)
			}

			var testData UniformDistTestCase
			if err := json.Unmarshal(data, &testData); err != nil {
				t.Fatalf("Failed to parse test data: %v", err)
			}

			rng := NewRngFromSeed(testData.Input.Seed)
			dist := NewUniform(testData.Input.Min, testData.Input.Max)

			for i := 0; i < testData.Input.Count; i++ {
				actual := dist.Sample(rng)
				expected := testData.Output[i]
				if !floatEquals(actual, expected, 1e-12) {
					t.Errorf("Uniform sample at index %d = %v, want %v", i, actual, expected)
				}
			}
		})
	}
}

func TestAdditiveDistributionReference(t *testing.T) {
	dirPath := filepath.Join("../tests", "distributions", "additive")
	files, err := os.ReadDir(dirPath)
	if err != nil {
		t.Fatalf("Failed to read directory: %v", err)
	}

	for _, file := range files {
		if !strings.HasSuffix(file.Name(), ".json") {
			continue
		}

		testName := strings.TrimSuffix(file.Name(), ".json")
		t.Run(testName, func(t *testing.T) {
			filePath := filepath.Join(dirPath, file.Name())
			data, err := os.ReadFile(filePath)
			if err != nil {
				t.Fatalf("Failed to read test file: %v", err)
			}

			var testData AdditiveDistTestCase
			if err := json.Unmarshal(data, &testData); err != nil {
				t.Fatalf("Failed to parse test data: %v", err)
			}

			rng := NewRngFromSeed(testData.Input.Seed)
			dist := NewAdditive(testData.Input.Mean, testData.Input.StdDev)

			for i := 0; i < testData.Input.Count; i++ {
				actual := dist.Sample(rng)
				expected := testData.Output[i]
				if !floatEquals(actual, expected, 1e-12) {
					t.Errorf("Additive sample at index %d = %v, want %v", i, actual, expected)
				}
			}
		})
	}
}

func TestMultiplicDistributionReference(t *testing.T) {
	dirPath := filepath.Join("../tests", "distributions", "multiplic")
	files, err := os.ReadDir(dirPath)
	if err != nil {
		t.Fatalf("Failed to read directory: %v", err)
	}

	for _, file := range files {
		if !strings.HasSuffix(file.Name(), ".json") {
			continue
		}

		testName := strings.TrimSuffix(file.Name(), ".json")
		t.Run(testName, func(t *testing.T) {
			filePath := filepath.Join(dirPath, file.Name())
			data, err := os.ReadFile(filePath)
			if err != nil {
				t.Fatalf("Failed to read test file: %v", err)
			}

			var testData MultiplicDistTestCase
			if err := json.Unmarshal(data, &testData); err != nil {
				t.Fatalf("Failed to parse test data: %v", err)
			}

			rng := NewRngFromSeed(testData.Input.Seed)
			dist := NewMultiplic(testData.Input.LogMean, testData.Input.LogStdDev)

			for i := 0; i < testData.Input.Count; i++ {
				actual := dist.Sample(rng)
				expected := testData.Output[i]
				if !floatEquals(actual, expected, 1e-12) {
					t.Errorf("Multiplic sample at index %d = %v, want %v", i, actual, expected)
				}
			}
		})
	}
}

func TestExpDistributionReference(t *testing.T) {
	dirPath := filepath.Join("../tests", "distributions", "exp")
	files, err := os.ReadDir(dirPath)
	if err != nil {
		t.Fatalf("Failed to read directory: %v", err)
	}

	for _, file := range files {
		if !strings.HasSuffix(file.Name(), ".json") {
			continue
		}

		testName := strings.TrimSuffix(file.Name(), ".json")
		t.Run(testName, func(t *testing.T) {
			filePath := filepath.Join(dirPath, file.Name())
			data, err := os.ReadFile(filePath)
			if err != nil {
				t.Fatalf("Failed to read test file: %v", err)
			}

			var testData ExpDistTestCase
			if err := json.Unmarshal(data, &testData); err != nil {
				t.Fatalf("Failed to parse test data: %v", err)
			}

			rng := NewRngFromSeed(testData.Input.Seed)
			dist := NewExp(testData.Input.Rate)

			for i := 0; i < testData.Input.Count; i++ {
				actual := dist.Sample(rng)
				expected := testData.Output[i]
				if !floatEquals(actual, expected, 1e-12) {
					t.Errorf("Exp sample at index %d = %v, want %v", i, actual, expected)
				}
			}
		})
	}
}

func TestPowerDistributionReference(t *testing.T) {
	dirPath := filepath.Join("../tests", "distributions", "power")
	files, err := os.ReadDir(dirPath)
	if err != nil {
		t.Fatalf("Failed to read directory: %v", err)
	}

	for _, file := range files {
		if !strings.HasSuffix(file.Name(), ".json") {
			continue
		}

		testName := strings.TrimSuffix(file.Name(), ".json")
		t.Run(testName, func(t *testing.T) {
			filePath := filepath.Join(dirPath, file.Name())
			data, err := os.ReadFile(filePath)
			if err != nil {
				t.Fatalf("Failed to read test file: %v", err)
			}

			var testData PowerDistTestCase
			if err := json.Unmarshal(data, &testData); err != nil {
				t.Fatalf("Failed to parse test data: %v", err)
			}

			rng := NewRngFromSeed(testData.Input.Seed)
			dist := NewPower(testData.Input.Min, testData.Input.Shape)

			for i := 0; i < testData.Input.Count; i++ {
				actual := dist.Sample(rng)
				expected := testData.Output[i]
				if !floatEquals(actual, expected, 1e-12) {
					t.Errorf("Power sample at index %d = %v, want %v", i, actual, expected)
				}
			}
		})
	}
}

func TestSampleNegativeKPanics(t *testing.T) {
	defer func() {
		if r := recover(); r == nil {
			t.Errorf("Sample with negative k should panic")
		}
	}()
	rng := NewRngFromString("test-sample-validation")
	Sample(rng, []float64{1, 2, 3}, -1)
}

// SignedRankMarginInput represents input for signed-rank-margin tests
type SignedRankMarginInput struct {
	N       int     `json:"n"`
	Misrate float64 `json:"misrate"`
}

// OneSampleBoundsInput represents input for one-sample bounds tests
type OneSampleBoundsInput struct {
	X       []float64 `json:"x"`
	Misrate float64   `json:"misrate"`
}

// SpreadBoundsInput represents input for spread-bounds tests
type SpreadBoundsInput struct {
	X       []float64 `json:"x"`
	Misrate float64   `json:"misrate"`
	Seed    string    `json:"seed"`
}

// AvgSpreadBoundsInput represents input for avg-spread-bounds tests
type AvgSpreadBoundsInput struct {
	X       []float64 `json:"x"`
	Y       []float64 `json:"y"`
	Misrate float64   `json:"misrate"`
	Seed    string    `json:"seed"`
}

// DisparityBoundsInput represents input for disparity-bounds tests
type DisparityBoundsInput struct {
	X       []float64 `json:"x"`
	Y       []float64 `json:"y"`
	Misrate float64   `json:"misrate"`
	Seed    string    `json:"seed"`
}

func TestSignedRankMarginReference(t *testing.T) {
	dirPath := filepath.Join("../tests", "signed-rank-margin")
	files, err := os.ReadDir(dirPath)
	if err != nil {
		t.Skipf("Skipping signed-rank-margin tests: %v", err)
	}

	for _, file := range files {
		if !strings.HasSuffix(file.Name(), ".json") {
			continue
		}

		testName := strings.TrimSuffix(file.Name(), ".json")
		t.Run(testName, func(t *testing.T) {
			filePath := filepath.Join(dirPath, file.Name())
			data, err := os.ReadFile(filePath)
			if err != nil {
				t.Fatalf("Failed to read test file: %v", err)
			}

			var testData TestData
			if err := json.Unmarshal(data, &testData); err != nil {
				t.Fatalf("Failed to parse test data: %v", err)
			}

			var input SignedRankMarginInput
			if err := json.Unmarshal(testData.Input, &input); err != nil {
				t.Fatalf("Failed to parse input data: %v", err)
			}

			// Handle error test cases
			if len(testData.ExpectedError) > 0 {
				_, err := signedRankMargin(input.N, input.Misrate)
				if err == nil {
					t.Errorf("Expected error for signedRankMargin(%d, %v), but got none",
						input.N, input.Misrate)
					return
				}
				// Verify error details match expected
				var expectedError map[string]string
				if jsonErr := json.Unmarshal(testData.ExpectedError, &expectedError); jsonErr == nil {
					if ae, ok := err.(*AssumptionError); ok {
						if string(ae.Violation.ID) != expectedError["id"] {
							t.Errorf("Expected error id %q, got %q", expectedError["id"], ae.Violation.ID)
						}
					} else {
						t.Errorf("Expected *AssumptionError but got %T: %v", err, err)
					}
				}
				return
			}

			var expected int
			if err := json.Unmarshal(testData.Output, &expected); err != nil {
				t.Fatalf("Failed to parse output data: %v", err)
			}

			actual, err := signedRankMargin(input.N, input.Misrate)
			if err != nil {
				t.Fatalf("SignedRankMargin returned unexpected error: %v", err)
			}
			if actual != expected {
				t.Errorf("signedRankMargin(%d, %v) = %d, want %d",
					input.N, input.Misrate, actual, expected)
			}
		})
	}
}

func TestCenterBoundsReference(t *testing.T) {
	dirPath := filepath.Join("../tests", "center-bounds")
	files, err := os.ReadDir(dirPath)
	if err != nil {
		t.Skipf("Skipping center-bounds tests: %v", err)
	}

	for _, file := range files {
		if !strings.HasSuffix(file.Name(), ".json") {
			continue
		}

		testName := strings.TrimSuffix(file.Name(), ".json")
		t.Run(testName, func(t *testing.T) {
			filePath := filepath.Join(dirPath, file.Name())
			data, err := os.ReadFile(filePath)
			if err != nil {
				t.Fatalf("Failed to read test file: %v", err)
			}

			var testData TestData
			if err := json.Unmarshal(data, &testData); err != nil {
				t.Fatalf("Failed to parse test data: %v", err)
			}

			var input OneSampleBoundsInput
			if err := json.Unmarshal(testData.Input, &input); err != nil {
				t.Fatalf("Failed to parse input data: %v", err)
			}

			// Handle error test cases
			if len(testData.ExpectedError) > 0 {
				_, err := CenterBounds(input.X, input.Misrate)
				if err == nil {
					t.Errorf("CenterBounds(%v, %v) expected error but got nil",
						input.X, input.Misrate)
					return
				}
				// Verify error details match expected
				var expectedError map[string]string
				if jsonErr := json.Unmarshal(testData.ExpectedError, &expectedError); jsonErr == nil {
					if ae, ok := err.(*AssumptionError); ok {
						if string(ae.Violation.ID) != expectedError["id"] {
							t.Errorf("Expected error id %q, got %q", expectedError["id"], ae.Violation.ID)
						}
					} else {
						t.Errorf("Expected *AssumptionError but got %T: %v", err, err)
					}
				}
				return
			}

			var expected BoundsOutput
			if err := json.Unmarshal(testData.Output, &expected); err != nil {
				t.Fatalf("Failed to parse output data: %v", err)
			}

			actual, err := CenterBounds(input.X, input.Misrate)
			if err != nil {
				t.Fatalf("CenterBounds(%v, %v) error: %v",
					input.X, input.Misrate, err)
			}
			if !floatEquals(actual.Lower, expected.Lower, 1e-9) ||
				!floatEquals(actual.Upper, expected.Upper, 1e-9) {
				t.Errorf("CenterBounds(%v, %v) = [%v, %v], want [%v, %v]",
					input.X, input.Misrate,
					actual.Lower, actual.Upper,
					expected.Lower, expected.Upper)
			}
		})
	}
}

func TestSpreadBoundsReference(t *testing.T) {
	dirPath := filepath.Join("../tests", "spread-bounds")
	files, err := os.ReadDir(dirPath)
	if err != nil {
		t.Skipf("Skipping spread-bounds tests: %v", err)
	}

	for _, file := range files {
		if !strings.HasSuffix(file.Name(), ".json") {
			continue
		}

		testName := strings.TrimSuffix(file.Name(), ".json")
		t.Run(testName, func(t *testing.T) {
			filePath := filepath.Join(dirPath, file.Name())
			data, err := os.ReadFile(filePath)
			if err != nil {
				t.Fatalf("Failed to read test file: %v", err)
			}

			var testData TestData
			if err := json.Unmarshal(data, &testData); err != nil {
				t.Fatalf("Failed to parse test data: %v", err)
			}

			var input SpreadBoundsInput
			if err := json.Unmarshal(testData.Input, &input); err != nil {
				t.Fatalf("Failed to parse input data: %v", err)
			}

			// Handle error test cases
			if len(testData.ExpectedError) > 0 {
				_, err := SpreadBoundsWithSeed(input.X, input.Misrate, input.Seed)
				if err == nil {
					t.Errorf("SpreadBoundsWithSeed(%v, %v, %q) expected error but got nil",
						input.X, input.Misrate, input.Seed)
					return
				}
				var expectedError map[string]string
				if jsonErr := json.Unmarshal(testData.ExpectedError, &expectedError); jsonErr == nil {
					if ae, ok := err.(*AssumptionError); ok {
						if string(ae.Violation.ID) != expectedError["id"] {
							t.Errorf("Expected error id %q, got %q", expectedError["id"], ae.Violation.ID)
						}
					} else {
						t.Errorf("Expected *AssumptionError but got %T: %v", err, err)
					}
				}
				return
			}

			var expected BoundsOutput
			if err := json.Unmarshal(testData.Output, &expected); err != nil {
				t.Fatalf("Failed to parse output data: %v", err)
			}

			actual, err := SpreadBoundsWithSeed(input.X, input.Misrate, input.Seed)
			if err != nil {
				t.Fatalf("SpreadBoundsWithSeed(%v, %v, %q) error: %v",
					input.X, input.Misrate, input.Seed, err)
			}
			if !floatEquals(actual.Lower, expected.Lower, 1e-9) ||
				!floatEquals(actual.Upper, expected.Upper, 1e-9) {
				t.Errorf("SpreadBoundsWithSeed(%v, %v, %q) = [%v, %v], want [%v, %v]",
					input.X, input.Misrate, input.Seed,
					actual.Lower, actual.Upper,
					expected.Lower, expected.Upper)
			}
		})
	}
}

func TestAvgSpreadBoundsReference(t *testing.T) {
	dirPath := filepath.Join("../tests", "avg-spread-bounds")
	files, err := os.ReadDir(dirPath)
	if err != nil {
		t.Skipf("Skipping avg-spread-bounds tests: %v", err)
	}

	for _, file := range files {
		if !strings.HasSuffix(file.Name(), ".json") {
			continue
		}

		testName := strings.TrimSuffix(file.Name(), ".json")
		t.Run(testName, func(t *testing.T) {
			filePath := filepath.Join(dirPath, file.Name())
			data, err := os.ReadFile(filePath)
			if err != nil {
				t.Fatalf("Failed to read test file: %v", err)
			}

			var testData TestData
			if err := json.Unmarshal(data, &testData); err != nil {
				t.Fatalf("Failed to parse test data: %v", err)
			}

			var input AvgSpreadBoundsInput
			if err := json.Unmarshal(testData.Input, &input); err != nil {
				t.Fatalf("Failed to parse input data: %v", err)
			}

			// Handle error test cases
			if len(testData.ExpectedError) > 0 {
				_, err := avgSpreadBounds(input.X, input.Y, input.Misrate)
				if err == nil {
					t.Errorf("avgSpreadBounds(%v, %v, %v) expected error but got nil",
						input.X, input.Y, input.Misrate)
					return
				}
				var expectedError map[string]string
				if jsonErr := json.Unmarshal(testData.ExpectedError, &expectedError); jsonErr == nil {
					if ae, ok := err.(*AssumptionError); ok {
						if string(ae.Violation.ID) != expectedError["id"] {
							t.Errorf("Expected error id %q, got %q", expectedError["id"], ae.Violation.ID)
						}
					} else {
						t.Errorf("Expected *AssumptionError but got %T: %v", err, err)
					}
				}
				return
			}

			var expected BoundsOutput
			if err := json.Unmarshal(testData.Output, &expected); err != nil {
				t.Fatalf("Failed to parse output data: %v", err)
			}

			actual, err := avgSpreadBoundsWithSeed(input.X, input.Y, input.Misrate, input.Seed)
			if err != nil {
				t.Fatalf("avgSpreadBoundsWithSeed(%v, %v, %v, %q) error: %v",
					input.X, input.Y, input.Misrate, input.Seed, err)
			}
			if !floatEquals(actual.Lower, expected.Lower, 1e-9) ||
				!floatEquals(actual.Upper, expected.Upper, 1e-9) {
				t.Errorf("avgSpreadBoundsWithSeed(%v, %v, %v, %q) = [%v, %v], want [%v, %v]",
					input.X, input.Y, input.Misrate, input.Seed,
					actual.Lower, actual.Upper,
					expected.Lower, expected.Upper)
			}
		})
	}
}

func TestDisparityBoundsReference(t *testing.T) {
	dirPath := filepath.Join("../tests", "disparity-bounds")
	files, err := os.ReadDir(dirPath)
	if err != nil {
		t.Skipf("Skipping disparity-bounds tests: %v", err)
	}

	for _, file := range files {
		if !strings.HasSuffix(file.Name(), ".json") {
			continue
		}

		testName := strings.TrimSuffix(file.Name(), ".json")
		t.Run(testName, func(t *testing.T) {
			filePath := filepath.Join(dirPath, file.Name())
			data, err := os.ReadFile(filePath)
			if err != nil {
				t.Fatalf("Failed to read test file: %v", err)
			}

			var testData TestData
			if err := json.Unmarshal(data, &testData); err != nil {
				t.Fatalf("Failed to parse test data: %v", err)
			}

			var input DisparityBoundsInput
			if err := json.Unmarshal(testData.Input, &input); err != nil {
				t.Fatalf("Failed to parse input data: %v", err)
			}

			// Handle error test cases
			if len(testData.ExpectedError) > 0 {
				_, err := DisparityBoundsWithSeed(input.X, input.Y, input.Misrate, input.Seed)
				if err == nil {
					t.Errorf("DisparityBoundsWithSeed(%v, %v, %v, %q) expected error but got nil",
						input.X, input.Y, input.Misrate, input.Seed)
					return
				}
				var expectedError map[string]string
				if jsonErr := json.Unmarshal(testData.ExpectedError, &expectedError); jsonErr == nil {
					if ae, ok := err.(*AssumptionError); ok {
						if string(ae.Violation.ID) != expectedError["id"] {
							t.Errorf("Expected error id %q, got %q", expectedError["id"], ae.Violation.ID)
						}
					} else {
						t.Errorf("Expected *AssumptionError but got %T: %v", err, err)
					}
				}
				return
			}

			var expected BoundsOutput
			if err := json.Unmarshal(testData.Output, &expected); err != nil {
				t.Fatalf("Failed to parse output data: %v", err)
			}

			actual, err := DisparityBoundsWithSeed(input.X, input.Y, input.Misrate, input.Seed)
			if err != nil {
				t.Fatalf("DisparityBoundsWithSeed(%v, %v, %v, %q) error: %v",
					input.X, input.Y, input.Misrate, input.Seed, err)
			}
			if !floatEquals(actual.Lower, expected.Lower, 1e-9) ||
				!floatEquals(actual.Upper, expected.Upper, 1e-9) {
				t.Errorf("DisparityBoundsWithSeed(%v, %v, %v, %q) = [%v, %v], want [%v, %v]",
					input.X, input.Y, input.Misrate, input.Seed,
					actual.Lower, actual.Upper,
					expected.Lower, expected.Upper)
			}
		})
	}
}
