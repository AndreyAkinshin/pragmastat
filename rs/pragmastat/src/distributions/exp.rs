//! Exponential distribution.

use crate::Rng;

use crate::exp_function::exp_function;
use crate::log_function::log_function;

use super::Distribution;

/// Exponential distribution with given rate parameter.
///
/// The mean of this distribution is `1/rate`.
///
/// # Example
/// ```
/// use pragmastat::{Rng, distributions::{Distribution, Exp}};
///
/// let mut rng = Rng::from_string("demo-dist-exp");
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
    /// Panics if `rate` is not finite and positive.
    pub fn new(rate: f64) -> Self {
        assert!(
            rate.is_finite() && rate > 0.0,
            "rate must be finite and positive"
        );
        Self { rate }
    }
}

impl Distribution for Exp {
    fn sample(&self, rng: &mut Rng) -> f64 {
        self.quantile_at(rng.uniform_f64())
    }

    fn density_at(&self, x: f64) -> f64 {
        if x < 0.0 {
            return 0.0;
        }
        self.rate * exp_function(-self.rate * x)
    }

    fn cumulative_at(&self, x: f64) -> f64 {
        if x < 0.0 {
            return 0.0;
        }
        1.0 - exp_function(-self.rate * x)
    }

    fn quantile_at(&self, p: f64) -> f64 {
        if p <= 0.0 {
            return 0.0;
        }
        if p >= 1.0 {
            return f64::INFINITY;
        }
        -log_function(1.0 - p) / self.rate
    }
}
