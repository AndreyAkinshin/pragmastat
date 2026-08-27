package pragmastat

import (
	"encoding/json"
	"testing"
)

// LinearDistModel and NeighborDistModel against their shared dist-linear/dist-neighbor
// fixtures.
//
// Unlike the five parametric families, this suite's oracle is not one of the seven ports: it is
// an independent binary64 + exact-Fraction reimplementation (tests/oracles/dist_reference.py),
// written directly from the normative algorithm so a bug shared by every language cannot pass
// itself. Every case is compared by binary64 payload, matching the suite's declared exact class.

// distQueryModel is the surface every dist-* fixture query reaches, shared by both
// reconstructions so the loader below does not need to know which one it is holding.
type distQueryModel interface {
	Quantile(cumulances []float64) ([]float64, error)
	Cumulative(values []float64) ([]float64, error)
	Density(values []float64) ([]float64, error)
}

// distSampleModel is the surface the sample sub-suite reaches.
type distSampleModel interface {
	Samples(rng *Rng, count int) []float64
}

// DistQueryInput is the input shape shared by the quantile/cumulative/density sub-suites. X and
// Y arrive as []interface{}, not []float64: a handful of error fixtures carry a NaN or an
// Infinity, and JSON has no literal for either, so the generator spells them as the strings
// "NaN"/"Infinity"/"-Infinity" -- parseSpecialFloats is what reads them back.
type DistQueryInput struct {
	X            []interface{} `json:"x"`
	Y            []interface{} `json:"y"`
	AssumeSorted bool          `json:"assume_sorted"`
}

// DistSampleInput is the input shape for the sample sub-suite.
type DistSampleInput struct {
	X     []interface{} `json:"x"`
	Seed  string        `json:"seed"`
	Count int           `json:"count"`
}

// DistLandscapeInput is the input shape for dist-neighbor's landscape sub-suite.
type DistLandscapeInput struct {
	X []interface{} `json:"x"`
}

// DistLandscapeOutput is the expected staircase: k0, radius, cell boundaries, cumulative
// values, masses, heights and source indices. The float fields are []interface{} for the same
// reason DistQueryInput's are: a subulp density can be an Infinity.
type DistLandscapeOutput struct {
	K0         int           `json:"k0"`
	Radius     int           `json:"radius"`
	Boundaries []interface{} `json:"boundaries"`
	Cumulative []interface{} `json:"cumulative"`
	Masses     []interface{} `json:"masses"`
	Densities  []interface{} `json:"densities"`
	Sources    []int         `json:"sources"`
}

// buildDistModel constructs the named reconstruction from a raw vector. Both constructors run
// validity(x) before this is reached, and the two remaining assumptions (domain, distinctness)
// inside it, so a build error here is always one of those three.
func buildDistModel(t *testing.T, model string, x []float64, assumeSorted bool) (distQueryModel, error) {
	t.Helper()
	switch model {
	case "linear":
		return NewLinearDistModel(x, assumeSorted)
	case "neighbor":
		return NewNeighborDistModel(x, assumeSorted)
	default:
		t.Fatalf("unknown dist model %q", model)
		return nil, nil
	}
}

// runDistOp dispatches to the operation the fixture directory names.
func runDistOp(t *testing.T, d distQueryModel, op string, y []float64) ([]float64, error) {
	t.Helper()
	switch op {
	case "quantile":
		return d.Quantile(y)
	case "cumulative":
		return d.Cumulative(y)
	case "density":
		return d.Density(y)
	default:
		t.Fatalf("unknown dist operation %q", op)
		return nil, nil
	}
}

// checkDistQuerySuite runs every fixture under dist-<model>/<op>. A case that declares
// expected_error is checked against the first place the model actually raises it -- construction
// for domain/distinctness (and validity, though that never reaches a stored fixture: it fails
// before a case could be written), the query call for cumulance and the y-side of validity.
func checkDistQuerySuite(t *testing.T, model, op string) {
	t.Helper()
	forEachFixture(t, "dist-"+model+"/"+op, func(t *testing.T, td TestData, input DistQueryInput) {
		x := parseSpecialFloats(input.X)
		y := parseSpecialFloats(input.Y)

		if len(td.ExpectedError) > 0 {
			d, buildErr := buildDistModel(t, model, x, input.AssumeSorted)
			if buildErr != nil {
				assertErrorMatches(t, td.ExpectedError, buildErr, true)
				return
			}
			_, queryErr := runDistOp(t, d, op, y)
			assertErrorMatches(t, td.ExpectedError, queryErr, true)
			return
		}

		d, err := buildDistModel(t, model, x, input.AssumeSorted)
		if err != nil {
			t.Fatalf("%s: unexpected build error: %v", model, err)
		}
		actual, err := runDistOp(t, d, op, y)
		if err != nil {
			t.Fatalf("%s: unexpected error: %v", op, err)
		}

		var rawExpected []interface{}
		if err := json.Unmarshal(td.Output, &rawExpected); err != nil {
			t.Fatalf("failed to parse output: %v", err)
		}
		assertExactSequence(t, op, actual, parseSpecialFloats(rawExpected))
	})
}

