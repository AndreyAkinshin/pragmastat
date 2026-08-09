package pragmastat

import (
	"encoding/json"
	"fmt"
	"math"
	"testing"
)

// The five parametric families against their shared dist-* fixtures.
//
// Each family answers three questions about itself, and each answer is assembled out of the
// toolkit's own exponential, logarithm, normal cumulative and normal quantile. Nothing on these
// paths reaches a platform transcendental, so the seven ports owe each other identical bits and
// the suites are declared exact in tests/manifest.json: the comparison is by binary64 payload. A
// tolerance would pass on exactly the divergence these suites exist to catch.
//
// The sibling distributions suite stays approximate, because drawing a sample still goes through
// the platform's log, exp, cos and pow. That is a separate change with its own effect on every
// seeded stream.

// distFamilyValueCount is how many values the five suites hold between them: for each family, a
// density and a cumulative at every x and a quantile at every p. It is asserted because a loader
// that finds no files, or four suites out of five, passes every other check in this test.
const distFamilyValueCount = 7800

type distFamilyInput struct {
	Parameters map[string]float64 `json:"parameters"`
	X          []float64          `json:"x"`
	P          []float64          `json:"p"`
}

type distFamilyOutput struct {
	Density    []float64 `json:"density"`
	Cumulative []float64 `json:"cumulative"`
	Quantile   []float64 `json:"quantile"`
}

// parameter reads one constructor argument, failing loudly when a fixture does not carry it.
// A missing key would otherwise read as zero and build a different distribution.
func parameter(t *testing.T, parameters map[string]float64, name string) float64 {
	t.Helper()
	value, ok := parameters[name]
	if !ok {
		t.Fatalf("Fixture has no parameter named %q", name)
	}
	return value
}

// build makes the family the suite names, rather than letting the parameter names decide: two
// families could come to share a spelling, and a fixture read into the wrong family would still
// produce numbers and still be compared.
func build(t *testing.T, suite string, parameters map[string]float64) Distribution {
	t.Helper()
	switch suite {
	case "dist-uniform":
		return NewUniform(parameter(t, parameters, "min"), parameter(t, parameters, "max"))
	case "dist-exp":
		return NewExp(parameter(t, parameters, "rate"))
	case "dist-additive":
		return NewAdditive(parameter(t, parameters, "mean"), parameter(t, parameters, "stdDev"))
	case "dist-multiplic":
		return NewMultiplic(parameter(t, parameters, "logMean"), parameter(t, parameters, "logStdDev"))
	case "dist-power":
		return NewPower(parameter(t, parameters, "min"), parameter(t, parameters, "shape"))
	default:
		t.Fatalf("Unknown suite %q", suite)
		return nil
	}
}

func checkDistFamilySuite(t *testing.T, suite string) int {
	t.Helper()
	checked := 0
	forEachFixture(t, suite, func(t *testing.T, td TestData, input distFamilyInput) {
		var expected distFamilyOutput
		if err := json.Unmarshal(td.Output, &expected); err != nil {
			t.Fatalf("Failed to parse output data: %v", err)
		}
		// A file that parses into empty slices passes every comparison below, which is the one way
		// this test could report success while checking nothing.
		if len(input.X) == 0 || len(input.P) == 0 {
			t.Fatalf("Fixture carries %d x values and %d p values", len(input.X), len(input.P))
		}
		if len(expected.Density) != len(input.X) || len(expected.Cumulative) != len(input.X) {
			t.Fatalf("Fixture has %d x values, %d densities and %d cumulatives",
				len(input.X), len(expected.Density), len(expected.Cumulative))
		}
		if len(expected.Quantile) != len(input.P) {
			t.Fatalf("Fixture has %d p values and %d quantiles", len(input.P), len(expected.Quantile))
		}

		d := build(t, suite, input.Parameters)
		for i, x := range input.X {
			assertFloat(t, compareExact, fmt.Sprintf("DensityAt(%s)", formatFloatBits(x)),
				d.DensityAt(x), expected.Density[i])
			assertFloat(t, compareExact, fmt.Sprintf("CumulativeAt(%s)", formatFloatBits(x)),
				d.CumulativeAt(x), expected.Cumulative[i])
		}
		for i, p := range input.P {
			assertFloat(t, compareExact, fmt.Sprintf("QuantileAt(%s)", formatFloatBits(p)),
				d.QuantileAt(p), expected.Quantile[i])
		}
		checked += 2*len(input.X) + len(input.P)
	})
	return checked
}

func TestDistFamilyReference(t *testing.T) {
	checked := 0
	for _, suite := range []string{"dist-uniform", "dist-exp", "dist-additive", "dist-multiplic", "dist-power"} {
		checked += checkDistFamilySuite(t, suite)
	}
	if checked != distFamilyValueCount {
		t.Errorf("Checked %d values, want %d", checked, distFamilyValueCount)
	}
}

