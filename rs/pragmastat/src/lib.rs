//! Pragmastat: A unified statistical toolkit for reliable analysis of real-world data
//!
//! This library provides robust statistical estimators that:
//! - Nearly match the efficiency of traditional statistical estimators under normality
//! - Are robust enough to omit outlier handling completely
//! - Enable simple implementations without advanced statistical libraries
//! - Provide clear explanations accessible to practitioners without deep statistical training

// `a.mul_add(b, c)` is not an optimization of `a * b + c` here, it is a different algorithm:
// it rounds once where the specification rounds twice. Every estimator in this toolkit is
// defined as a fixed sequence of binary64 operations that seven implementations must reproduce,
// and none of the other six has a fused multiply-add anywhere. Taking this lint's advice once
// already shipped a divergence: it rewrote `uniform_f64_range`, and Rust drew different numbers
// from the same seed as the other six for months, on every platform, while a 1e-12 comparison
// reported it as a pass. The randomization suites now compare bitwise, which would catch it
// again, but the lint should not be able to propose it in the first place.
#![allow(clippy::suboptimal_flops)]

pub mod assumptions;
pub mod bounds;
pub mod compare;
pub mod distributions;
pub mod estimators;
pub mod measurement;
pub mod measurement_unit;
pub mod sample;
pub mod unit_registry;

pub(crate) mod binomial;
pub(crate) mod gauss_cdf;
pub(crate) mod min_misrate;
pub(crate) mod pairwise_margin;
pub mod rng;
pub(crate) mod sign_margin;
pub(crate) mod signed_rank_margin;

// Internal algorithm implementations
mod center_impl;
mod center_quantiles_impl;
mod shift_impl;
mod spread_impl;

mod fnv1a;
mod splitmix64;
mod xoshiro256;

#[cfg(test)]
mod avg_spread_bounds_tests;
#[cfg(test)]
mod avg_spread_tests;
#[cfg(test)]
mod conformance;
#[cfg(test)]
mod disparity_bounds_tests;
#[cfg(test)]
mod pairwise_margin_tests;
#[cfg(test)]
mod ratio_bounds_tests;
#[cfg(test)]
mod signed_rank_margin_tests;

// Re-exports for convenient access
pub use assumptions::{AssumptionError, AssumptionId, EstimatorError, Subject, Violation};
pub use bounds::Bounds;
pub use compare::{
    ComparisonVerdict, Metric, Projection, Threshold, compare1, compare1_with_seed, compare2,
    compare2_with_seed,
};
pub use distributions::{Additive, Distribution, Exp, Multiplic, Power, Uniform};
pub use estimators::{
    DEFAULT_MISRATE, center, center_bounds, disparity, disparity_bounds,
    disparity_bounds_with_seed, ratio, ratio_bounds, shift, shift_bounds, spread, spread_bounds,
    spread_bounds_with_seed,
};
pub use measurement::Measurement;
pub use measurement_unit::{
    MeasurementUnit, UnitMismatchError, conversion_factor, finer, is_compatible,
};
pub use rng::Rng;
pub use sample::Sample;
pub use unit_registry::UnitRegistry;
