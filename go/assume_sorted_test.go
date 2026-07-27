package pragmastat

import (
	"math"
	"sort"
	"testing"
)

// assertSameBits fails unless got and want carry the same binary64 payload.
//
// The comparison is bitwise rather than ==, because == reads +0 and -0 as equal
// and every NaN as unequal: two routes into the same kernel have to agree on
// the payload, not merely on the numeric reading. The message prints both raw
// payloads, since a one-ULP disagreement is invisible in the decimal rendering
// and being able to read it is the entire point of comparing bitwise.
func assertSameBits(t *testing.T, label string, got, want float64) {
	t.Helper()
	if math.Float64bits(got) != math.Float64bits(want) {
		t.Errorf("%s: assumeSorted=true %s != assumeSorted=false %s",
			label, formatFloatBits(got), formatFloatBits(want))
	}
}

// assertBoundsSameBits applies assertSameBits to both ends of an interval.
func assertBoundsSameBits(t *testing.T, label string, got, want Bounds) {
	t.Helper()
	assertSameBits(t, label+" lower", got.Lower, want.Lower)
	assertSameBits(t, label+" upper", got.Upper, want.Upper)
}

// TestAssumeSortedRoundtrip directly exercises the raw API's assumeSorted=true
// branch. The dual-path reference tests only ever pass assumeSorted=false; the
// =true branch is reached only transitively via Sample. This adds a direct
// check.
//
// For ORDER-INDEPENDENT estimators, sorting the input ascending and calling
// with assumeSorted=true must equal the call on the unsorted input with
// assumeSorted=false.
//
// For SHUFFLE-based bounds, assumeSorted only affects the internal sparity
// check, never the shuffle, so the result must be IDENTICAL for true vs false
// on the SAME unsorted slice with the SAME seed.
//
// Every assertion here is EXACT, down to the binary64 payload. The flag decides
// only whether the kernel sorts a copy or reads the caller's already-sorted
// buffer; either way the kernel then sees the same sorted data and runs the
// same sequence of operations, so the two calls agree to the last bit or the
// flag changed the arithmetic, which is a defect. That holds for Ratio and
// RatioBounds too: both sides take the same trip through log and exp, so the
// libm results cancel exactly. A tolerance here would express no numerical
// fact, only a window in which a real bug goes unreported.
func TestAssumeSortedRoundtrip(t *testing.T) {
	const misrate = 0.3
	const seed = "pragmastat"

	x := []float64{3, 1, 2, 5, 4, 8, 6, 7}
	y := []float64{9, 11, 10, 13, 12, 16, 14, 15}

	sortedX := append([]float64(nil), x...)
	sort.Float64s(sortedX)
	sortedY := append([]float64(nil), y...)
	sort.Float64s(sortedY)

	// --- Order-independent scalar estimators ---

	t.Run("Center", func(t *testing.T) {
		want, err := Center(x, false)
		if err != nil {
			t.Fatal(err)
		}
		got, err := Center(sortedX, true)
		if err != nil {
			t.Fatal(err)
		}
		assertSameBits(t, "Center", got, want)
	})

	t.Run("Spread", func(t *testing.T) {
		want, err := Spread(x, false)
		if err != nil {
			t.Fatal(err)
		}
		got, err := Spread(sortedX, true)
		if err != nil {
			t.Fatal(err)
		}
		assertSameBits(t, "Spread", got, want)
	})

	t.Run("Shift", func(t *testing.T) {
		want, err := Shift(x, y, false)
		if err != nil {
			t.Fatal(err)
		}
		got, err := Shift(sortedX, sortedY, true)
		if err != nil {
			t.Fatal(err)
		}
		assertSameBits(t, "Shift", got, want)
	})

	t.Run("Ratio", func(t *testing.T) {
		want, err := Ratio(x, y, false)
		if err != nil {
			t.Fatal(err)
		}
		got, err := Ratio(sortedX, sortedY, true)
		if err != nil {
			t.Fatal(err)
		}
		assertSameBits(t, "Ratio", got, want)
	})

	t.Run("Disparity", func(t *testing.T) {
		want, err := Disparity(x, y, false)
		if err != nil {
			t.Fatal(err)
		}
		got, err := Disparity(sortedX, sortedY, true)
		if err != nil {
			t.Fatal(err)
		}
		assertSameBits(t, "Disparity", got, want)
	})

	// --- Order-independent bounds estimators ---

	t.Run("CenterBounds", func(t *testing.T) {
		want, err := CenterBounds(x, misrate, false)
		if err != nil {
			t.Fatal(err)
		}
		got, err := CenterBounds(sortedX, misrate, true)
		if err != nil {
			t.Fatal(err)
		}
		assertBoundsSameBits(t, "CenterBounds", got, want)
	})

	t.Run("ShiftBounds", func(t *testing.T) {
		want, err := ShiftBounds(x, y, misrate, false)
		if err != nil {
			t.Fatal(err)
		}
		got, err := ShiftBounds(sortedX, sortedY, misrate, true)
		if err != nil {
			t.Fatal(err)
		}
		assertBoundsSameBits(t, "ShiftBounds", got, want)
	})

	t.Run("RatioBounds", func(t *testing.T) {
		want, err := RatioBounds(x, y, misrate, false)
		if err != nil {
			t.Fatal(err)
		}
		got, err := RatioBounds(sortedX, sortedY, misrate, true)
		if err != nil {
			t.Fatal(err)
		}
		assertBoundsSameBits(t, "RatioBounds", got, want)
	})

	// --- Shuffle-based bounds: assumeSorted is INERT only on SORTED input.
	// The shuffle always runs on the original order regardless of the flag, so
	// the only flag-sensitive step is the internal sparity check, which runs
	// spreadImpl(x, assumeSorted). With assumeSorted=true that kernel REQUIRES a
	// sorted slice; on UNSORTED input it is UNDEFINED BEHAVIOR (may hit the
	// iteration cap and ERROR, or pass by luck) — exactly like every other
	// estimator. So the flag is inert only when the slice is genuinely sorted:
	// then the shuffle order is identical for both calls and the only difference
	// is whether the (valid) sorted view is reused. ---

	t.Run("SpreadBounds", func(t *testing.T) {
		want, err := SpreadBoundsWithSeed(sortedX, misrate, seed, false)
		if err != nil {
			t.Fatal(err)
		}
		got, err := SpreadBoundsWithSeed(sortedX, misrate, seed, true)
		if err != nil {
			t.Fatal(err)
		}
		assertBoundsSameBits(t, "SpreadBounds", got, want)
	})

	// NOTE: There is deliberately NO "SpreadBounds inert on UNSORTED input" test.
	// On unsorted input with assumeSorted=true the sparity check feeds unsorted
	// data to the sorted-only spreadImpl kernel, which is UNDEFINED BEHAVIOR: it
	// may hit the iteration cap and ERROR, or pass by luck for a particular
	// input. The only well-defined inertness is on a SORTED slice (covered by the
	// SpreadBounds subtest above).

	t.Run("DisparityBounds", func(t *testing.T) {
		want, err := DisparityBoundsWithSeed(sortedX, sortedY, misrate, seed, false)
		if err != nil {
			t.Fatal(err)
		}
		got, err := DisparityBoundsWithSeed(sortedX, sortedY, misrate, seed, true)
		if err != nil {
			t.Fatal(err)
		}
		assertBoundsSameBits(t, "DisparityBounds", got, want)
	})
}
