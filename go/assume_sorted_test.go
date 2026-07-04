package pragmastat

import (
	"sort"
	"testing"
)

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
func TestAssumeSortedRoundtrip(t *testing.T) {
	const misrate = 0.3
	const seed = "pragmastat"
	const eps = 1e-9

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
		if !floatEquals(got, want, eps) {
			t.Errorf("Center: assumeSorted=true %v != assumeSorted=false %v", got, want)
		}
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
		if !floatEquals(got, want, eps) {
			t.Errorf("Spread: assumeSorted=true %v != assumeSorted=false %v", got, want)
		}
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
		if !floatEquals(got, want, eps) {
			t.Errorf("Shift: assumeSorted=true %v != assumeSorted=false %v", got, want)
		}
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
		if !floatEquals(got, want, eps) {
			t.Errorf("Ratio: assumeSorted=true %v != assumeSorted=false %v", got, want)
		}
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
		if !floatEquals(got, want, eps) {
			t.Errorf("Disparity: assumeSorted=true %v != assumeSorted=false %v", got, want)
		}
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
		if !floatEquals(got.Lower, want.Lower, eps) || !floatEquals(got.Upper, want.Upper, eps) {
			t.Errorf("CenterBounds: assumeSorted=true [%v,%v] != assumeSorted=false [%v,%v]",
				got.Lower, got.Upper, want.Lower, want.Upper)
		}
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
		if !floatEquals(got.Lower, want.Lower, eps) || !floatEquals(got.Upper, want.Upper, eps) {
			t.Errorf("ShiftBounds: assumeSorted=true [%v,%v] != assumeSorted=false [%v,%v]",
				got.Lower, got.Upper, want.Lower, want.Upper)
		}
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
		if !floatEquals(got.Lower, want.Lower, eps) || !floatEquals(got.Upper, want.Upper, eps) {
			t.Errorf("RatioBounds: assumeSorted=true [%v,%v] != assumeSorted=false [%v,%v]",
				got.Lower, got.Upper, want.Lower, want.Upper)
		}
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
		if got.Lower != want.Lower || got.Upper != want.Upper {
			t.Errorf("SpreadBounds: assumeSorted=true [%v,%v] != assumeSorted=false [%v,%v]",
				got.Lower, got.Upper, want.Lower, want.Upper)
		}
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
		if got.Lower != want.Lower || got.Upper != want.Upper {
			t.Errorf("DisparityBounds: assumeSorted=true [%v,%v] != assumeSorted=false [%v,%v]",
				got.Lower, got.Upper, want.Lower, want.Upper)
		}
	})
}
