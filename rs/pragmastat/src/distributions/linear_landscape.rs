//! The staircase a [`LinearDistModel`](super::LinearDistModel) is.

/// The staircase a [`LinearDistModel`](super::LinearDistModel) is: cell boundaries, cumulative
/// values and heights.
///
/// The model interpolates the order statistics linearly, so its density is a staircase: one cell
/// between each pair of adjacent observations, each carrying the same mass `1/(n-1)` and
/// therefore a height that is that mass over its width.
///
/// Every accessor returns a borrow of the model's own copy, so building a landscape allocates
/// nothing beyond it and holding one can never mutate it.
pub struct LinearLandscape {
    boundaries: Vec<f64>,
    cumulative: Vec<f64>,
    densities: Vec<f64>,
}

impl LinearLandscape {
    pub(crate) fn new(boundaries: Vec<f64>, cumulative: Vec<f64>, densities: Vec<f64>) -> Self {
        Self {
            boundaries,
            cumulative,
            densities,
        }
    }

    /// The sorted observations, which are the cell boundaries; length `cell_count() + 1`.
    pub fn boundaries(&self) -> &[f64] {
        &self.boundaries
    }

    /// Cumulative values at the boundaries, from 0 to exactly 1.
    pub fn cumulative(&self) -> &[f64] {
        &self.cumulative
    }

    /// Cell heights, mass divided by width; length `cell_count()`.
    pub fn densities(&self) -> &[f64] {
        &self.densities
    }

    /// Number of cells, one fewer than the number of observations.
    pub fn cell_count(&self) -> usize {
        self.densities.len()
    }
}
