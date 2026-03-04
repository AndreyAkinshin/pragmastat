package pragmastat

import (
	"strings"
	"testing"
)

func TestCompare1RejectsNilThreshold(t *testing.T) {
	x, err := NewSample([]int{1, 2, 3, 4, 5})
	if err != nil {
		t.Fatalf("failed to create sample x: %v", err)
	}

	_, err = Compare1WithSeed(x, []*Threshold{nil}, "seed")
	if err == nil {
		t.Fatal("expected Compare1WithSeed to fail on nil threshold, but got nil error")
	}
	if !strings.Contains(err.Error(), "thresholds[0] cannot be nil") {
		t.Fatalf("unexpected error message: %v", err)
	}
}

func TestCompare2RejectsNilThreshold(t *testing.T) {
	x, err := NewSample([]int{1, 2, 3, 4, 5})
	if err != nil {
		t.Fatalf("failed to create sample x: %v", err)
	}
	y, err := NewSample([]int{2, 3, 4, 5, 6})
	if err != nil {
		t.Fatalf("failed to create sample y: %v", err)
	}

	_, err = Compare2WithSeed(x, y, []*Threshold{nil}, "seed")
	if err == nil {
		t.Fatal("expected Compare2WithSeed to fail on nil threshold, but got nil error")
	}
	if !strings.Contains(err.Error(), "thresholds[0] cannot be nil") {
		t.Fatalf("unexpected error message: %v", err)
	}
}
