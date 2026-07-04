package pragmastat

import (
	"reflect"
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
	if center1.Value != center2.Value {
		t.Errorf("Center changed after SortedValues mutation: %v -> %v", center1.Value, center2.Value)
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
	if got := s.Weights(); !reflect.DeepEqual(got, want) {
		t.Errorf("sample weights aliased the caller's slice: got %v, want %v", got, want)
	}

	// Verify Weights() also returns a copy
	w := s.Weights()
	w[0] = 888.0
	w2 := s.Weights()
	if w2[0] == 888.0 {
		t.Error("Weights() returned internal reference instead of copy")
	}
}
