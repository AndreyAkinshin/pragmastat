//! Uniform distribution.

use crate::Rng;

use super::Distribution;

/// Uniform distribution on `[min, max)`.
///
/// # Example
/// ```
/// use pragmastat::{Rng, distributions::{Distribution, Uniform}};
///
/// let mut rng = Rng::from_seed(1729);
/// let dist = Uniform::new(0.0, 10.0);
/// let sample = dist.sample(&mut rng);
/// assert!(sample >= 0.0 && sample < 10.0);
/// ```
#[derive(Debug, Clone, Copy)]
pub struct Uniform {
    min: f64,
    max: f64,
}

impl Uniform {
    /// Create a new uniform distribution on `[min, max)`.
    ///
    /// # Panics
    /// Panics if `min >= max`.
    pub fn new(min: f64, max: f64) -> Self {
        assert!(min < max, "min must be less than max");
        Self { min, max }
    }
}

impl Distribution for Uniform {
    fn sample(&self, rng: &mut Rng) -> f64 {
        self.min + rng.uniform() * (self.max - self.min)
    }
}