// The arguments the fixtures cannot carry, because JSON writes neither a NaN nor an infinity.
//
// Without this table the fixtures are the only check the trio has, and they sample the interior of
// each support through the family's own quantile. That left two defects alive: Uniform reported a
// perfectly ordinary density for NaN in six ports, since neither half of `x < min || x >= max`
// holds for a NaN and control fell straight through, and R raised an error from thirteen of the
// fifteen functions. Neither is reachable from a stored fixture.
func distFamilies() []struct {
	name         string
	d            Distribution
	lower, upper float64
} {
	return []struct {
		name         string
		d            Distribution
		lower, upper float64
	}{
		{"dist-uniform", NewUniform(0, 1), 0, 1},
		{"dist-exp", NewExp(1), 0, math.Inf(1)},
		{"dist-additive", NewAdditive(0, 1), math.Inf(-1), math.Inf(1)},
		{"dist-multiplic", NewMultiplic(0, 1), 0, math.Inf(1)},
		{"dist-power", NewPower(1, 2), 1, math.Inf(1)},
	}
}

func TestDistFamilyNaNPropagates(t *testing.T) {
	nan := math.NaN()
	for _, f := range distFamilies() {
		if !math.IsNaN(f.d.DensityAt(nan)) {
			t.Errorf("%s: DensityAt(NaN) = %v, want NaN", f.name, f.d.DensityAt(nan))
		}
		if !math.IsNaN(f.d.CumulativeAt(nan)) {
			t.Errorf("%s: CumulativeAt(NaN) = %v, want NaN", f.name, f.d.CumulativeAt(nan))
		}
		if !math.IsNaN(f.d.QuantileAt(nan)) {
			t.Errorf("%s: QuantileAt(NaN) = %v, want NaN", f.name, f.d.QuantileAt(nan))
		}
	}
}

func TestDistFamilyInfinitiesLieOutsideEverySupport(t *testing.T) {
	for _, f := range distFamilies() {
		for _, c := range []struct {
			label string
			got   float64
			want  float64
		}{
			{"DensityAt(-Inf)", f.d.DensityAt(math.Inf(-1)), 0},
			{"DensityAt(+Inf)", f.d.DensityAt(math.Inf(1)), 0},
			{"CumulativeAt(-Inf)", f.d.CumulativeAt(math.Inf(-1)), 0},
			{"CumulativeAt(+Inf)", f.d.CumulativeAt(math.Inf(1)), 1},
		} {
			if c.got != c.want {
				t.Errorf("%s: %s = %v, want %v", f.name, c.label, c.got, c.want)
			}
		}
	}
}

func TestDistFamilyQuantileOutsideTheUnitIntervalReturnsTheEndpoints(t *testing.T) {
	for _, f := range distFamilies() {
		for _, p := range []float64{0, -0.1, math.Inf(-1)} {
			if got := f.d.QuantileAt(p); got != f.lower {
				t.Errorf("%s: QuantileAt(%v) = %v, want %v", f.name, p, got, f.lower)
			}
		}
		for _, p := range []float64{1, 1.1, math.Inf(1)} {
			if got := f.d.QuantileAt(p); got != f.upper {
				t.Errorf("%s: QuantileAt(%v) = %v, want %v", f.name, p, got, f.upper)
			}
		}
	}
}

// Every family draws by inverting its own quantile.
//
// A fixture says two numbers agreed on the day it was written. This says they cannot disagree:
// the same seed drives a draw and a direct quantile call, and the two must land on the same bits
// for every family. It is what makes "a distribution is three functions" a statement about the
// code rather than about the documentation, and it would fail the moment a family grew a second
// sampling route.
func TestSamplingIsTheQuantileOfADrawnCumulance(t *testing.T) {
	families := []struct {
		name  string
		dist  Distribution
		lifts bool // true for the two families whose QuantileAt(0) is not a sample
	}{
		{"Uniform", NewUniform(-2.5, 7.5), false},
		{"Exp", NewExp(0.5), false},
		{"Additive", NewAdditive(1.0, 2.0), true},
		{"Multiplic", NewMultiplic(-1.0, 0.5), true},
		{"Power", NewPower(2.0, 3.0), false},
	}

	for _, f := range families {
		drawing := NewRngFromString("identity-seed")
		inverting := NewRngFromString("identity-seed")
		for i := range 1000 {
			drawn := f.dist.Sample(drawing)
			// The same cumulance the draw consumed, taken from a generator on the same seed.
			u := inverting.UniformFloat64()
			if f.lifts && u == 0.0 {
				u = smallestPositiveSubnormal
			}
			inverted := f.dist.QuantileAt(u)
			if math.Float64bits(drawn) != math.Float64bits(inverted) {
				t.Fatalf("%s draw %d: Sample gave %v, QuantileAt(%v) gave %v", f.name, i, drawn, u, inverted)
			}
		}
	}
}
