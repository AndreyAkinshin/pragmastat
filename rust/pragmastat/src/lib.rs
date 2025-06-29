//! Pragmastat: A unified statistical toolkit for reliable analysis of real-world data
//!
//! This library provides robust statistical estimators that:
//! - Nearly match the efficiency of traditional statistical estimators under normality
//! - Are robust enough to omit outlier handling completely
//! - Enable simple implementations without advanced statistical libraries
//! - Provide clear explanations accessible to practitioners without deep statistical training

pub mod estimators;

pub use estimators::{
    center, med_disparity, med_ratio, med_shift, med_spread, precision, spread, volatility,
};
