//! Power (Pareto) distribution.

use crate::Rng;

use super::{Distribution, MACHINE_EPSILON};

/// Power (Pareto) distribution with minimum value and shape parameter.
///
/// Follows a power-law distribution where large values are rare but possible.
///
/// # Example
/// ```
/// use pragmastat::{Rng, distributions::{Distribution, Power}};
///
/// let mut rng = Rng::from_string("demo-dist-power");
/// let dist = Power::new(1.0, 2.0);  // min=1, shape=2
/// let sample = dist.sample(&mut rng);
/// assert!(sample >= 1.0);
/// ```
#[derive(Debug, Clone, Copy)]
pub struct Power {
    min: f64,
    shape: f64,
}

impl Power {
    /// Create a new power (Pareto) distribution.
    ///
    /// # Panics
    /// Panics if `min <= 0` or `shape <= 0`.
    pub fn new(min: f64, shape: f64) -> Self {
        assert!(min > 0.0, "min must be positive");
        assert!(shape > 0.0, "shape must be positive");
        Self { min, shape }
    }
}

impl Distribution for Power {
    fn sample(&self, rng: &mut Rng) -> f64 {
        // Inverse CDF method: min / (1 - U)^(1/shape)
        let u = rng.uniform();
        // Avoid division by zero - use machine epsilon for cross-language consistency
        let u = if u == 1.0 { 1.0 - MACHINE_EPSILON } else { u };
        self.min / (1.0 - u).powf(1.0 / self.shape)
    }
}
