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
    /// Panics if `log_mean` is not finite, or if `log_std_dev` is not finite and positive.
    pub fn new(log_mean: f64, log_std_dev: f64) -> Self {
        // Additive::new checks the same two values, but it reports them as `mean` and `std_dev`.
        // These are named for the log scale, and the message has to say which parameter is wrong.
        assert!(log_mean.is_finite(), "log_mean must be finite");
        assert!(
            log_std_dev.is_finite() && log_std_dev > 0.0,
            "log_std_dev must be finite and positive"
        );
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
