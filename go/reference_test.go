package pragmastat

import (
	"encoding/json"
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
	// pairwise-margin operates on (n, m, misrate), not on a sample, so it has a
	// single path.
	t.Run("pairwise-margin", func(t *testing.T) {
		forEachFixture(t, "pairwise-margin", func(t *testing.T, td TestData, input PairwiseMarginInput) {
			if len(td.ExpectedError) > 0 {
				_, err := pairwiseMargin(input.N, input.M, input.Misrate)
				assertErrorMatches(t, td.ExpectedError, err, true)
				return
			}
			var expected int
			if err := json.Unmarshal(td.Output, &expected); err != nil {
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
	})

	// One-sample scalar estimators: center, spread.
	oneSampleScalar := []struct {
		name   string
		rawFn  func(x []float64, assumeSorted bool) (float64, error)
		sampFn func(x *Sample) (Measurement, error)
	}{
		{"center", Center, (*Sample).Center},
		{"spread", Spread, (*Sample).Spread},
	}
	for _, est := range oneSampleScalar {
		est := est
		t.Run(est.name, func(t *testing.T) {
			forEachFixture(t, est.name, func(t *testing.T, td TestData, input OneSampleInput) {
				entries := []scalarEntry{
					{
						name: "raw",
						run: func(t *testing.T) (float64, error, bool) {
							v, err := est.rawFn(input.X, false)
							return v, err, false
						},
					},
					{
						name: "sample",
						run: func(t *testing.T) (float64, error, bool) {
							sx, err := sampleX(input.X)
							if err != nil {
								return 0, err, true
							}
							m, err := est.sampFn(sx)
							return m.Value, err, false
						},
					},
				}
				runScalarDualPath(t, td, entries)
			})
		})
	}

	// Two-sample scalar estimators: shift, ratio, disparity (public).
	twoSampleScalar := []struct {
		name   string
		rawFn  func(x, y []float64, assumeSorted bool) (float64, error)
		sampFn func(x, y *Sample) (Measurement, error)
	}{
		{"shift", Shift, func(x, y *Sample) (Measurement, error) { return x.Shift(y) }},
		{"ratio", Ratio, func(x, y *Sample) (Measurement, error) { return x.Ratio(y) }},
		{"disparity", Disparity, func(x, y *Sample) (Measurement, error) { return x.Disparity(y) }},
	}
	for _, est := range twoSampleScalar {
		est := est
		t.Run(est.name, func(t *testing.T) {
			forEachFixture(t, est.name, func(t *testing.T, td TestData, input TwoSampleInput) {
				entries := []scalarEntry{
					{
						name: "raw",
						run: func(t *testing.T) (float64, error, bool) {
							v, err := est.rawFn(input.X, input.Y, false)
							return v, err, false
						},
					},
					{
						name: "sample",
						run: func(t *testing.T) (float64, error, bool) {
							sx, err := sampleX(input.X)
							if err != nil {
								return 0, err, true
							}
							sy, err := sampleY(input.Y)
							if err != nil {
								return 0, err, true
							}
							m, err := est.sampFn(sx, sy)
							return m.Value, err, false
						},
					},
				}
				runScalarDualPath(t, td, entries)
			})
		})
	}

	// avg-spread is an internal helper with no public raw entry; single path.
	t.Run("avg-spread", func(t *testing.T) {
		forEachFixture(t, "avg-spread", func(t *testing.T, td TestData, input TwoSampleInput) {
			entries := []scalarEntry{
				{
					name: "raw",
					run: func(t *testing.T) (float64, error, bool) {
						v, err := avgSpread(input.X, input.Y, false)
						return v, err, false
					},
				},
				{
					name: "sample",
					run: func(t *testing.T) (float64, error, bool) {
						sx, err := sampleX(input.X)
						if err != nil {
							return 0, err, true
						}
						sy, err := sampleY(input.Y)
						if err != nil {
							return 0, err, true
						}
						m, err := sx.avgSpread(sy)
						return m.Value, err, false
					},
				},
			}
			runScalarDualPath(t, td, entries)
		})
	})

	// Two-sample bounds estimators (deterministic): shift-bounds, ratio-bounds.
	twoSampleBounds := []struct {
		name   string
		rawFn  func(x, y []float64, misrate float64, assumeSorted bool) (Bounds, error)
		sampFn func(x, y *Sample, misrate float64) (Bounds, error)
	}{
		{"shift-bounds", ShiftBounds, func(x, y *Sample, m float64) (Bounds, error) { return x.ShiftBounds(y, m) }},
		{"ratio-bounds", RatioBounds, func(x, y *Sample, m float64) (Bounds, error) { return x.RatioBounds(y, m) }},
	}
	for _, est := range twoSampleBounds {
		est := est
		t.Run(est.name, func(t *testing.T) {
			forEachFixture(t, est.name, func(t *testing.T, td TestData, input ShiftBoundsInput) {
				entries := []boundsEntry{
					{
						name: "raw",
						run: func(t *testing.T) (Bounds, error, bool) {
							b, err := est.rawFn(input.X, input.Y, input.Misrate, false)
							return b, err, false
						},
					},
					{
						name: "sample",
						run: func(t *testing.T) (Bounds, error, bool) {
							sx, err := sampleX(input.X)
							if err != nil {
								return Bounds{}, err, true
							}
							sy, err := sampleY(input.Y)
							if err != nil {
								return Bounds{}, err, true
							}
							b, err := est.sampFn(sx, sy, input.Misrate)
							return b, err, false
						},
					},
				}
				runBoundsDualPath(t, td, entries)
			})
		})
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
	forEachFixture(t, "center-bounds", func(t *testing.T, td TestData, input OneSampleBoundsInput) {
		entries := []boundsEntry{
			{
				name: "raw",
				run: func(t *testing.T) (Bounds, error, bool) {
					b, err := CenterBounds(input.X, input.Misrate, false)
					return b, err, false
				},
			},
			{
				name: "sample",
				run: func(t *testing.T) (Bounds, error, bool) {
					sx, err := sampleX(input.X)
					if err != nil {
						return Bounds{}, err, true
					}
					b, err := sx.CenterBounds(input.Misrate)
					return b, err, false
				},
			},
		}
		runBoundsDualPath(t, td, entries)
	})
}

func TestSpreadBoundsReference(t *testing.T) {
	forEachFixture(t, "spread-bounds", func(t *testing.T, td TestData, input SpreadBoundsInput) {
		entries := []boundsEntry{
			{
				name: "raw",
				run: func(t *testing.T) (Bounds, error, bool) {
					// The shuffle always runs on the passed order, so
					// assumeSorted never changes the result.
					b, err := SpreadBoundsWithSeed(input.X, input.Misrate, input.Seed, false)
					return b, err, false
				},
			},
			{
				name: "sample",
				run: func(t *testing.T) (Bounds, error, bool) {
					sx, err := sampleX(input.X)
					if err != nil {
						return Bounds{}, err, true
					}
					b, err := sx.SpreadBoundsWithSeed(input.Misrate, input.Seed)
					return b, err, false
				},
			},
		}
		runBoundsDualPath(t, td, entries)
	})
}

// avg-spread-bounds is an internal helper with no public raw/Sample API; it is
// exercised through its internal entry points only (single path).
func TestAvgSpreadBoundsReference(t *testing.T) {
	forEachFixture(t, "avg-spread-bounds", func(t *testing.T, td TestData, input AvgSpreadBoundsInput) {
		entries := []boundsEntry{
			{
				name: "raw",
				run: func(t *testing.T) (Bounds, error, bool) {
					b, err := avgSpreadBoundsImpl(input.X, nil, input.Y, nil, input.Misrate,
						NewRngFromString(input.Seed), NewRngFromString(input.Seed))
					return b, err, false
				},
			},
			{
				name: "sample",
				run: func(t *testing.T) (Bounds, error, bool) {
					sx, err := sampleX(input.X)
					if err != nil {
						return Bounds{}, err, true
					}
					sy, err := sampleY(input.Y)
					if err != nil {
						return Bounds{}, err, true
					}
					b, err := sx.avgSpreadBoundsWithRngs(sy, input.Misrate,
						NewRngFromString(input.Seed), NewRngFromString(input.Seed))
					return b, err, false
				},
			},
		}
		runBoundsDualPath(t, td, entries)
	})
}

func TestDisparityBoundsReference(t *testing.T) {
	forEachFixture(t, "disparity-bounds", func(t *testing.T, td TestData, input DisparityBoundsInput) {
		entries := []boundsEntry{
			{
				name: "raw",
				run: func(t *testing.T) (Bounds, error, bool) {
					b, err := DisparityBoundsWithSeed(input.X, input.Y, input.Misrate, input.Seed, false)
					return b, err, false
				},
			},
			{
				name: "sample",
				run: func(t *testing.T) (Bounds, error, bool) {
					sx, err := sampleX(input.X)
					if err != nil {
						return Bounds{}, err, true
					}
					sy, err := sampleY(input.Y)
					if err != nil {
						return Bounds{}, err, true
					}
					b, err := sx.DisparityBoundsWithSeed(sy, input.Misrate, input.Seed)
					return b, err, false
				},
			},
		}
		runBoundsDualPath(t, td, entries)
	})
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
			if s.IsWeighted() != output.IsWeighted {
				t.Errorf("IsWeighted = %v, want %v", s.IsWeighted(), output.IsWeighted)
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
				_, err := sx.Center()
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
				m, err := sx.Center()
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
				m, err := sx.Spread()
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
				sy, err := newSample(input.Y, nil, yUnit)
				if err != nil {
					t.Fatalf("Failed to create sample Y: %v", err)
				}
				m, err := sx.Shift(sy)
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
				sy, err := newSample(input.Y, nil, yUnit)
				if err != nil {
					t.Fatalf("Failed to create sample Y: %v", err)
				}
				m, err := sx.Ratio(sy)
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
				sy, err := newSample(input.Y, nil, yUnit)
				if err != nil {
					t.Fatalf("Failed to create sample Y: %v", err)
				}
				m, err := sx.Disparity(sy)
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

// Compare1Input represents input for compare1 tests
type Compare1Input struct {
	X          []float64 `json:"x"`
	Seed       string    `json:"seed"`
	Thresholds []struct {
		Metric  string  `json:"metric"`
		Value   float64 `json:"value"`
		Misrate float64 `json:"misrate"`
	} `json:"thresholds"`
}

// Compare2Input represents input for compare2 tests
type Compare2Input struct {
	X          []float64 `json:"x"`
	Y          []float64 `json:"y"`
	Seed       string    `json:"seed"`
	Thresholds []struct {
		Metric  string  `json:"metric"`
		Value   float64 `json:"value"`
		Misrate float64 `json:"misrate"`
	} `json:"thresholds"`
}

// ProjectionOutput represents expected projection output
type ProjectionOutput struct {
	Estimate float64 `json:"estimate"`
	Lower    float64 `json:"lower"`
	Upper    float64 `json:"upper"`
	Verdict  string  `json:"verdict"`
}

// CompareOutput represents expected output for compare tests
type CompareOutput struct {
	Projections []ProjectionOutput `json:"projections"`
}

// mustParseMetric parses a metric string into Metric type
func mustParseMetric(t *testing.T, s string) Metric {
	t.Helper()
	switch s {
	case "center":
		return MetricCenter
	case "spread":
		return MetricSpread
	case "shift":
		return MetricShift
	case "ratio":
		return MetricRatio
	case "disparity":
		return MetricDisparity
	default:
		t.Fatalf("Unknown metric: %q", s)
		return -1
	}
}

// mustParseVerdict parses a verdict string into ComparisonVerdict type
func mustParseVerdict(t *testing.T, s string) ComparisonVerdict {
	t.Helper()
	switch s {
	case "less":
		return VerdictLess
	case "greater":
		return VerdictGreater
	case "inconclusive":
		return VerdictInconclusive
	default:
		t.Fatalf("Unknown verdict: %q", s)
		return -1
	}
}

func TestCompare1Reference(t *testing.T) {
	dirPath := filepath.Join("../tests", "compare1")
	files, err := os.ReadDir(dirPath)
	if err != nil {
		t.Fatalf("Test data directory not found for compare1: %v", err)
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

			var input Compare1Input
			if err := json.Unmarshal(testData.Input, &input); err != nil {
				t.Fatalf("Failed to parse input data: %v", err)
			}

			if len(testData.ExpectedError) > 0 {
				sx, sErr := NewSample(input.X)
				if sErr != nil {
					// Sample construction error counts as expected error
					return
				}

				thresholds := make([]*Threshold, len(input.Thresholds))
				for i, th := range input.Thresholds {
					thresholds[i] = &Threshold{
						Metric:  mustParseMetric(t, th.Metric),
						Value:   NewNumberMeasurement(th.Value),
						Misrate: th.Misrate,
					}
				}

				_, err := Compare1WithSeed(sx, thresholds, input.Seed)
				if err == nil {
					t.Errorf("Expected error for Compare1, but got none")
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

			var expected CompareOutput
			if err := json.Unmarshal(testData.Output, &expected); err != nil {
				t.Fatalf("Failed to parse output data: %v", err)
			}

			sx := mustSample(t, input.X)

			thresholds := make([]*Threshold, len(input.Thresholds))
			for i, th := range input.Thresholds {
				thresholds[i] = &Threshold{
					Metric:  mustParseMetric(t, th.Metric),
					Value:   NewNumberMeasurement(th.Value),
					Misrate: th.Misrate,
				}
			}

			actual, err := Compare1WithSeed(sx, thresholds, input.Seed)
			if err != nil {
				t.Fatalf("Compare1 error: %v", err)
			}

			if len(actual) != len(expected.Projections) {
				t.Fatalf("Expected %d projections, got %d", len(expected.Projections), len(actual))
			}

			for i, proj := range actual {
				exp := expected.Projections[i]
				if !floatEquals(proj.Estimate.Value, exp.Estimate, 1e-9) {
					t.Errorf("Projection %d: Estimate = %v, want %v", i, proj.Estimate.Value, exp.Estimate)
				}
				if !floatEquals(proj.Bounds.Lower, exp.Lower, 1e-9) {
					t.Errorf("Projection %d: Lower = %v, want %v", i, proj.Bounds.Lower, exp.Lower)
				}
				if !floatEquals(proj.Bounds.Upper, exp.Upper, 1e-9) {
					t.Errorf("Projection %d: Upper = %v, want %v", i, proj.Bounds.Upper, exp.Upper)
				}
				if proj.Verdict != mustParseVerdict(t, exp.Verdict) {
					t.Errorf("Projection %d: Verdict = %v, want %v", i, proj.Verdict, exp.Verdict)
				}
			}
		})
	}
}

func TestCompare2Reference(t *testing.T) {
	dirPath := filepath.Join("../tests", "compare2")
	files, err := os.ReadDir(dirPath)
	if err != nil {
		t.Fatalf("Test data directory not found for compare2: %v", err)
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

			var input Compare2Input
			if err := json.Unmarshal(testData.Input, &input); err != nil {
				t.Fatalf("Failed to parse input data: %v", err)
			}

			if len(testData.ExpectedError) > 0 {
				sx, sxErr := NewSample(input.X)
				if sxErr != nil {
					return
				}
				sy, syErr := newSample(input.Y, nil, nil)
				if syErr != nil {
					return
				}

				thresholds := make([]*Threshold, len(input.Thresholds))
				for i, th := range input.Thresholds {
					thresholds[i] = &Threshold{
						Metric:  mustParseMetric(t, th.Metric),
						Value:   NewNumberMeasurement(th.Value),
						Misrate: th.Misrate,
					}
				}

				_, err := Compare2WithSeed(sx, sy, thresholds, input.Seed)
				if err == nil {
					t.Errorf("Expected error for Compare2, but got none")
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

			var expected CompareOutput
			if err := json.Unmarshal(testData.Output, &expected); err != nil {
				t.Fatalf("Failed to parse output data: %v", err)
			}

			sx := mustSample(t, input.X)
			sy, err := newSample(input.Y, nil, nil)
			if err != nil {
				t.Fatalf("Failed to create sample Y: %v", err)
			}

			thresholds := make([]*Threshold, len(input.Thresholds))
			for i, th := range input.Thresholds {
				thresholds[i] = &Threshold{
					Metric:  mustParseMetric(t, th.Metric),
					Value:   NewNumberMeasurement(th.Value),
					Misrate: th.Misrate,
				}
			}

			actual, err := Compare2WithSeed(sx, sy, thresholds, input.Seed)
			if err != nil {
				t.Fatalf("Compare2 error: %v", err)
			}

			if len(actual) != len(expected.Projections) {
				t.Fatalf("Expected %d projections, got %d", len(expected.Projections), len(actual))
			}

			for i, proj := range actual {
				exp := expected.Projections[i]
				if !floatEquals(proj.Estimate.Value, exp.Estimate, 1e-9) {
					t.Errorf("Projection %d: Estimate = %v, want %v", i, proj.Estimate.Value, exp.Estimate)
				}
				if !floatEquals(proj.Bounds.Lower, exp.Lower, 1e-9) {
					t.Errorf("Projection %d: Lower = %v, want %v", i, proj.Bounds.Lower, exp.Lower)
				}
				if !floatEquals(proj.Bounds.Upper, exp.Upper, 1e-9) {
					t.Errorf("Projection %d: Upper = %v, want %v", i, proj.Bounds.Upper, exp.Upper)
				}
				if proj.Verdict != mustParseVerdict(t, exp.Verdict) {
					t.Errorf("Projection %d: Verdict = %v, want %v", i, proj.Verdict, exp.Verdict)
				}
			}
		})
	}
}
