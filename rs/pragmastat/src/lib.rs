//! Pragmastat: A unified statistical toolkit for reliable analysis of real-world data
//!
//! This library provides robust statistical estimators that:
//! - Nearly match the efficiency of traditional statistical estimators under normality
//! - Are robust enough to omit outlier handling completely
//! - Enable simple implementations without advanced statistical libraries
//! - Provide clear explanations accessible to practitioners without deep statistical training

pub mod assumptions;
pub mod bounds;
pub mod distributions;
pub mod estimators;
pub mod measurement;
pub mod measurement_unit;
pub mod sample;
pub mod unit_registry;

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

// Re-exports for convenient access
pub use assumptions::{AssumptionError, AssumptionId, EstimatorError, Subject, Violation};
pub use bounds::Bounds;
pub use distributions::{Additive, Distribution, Exp, Multiplic, Power, Uniform};
pub use estimators::{
    center, center_bounds, disparity, disparity_bounds, disparity_bounds_with_seed, ratio,
    ratio_bounds, shift, shift_bounds, spread, spread_bounds, spread_bounds_with_seed,
    DEFAULT_MISRATE,
};
pub use measurement::Measurement;
pub use measurement_unit::{
    conversion_factor, finer, is_compatible, CustomUnit, DisparityUnit, MeasurementUnit,
    NumberUnit, RatioUnit, UnitMismatchError,
};
pub use rng::Rng;
pub use sample::Sample;
pub use unit_registry::UnitRegistry;
