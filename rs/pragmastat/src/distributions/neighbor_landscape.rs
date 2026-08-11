//! The staircase a [`NeighborDistModel`](super::NeighborDistModel) is.

/// The staircase a [`NeighborDistModel`](super::NeighborDistModel) is: cell boundaries,
/// cumulative values, masses and heights.
///
/// Point evaluation answers what the density is at a value a caller picked; this answers what
/// the density *is*, on the boundaries the model itself uses.
///
/// Every accessor returns a borrow of the model's own copy, so building a landscape allocates
/// nothing beyond it and holding one can never mutate it.
pub struct NeighborLandscape {
    scale_index: usize,
    radius: usize,
    boundaries: Vec<f64>,
    cumulative: Vec<f64>,
    masses: Vec<f64>,
    densities: Vec<f64>,
    sources: Vec<usize>,
}

impl NeighborLandscape {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(
        scale_index: usize,
        radius: usize,
        boundaries: Vec<f64>,
        cumulative: Vec<f64>,
        masses: Vec<f64>,
        densities: Vec<f64>,
        sources: Vec<usize>,
    ) -> Self {
        Self {
            scale_index,
            radius,
            boundaries,
            cumulative,
            masses,
            densities,
            sources,
        }
    }

    /// Smoothing scale index `k0 = ceil(sqrt(n))`.
    pub fn scale_index(&self) -> usize {
        self.scale_index
    }

    /// Window radius `r = floor(k0 / 2)`.
    pub fn radius(&self) -> usize {
        self.radius
    }

    /// Strictly increasing cell boundaries; length `cell_count() + 1`.
    pub fn boundaries(&self) -> &[f64] {
        &self.boundaries
    }

    /// Cumulative values at the boundaries, from 0 to exactly 1.
    pub fn cumulative(&self) -> &[f64] {
        &self.cumulative
    }

    /// Strictly positive cell masses; length `cell_count()`.
    pub fn masses(&self) -> &[f64] {
        &self.masses
    }

    /// Cell heights, mass divided by width.
    pub fn densities(&self) -> &[f64] {
        &self.densities
    }

    /// Index of the sorted observation each surviving cell was derived from.
    pub fn sources(&self) -> &[usize] {
        &self.sources
    }

    /// Number of surviving cells.
    pub fn cell_count(&self) -> usize {
        self.masses.len()
    }
}
