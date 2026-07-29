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

	// One-sample scalar estimators: center, spread. Both select their result
	// out of the pairwise set, so both compare bit for bit.
	oneSampleScalar := []struct {
		name   string
		rawFn  func(x []float64, assumeSorted bool) (float64, error)
		sampFn func(x *Sample) (Measurement, error)
	}{
		{"center", Center, (*Sample).Center},
		{"spread", Spread, (*Sample).Spread},
	}
	for _, est := range oneSampleScalar {
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
				runScalarDualPath(t, td, compareExact, entries)
			})
		})
	}

	// Two-sample scalar estimators: shift, ratio, disparity (public).
	//
	// ratio is the one tolerant member: it is exp(median(log x - log y)), so the
	// result is a libm approximation rather than an element selected out of the
	// pairwise set. shift and disparity select, and compare bit for bit.
	twoSampleScalar := []struct {
		name   string
		mode   compareMode
		rawFn  func(x, y []float64, assumeSorted bool) (float64, error)
		sampFn func(x, y *Sample) (Measurement, error)
	}{
		{"shift", compareExact, Shift, func(x, y *Sample) (Measurement, error) { return x.Shift(y) }},
		{"ratio", compareTolerant, Ratio, func(x, y *Sample) (Measurement, error) { return x.Ratio(y) }},
		{"disparity", compareExact, Disparity, func(x, y *Sample) (Measurement, error) { return x.Disparity(y) }},
	}
	for _, est := range twoSampleScalar {
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
				runScalarDualPath(t, td, est.mode, entries)
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
			runScalarDualPath(t, td, compareExact, entries)
		})
	})

	// Two-sample bounds estimators (deterministic): shift-bounds, ratio-bounds.
	// Both bounds are order statistics of the pairwise set, so shift-bounds is
	// exact; ratio-bounds carries the same libm exponentiation as ratio.
	twoSampleBounds := []struct {
		name   string
		mode   compareMode
		rawFn  func(x, y []float64, misrate float64, assumeSorted bool) (Bounds, error)
		sampFn func(x, y *Sample, misrate float64) (Bounds, error)
	}{
		{"shift-bounds", compareExact, ShiftBounds,
			func(x, y *Sample, m float64) (Bounds, error) { return x.ShiftBounds(y, m) }},
		{"ratio-bounds", compareTolerant, RatioBounds,
			func(x, y *Sample, m float64) (Bounds, error) { return x.RatioBounds(y, m) }},
	}
	for _, est := range twoSampleBounds {
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
				runBoundsDualPath(t, td, est.mode, entries)
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
			// Bitwise, not tolerant. The randomization contract is that a seeded
			// stream is identical in every language, so "close enough" is not the
			// property under test: a one-ULP drift here is a broken contract, and a
			// tolerance would report it as a pass. This is what catches an FMA
			// contraction on an arm64 runner, where the compiler is free to fuse a
			// multiply into an add and change the last bit of a draw.
			for i := range testData.Input.Count {
				actual := rng.UniformFloat64()
				expected := testData.Output[i]
				assertFloat(t, compareExact, fmt.Sprintf("UniformFloat64() at index %d", i), actual, expected)
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
			for i := range testData.Input.Count {
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
			for i := range testData.Input.Count {
				actual := rng.UniformFloat64()
				expected := testData.Output[i]
				assertFloat(t, compareExact, fmt.Sprintf("UniformFloat64() at index %d", i), actual, expected)
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
			for i := range testData.Input.Count {
				actual := rng.UniformFloat64Range(testData.Input.Min, testData.Input.Max)
				expected := testData.Output[i]
				assertFloat(t, compareExact,
					fmt.Sprintf("UniformFloat64Range(%v, %v) at index %d",
						testData.Input.Min, testData.Input.Max, i),
					actual, expected)
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
			for i := range testData.Input.Count {
				actual := rng.UniformFloat32()
				expected := testData.Output[i]
				assertExactFloat32(t, fmt.Sprintf("UniformFloat32() at index %d", i), actual, expected)
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
			for i := range testData.Input.Count {
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
			for i := range testData.Input.Count {
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

			// A permutation carries the input values through untouched, so an
			// inexactness here would be a wrong element, not a rounding error.
			assertExactSequence(t, "RngShuffle()", actual, testData.Output)
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

			// Selection without replacement: every element is an input value
			// carried through untouched.
			assertExactSequence(t, "RngSample()", actual, testData.Output)
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

			// Selection with replacement: same argument as RngSample.
			assertExactSequence(t, "RngResample()", actual, testData.Output)
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

			for i := range testData.Input.Count {
				actual := dist.Sample(rng)
				expected := testData.Output[i]
				assertFloat(t, compareExact, fmt.Sprintf("Uniform sample at index %d", i), actual, expected)
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

			for i := range testData.Input.Count {
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

			for i := range testData.Input.Count {
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

			for i := range testData.Input.Count {
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

			for i := range testData.Input.Count {
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
		runBoundsDualPath(t, td, compareExact, entries)
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
		runBoundsDualPath(t, td, compareExact, entries)
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
		runBoundsDualPath(t, td, compareExact, entries)
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
		runBoundsDualPath(t, td, compareExact, entries)
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
				Size         int      `json:"size"`
				IsWeighted   bool     `json:"is_weighted"`
				TotalWeight  *float64 `json:"total_weight"`
				WeightedSize *float64 `json:"weighted_size"`
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
			// Bitwise. Both are public values derived by summing the weights, and a sum
			// depends on the order it is taken in: floating-point addition is not
			// associative. A tolerance here would accept an implementation that reduces
			// pairwise or accumulates in extended precision, which is exactly the
			// divergence these fields exist to pin.
			if output.TotalWeight != nil {
				assertFloat(t, compareExact, "TotalWeight", s.TotalWeight(), *output.TotalWeight)
			}
			if output.WeightedSize != nil {
				assertFloat(t, compareExact, "WeightedSize", s.WeightedSize(), *output.WeightedSize)
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
				if output.Value != nil {
					assertFloat(t, compareExact, "Value", m.Value, *output.Value)
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

// projectionMode is the comparison one projection is held to, decided by the
// metric of the threshold it answers.
//
// A projection is produced by the bounds estimator of its own metric, so the
// comparison belongs per projection and not per suite. Shift and disparity
// select their result out of the pairwise set and are therefore bit-exact;
// ratio passes through log and exp and is not. A suite-wide mode would have to
// be the weakest one present, which is how a single ratio threshold ends up
// lowering the guarantee on every exact projection standing next to it.
func projectionMode(metric string) compareMode {
	if metric == "ratio" {
		return compareTolerant
	}
	return compareExact
}

// assertProjections compares a comparison result against the fixture's
// projections, each under the mode its own threshold's metric earns.
//
// metrics comes from the fixture's input.thresholds, which the fixture keeps
// aligned with output.projections one-to-one and in order; the order-* fixtures
// exist to pin that alignment. Every field of a projection is in the same
// class as the projection, so estimate, lower and upper share one mode.
func assertProjections(t *testing.T, metrics []string, actual []Projection, expected []ProjectionOutput) {
	t.Helper()
	if len(actual) != len(expected) {
		t.Fatalf("Expected %d projections, got %d", len(expected), len(actual))
	}
	// Reachable only for a self-inconsistent fixture, and worth keeping for exactly that: a
	// case listing three thresholds beside two projections would pass the check above and then
	// silently drop the third, comparing two projections against the wrong two metrics.
	if len(metrics) != len(expected) {
		t.Fatalf("Fixture lists %d thresholds but %d projections", len(metrics), len(expected))
	}
	for i, proj := range actual {
		exp := expected[i]
		mode := projectionMode(metrics[i])
		label := fmt.Sprintf("Projection %d (%s)", i, metrics[i])
		assertFloat(t, mode, label+": Estimate", proj.Estimate.Value, exp.Estimate)
		assertFloat(t, mode, label+": Lower", proj.Bounds.Lower, exp.Lower)
		assertFloat(t, mode, label+": Upper", proj.Bounds.Upper, exp.Upper)
		if proj.Verdict != mustParseVerdict(t, exp.Verdict) {
			t.Errorf("%s: Verdict = %v, want %v", label, proj.Verdict, exp.Verdict)
		}
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
			metrics := make([]string, len(input.Thresholds))
			for i, th := range input.Thresholds {
				thresholds[i] = &Threshold{
					Metric:  mustParseMetric(t, th.Metric),
					Value:   NewNumberMeasurement(th.Value),
					Misrate: th.Misrate,
				}
				metrics[i] = th.Metric
			}

			actual, err := Compare1WithSeed(sx, thresholds, input.Seed)
			if err != nil {
				t.Fatalf("Compare1 error: %v", err)
			}

			// compare1 projects center and spread only, both of which select
			// their result out of the pairwise set, so every projection here
			// resolves to compareExact.
			assertProjections(t, metrics, actual, expected.Projections)
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
			metrics := make([]string, len(input.Thresholds))
			for i, th := range input.Thresholds {
				thresholds[i] = &Threshold{
					Metric:  mustParseMetric(t, th.Metric),
					Value:   NewNumberMeasurement(th.Value),
					Misrate: th.Misrate,
				}
				metrics[i] = th.Metric
			}

			actual, err := Compare2WithSeed(sx, sy, thresholds, input.Seed)
			if err != nil {
				t.Fatalf("Compare2 error: %v", err)
			}

			// compare2 composes ratio projections alongside exact ones, so each
			// projection is compared according to its own threshold's metric.
			assertProjections(t, metrics, actual, expected.Projections)
		})
	}
}

// SingleDoubleValueInput represents input for a suite that evaluates one named
// function at a list of arguments.
type SingleDoubleValueInput struct {
	Name string    `json:"name"`
	Arg  []float64 `json:"arg"`
}

// expFunctionArgCount is how many arguments the exp-function fixtures carry
// between them: 401 over the band exp(-t*t) reaches, 201 over the wider band the
// Edgeworth expansion reaches, 401 over the whole finite range, and 29
// boundaries. It is asserted because a loader that finds no files, or three
// files out of four, passes every other check in this test.
const expFunctionArgCount = 1032

// TestExpFunctionReference checks the reproducible exponential directly.
//
// Every other suite reaches expFunction only through a margin, which evaluates
// it wherever an Edgeworth expansion happens to look and nowhere else. The
// exact class of all those suites rests on this one function agreeing in all
// seven languages, so it is worth checking on its own arguments rather than
// inferring it from the margins that happen to call it.
//
// Bitwise, via the same assertFloat every exact suite uses. A tolerance would
// pass on precisely the divergence this suite exists to catch: the last-bit
// disagreement between two conforming exponentials, which moves a margin by
// selecting a different order statistic.
//
// Only the outputs are compared. boundaries.json carries both 0 and -0 as
// arguments and JSON does not distinguish them, so the parser yields +0 for
// both; both map to exp(0) = 1, and the suite makes no claim about the payload
// of an input. The outputs do reach down to the smallest denormal, and
// comparing those by payload is what shows the parser round-trips them.
func TestExpFunctionReference(t *testing.T) {
	checked := 0
	forEachFixture(t, "exp-function", func(t *testing.T, td TestData, input SingleDoubleValueInput) {
		if input.Name != "exp_function" {
			t.Fatalf("Fixture names function %q, want %q", input.Name, "exp_function")
		}
		var expected []float64
		if err := json.Unmarshal(td.Output, &expected); err != nil {
			t.Fatalf("Failed to parse output data: %v", err)
		}
		if len(expected) != len(input.Arg) {
			t.Fatalf("Fixture has %d arguments and %d outputs", len(input.Arg), len(expected))
		}
		for i, arg := range input.Arg {
			label := fmt.Sprintf("expFunction(%s)", formatFloatBits(arg))
			assertFloat(t, compareExact, label, expFunction(arg), expected[i])
		}
		checked += len(input.Arg)
	})
	if checked != expFunctionArgCount {
		t.Errorf("Checked %d arguments, want %d", checked, expFunctionArgCount)
	}
}
