package pragmastat

import "slices"

// LinearLandscape is the staircase a LinearDistModel is: cell boundaries, cumulative values and
// heights.
//
// The model interpolates the order statistics linearly, so its density is a staircase: one cell
// between each pair of adjacent observations, each carrying the same mass 1/(n-1) and therefore
// a height that is that mass over its width. Every slice returned is a copy, so holding a
// landscape can never mutate the model it came from.
type LinearLandscape struct {
	boundaries []float64
	cumulative []float64
	densities  []float64
}

// Boundaries returns the sorted observations, which are the cell boundaries; length
// CellCount() + 1.
func (l *LinearLandscape) Boundaries() []float64 {
	return slices.Clone(l.boundaries)
}

// Cumulative returns the cumulative values at the boundaries, from 0 to exactly 1.
func (l *LinearLandscape) Cumulative() []float64 {
	return slices.Clone(l.cumulative)
}

// Densities returns the cell heights, mass divided by width; length CellCount().
func (l *LinearLandscape) Densities() []float64 {
	return slices.Clone(l.densities)
}

// CellCount returns the number of cells, one fewer than the number of observations.
func (l *LinearLandscape) CellCount() int {
	return len(l.densities)
}