// checkDistSampleSuite runs every fixture under dist-<model>/sample. No case here declares an
// error: an invalid x would already be covered by the query suites above, and drawing from a
// built model cannot fail on its own.
func checkDistSampleSuite(t *testing.T, model string) {
	t.Helper()
	forEachFixture(t, "dist-"+model+"/sample", func(t *testing.T, td TestData, input DistSampleInput) {
		x := parseSpecialFloats(input.X)
		built, err := buildDistModel(t, model, x, false)
		if err != nil {
			t.Fatalf("%s: unexpected build error: %v", model, err)
		}
		d, ok := built.(distSampleModel)
		if !ok {
			t.Fatalf("%s: model does not implement Samples", model)
		}

		rng := NewRngFromString(input.Seed)
		actual := d.Samples(rng, input.Count)

		var rawExpected []interface{}
		if err := json.Unmarshal(td.Output, &rawExpected); err != nil {
			t.Fatalf("failed to parse output: %v", err)
		}
		assertExactSequence(t, "Samples", actual, parseSpecialFloats(rawExpected))
	})
}

// checkDistNeighborLandscapeSuite runs every fixture under dist-neighbor/landscape. Every case
// here builds successfully by construction: the generator only emits a landscape for a sample
// that already passed the query suites' validity checks.
func checkDistNeighborLandscapeSuite(t *testing.T) {
	t.Helper()
	forEachFixture(t, "dist-neighbor/landscape", func(t *testing.T, td TestData, input DistLandscapeInput) {
		x := parseSpecialFloats(input.X)
		d, err := NewNeighborDistModel(x, false)
		if err != nil {
			t.Fatalf("NewNeighborDistModel: unexpected error: %v", err)
		}
		landscape := d.GetLandscape()

		var expected DistLandscapeOutput
		if err := json.Unmarshal(td.Output, &expected); err != nil {
			t.Fatalf("failed to parse output: %v", err)
		}

		if landscape.ScaleIndex() != expected.K0 {
			t.Errorf("ScaleIndex() = %d, want %d", landscape.ScaleIndex(), expected.K0)
		}
		if landscape.Radius() != expected.Radius {
			t.Errorf("Radius() = %d, want %d", landscape.Radius(), expected.Radius)
		}
		assertExactSequence(t, "Boundaries", landscape.Boundaries(), parseSpecialFloats(expected.Boundaries))
		assertExactSequence(t, "Cumulative", landscape.Cumulative(), parseSpecialFloats(expected.Cumulative))
		assertExactSequence(t, "Masses", landscape.Masses(), parseSpecialFloats(expected.Masses))
		assertExactSequence(t, "Densities", landscape.Densities(), parseSpecialFloats(expected.Densities))

		actualSources := landscape.Sources()
		if len(actualSources) != len(expected.Sources) {
			t.Fatalf("Sources length = %d, want %d", len(actualSources), len(expected.Sources))
		}
		for i := range actualSources {
			if actualSources[i] != expected.Sources[i] {
				t.Errorf("Sources[%d] = %d, want %d", i, actualSources[i], expected.Sources[i])
			}
		}
	})
}

func TestDistLinearReference(t *testing.T) {
	for _, op := range []string{"quantile", "cumulative", "density"} {
		t.Run(op, func(t *testing.T) { checkDistQuerySuite(t, "linear", op) })
	}
	t.Run("sample", func(t *testing.T) { checkDistSampleSuite(t, "linear") })
}

func TestDistNeighborReference(t *testing.T) {
	for _, op := range []string{"quantile", "cumulative", "density"} {
		t.Run(op, func(t *testing.T) { checkDistQuerySuite(t, "neighbor", op) })
	}
	t.Run("sample", func(t *testing.T) { checkDistSampleSuite(t, "neighbor") })
	t.Run("landscape", checkDistNeighborLandscapeSuite)
}
