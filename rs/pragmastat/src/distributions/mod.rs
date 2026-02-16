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

/// Machine epsilon for IEEE 754 double-precision (binary64).
///
/// Value: 2^(-52) ≈ 2.220446049250313e-16
///
/// This is the smallest ε such that 1.0 + ε ≠ 1.0 in float64 arithmetic.
/// Represents the distance between 1.0 and the next representable number.
///
/// Used to avoid log(0) or division by zero when uniform_f64() returns exactly 1.0.
/// All language implementations use this same value to ensure cross-language
/// determinism in distribution sampling.
const MACHINE_EPSILON: f64 = 2.220446049250313e-16;

/// Smallest positive subnormal (denormalized) IEEE 754 double-precision value.
///
/// Value: 2^(-1074) ≈ 4.94e-324, represented as 5e-324 for cross-language consistency.
///
/// This is the smallest positive value representable in IEEE 754 binary64 format.
/// Unlike machine epsilon (which is the smallest ε where 1+ε ≠ 1), this is the
/// absolute smallest positive number before underflow to zero.
///
/// Used to avoid log(0) in Box-Muller transform when uniform_f64() returns exactly 0.
/// All language implementations use this same value to ensure cross-language
/// determinism in distribution sampling.
const SMALLEST_POSITIVE_SUBNORMAL: f64 = 5e-324;

mod additive;
mod distribution;
mod exp;
mod multiplic;
mod power;
mod uniform;

pub use additive::Additive;
pub use distribution::Distribution;
pub use exp::Exp;
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
