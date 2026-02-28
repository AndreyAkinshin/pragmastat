//! Pragmastat: A unified statistical toolkit for reliable analysis of real-world data
//!
//! This library provides robust statistical estimators that:
//! - Nearly match the efficiency of traditional statistical estimators under normality
//! - Are robust enough to omit outlier handling completely
//! - Enable simple implementations without advanced statistical libraries
//! - Provide clear explanations accessible to practitioners without deep statistical training

pub mod assumptions;
pub mod distributions;
pub mod estimators;
pub(crate) mod gauss_cdf;
pub(crate) mod min_misrate;
pub(crate) mod pairwise_margin;
pub mod rng;
pub(crate) mod sign_margin;
pub(crate) mod signed_rank_margin;

// Internal fast algorithm implementations
mod fast_center;
mod fast_center_quantiles;
mod fast_shift;
mod fast_spread;

mod fnv1a;
mod splitmix64;
mod xoshiro256;

#[cfg(test)]
mod avg_spread_bounds_tests;
#[cfg(test)]
mod avg_spread_tests;
#[cfg(test)]
mod disparity_bounds_tests;
#[cfg(test)]
mod pairwise_margin_tests;
#[cfg(test)]
mod signed_rank_margin_tests;

pub use assumptions::{AssumptionError, AssumptionId, EstimatorError, Subject, Violation};
pub use distributions::{Additive, Distribution, Exp, Multiplic, Power, Uniform};
#[allow(deprecated)]
pub use estimators::{
    center, center_bounds, disparity, disparity_bounds, disparity_bounds_with_seed, ratio,
    ratio_bounds, rel_spread, shift, shift_bounds, spread, spread_bounds, spread_bounds_with_seed,
    Bounds, DEFAULT_MISRATE,
};

pub use rng::Rng;
