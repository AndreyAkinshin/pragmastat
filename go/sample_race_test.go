package pragmastat

import (
	"sync"
	"testing"
)

// TestSampleConcurrentEstimators shares a single *Sample across many goroutines
// that all trigger the lazy sorted-values cache at once. Run with -race, this
// fails if computeSorted is unsynchronized.
func TestSampleConcurrentEstimators(t *testing.T) {
	s, err := NewSample([]float64{5, 3, 1, 4, 2, 9, 7, 6, 8, 10})
	if err != nil {
		t.Fatalf("NewSample: %v", err)
	}

	const goroutines = 64
	var wg sync.WaitGroup
	start := make(chan struct{})
	wg.Add(goroutines)
	for i := 0; i < goroutines; i++ {
		go func() {
			defer wg.Done()
			<-start // release all goroutines simultaneously to widen the race window
			if _, err := s.Center(); err != nil {
				t.Errorf("Center: %v", err)
			}
			if _, err := s.Spread(); err != nil {
				t.Errorf("Spread: %v", err)
			}
			_ = s.SortedValues()
		}()
	}
	close(start)
	wg.Wait()
}
