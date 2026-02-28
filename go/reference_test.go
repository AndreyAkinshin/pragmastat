package pragmastat

import (
	"encoding/json"
	"fmt"
	"math"
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

// mustSample creates a Sample or fatals.
func mustSample(t *testing.T, values []float64) *Sample {
	t.Helper()
	s, err := NewSample(values)
	if err != nil {
		t.Fatalf("NewSample failed: %v", err)
	}
	return s
}

func TestReferenceData(t *testing.T) {
	// Map estimator names to functions
	oneSampleEstimators := map[string]func(*Sample) (Measurement, error){
		"center":     Center,
		"spread":     Spread,
		"rel-spread": RelSpread,
	}

	twoSampleEstimators := map[string]func(*Sample, *Sample) (Measurement, error){
		"shift":     Shift,
		"ratio":     Ratio,
		"disparity": Disparity,
	}

	twoSampleInternalEstimators := map[string]func(*Sample, *Sample) (Measurement, error){
		"avg-spread": avgSpread,
	}

	// Special test for pairwise-margin
	t.Run("pairwise-margin", func(t *testing.T) {
		dirPath := filepath.Join("../tests", "pairwise-margin")
		files, err := os.ReadDir(dirPath)
		if err != nil {
			t.Fatalf("Test data directory not found for pairwise-margin: %v", err)
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

				if len(testData.ExpectedError) > 0 {
					_, err := pairwiseMargin(input.N, input.M, input.Misrate)
					if err == nil {
						t.Errorf("Expected error for pairwiseMargin(%d, %d, %v), but got none", input.N, input.M, input.Misrate)
						return
					}
					var expectedError map[string]string
					if jsonErr := json.Unmarshal(testData.ExpectedError, &expectedError); jsonErr == nil {
						if ae, ok := err.(*AssumptionError); ok {
							if string(ae.Violation.ID) != expectedError["id"] {
								t.Errorf("Expected error id %q, got %q", expectedError["id"], ae.Violation.ID)
							}
							if subj, ok := expectedError["subject"]; ok {
								if string(ae.Violation.Subject) != subj {
									t.Errorf("Expected error subject %q, got %q", subj, ae.Violation.Subject)
								}
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
			t.Fatalf("Test data directory not found for shift-bounds: %v", err)
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

				if len(testData.ExpectedError) > 0 {
					sx, sxErr := NewSample(input.X)
					if sxErr != nil {
						return // Sample construction error counts as expected
					}
					sy, syErr := newSample(input.Y, nil, nil, SubjectY)
					if syErr != nil {
						return // Sample construction error counts as expected
					}
					_, err := ShiftBounds(sx, sy, input.Misrate)
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
							if subj, ok := expectedError["subject"]; ok {
								if string(ae.Violation.Subject) != subj {
									t.Errorf("Expected error subject %q, got %q", subj, ae.Violation.Subject)
								}
							}
						}
					}
					return
				}

				var expected BoundsOutput
				if err := json.Unmarshal(testData.Output, &expected); err != nil {
					t.Fatalf("Failed to parse output data: %v", err)
				}

				sx := mustSample(t, input.X)
				sy, err := newSample(input.Y, nil, nil, SubjectY)
				if err != nil {
					t.Fatalf("Failed to create sample Y: %v", err)
				}
				actual, err := ShiftBounds(sx, sy, input.Misrate)
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
			t.Fatalf("Test data directory not found for ratio-bounds: %v", err)
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

				if len(testData.ExpectedError) > 0 {
					sx, sxErr := NewSample(input.X)
					if sxErr != nil {
						return
					}
					sy, syErr := newSample(input.Y, nil, nil, SubjectY)
					if syErr != nil {
						return
					}
					_, err := RatioBounds(sx, sy, input.Misrate)
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
							if subj, ok := expectedError["subject"]; ok {
								if string(ae.Violation.Subject) != subj {
									t.Errorf("Expected error subject %q, got %q", subj, ae.Violation.Subject)
								}
							}
						}
					}
					return
				}

				var expected BoundsOutput
				if err := json.Unmarshal(testData.Output, &expected); err != nil {
					t.Fatalf("Failed to parse output data: %v", err)
				}

				sx := mustSample(t, input.X)
				sy, err := newSample(input.Y, nil, nil, SubjectY)
				if err != nil {
					t.Fatalf("Failed to create sample Y: %v", err)
				}
				actual, err := RatioBounds(sx, sy, input.Misrate)
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
			t.Fatalf("Test data directory not found for %s: %v", estimatorName, err)
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

				// Handle error test cases
				if len(testData.ExpectedError) > 0 {
					var input OneSampleInput
					if err := json.Unmarshal(testData.Input, &input); err != nil {
						t.Fatalf("Failed to parse input data: %v", err)
					}
					sx, sErr := NewSample(input.X)
					if sErr != nil {
						// Sample construction error counts as the expected error
						var expectedError map[string]string
						if jsonErr := json.Unmarshal(testData.ExpectedError, &expectedError); jsonErr == nil {
							if ae, ok := sErr.(*AssumptionError); ok {
								if string(ae.Violation.ID) != expectedError["id"] {
									t.Errorf("Expected error id %q, got %q", expectedError["id"], ae.Violation.ID)
								}
							}
						}
						return
					}
					_, err := estimatorFunc(sx)
					if err == nil {
						t.Errorf("Expected error for %s, but got none", testName)
						return
					}
					var expectedError map[string]string
					if jsonErr := json.Unmarshal(testData.ExpectedError, &expectedError); jsonErr == nil {
						if ae, ok := err.(*AssumptionError); ok {
							if string(ae.Violation.ID) != expectedError["id"] {
								t.Errorf("Expected error id %q, got %q", expectedError["id"], ae.Violation.ID)
							}
							if subj, ok := expectedError["subject"]; ok {
								if string(ae.Violation.Subject) != subj {
									t.Errorf("Expected error subject %q, got %q", subj, ae.Violation.Subject)
								}
							}
						}
					}
					return
				}

				var expected float64
				if err := json.Unmarshal(testData.Output, &expected); err != nil {
					t.Fatalf("Failed to parse output data: %v", err)
				}

				var input OneSampleInput
				if err := json.Unmarshal(testData.Input, &input); err == nil && input.X != nil {
					sx := mustSample(t, input.X)
					m, err := estimatorFunc(sx)
					if err != nil {
						t.Fatalf("%s(%v) error: %v", estimatorName, input.X, err)
					}
					if !floatEquals(m.Value, expected, 1e-9) {
						t.Errorf("%s(%v) = %v, want %v", estimatorName, input.X, m.Value, expected)
					}
					return
				}

				var directInput []float64
				if err := json.Unmarshal(testData.Input, &directInput); err == nil {
					sx := mustSample(t, directInput)
					m, err := estimatorFunc(sx)
					if err != nil {
						t.Fatalf("%s(%v) error: %v", estimatorName, directInput, err)
					}
					if !floatEquals(m.Value, expected, 1e-9) {
						t.Errorf("%s(%v) = %v, want %v", estimatorName, directInput, m.Value, expected)
					}
					return
				}

				t.Fatalf("Failed to parse input data")
			})
		}
	}

	// Test two-sample estimators (public + internal)
	allTwoSample := make(map[string]func(*Sample, *Sample) (Measurement, error))
	for k, v := range twoSampleEstimators {
		allTwoSample[k] = v
	}
	for k, v := range twoSampleInternalEstimators {
		allTwoSample[k] = v
	}

	for estimatorName, estimatorFunc := range allTwoSample {
		dirPath := filepath.Join(testDataPath, estimatorName)
		files, err := os.ReadDir(dirPath)
		if err != nil {
			t.Fatalf("Test data directory not found for %s: %v", estimatorName, err)
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

				// Handle error test cases
				if len(testData.ExpectedError) > 0 {
					var input TwoSampleInput
					if err := json.Unmarshal(testData.Input, &input); err != nil {
						t.Fatalf("Failed to parse input data: %v", err)
					}
					sx, sxErr := NewSample(input.X)
					if sxErr != nil {
						var expectedError map[string]string
						if jsonErr := json.Unmarshal(testData.ExpectedError, &expectedError); jsonErr == nil {
							if ae, ok := sxErr.(*AssumptionError); ok {
								if string(ae.Violation.ID) != expectedError["id"] {
									t.Errorf("Expected error id %q, got %q", expectedError["id"], ae.Violation.ID)
								}
							}
						}
						return
					}
					sy, syErr := newSample(input.Y, nil, nil, SubjectY)
					if syErr != nil {
						var expectedError map[string]string
						if jsonErr := json.Unmarshal(testData.ExpectedError, &expectedError); jsonErr == nil {
							if ae, ok := syErr.(*AssumptionError); ok {
								if string(ae.Violation.ID) != expectedError["id"] {
									t.Errorf("Expected error id %q, got %q", expectedError["id"], ae.Violation.ID)
								}
							}
						}
						return
					}
					_, err := estimatorFunc(sx, sy)
					if err == nil {
						t.Errorf("Expected error for %s, but got none", testName)
						return
					}
					var expectedError map[string]string
					if jsonErr := json.Unmarshal(testData.ExpectedError, &expectedError); jsonErr == nil {
						if ae, ok := err.(*AssumptionError); ok {
							if string(ae.Violation.ID) != expectedError["id"] {
								t.Errorf("Expected error id %q, got %q", expectedError["id"], ae.Violation.ID)
							}
							if subj, ok := expectedError["subject"]; ok {
								if string(ae.Violation.Subject) != subj {
									t.Errorf("Expected error subject %q, got %q", subj, ae.Violation.Subject)
								}
							}
						}
					}
					return
				}

				var expected float64
				if err := json.Unmarshal(testData.Output, &expected); err != nil {
					t.Fatalf("Failed to parse output data: %v", err)
				}

				var input TwoSampleInput
				if err := json.Unmarshal(testData.Input, &input); err != nil {
					t.Fatalf("Failed to parse input data: %v", err)
				}

				sx := mustSample(t, input.X)
				sy, err := newSample(input.Y, nil, nil, SubjectY)
				if err != nil {
					t.Fatalf("Failed to create sample Y: %v", err)
				}
				m, err := estimatorFunc(sx, sy)
				if err != nil {
					t.Fatalf("%s(%v, %v) error: %v", estimatorName, input.X, input.Y, err)
				}
				if !floatEquals(m.Value, expected, 1e-9) {
					t.Errorf("%s(%v, %v) = %v, want %v", estimatorName, input.X, input.Y, m.Value, expected)
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
			actual := RngShuffle(rng, testData.Input.X)

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
			actual := RngSample(rng, testData.Input.X, testData.Input.K)

			if len(actual) != len(testData.Output) {
				t.Fatalf("RngSample() length = %d, want %d", len(actual), len(testData.Output))
			}
			for i, v := range actual {
				if !floatEquals(v, testData.Output[i], 1e-15) {
					t.Errorf("RngSample() at index %d = %v, want %v", i, v, testData.Output[i])
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
			actual := RngResample(rng, testData.Input.X, testData.Input.K)

			if len(actual) != len(testData.Output) {
				t.Fatalf("RngResample() length = %d, want %d", len(actual), len(testData.Output))
			}
			for i, v := range actual {
				if !floatEquals(v, testData.Output[i], 1e-15) {
					t.Errorf("RngResample() at index %d = %v, want %v", i, v, testData.Output[i])
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
			t.Errorf("RngSample with negative k should panic")
		}
	}()
	rng := NewRngFromString("test-sample-validation")
	RngSample(rng, []float64{1, 2, 3}, -1)
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
		t.Fatalf("Test data directory not found for signed-rank-margin: %v", err)
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

			if len(testData.ExpectedError) > 0 {
				_, err := signedRankMargin(input.N, input.Misrate)
				if err == nil {
					t.Errorf("Expected error for signedRankMargin(%d, %v), but got none",
						input.N, input.Misrate)
					return
				}
				var expectedError map[string]string
				if jsonErr := json.Unmarshal(testData.ExpectedError, &expectedError); jsonErr == nil {
					if ae, ok := err.(*AssumptionError); ok {
						if string(ae.Violation.ID) != expectedError["id"] {
							t.Errorf("Expected error id %q, got %q", expectedError["id"], ae.Violation.ID)
						}
						if subj, ok := expectedError["subject"]; ok {
							if string(ae.Violation.Subject) != subj {
								t.Errorf("Expected error subject %q, got %q", subj, ae.Violation.Subject)
							}
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
		t.Fatalf("Test data directory not found for center-bounds: %v", err)
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

			if len(testData.ExpectedError) > 0 {
				sx, sErr := NewSample(input.X)
				if sErr != nil {
					// Construction error is the expected error
					return
				}
				_, err := CenterBounds(sx, input.Misrate)
				if err == nil {
					t.Errorf("CenterBounds(%v, %v) expected error but got nil",
						input.X, input.Misrate)
					return
				}
				var expectedError map[string]string
				if jsonErr := json.Unmarshal(testData.ExpectedError, &expectedError); jsonErr == nil {
					if ae, ok := err.(*AssumptionError); ok {
						if string(ae.Violation.ID) != expectedError["id"] {
							t.Errorf("Expected error id %q, got %q", expectedError["id"], ae.Violation.ID)
						}
						if subj, ok := expectedError["subject"]; ok {
							if string(ae.Violation.Subject) != subj {
								t.Errorf("Expected error subject %q, got %q", subj, ae.Violation.Subject)
							}
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

			sx := mustSample(t, input.X)
			actual, err := CenterBounds(sx, input.Misrate)
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
		t.Fatalf("Test data directory not found for spread-bounds: %v", err)
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

			if len(testData.ExpectedError) > 0 {
				sx, sErr := NewSample(input.X)
				if sErr != nil {
					return
				}
				_, err := SpreadBoundsWithSeed(sx, input.Misrate, input.Seed)
				if err == nil {
					t.Errorf("SpreadBounds(%v, %v, %q) expected error but got nil",
						input.X, input.Misrate, input.Seed)
					return
				}
				var expectedError map[string]string
				if jsonErr := json.Unmarshal(testData.ExpectedError, &expectedError); jsonErr == nil {
					if ae, ok := err.(*AssumptionError); ok {
						if string(ae.Violation.ID) != expectedError["id"] {
							t.Errorf("Expected error id %q, got %q", expectedError["id"], ae.Violation.ID)
						}
						if subj, ok := expectedError["subject"]; ok {
							if string(ae.Violation.Subject) != subj {
								t.Errorf("Expected error subject %q, got %q", subj, ae.Violation.Subject)
							}
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

			sx := mustSample(t, input.X)
			actual, err := SpreadBoundsWithSeed(sx, input.Misrate, input.Seed)
			if err != nil {
				t.Fatalf("SpreadBounds(%v, %v, %q) error: %v",
					input.X, input.Misrate, input.Seed, err)
			}
			if !floatEquals(actual.Lower, expected.Lower, 1e-9) ||
				!floatEquals(actual.Upper, expected.Upper, 1e-9) {
				t.Errorf("SpreadBounds(%v, %v, %q) = [%v, %v], want [%v, %v]",
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
		t.Fatalf("Test data directory not found for avg-spread-bounds: %v", err)
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

			if len(testData.ExpectedError) > 0 {
				sx, sxErr := NewSample(input.X)
				if sxErr != nil {
					return
				}
				sy, syErr := newSample(input.Y, nil, nil, SubjectY)
				if syErr != nil {
					return
				}
				_, err := avgSpreadBoundsWithRngs(sx, sy, input.Misrate, NewRngFromString(input.Seed), NewRngFromString(input.Seed))
				if err == nil {
					t.Errorf("avgSpreadBoundsWithRngs(%v, %v, %v, %q) expected error but got nil",
						input.X, input.Y, input.Misrate, input.Seed)
					return
				}
				var expectedError map[string]string
				if jsonErr := json.Unmarshal(testData.ExpectedError, &expectedError); jsonErr == nil {
					if ae, ok := err.(*AssumptionError); ok {
						if string(ae.Violation.ID) != expectedError["id"] {
							t.Errorf("Expected error id %q, got %q", expectedError["id"], ae.Violation.ID)
						}
						if subj, ok := expectedError["subject"]; ok {
							if string(ae.Violation.Subject) != subj {
								t.Errorf("Expected error subject %q, got %q", subj, ae.Violation.Subject)
							}
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

			sx := mustSample(t, input.X)
			sy, err := newSample(input.Y, nil, nil, SubjectY)
			if err != nil {
				t.Fatalf("Failed to create sample Y: %v", err)
			}
			actual, err := avgSpreadBoundsWithRngs(sx, sy, input.Misrate, NewRngFromString(input.Seed), NewRngFromString(input.Seed))
			if err != nil {
				t.Fatalf("avgSpreadBoundsWithRngs(%v, %v, %v, %q) error: %v",
					input.X, input.Y, input.Misrate, input.Seed, err)
			}
			if !floatEquals(actual.Lower, expected.Lower, 1e-9) ||
				!floatEquals(actual.Upper, expected.Upper, 1e-9) {
				t.Errorf("avgSpreadBoundsWithRngs(%v, %v, %v, %q) = [%v, %v], want [%v, %v]",
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
		t.Fatalf("Test data directory not found for disparity-bounds: %v", err)
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

			if len(testData.ExpectedError) > 0 {
				sx, sxErr := NewSample(input.X)
				if sxErr != nil {
					return
				}
				sy, syErr := newSample(input.Y, nil, nil, SubjectY)
				if syErr != nil {
					return
				}
				_, err := DisparityBoundsWithSeed(sx, sy, input.Misrate, input.Seed)
				if err == nil {
					t.Errorf("DisparityBounds(%v, %v, %v, %q) expected error but got nil",
						input.X, input.Y, input.Misrate, input.Seed)
					return
				}
				var expectedError map[string]string
				if jsonErr := json.Unmarshal(testData.ExpectedError, &expectedError); jsonErr == nil {
					if ae, ok := err.(*AssumptionError); ok {
						if string(ae.Violation.ID) != expectedError["id"] {
							t.Errorf("Expected error id %q, got %q", expectedError["id"], ae.Violation.ID)
						}
						if subj, ok := expectedError["subject"]; ok {
							if string(ae.Violation.Subject) != subj {
								t.Errorf("Expected error subject %q, got %q", subj, ae.Violation.Subject)
							}
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

			sx := mustSample(t, input.X)
			sy, err := newSample(input.Y, nil, nil, SubjectY)
			if err != nil {
				t.Fatalf("Failed to create sample Y: %v", err)
			}
			actual, err := DisparityBoundsWithSeed(sx, sy, input.Misrate, input.Seed)
			if err != nil {
				t.Fatalf("DisparityBounds(%v, %v, %v, %q) error: %v",
					input.X, input.Y, input.Misrate, input.Seed, err)
			}
			if !floatEquals(actual.Lower, expected.Lower, 1e-9) ||
				!floatEquals(actual.Upper, expected.Upper, 1e-9) {
				t.Errorf("DisparityBounds(%v, %v, %v, %q) = [%v, %v], want [%v, %v]",
					input.X, input.Y, input.Misrate, input.Seed,
					actual.Lower, actual.Upper,
					expected.Lower, expected.Upper)
			}
		})
	}
}

// Metrology tests

func TestSampleConstruction(t *testing.T) {
	dirPath := filepath.Join("../tests", "sample-construction")
	files, err := os.ReadDir(dirPath)
	if err != nil {
		t.Fatalf("Test data directory not found: %v", err)
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

			var raw map[string]json.RawMessage
			if err := json.Unmarshal(data, &raw); err != nil {
				t.Fatalf("Failed to parse: %v", err)
			}

			var input struct {
				Values  []interface{} `json:"values"`
				Weights []float64     `json:"weights"`
			}
			if err := json.Unmarshal(raw["input"], &input); err != nil {
				t.Fatalf("Failed to parse input: %v", err)
			}

			// Convert values handling special floats
			values := make([]float64, len(input.Values))
			for i, v := range input.Values {
				switch val := v.(type) {
				case float64:
					values[i] = val
				case string:
					switch val {
					case "NaN":
						values[i] = math.NaN()
					case "Infinity":
						values[i] = math.Inf(1)
					case "-Infinity":
						values[i] = math.Inf(-1)
					}
				}
			}

			if _, ok := raw["expected_error"]; ok {
				var s *Sample
				var sErr error
				if input.Weights != nil {
					s, sErr = NewWeightedSample(values, input.Weights, nil)
				} else {
					s, sErr = NewSample(values)
				}
				if sErr == nil {
					t.Errorf("Expected error but got sample: %v", s)
				}
				return
			}

			var output struct {
				Size       int  `json:"size"`
				IsWeighted bool `json:"is_weighted"`
			}
			if err := json.Unmarshal(raw["output"], &output); err != nil {
				t.Fatalf("Failed to parse output: %v", err)
			}

			var s *Sample
			var sErr error
			if input.Weights != nil {
				s, sErr = NewWeightedSample(values, input.Weights, nil)
			} else {
				s, sErr = NewSample(values)
			}
			if sErr != nil {
				t.Fatalf("Unexpected error: %v", sErr)
			}
			if s.Size() != output.Size {
				t.Errorf("Size = %d, want %d", s.Size(), output.Size)
			}
			if s.IsWeighted != output.IsWeighted {
				t.Errorf("IsWeighted = %v, want %v", s.IsWeighted, output.IsWeighted)
			}
		})
	}
}

func TestUnitPropagation(t *testing.T) {
	dirPath := filepath.Join("../tests", "unit-propagation")
	files, err := os.ReadDir(dirPath)
	if err != nil {
		t.Fatalf("Test data directory not found: %v", err)
	}

	registry := StandardRegistry()

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

			var raw map[string]json.RawMessage
			if err := json.Unmarshal(data, &raw); err != nil {
				t.Fatalf("Failed to parse: %v", err)
			}

			// Check for expected_error (weighted-rejected test)
			if _, ok := raw["expected_error"]; ok {
				var input struct {
					Estimator string    `json:"estimator"`
					X         []float64 `json:"x"`
					XWeights  []float64 `json:"x_weights"`
				}
				if err := json.Unmarshal(raw["input"], &input); err != nil {
					t.Fatalf("Failed to parse input: %v", err)
				}
				sx, sErr := NewWeightedSample(input.X, input.XWeights, nil)
				if sErr != nil {
					t.Fatalf("Failed to create weighted sample: %v", sErr)
				}
				_, err := Center(sx)
				if err == nil {
					t.Errorf("Expected error for weighted sample, got none")
				}
				return
			}

			var input struct {
				Estimator string    `json:"estimator"`
				X         []float64 `json:"x"`
				Y         []float64 `json:"y"`
				XUnit     string    `json:"x_unit"`
				YUnit     string    `json:"y_unit"`
			}
			if err := json.Unmarshal(raw["input"], &input); err != nil {
				t.Fatalf("Failed to parse input: %v", err)
			}

			var output struct {
				Value *float64 `json:"value"`
				Unit  string   `json:"unit"`
			}
			if err := json.Unmarshal(raw["output"], &output); err != nil {
				t.Fatalf("Failed to parse output: %v", err)
			}

			xUnit, err := registry.Resolve(input.XUnit)
			if err != nil {
				t.Fatalf("Failed to resolve x_unit %q: %v", input.XUnit, err)
			}

			sx, err := NewSampleWithUnit(input.X, xUnit)
			if err != nil {
				t.Fatalf("Failed to create sample X: %v", err)
			}

			switch input.Estimator {
			case "center":
				m, err := Center(sx)
				if err != nil {
					t.Fatalf("Center error: %v", err)
				}
				if m.Unit.ID != output.Unit {
					t.Errorf("Unit = %q, want %q", m.Unit.ID, output.Unit)
				}
				if output.Value != nil && !floatEquals(m.Value, *output.Value, 1e-9) {
					t.Errorf("Value = %v, want %v", m.Value, *output.Value)
				}

			case "spread":
				m, err := Spread(sx)
				if err != nil {
					t.Fatalf("Spread error: %v", err)
				}
				if m.Unit.ID != output.Unit {
					t.Errorf("Unit = %q, want %q", m.Unit.ID, output.Unit)
				}

			case "shift":
				yUnit, err := registry.Resolve(input.YUnit)
				if err != nil {
					t.Fatalf("Failed to resolve y_unit: %v", err)
				}
				sy, err := newSample(input.Y, nil, yUnit, SubjectY)
				if err != nil {
					t.Fatalf("Failed to create sample Y: %v", err)
				}
				m, err := Shift(sx, sy)
				if err != nil {
					t.Fatalf("Shift error: %v", err)
				}
				if m.Unit.ID != output.Unit {
					t.Errorf("Unit = %q, want %q", m.Unit.ID, output.Unit)
				}

			case "ratio":
				yUnit, err := registry.Resolve(input.YUnit)
				if err != nil {
					t.Fatalf("Failed to resolve y_unit: %v", err)
				}
				sy, err := newSample(input.Y, nil, yUnit, SubjectY)
				if err != nil {
					t.Fatalf("Failed to create sample Y: %v", err)
				}
				m, err := Ratio(sx, sy)
				if err != nil {
					t.Fatalf("Ratio error: %v", err)
				}
				if m.Unit.ID != output.Unit {
					t.Errorf("Unit = %q, want %q", m.Unit.ID, output.Unit)
				}

			case "disparity":
				yUnit, err := registry.Resolve(input.YUnit)
				if err != nil {
					t.Fatalf("Failed to resolve y_unit: %v", err)
				}
				sy, err := newSample(input.Y, nil, yUnit, SubjectY)
				if err != nil {
					t.Fatalf("Failed to create sample Y: %v", err)
				}
				m, err := Disparity(sx, sy)
				if err != nil {
					t.Fatalf("Disparity error: %v", err)
				}
				if m.Unit.ID != output.Unit {
					t.Errorf("Unit = %q, want %q", m.Unit.ID, output.Unit)
				}

			default:
				t.Fatalf("Unknown estimator: %q", input.Estimator)
			}
		})
	}
}
