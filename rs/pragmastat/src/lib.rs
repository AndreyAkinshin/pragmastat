//! Pragmastat: A unified statistical toolkit for reliable analysis of real-world data
//!
//! This library provides robust statistical estimators that:
//! - Nearly match the efficiency of traditional statistical estimators under normality
//! - Are robust enough to omit outlier handling completely
//! - Enable simple implementations without advanced statistical libraries
//! - Provide clear explanations accessible to practitioners without deep statistical training

pub mod distributions;
pub mod estimators;
pub mod fast_center;
pub mod fast_shift;
pub mod fast_spread;
pub mod pairwise_margin;
pub mod rng;

mod fnv1a;
mod splitmix64;
mod xoshiro256;

pub use distributions::{Additive, Distribution, Exp, Multiplic, Power, Uniform};
pub use estimators::{
    avg_spread, center, disparity, median, ratio, rel_spread, shift, shift_bounds, spread, Bounds,
};
pub use pairwise_margin::pairwise_margin;
pub use rng::Rng;
