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
	if _, err := fmt.Fprintf(probeOut, format, args...); err != nil {
		panic(err)
	}
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
// ordinary ones, the achievable floor with its first few neighbors, and the misrates that
// sit exactly on an Edgeworth decision.
//
// The floor is where this matters. There the admissibility test compares a quantity
// against itself computed by another route, so it is an exact tie in exact arithmetic and
// rounding alone decides it. A sweep of round misrates never lands there, which is
// precisely why this class of divergence stayed invisible for so long.
//
// The Edgeworth decisions are the same argument applied to the other search. Both margins
// binary-search for the largest index whose expansion stays below the misrate, so feeding
// back an expansion value produces a comparison that exact arithmetic leaves tied. The
// floor and the crossover are separate boundaries and covering one does not cover the
// other: this probe reported every margin unmoved while the crossover was returning a
// different integer in three of the seven ports.
func probeMisrates(n, m int) []float64 {
	out := []float64{1e-3, 1e-2, 0.05, 0.1, 0.5}
	f, err := minAchievableMisrateTwoSample(n, m)
	if err == nil && f > 0 && f < 1 {
		out = append(out, f)
		for range 4 {
			f = math.Nextafter(f, 1.0)
			out = append(out, f)
		}
	}
	out = append(out, probeEdgeworthMisrates(n, m)...)
	return out
}

// probeEdgeworthMisrates returns misrates that land on an Edgeworth decision boundary.
//
// Each margin doubles a one-sided search, so the misrate that ties the comparison at index
// i is twice the expansion there. Values outside (0, 1) are dropped: they are ties the
// domain check rejects before the search runs, so they measure nothing.
func probeEdgeworthMisrates(n, m int) []float64 {
	var out []float64
	add := func(v float64) {
		if v > 0 && v < 1 {
			out = append(out, v, math.Nextafter(v, 0), math.Nextafter(v, 1))
		}
	}
	for _, q := range []float64{0.05, 0.1, 0.2, 0.3, 0.4} {
		if n > signedRankMaxExactSize {
			w := int64(q * float64(n) * float64(n+1) / 2)
			add(2 * signedRankEdgeworthCdf(n, w))
		}
		if n+m > maxExactSize {
			add(2 * edgeworthCdf(n, m, int64(q*float64(n)*float64(m))))
		}
	}
	return out
}

// probeCumulances spans the endpoints, the midpoint, and cumulances that fall strictly inside an
// interval rather than on one of its edges.
var probeCumulances = []float64{0.0, 0.125, 0.25, 0.375, 0.5, 0.625, 0.75, 0.875, 1.0}

// probeFamilies evaluates the five parametric families: the three functions that define each one,
// and the draw that is now derived from the third.
//
// Their exposure is different in kind from the core estimators'. A family evaluates a closed
// formula, so a moved library value changes the answer directly, in every last bit it touches.
// That makes this the cheaper thing to get wrong and the easier thing to detect: any surviving
// call to the platform's exp, log, pow or cos shows up on nearly every probe rather than on the
// few that sit on a boundary.
//
// The draw is probed alongside the trio because it is the same code path now: every family
// samples by inverting its own quantile.
func probeFamilies(k string, seed int64) {
	families := []struct {
		name string
		dist Distribution
		at   []float64
	}{
		{"uniform", NewUniform(-2.5, 7.5), []float64{-2.5, -1.0, 0.0, 2.5, 7.5}},
		{"exp", NewExp(0.5), []float64{0.0, 0.25, 1.0, 4.0, 16.0}},
		{"additive", NewAdditive(1.0, 2.0), []float64{-8.0, -1.0, 1.0, 3.0, 9.0}},
		{"multiplic", NewMultiplic(-1.0, 0.5), []float64{0.0625, 0.25, 1.0, 4.0, 16.0}},
		{"power", NewPower(2.0, 3.0), []float64{2.0, 2.5, 4.0, 16.0, 256.0}},
	}

	for _, f := range families {
		for _, v := range f.at {
			vk := fmt.Sprintf("%s.v%s", k, probeBits(v))
			probePrintf("%s/%sDensity\t%s\n", vk, f.name, probeBits(f.dist.DensityAt(v)))
			probePrintf("%s/%sCumulative\t%s\n", vk, f.name, probeBits(f.dist.CumulativeAt(v)))
		}
		// The cumulances are literals for the reason the queries above are: a computed one would
		// be computed by both builds and could differ on its own.
		for i, c := range probeCumulances {
			if c <= 0.0 || c >= 1.0 {
				// Every family's QuantileAt short-circuits at these two exact cumulances --
				// Min, 0, or an infinity, none of them computed -- so no perturbed library call
				// sits on this path for any family, Uniform's finite Min/Max included.
				continue
			}
			probePrintf("%s.c%d/%sQuantile\t%s\n", k, i, f.name, probeBits(f.dist.QuantileAt(c)))
		}
		// One seeded draw per family: the identity under test is that this is the quantile of a
		// drawn cumulance, so a library call reaching it would move here too.
		r := NewRngFromSeed(seed)
		probePrintf("%s/%sSample\t%s\n", k, f.name, probeBits(f.dist.Sample(r)))
	}
}

