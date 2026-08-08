//! Uniform distribution.

use crate::Rng;

use super::Distribution;

/// Uniform distribution on `[min, max)`.
///
/// # Example
/// ```
/// use pragmastat::{Rng, distributions::{Distribution, Uniform}};
///
/// let mut rng = Rng::from_string("demo-dist-uniform");
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
    /// Panics if `min` or `max` is not finite, or if `min` is not less than `max`.
    pub fn new(min: f64, max: f64) -> Self {
        assert!(min.is_finite(), "min must be finite");
        assert!(max.is_finite(), "max must be finite");
        assert!(min < max, "min must be less than max");
        Self { min, max }
    }
}

impl Distribution for Uniform {
    fn sample(&self, rng: &mut Rng) -> f64 {
        self.min + rng.uniform_f64() * (self.max - self.min)
    }

    /// The support is half-open, so the density is zero at `max` and positive at `min`. This is
    /// the one family assembled from arithmetic alone: no primitive of the toolkit appears on any
    /// of its three paths.
    fn density_at(&self, x: f64) -> f64 {
        // NaN is refused before the bounds are compared. This is the only one of the fifteen
        // functions whose result does not depend on its argument, so a NaN reaching the
        // comparison satisfies neither half of it and falls through to an ordinary density.
        if x.is_nan() {
            return x;
        }
        if x < self.min || x >= self.max {
            return 0.0;
        }
        1.0 / (self.max - self.min)
    }

    fn cumulative_at(&self, x: f64) -> f64 {
        if x < self.min {
            return 0.0;
        }
        if x >= self.max {
            return 1.0;
        }
        (x - self.min) / (self.max - self.min)
    }

    fn quantile_at(&self, p: f64) -> f64 {
        if p <= 0.0 {
            return self.min;
        }
        if p >= 1.0 {
            return self.max;
        }
        self.min + p * (self.max - self.min)
    }
}
