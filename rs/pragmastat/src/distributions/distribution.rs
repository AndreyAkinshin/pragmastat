//! Distribution trait definition.

use crate::Rng;

/// A trait for distributions that can generate samples.
pub trait Distribution {
    /// Generate a single sample from this distribution.
    fn sample(&self, rng: &mut Rng) -> f64;

    /// Generate multiple samples from this distribution.
    fn samples(&self, rng: &mut Rng, count: usize) -> Vec<f64> {
        (0..count).map(|_| self.sample(rng)).collect()
    }
}
