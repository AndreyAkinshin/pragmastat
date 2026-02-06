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
pub mod min_misrate;
pub mod pairwise_margin;
pub mod rng;
pub mod signed_rank_margin;

// Internal fast algorithm implementations
mod fast_center;
mod fast_center_quantiles;
mod fast_shift;
mod fast_spread;

mod fnv1a;
mod splitmix64;
mod xoshiro256;

pub use assumptions::{AssumptionError, AssumptionId, EstimatorError, Subject, Violation};
pub use distributions::{Additive, Distribution, Exp, Multiplic, Power, Uniform};
pub use estimators::{
    avg_spread, center, center_bounds, disparity, median, median_bounds, ratio, ratio_bounds,
    rel_spread, shift, shift_bounds, spread, Bounds,
};

pub use min_misrate::{min_achievable_misrate_one_sample, min_achievable_misrate_two_sample};
pub use pairwise_margin::pairwise_margin;
pub use rng::Rng;
pub use signed_rank_margin::signed_rank_margin;
