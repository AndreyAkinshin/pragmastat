//! Additive (Normal/Gaussian) distribution.

use crate::Rng;

use crate::additive_cumulative::additive_cumulative;
use crate::additive_quantile::additive_quantile;
use crate::exp_function::exp_function;

use super::{Distribution, SQRT_2PI, sampling_cumulance};

/// Additive (Normal/Gaussian) distribution with given mean and standard deviation.
///
/// Draws by inverse transform, through the toolkit's own inverse normal CDF.
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
    /// Panics if `mean` is not finite, or if `std_dev` is not finite and positive.
    /// The location parameter, readable inside the crate so Multiplic can spell its own density
    /// in the same order of operations the other six ports spell it in. Reassociating the divisor
    /// would change the last bit, and these suites are compared by payload.
    pub(crate) fn mean(&self) -> f64 {
        self.mean
    }

    /// The scale parameter. Same reason as `mean`.
    pub(crate) fn std_dev(&self) -> f64 {
        self.std_dev
    }

    pub fn new(mean: f64, std_dev: f64) -> Self {
        assert!(mean.is_finite(), "mean must be finite");
        assert!(
            std_dev.is_finite() && std_dev > 0.0,
            "std_dev must be finite and positive"
        );
        Self { mean, std_dev }
    }
}

impl Distribution for Additive {
    fn sample(&self, rng: &mut Rng) -> f64 {
        self.quantile_at(sampling_cumulance(rng))
    }

    fn density_at(&self, x: f64) -> f64 {
        let z = (x - self.mean) / self.std_dev;
        exp_function(-z * z / 2.0) / (self.std_dev * SQRT_2PI)
    }

    fn cumulative_at(&self, x: f64) -> f64 {
        additive_cumulative((x - self.mean) / self.std_dev)
    }

    /// The inverse goes through `additive_quantile`, which is what that function was built for. It
    /// replaces two Newton steps taken against an error function accurate to 1.5e-07, which had
    /// left this quantile eight orders of magnitude coarser than the cumulative beside it. The
    /// infinities at the ends are the ones `additive_quantile` already returns.
    fn quantile_at(&self, p: f64) -> f64 {
        self.mean + self.std_dev * additive_quantile(p)
    }
}