// probeDistModels evaluates the two distribution-estimation reconstructions, LinearDistModel and
// NeighborDistModel. Neither reaches a library function the specification leaves free --
// construction is sorting and arithmetic, and neighbor's only irrational step is sqrt, which IEEE
// 754 fixes -- so both are expected to report moved: 0. They are probed anyway, for the same
// reason the five families are: the manifest's exact class is a measured claim, not an assumed
// one, and every suite that claims it needs a row here to be measured rather than asserted.
func probeDistModels(k string, x []float64, seed int64) {
	models := []struct {
		name  string
		build func([]float64) (distQueryModel, error)
	}{
		{"linear", func(v []float64) (distQueryModel, error) { return NewLinearDistModel(v, false) }},
		{"neighbor", func(v []float64) (distQueryModel, error) { return NewNeighborDistModel(v, false) }},
	}

	for _, m := range models {
		d, err := m.build(x)
		if err != nil {
			probePrintf("%s/%sQuantile\tERR\n", k, m.name)
			probePrintf("%s/%sCumulative\tERR\n", k, m.name)
			probePrintf("%s/%sDensity\tERR\n", k, m.name)
			probePrintf("%s/%sSample\tERR\n", k, m.name)
			continue
		}
		for i, c := range probeCumulances {
			if c <= 0.0 || c >= 1.0 {
				// Both models' QuantileAt short-circuits at these two exact cumulances -- a
				// support endpoint, not computed -- so no perturbed library call sits on this
				// path either, the same reason probeFamilies skips them.
				continue
			}
			vals, qErr := d.Quantile([]float64{c})
			if qErr != nil {
				probePrintf("%s.c%d/%sQuantile\tERR\n", k, i, m.name)
				continue
			}
			probePrintf("%s.c%d/%sQuantile\t%s\n", k, i, m.name, probeBits(vals[0]))
		}
		// Cumulative and Density are asked about the model's own interval boundaries, not
		// arbitrary points. Both models answer by searching for the interval a query falls in,
		// and that search only decides differently when the query sits on a boundary: a point
		// mid-cell stays in that cell however the boundary moved. Probing at the sample's own
		// values instead -- which this probe once did -- reported both models unmoved against a
		// deliberately planted exp(log(v)) round-trip in NeighborDistModel's search, because the
		// sample's raw values are LinearDistModel's own knots but sit mid-cell for
		// NeighborDistModel, whose boundaries fall between observations.
		//
		// The boundary value goes into the key, not only into the query: a perturbation that
		// moves a boundary then produces a key the other build does not have, which the
		// comparison counts as a divergence rather than silently comparing two different
		// questions.
		for _, v := range probeEdges(distBoundaries(d)) {
			vk := fmt.Sprintf("%s.b%s", k, probeBits(v))
			cum, cErr := d.Cumulative([]float64{v})
			if cErr != nil {
				probePrintf("%s/%sCumulative\tERR\n", vk, m.name)
			} else {
				probePrintf("%s/%sCumulative\t%s\n", vk, m.name, probeBits(cum[0]))
			}
			den, dErr := d.Density([]float64{v})
			if dErr != nil {
				probePrintf("%s/%sDensity\tERR\n", vk, m.name)
			} else {
				probePrintf("%s/%sDensity\t%s\n", vk, m.name, probeBits(den[0]))
			}
		}
		// One seeded draw, for the same reason probeFamilies takes one: the draw is the
		// quantile of a drawn cumulance, the same code path a library call would also reach.
		if sampler, ok := d.(distSampleModel); ok {
			r := NewRngFromSeed(seed)
			s := sampler.Samples(r, 1)
			probePrintf("%s/%sSample\t%s\n", k, m.name, probeBits(s[0]))
		}
	}
}

// distBoundaries returns the interval boundaries probeDistModels asks Cumulative and Density
// about: LinearDistModel interpolates between order statistics, so its sorted sample is its
// boundary list; NeighborDistModel exposes its cell boundaries directly.
func distBoundaries(d distQueryModel) []float64 {
	switch model := d.(type) {
	case *LinearDistModel:
		return model.x
	case *NeighborDistModel:
		return model.boundaries
	default:
		return nil
	}
}

// probeEdges thins a boundary list to at most nine entries, keeping both ends. The endpoints
// carry the closed-left open-right convention and the last-interval exception, so they are the
// two that must never be dropped.
func probeEdges(values []float64) []float64 {
	if len(values) <= 9 {
		return values
	}
	out := make([]float64, 0, 9)
	last := len(values) - 1
	for i := range 8 {
		out = append(out, values[i*last/8])
	}
	return append(out, values[last])
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
	defer func() {
		if err := f.Close(); err != nil {
			t.Errorf("cannot close the probe output: %v", err)
		}
	}()
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

			probeFamilies(k, seed)
			probeDistModels(k, x, seed)

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
