//! Exponential distribution.

use crate::Rng;

use super::{Distribution, MACHINE_EPSILON};

/// Exponential distribution with given rate parameter.
///
/// The mean of this distribution is `1/rate`.
///
/// # Example
/// ```
/// use pragmastat::{Rng, distributions::{Distribution, Exp}};
///
/// let mut rng = Rng::from_seed(1729);
/// let dist = Exp::new(1.0);  // rate = 1, mean = 1
/// let sample = dist.sample(&mut rng);
/// assert!(sample >= 0.0);
/// ```
#[derive(Debug, Clone, Copy)]
pub struct Exp {
    rate: f64,
}

impl Exp {
    /// Create a new exponential distribution with given rate.
    ///
    /// # Panics
    /// Panics if `rate <= 0`.
    pub fn new(rate: f64) -> Self {
        assert!(rate > 0.0, "rate must be positive");
        Self { rate }
    }
}

impl Distribution for Exp {
    fn sample(&self, rng: &mut Rng) -> f64 {
        // Inverse CDF method: -ln(1 - U) / rate
        let u = rng.uniform();
        // Avoid log(0) - use machine epsilon for cross-language consistency
        let u = if u == 1.0 { 1.0 - MACHINE_EPSILON } else { u };
        -(1.0 - u).ln() / self.rate
    }
}
