//! Multiplicative (Log-Normal) distribution.

use crate::Rng;

use crate::additive_cumulative::additive_cumulative;
use crate::additive_quantile::additive_quantile;
use crate::exp_function::exp_function;
use crate::log_function::log_function;

use super::{Additive, Distribution, SQRT_2PI, sampling_cumulance};

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
        self.quantile_at(sampling_cumulance(rng))
    }

    /// The support is `x > 0`, and that is the test. It used to be `x < 1e-9`, which is not the
    /// edge of the support but an arbitrary point inside it: a `Multiplic::new(-30.0, 1.0)` has
    /// its median at 9.4e-14, so that threshold reported zero density and zero probability across
    /// essentially the whole distribution.
    fn density_at(&self, x: f64) -> f64 {
        if x <= 0.0 {
            return 0.0;
        }
        let log_std_dev = self.additive.std_dev();
        let z = (log_function(x) - self.additive.mean()) / log_std_dev;
        exp_function(-z * z / 2.0) / (x * log_std_dev * SQRT_2PI)
    }

    /// A lognormal variable's distribution function is the normal one applied to log x, so this
    /// goes through `additive_cumulative` rather than spelling the same thing out through erf.
    fn cumulative_at(&self, x: f64) -> f64 {
        if x <= 0.0 {
            return 0.0;
        }
        additive_cumulative((log_function(x) - self.additive.mean()) / self.additive.std_dev())
    }

    fn quantile_at(&self, p: f64) -> f64 {
        if p <= 0.0 {
            return 0.0;
        }
        if p >= 1.0 {
            return f64::INFINITY;
        }
        exp_function(self.additive.mean() + self.additive.std_dev() * additive_quantile(p))
    }
}
