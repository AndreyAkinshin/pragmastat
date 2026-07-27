//go:build conformance

package pragmastat

import (
	"fmt"
	"math"
	"os"
	"strconv"
	"testing"
)

// The conformance probe. It is not a test: it prints one tagged line per estimator
// evaluation, with every floating-point value written as an exact binary64 payload.
// `mise run tests:check:conformance` runs it twice, once against this package and once
// against a copy whose library calls are perturbed, and compares the two outputs.
//
// It lives behind a build tag so it never runs in the ordinary suite, and it is written
// as a test rather than a command so it can reach the unexported margins.
//
// Go stands in for all seven implementations here. The question the probe answers is
// whether an estimator's result depends on a library function the specification does not
// fix, and that is a property of the algorithm rather than of the language.

// probeOut is where the rows go. Writing to a file rather than to stdout keeps the output
// independent of how the test runner decides to buffer, prefix or suppress it.
var probeOut *os.File

func probeBits(v float64) string { return strconv.FormatFloat(v, 'b', -1, 64) }

func probePrintf(format string, args ...interface{}) {
	fmt.Fprintf(probeOut, format, args...)
}

func probeScalar(tag string, v float64, err error) {
	if err != nil {
		probePrintf("%s\tERR\n", tag)
		return
	}
	probePrintf("%s\t%s\n", tag, probeBits(v))
}

func probeBounds(tag string, b Bounds, err error) {
	if err != nil {
		probePrintf("%s\tERR\n", tag)
		return
	}
	probePrintf("%s\t%s\t%s\n", tag, probeBits(b.Lower), probeBits(b.Upper))
}

func probeInt(tag string, v int, err error) {
	if err != nil {
		probePrintf("%s\tERR\n", tag)
		return
	}
	probePrintf("%s\t%d\n", tag, v)
}

// probeMisrates returns the misrates worth asking about for a given pair of sizes: a few
// ordinary ones, plus the achievable floor and its first few neighbours.
//
// The floor is where this matters. There the admissibility test compares a quantity
// against itself computed by another route, so it is an exact tie in exact arithmetic and
// rounding alone decides it. A sweep of round misrates never lands there, which is
// precisely why this class of divergence stayed invisible for so long.
func probeMisrates(n, m int) []float64 {
	out := []float64{1e-3, 1e-2, 0.05, 0.1, 0.5}
	f, err := minAchievableMisrateTwoSample(n, m)
	if err == nil && f > 0 && f < 1 {
		out = append(out, f)
		for k := 0; k < 4; k++ {
			f = math.Nextafter(f, 1.0)
			out = append(out, f)
		}
	}
	return out
}

func TestConformanceProbe(t *testing.T) {
	path := os.Getenv("PRAGMASTAT_PROBE_OUT")
	if path == "" {
		t.Skip("PRAGMASTAT_PROBE_OUT is unset; this probe is driven by tests/conformance_probe.sh")
	}
	f, err := os.Create(path)
	if err != nil {
		t.Fatalf("cannot open the probe output: %v", err)
	}
	defer f.Close()
	probeOut = f

	sizes := []int{3, 4, 5, 6, 8, 11, 16, 23, 32, 47, 64, 89, 128, 180, 256}
	seeds := []int64{1, 2, 3, 5, 7, 11, 13, 17, 42, 99, 1234, 31337}

	for _, n := range sizes {
		for _, seed := range seeds {
			// Raw draws with no arithmetic applied: any expression here would be
			// evaluated by both builds and could itself differ, which would compare
			// two different samples rather than two arithmetics.
			r := NewRngFromSeed(seed)
			x := make([]float64, n)
			y := make([]float64, n)
			for i := range x {
				x[i] = r.UniformFloat64()
				y[i] = r.UniformFloat64()
			}
			k := fmt.Sprintf("n%d.s%d", n, seed)

			v, err := Center(x, false)
			probeScalar(k+"/center", v, err)
			v, err = Spread(x, false)
			probeScalar(k+"/spread", v, err)
			v, err = Shift(x, y, false)
			probeScalar(k+"/shift", v, err)
			v, err = Ratio(x, y, false)
			probeScalar(k+"/ratio", v, err)
			v, err = Disparity(x, y, false)
			probeScalar(k+"/disparity", v, err)
			v, err = avgSpread(x, y, false)
			probeScalar(k+"/avgSpread", v, err)

			if f, err := minAchievableMisrateTwoSample(n, n); err == nil {
				probePrintf("%s/misrateFloor\t%s\n", k, probeBits(f))
			}

			for _, mis := range probeMisrates(n, n) {
				mk := fmt.Sprintf("%s.m%s", k, probeBits(mis))
				b, err := CenterBounds(x, mis, false)
				probeBounds(mk+"/centerBounds", b, err)
				b, err = SpreadBoundsWithSeed(x, mis, "probe", false)
				probeBounds(mk+"/spreadBounds", b, err)
				b, err = ShiftBounds(x, y, mis, false)
				probeBounds(mk+"/shiftBounds", b, err)
				b, err = RatioBounds(x, y, mis, false)
				probeBounds(mk+"/ratioBounds", b, err)
				b, err = DisparityBoundsWithSeed(x, y, mis, "probe", false)
				probeBounds(mk+"/disparityBounds", b, err)
				pm, pmErr := pairwiseMargin(n, n, mis)
				probeInt(mk+"/pairwiseMargin", pm, pmErr)
				sm, smErr := signedRankMargin(n, mis)
				probeInt(mk+"/signedRankMargin", sm, smErr)
			}
		}
	}
}
