//! Additive (Normal/Gaussian) distribution.

use crate::Rng;

use super::{Distribution, SMALLEST_POSITIVE_SUBNORMAL};

/// Additive (Normal/Gaussian) distribution with given mean and standard deviation.
///
/// Uses the Box-Muller transform to generate samples.
///
/// # Example
/// ```
/// use pragmastat::{Rng, distributions::{Distribution, Additive}};
///
/// let mut rng = Rng::from_string("demo-dist-additive");
/// let dist = Additive::new(0.0, 1.0);  // Standard normal
/// let sample = dist.sample(&mut rng);
/// ```
#[derive(Debug, Clone, Copy)]
pub struct Additive {
    mean: f64,
    std_dev: f64,
}

impl Additive {
    /// Create a new additive (normal) distribution.
    ///
    /// # Panics
    /// Panics if `std_dev <= 0`.
    pub fn new(mean: f64, std_dev: f64) -> Self {
        assert!(std_dev > 0.0, "std_dev must be positive");
        Self { mean, std_dev }
    }
}

impl Distribution for Additive {
    fn sample(&self, rng: &mut Rng) -> f64 {
        // Box-Muller transform
        // We use both uniforms each time to maintain determinism across languages
        let u1 = rng.uniform_f64();
        let u2 = rng.uniform_f64();

        // Avoid log(0) - use smallest positive subnormal for cross-language consistency
        let u1 = if u1 == 0.0 {
            SMALLEST_POSITIVE_SUBNORMAL
        } else {
            u1
        };

        let r = (-2.0 * u1.ln()).sqrt();
        let theta = 2.0 * std::f64::consts::PI * u2;

        // Use the first of the two Box-Muller outputs
        let z = r * theta.cos();

        self.mean + z * self.std_dev
    }
}
