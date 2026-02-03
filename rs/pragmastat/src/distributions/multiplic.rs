//! Multiplicative (Log-Normal) distribution.

use crate::Rng;

use super::{Additive, Distribution};

/// Multiplicative (Log-Normal) distribution.
///
/// The logarithm of samples follows an Additive (Normal) distribution.
///
/// # Example
/// ```
/// use pragmastat::{Rng, distributions::{Distribution, Multiplic}};
///
/// let mut rng = Rng::from_string("demo-dist-multiplic");
/// let dist = Multiplic::new(0.0, 1.0);
/// let sample = dist.sample(&mut rng);
/// assert!(sample > 0.0);
/// ```
#[derive(Debug, Clone, Copy)]
pub struct Multiplic {
    additive: Additive,
}

impl Multiplic {
    /// Create a new multiplicative (log-normal) distribution.
    ///
    /// # Panics
    /// Panics if `log_std_dev <= 0`.
    pub fn new(log_mean: f64, log_std_dev: f64) -> Self {
        Self {
            additive: Additive::new(log_mean, log_std_dev),
        }
    }
}

impl Distribution for Multiplic {
    fn sample(&self, rng: &mut Rng) -> f64 {
        self.additive.sample(rng).exp()
    }
}
