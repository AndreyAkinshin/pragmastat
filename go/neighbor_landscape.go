package pragmastat

import "slices"

// NeighborLandscape is the staircase a NeighborDistModel is: cell boundaries, cumulative values,
// masses and heights.
//
// Point evaluation answers what the density is at a value a caller picked; this answers what the
// density is, on the boundaries the model itself uses. Every slice returned is a copy, so
// holding a landscape can never mutate the model it came from.
type NeighborLandscape struct {
	scaleIndex int
	radius     int
	boundaries []float64
	cumulative []float64
	masses     []float64
	densities  []float64
	sources    []int
}

// ScaleIndex returns the smoothing scale index k0 = ceil(sqrt(n)).
func (l *NeighborLandscape) ScaleIndex() int {
	return l.scaleIndex
}

// Radius returns the window radius r = floor(k0 / 2).
func (l *NeighborLandscape) Radius() int {
	return l.radius
}

// Boundaries returns the strictly increasing cell boundaries; length CellCount() + 1.
func (l *NeighborLandscape) Boundaries() []float64 {
	return slices.Clone(l.boundaries)
}

// Cumulative returns the cumulative values at the boundaries, from 0 to exactly 1.
func (l *NeighborLandscape) Cumulative() []float64 {
	return slices.Clone(l.cumulative)
}

// Masses returns the strictly positive cell masses; length CellCount().
func (l *NeighborLandscape) Masses() []float64 {
	return slices.Clone(l.masses)
}

// Densities returns the cell heights, mass divided by width.
func (l *NeighborLandscape) Densities() []float64 {
	return slices.Clone(l.densities)
}

// Sources returns the index of the sorted observation each surviving cell was derived from.
func (l *NeighborLandscape) Sources() []int {
	return slices.Clone(l.sources)
}

// CellCount returns the number of surviving cells.
func (l *NeighborLandscape) CellCount() int {
	return len(l.masses)
}
