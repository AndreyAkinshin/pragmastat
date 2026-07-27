package pragmastat

import (
	"testing"
)

func TestSortedValuesMutationSafety(t *testing.T) {
	s, err := NewSample([]float64{5, 3, 1, 4, 2})
	if err != nil {
		t.Fatalf("NewSample failed: %v", err)
	}

	center1, err := s.Center()
	if err != nil {
		t.Fatalf("Center failed: %v", err)
	}

	// Mutate returned SortedValues — must not affect internal state
	sv := s.SortedValues()
	sv[0] = 999

	center2, err := s.Center()
	if err != nil {
		t.Fatalf("Center failed after mutation: %v", err)
	}
	// The same sample re-evaluated by the same kernel: identical to the last
	// bit, or the returned slice aliased the internal cache.
	if !sameFloatBits(center1.Value, center2.Value) {
		t.Errorf("Center changed after SortedValues mutation: %s -> %s",
			formatFloatBits(center1.Value), formatFloatBits(center2.Value))
	}
}

func TestWeightsMutationSafety(t *testing.T) {
	weights := []float64{1.0, 2.0, 3.0}
	s, err := NewWeightedSample([]float64{10, 20, 30}, weights, nil)
	if err != nil {
		t.Fatalf("NewWeightedSample failed: %v", err)
	}

	// Mutate the caller's weights slice — the constructor must have taken a
	// defensive copy, so the sample's stored weights stay unchanged.
	weights[0] = 999.0

	want := []float64{1.0, 2.0, 3.0}
	assertBitsUnchanged(t, "sample weights", s.Weights(), want)

	// Verify Weights() also returns a copy
	w := s.Weights()
	w[0] = 888.0
	w2 := s.Weights()
	if sameFloatBits(w2[0], 888.0) {
		t.Error("Weights() returned internal reference instead of copy")
	}
}
