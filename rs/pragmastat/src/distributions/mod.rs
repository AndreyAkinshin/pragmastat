//! Statistical distributions for sampling
//!
//! This module provides five distributions for generating random samples:
//! - [`Uniform`]: uniform distribution on a bounded interval
//! - [`Additive`]: normal (Gaussian) distribution
//! - [`Multiplic`]: log-normal distribution
//! - [`Exp`]: exponential distribution
//! - [`Power`]: Pareto (power-law) distribution
//!
//! All distributions produce identical sequences across all Pragmastat language
//! implementations when using the same seed.

/// Smallest positive subnormal (denormalized) IEEE 754 double-precision value.
///
/// Value: 2^(-1074) ≈ 4.94e-324, represented as 5e-324 for cross-language consistency.
///
/// This is the smallest positive value representable in IEEE 754 binary64 format.
/// Unlike machine epsilon (which is the smallest ε where 1+ε ≠ 1), this is the
/// absolute smallest positive number before underflow to zero.
///
/// Used by `sampling_cumulance` to lift a drawn cumulance off zero. All language implementations
/// use this same value to ensure cross-language determinism in distribution sampling.
const SMALLEST_POSITIVE_SUBNORMAL: f64 = 5e-324;

/// The cumulance [`Additive`] and [`Multiplic`] consume, the two families whose `quantile_at(0)`
/// is not a sample: Additive returns negative infinity there, Multiplic returns the zero its own
/// support excludes. The other three families' `quantile_at(0)` is the finite value their
/// support already includes, so they draw `uniform_f64` directly and never call this.
///
/// A drawn 0 is lifted to the smallest positive subnormal. `uniform_f64` never returns 1, so the
/// other endpoint needs no guard.
fn sampling_cumulance(rng: &mut crate::Rng) -> f64 {
    let u = rng.uniform_f64();
    if u == 0.0 {
        SMALLEST_POSITIVE_SUBNORMAL
    } else {
        u
    }
}

/// `sqrt(2 * pi)`, the normalizing constant of the normal density.
///
/// A literal rather than `(2.0 * PI).sqrt()`: the seven ports must divide by the same double, and
/// a runtime square root of a runtime product is one more place for them to differ.
const SQRT_2PI: f64 = 2.506628274631000241612355239340;

mod additive;
mod distribution;
mod exp;
mod linear_dist_model;
mod linear_landscape;
mod measured_dist;
mod multiplic;
mod power;
mod uniform;

pub use additive::Additive;
pub use distribution::Distribution;
pub use exp::Exp;
pub use linear_dist_model::{LINEAR_DIST_MODEL_ALGORITHM, LinearDistModel};
pub use linear_landscape::LinearLandscape;
pub use measured_dist::{MeasuredDist, MeasuredModel};
pub use multiplic::Multiplic;
pub use power::Power;
pub use uniform::Uniform;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Rng;

    #[test]
    fn uniform_bounds() {
        let mut rng = Rng::from_string("test-dist-uniform");
        let dist = Uniform::new(5.0, 10.0);
        for _ in 0..100 {
            let x = dist.sample(&mut rng);
            assert!(x >= 5.0 && x < 10.0);
        }
    }

    #[test]
    fn additive_basic() {
        let mut rng = Rng::from_string("test-dist-additive");
        let dist = Additive::new(100.0, 10.0);
        let samples: Vec<f64> = (0..1000).map(|_| dist.sample(&mut rng)).collect();
        let mean: f64 = samples.iter().sum::<f64>() / samples.len() as f64;
        // Should be roughly 100, with some tolerance
        assert!((mean - 100.0).abs() < 1.0);
    }

    #[test]
    fn multiplic_positive() {
        let mut rng = Rng::from_string("test-dist-multiplic");
        let dist = Multiplic::new(0.0, 1.0);
        for _ in 0..100 {
            let x = dist.sample(&mut rng);
            assert!(x > 0.0);
        }
    }

    #[test]
    fn exp_positive() {
        let mut rng = Rng::from_string("test-dist-exp");
        let dist = Exp::new(1.0);
        for _ in 0..100 {
            let x = dist.sample(&mut rng);
            assert!(x >= 0.0);
        }
    }

    #[test]
    fn power_bounds() {
        let mut rng = Rng::from_string("test-dist-power");
        let dist = Power::new(5.0, 2.0);
        for _ in 0..100 {
            let x = dist.sample(&mut rng);
            assert!(x >= 5.0);
        }
    }
}
