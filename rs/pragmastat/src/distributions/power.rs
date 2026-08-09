//! Power (Pareto) distribution.

use crate::Rng;

use crate::exp_function::exp_function;
use crate::log_function::log_function;

use super::Distribution;

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
    /// Panics if `min` or `shape` is not finite and positive.
    pub fn new(min: f64, shape: f64) -> Self {
        assert!(
            min.is_finite() && min > 0.0,
            "min must be finite and positive"
        );
        assert!(
            shape.is_finite() && shape > 0.0,
            "shape must be finite and positive"
        );
        Self { min, shape }
    }
}

impl Distribution for Power {
    fn sample(&self, rng: &mut Rng) -> f64 {
        self.quantile_at(rng.uniform_f64())
    }

    /// `shape * min^shape / x^(shape+1)`, rearranged as
    /// `(shape / x) * exp(shape * (log min - log x))`. The toolkit owns an exponential and a
    /// logarithm and owns no general power, so a real exponent is spelled as the exponential of a
    /// logarithm. This family is the reason the logarithm had to exist before the trio could.
    ///
    /// The support is `[min, infinity)`, closed at the left, so the density at `min` is
    /// `shape / min` rather than zero, while the cumulative there is zero. Uniform reads the same
    /// way at its own lower endpoint.
    fn density_at(&self, x: f64) -> f64 {
        if x < self.min {
            return 0.0;
        }
        (self.shape / x) * exp_function(self.shape * (log_function(self.min) - log_function(x)))
    }

    fn cumulative_at(&self, x: f64) -> f64 {
        if x <= self.min {
            return 0.0;
        }
        1.0 - exp_function(self.shape * (log_function(self.min) - log_function(x)))
    }

    fn quantile_at(&self, p: f64) -> f64 {
        if p <= 0.0 {
            return self.min;
        }
        if p >= 1.0 {
            return f64::INFINITY;
        }
        self.min * exp_function(-log_function(1.0 - p) / self.shape)
    }
}
