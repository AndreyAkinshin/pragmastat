//! Statistical estimators for one-sample and two-sample analysis.
//!
//! Public API accepts [`Sample`] and returns [`Measurement`] or [`Bounds`].
//! Raw `&[f64]`-based helpers are available via the `raw` submodule for
//! backward compatibility and internal tests.

use crate::assumptions::{
    check_positivity, check_validity, log, AssumptionError, EstimatorError, Subject,
};
use crate::bounds::Bounds;
use crate::measurement::Measurement;
use crate::measurement_unit::{DisparityUnit, NumberUnit, RatioUnit};
use crate::sample::{check_non_weighted, prepare_pair, Sample};

/// Default misclassification rate for bounds estimators.
pub const DEFAULT_MISRATE: f64 = 1e-3;

// =============================================================================
// Raw (slice-based) estimator functions — backward-compatible internal API
// =============================================================================

/// Raw slice-based estimator functions for backward compatibility and internal tests.
///
/// These accept `&[f64]` and return raw `f64` or the legacy `RawBounds` struct.
pub mod raw {
    use super::*;

    /// Legacy bounds struct without unit (for backward compatibility).
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct RawBounds {
        pub lower: f64,
        pub upper: f64,
    }

    pub fn center(x: &[f64]) -> Result<f64, EstimatorError> {
        check_validity(x, Subject::X)?;
        crate::fast_center::fast_center(x).map_err(EstimatorError::from)
    }

    pub fn spread(x: &[f64]) -> Result<f64, EstimatorError> {
        check_validity(x, Subject::X)?;
        let spread_val = crate::fast_spread::fast_spread(x).map_err(EstimatorError::from)?;
        if spread_val <= 0.0 {
            return Err(EstimatorError::from(AssumptionError::sparity(Subject::X)));
        }
        Ok(spread_val)
    }

    #[deprecated(since = "10.0.0", note = "use spread(x) / center(x).abs() instead")]
    pub fn rel_spread(x: &[f64]) -> Result<f64, EstimatorError> {
        check_validity(x, Subject::X)?;
        check_positivity(x, Subject::X)?;
        let center_val = crate::fast_center::fast_center(x).map_err(EstimatorError::from)?;
        let spread_val = crate::fast_spread::fast_spread(x).map_err(EstimatorError::from)?;
        Ok(spread_val / center_val.abs())
    }

    pub fn shift(x: &[f64], y: &[f64]) -> Result<f64, EstimatorError> {
        check_validity(x, Subject::X)?;
        check_validity(y, Subject::Y)?;
        crate::fast_shift::fast_shift(x, y).map_err(EstimatorError::from)
    }

    pub fn ratio(x: &[f64], y: &[f64]) -> Result<f64, EstimatorError> {
        check_validity(x, Subject::X)?;
        check_validity(y, Subject::Y)?;
        check_positivity(x, Subject::X)?;
        check_positivity(y, Subject::Y)?;
        crate::fast_shift::fast_ratio(x, y).map_err(EstimatorError::from)
    }

    #[cfg(test)]
    pub(crate) fn avg_spread(x: &[f64], y: &[f64]) -> Result<f64, EstimatorError> {
        check_validity(x, Subject::X)?;
        check_validity(y, Subject::Y)?;
        let n = x.len();
        let m = y.len();
        let spread_x = crate::fast_spread::fast_spread(x).map_err(EstimatorError::from)?;
        if spread_x <= 0.0 {
            return Err(EstimatorError::from(AssumptionError::sparity(Subject::X)));
        }
        let spread_y = crate::fast_spread::fast_spread(y).map_err(EstimatorError::from)?;
        if spread_y <= 0.0 {
            return Err(EstimatorError::from(AssumptionError::sparity(Subject::Y)));
        }
        Ok((n as f64 * spread_x + m as f64 * spread_y) / (n + m) as f64)
    }

    pub fn disparity(x: &[f64], y: &[f64]) -> Result<f64, EstimatorError> {
        check_validity(x, Subject::X)?;
        check_validity(y, Subject::Y)?;
        let n = x.len();
        let m = y.len();
        let spread_x = crate::fast_spread::fast_spread(x).map_err(EstimatorError::from)?;
        if spread_x <= 0.0 {
            return Err(EstimatorError::from(AssumptionError::sparity(Subject::X)));
        }
        let spread_y = crate::fast_spread::fast_spread(y).map_err(EstimatorError::from)?;
        if spread_y <= 0.0 {
            return Err(EstimatorError::from(AssumptionError::sparity(Subject::Y)));
        }
        let shift_val = crate::fast_shift::fast_shift(x, y).map_err(EstimatorError::from)?;
        let avg_spread_val = (n as f64 * spread_x + m as f64 * spread_y) / (n + m) as f64;
        Ok(shift_val / avg_spread_val)
    }

    pub fn shift_bounds(x: &[f64], y: &[f64], misrate: f64) -> Result<RawBounds, EstimatorError> {
        check_validity(x, Subject::X)?;
        check_validity(y, Subject::Y)?;
        if misrate.is_nan() || !(0.0..=1.0).contains(&misrate) {
            return Err(EstimatorError::from(AssumptionError::domain(
                Subject::Misrate,
            )));
        }
        let n = x.len();
        let m = y.len();
        let min_misrate = crate::min_misrate::min_achievable_misrate_two_sample(n, m)
            .map_err(EstimatorError::from)?;
        if misrate < min_misrate {
            return Err(EstimatorError::from(AssumptionError::domain(
                Subject::Misrate,
            )));
        }
        let mut xs = x.to_vec();
        let mut ys = y.to_vec();
        xs.sort_by(|a, b| a.total_cmp(b));
        ys.sort_by(|a, b| a.total_cmp(b));
        let total = n as u64 * m as u64;
        if total == 1 {
            let value = xs[0] - ys[0];
            return Ok(RawBounds {
                lower: value,
                upper: value,
            });
        }
        let margin =
            crate::pairwise_margin::pairwise_margin(n, m, misrate).map_err(EstimatorError::from)?;
        let max_half_margin = (total - 1) / 2;
        let mut half_margin = margin / 2;
        if half_margin > max_half_margin {
            half_margin = max_half_margin;
        }
        let k_left = half_margin;
        let k_right = total - 1 - half_margin;
        let denominator = (total - 1) as f64;
        let p = vec![k_left as f64 / denominator, k_right as f64 / denominator];
        let bounds = crate::fast_shift::fast_shift_quantiles(&xs, &ys, &p, true)
            .map_err(EstimatorError::from)?;
        let lower = bounds[0].min(bounds[1]);
        let upper = bounds[0].max(bounds[1]);
        Ok(RawBounds { lower, upper })
    }

    pub fn ratio_bounds(x: &[f64], y: &[f64], misrate: f64) -> Result<RawBounds, EstimatorError> {
        check_validity(x, Subject::X)?;
        check_validity(y, Subject::Y)?;
        if misrate.is_nan() || !(0.0..=1.0).contains(&misrate) {
            return Err(EstimatorError::from(AssumptionError::domain(
                Subject::Misrate,
            )));
        }
        let min_misrate = crate::min_misrate::min_achievable_misrate_two_sample(x.len(), y.len())
            .map_err(EstimatorError::from)?;
        if misrate < min_misrate {
            return Err(EstimatorError::from(AssumptionError::domain(
                Subject::Misrate,
            )));
        }
        let log_x = log(x, Subject::X)?;
        let log_y = log(y, Subject::Y)?;
        let log_bounds = shift_bounds(&log_x, &log_y, misrate)?;
        Ok(RawBounds {
            lower: log_bounds.lower.exp(),
            upper: log_bounds.upper.exp(),
        })
    }

    pub fn center_bounds(x: &[f64], misrate: f64) -> Result<RawBounds, EstimatorError> {
        check_validity(x, Subject::X)?;
        if misrate.is_nan() || !(0.0..=1.0).contains(&misrate) {
            return Err(EstimatorError::from(AssumptionError::domain(
                Subject::Misrate,
            )));
        }
        let n = x.len();
        if n < 2 {
            return Err(EstimatorError::from(AssumptionError::domain(Subject::X)));
        }
        let min_misrate = crate::min_misrate::min_achievable_misrate_one_sample(n)?;
        if misrate < min_misrate {
            return Err(EstimatorError::from(AssumptionError::domain(
                Subject::Misrate,
            )));
        }
        let margin = crate::signed_rank_margin::signed_rank_margin(n, misrate)?;
        let total_pairs = (n as i64) * (n as i64 + 1) / 2;
        let mut half_margin = (margin / 2) as i64;
        let max_half_margin = (total_pairs - 1) / 2;
        if half_margin > max_half_margin {
            half_margin = max_half_margin;
        }
        let k_left = half_margin + 1;
        let k_right = total_pairs - half_margin;
        let mut sorted = x.to_vec();
        sorted.sort_by(|a, b| a.total_cmp(b));
        let (lo, hi) =
            crate::fast_center_quantiles::fast_center_quantile_bounds(&sorted, k_left, k_right);
        Ok(RawBounds {
            lower: lo,
            upper: hi,
        })
    }

    pub fn spread_bounds(x: &[f64], misrate: f64) -> Result<RawBounds, EstimatorError> {
        let mut rng = crate::rng::Rng::new();
        spread_bounds_with_rng(x, misrate, &mut rng)
    }

    pub fn spread_bounds_with_seed(
        x: &[f64],
        misrate: f64,
        seed: &str,
    ) -> Result<RawBounds, EstimatorError> {
        let mut rng = crate::rng::Rng::from_string(seed);
        spread_bounds_with_rng(x, misrate, &mut rng)
    }

    #[cfg(test)]
    pub(crate) fn avg_spread_bounds(
        x: &[f64],
        y: &[f64],
        misrate: f64,
    ) -> Result<RawBounds, EstimatorError> {
        let mut rng_x = crate::rng::Rng::new();
        let mut rng_y = crate::rng::Rng::new();
        avg_spread_bounds_with_rngs(x, y, misrate, &mut rng_x, &mut rng_y)
    }

    #[cfg(test)]
    pub(crate) fn avg_spread_bounds_with_seed(
        x: &[f64],
        y: &[f64],
        misrate: f64,
        seed: &str,
    ) -> Result<RawBounds, EstimatorError> {
        let mut rng_x = crate::rng::Rng::from_string(seed);
        let mut rng_y = crate::rng::Rng::from_string(seed);
        avg_spread_bounds_with_rngs(x, y, misrate, &mut rng_x, &mut rng_y)
    }

    pub(crate) fn avg_spread_bounds_with_rngs(
        x: &[f64],
        y: &[f64],
        misrate: f64,
        rng_x: &mut crate::rng::Rng,
        rng_y: &mut crate::rng::Rng,
    ) -> Result<RawBounds, EstimatorError> {
        check_validity(x, Subject::X)?;
        check_validity(y, Subject::Y)?;
        if misrate.is_nan() || !(0.0..=1.0).contains(&misrate) {
            return Err(EstimatorError::from(AssumptionError::domain(
                Subject::Misrate,
            )));
        }
        let n = x.len();
        let m = y.len();
        if n < 2 {
            return Err(EstimatorError::from(AssumptionError::domain(Subject::X)));
        }
        if m < 2 {
            return Err(EstimatorError::from(AssumptionError::domain(Subject::Y)));
        }
        let mx = n / 2;
        let my = m / 2;
        let min_x = crate::min_misrate::min_achievable_misrate_one_sample(mx)?;
        let min_y = crate::min_misrate::min_achievable_misrate_one_sample(my)?;
        let alpha = misrate / 2.0;
        if alpha < min_x || alpha < min_y {
            return Err(EstimatorError::from(AssumptionError::domain(
                Subject::Misrate,
            )));
        }
        if crate::fast_spread::fast_spread(x).map_err(EstimatorError::from)? <= 0.0 {
            return Err(EstimatorError::from(AssumptionError::sparity(Subject::X)));
        }
        if crate::fast_spread::fast_spread(y).map_err(EstimatorError::from)? <= 0.0 {
            return Err(EstimatorError::from(AssumptionError::sparity(Subject::Y)));
        }
        let bounds_x = spread_bounds_with_rng(x, alpha, rng_x)?;
        let bounds_y = spread_bounds_with_rng(y, alpha, rng_y)?;
        let weight_x = n as f64 / (n + m) as f64;
        let weight_y = m as f64 / (n + m) as f64;
        Ok(RawBounds {
            lower: weight_x * bounds_x.lower + weight_y * bounds_y.lower,
            upper: weight_x * bounds_x.upper + weight_y * bounds_y.upper,
        })
    }

    pub fn disparity_bounds(
        x: &[f64],
        y: &[f64],
        misrate: f64,
    ) -> Result<RawBounds, EstimatorError> {
        let mut rng_x = crate::rng::Rng::new();
        let mut rng_y = crate::rng::Rng::new();
        disparity_bounds_with_rngs(x, y, misrate, &mut rng_x, &mut rng_y)
    }

    pub fn disparity_bounds_with_seed(
        x: &[f64],
        y: &[f64],
        misrate: f64,
        seed: &str,
    ) -> Result<RawBounds, EstimatorError> {
        let mut rng_x = crate::rng::Rng::from_string(seed);
        let mut rng_y = crate::rng::Rng::from_string(seed);
        disparity_bounds_with_rngs(x, y, misrate, &mut rng_x, &mut rng_y)
    }

    fn disparity_bounds_with_rngs(
        x: &[f64],
        y: &[f64],
        misrate: f64,
        rng_x: &mut crate::rng::Rng,
        rng_y: &mut crate::rng::Rng,
    ) -> Result<RawBounds, EstimatorError> {
        check_validity(x, Subject::X)?;
        check_validity(y, Subject::Y)?;
        if misrate.is_nan() || !(0.0..=1.0).contains(&misrate) {
            return Err(EstimatorError::from(AssumptionError::domain(
                Subject::Misrate,
            )));
        }
        let n = x.len();
        let m = y.len();
        if n < 2 {
            return Err(EstimatorError::from(AssumptionError::domain(Subject::X)));
        }
        if m < 2 {
            return Err(EstimatorError::from(AssumptionError::domain(Subject::Y)));
        }
        let min_shift = crate::min_misrate::min_achievable_misrate_two_sample(n, m)
            .map_err(EstimatorError::from)?;
        let min_x = crate::min_misrate::min_achievable_misrate_one_sample(n / 2)?;
        let min_y = crate::min_misrate::min_achievable_misrate_one_sample(m / 2)?;
        let min_avg = 2.0 * min_x.max(min_y);
        if misrate < min_shift + min_avg {
            return Err(EstimatorError::from(AssumptionError::domain(
                Subject::Misrate,
            )));
        }
        let extra = misrate - (min_shift + min_avg);
        let alpha_shift = min_shift + extra / 2.0;
        let alpha_avg = min_avg + extra / 2.0;
        if crate::fast_spread::fast_spread(x).map_err(EstimatorError::from)? <= 0.0 {
            return Err(EstimatorError::from(AssumptionError::sparity(Subject::X)));
        }
        if crate::fast_spread::fast_spread(y).map_err(EstimatorError::from)? <= 0.0 {
            return Err(EstimatorError::from(AssumptionError::sparity(Subject::Y)));
        }
        let sb = shift_bounds(x, y, alpha_shift)?;
        let ab = avg_spread_bounds_with_rngs(x, y, alpha_avg, rng_x, rng_y)?;
        let la = ab.lower;
        let ua = ab.upper;
        let ls = sb.lower;
        let us = sb.upper;
        if la > 0.0 {
            let r1 = ls / la;
            let r2 = ls / ua;
            let r3 = us / la;
            let r4 = us / ua;
            let lower = r1.min(r2).min(r3).min(r4);
            let upper = r1.max(r2).max(r3).max(r4);
            return Ok(RawBounds { lower, upper });
        }
        if ua <= 0.0 {
            if ls == 0.0 && us == 0.0 {
                return Ok(RawBounds {
                    lower: 0.0,
                    upper: 0.0,
                });
            }
            if ls >= 0.0 {
                return Ok(RawBounds {
                    lower: 0.0,
                    upper: f64::INFINITY,
                });
            }
            if us <= 0.0 {
                return Ok(RawBounds {
                    lower: f64::NEG_INFINITY,
                    upper: 0.0,
                });
            }
            return Ok(RawBounds {
                lower: f64::NEG_INFINITY,
                upper: f64::INFINITY,
            });
        }
        if ls > 0.0 {
            return Ok(RawBounds {
                lower: ls / ua,
                upper: f64::INFINITY,
            });
        }
        if us < 0.0 {
            return Ok(RawBounds {
                lower: f64::NEG_INFINITY,
                upper: us / ua,
            });
        }
        if ls == 0.0 && us == 0.0 {
            return Ok(RawBounds {
                lower: 0.0,
                upper: 0.0,
            });
        }
        if ls == 0.0 && us > 0.0 {
            return Ok(RawBounds {
                lower: 0.0,
                upper: f64::INFINITY,
            });
        }
        if ls < 0.0 && us == 0.0 {
            return Ok(RawBounds {
                lower: f64::NEG_INFINITY,
                upper: 0.0,
            });
        }
        Ok(RawBounds {
            lower: f64::NEG_INFINITY,
            upper: f64::INFINITY,
        })
    }

    pub(crate) fn spread_bounds_with_rng(
        x: &[f64],
        misrate: f64,
        rng: &mut crate::rng::Rng,
    ) -> Result<RawBounds, EstimatorError> {
        check_validity(x, Subject::X)?;
        if misrate.is_nan() || !(0.0..=1.0).contains(&misrate) {
            return Err(EstimatorError::from(AssumptionError::domain(
                Subject::Misrate,
            )));
        }
        let n = x.len();
        let m = n / 2;
        let min_misrate = crate::min_misrate::min_achievable_misrate_one_sample(m)?;
        if misrate < min_misrate {
            return Err(EstimatorError::from(AssumptionError::domain(
                Subject::Misrate,
            )));
        }
        if n < 2 {
            return Err(EstimatorError::from(AssumptionError::sparity(Subject::X)));
        }
        if crate::fast_spread::fast_spread(x).map_err(EstimatorError::from)? <= 0.0 {
            return Err(EstimatorError::from(AssumptionError::sparity(Subject::X)));
        }
        let margin = crate::sign_margin::sign_margin_randomized(m, misrate, rng)
            .map_err(EstimatorError::from)?;
        let mut half_margin = margin / 2;
        let max_half_margin = (m - 1) / 2;
        if half_margin > max_half_margin {
            half_margin = max_half_margin;
        }
        let k_left = half_margin + 1;
        let k_right = m - half_margin;
        let indices: Vec<usize> = (0..n).collect();
        let shuffled = rng.shuffle(&indices);
        let mut diffs = Vec::with_capacity(m);
        for i in 0..m {
            let a = shuffled[2 * i];
            let b = shuffled[2 * i + 1];
            diffs.push((x[a] - x[b]).abs());
        }
        diffs.sort_by(|a, b| a.total_cmp(b));
        let lower = diffs[k_left - 1];
        let upper = diffs[k_right - 1];
        Ok(RawBounds { lower, upper })
    }
}

// =============================================================================
// Sample-based public API
// =============================================================================

/// Estimates the central value of the data (center).
///
/// Calculates the median of all pairwise averages (x[i] + x[j])/2.
/// More robust than the mean and more efficient than the median.
///
/// Returns a [`Measurement`] with the same unit as the input sample.
pub fn center(x: &Sample) -> Result<Measurement, EstimatorError> {
    check_non_weighted("x", x)?;
    let result = crate::fast_center::fast_center(x.values()).map_err(EstimatorError::from)?;
    Ok(Measurement::new(result, x.unit().clone_box()))
}

/// Estimates data dispersion (spread).
///
/// Calculates the median of all pairwise absolute differences |x[i] - x[j]|.
///
/// Returns a [`Measurement`] with the same unit as the input sample.
///
/// # Assumptions
///
/// - `sparity(x)` - sample must be non tie-dominant (spread > 0)
pub fn spread(x: &Sample) -> Result<Measurement, EstimatorError> {
    check_non_weighted("x", x)?;
    let spread_val = crate::fast_spread::fast_spread(x.values()).map_err(EstimatorError::from)?;
    if spread_val <= 0.0 {
        return Err(EstimatorError::from(AssumptionError::sparity(x.subject())));
    }
    Ok(Measurement::new(spread_val, x.unit().clone_box()))
}

/// Measures the relative dispersion of a sample (rel_spread).
///
/// Deprecated: use `spread(x).value / center(x).value.abs()` instead.
#[deprecated(
    since = "10.0.0",
    note = "use spread(x).value / center(x).value.abs() instead"
)]
pub fn rel_spread(x: &Sample) -> Result<Measurement, EstimatorError> {
    check_non_weighted("x", x)?;
    for &v in x.values() {
        if v <= 0.0 {
            return Err(EstimatorError::from(AssumptionError::positivity(
                x.subject(),
            )));
        }
    }
    let center_val = crate::fast_center::fast_center(x.values()).map_err(EstimatorError::from)?;
    let spread_val = crate::fast_spread::fast_spread(x.values()).map_err(EstimatorError::from)?;
    Ok(Measurement::new(
        spread_val / center_val.abs(),
        Box::new(NumberUnit),
    ))
}

/// Measures the typical difference between elements of x and y (shift).
///
/// Returns a [`Measurement`] with the finer of x's and y's units.
pub fn shift(x: &Sample, y: &Sample) -> Result<Measurement, EstimatorError> {
    check_non_weighted("x", x)?;
    check_non_weighted("y", y)?;
    let (x, y) = prepare_pair(x, y)?;
    let result =
        crate::fast_shift::fast_shift(x.values(), y.values()).map_err(EstimatorError::from)?;
    Ok(Measurement::new(result, x.unit().clone_box()))
}

/// Measures how many times larger x is compared to y (ratio).
///
/// Returns a [`Measurement`] with [`RatioUnit`].
///
/// # Assumptions
///
/// - `positivity(x)` - all values in x must be strictly positive
/// - `positivity(y)` - all values in y must be strictly positive
pub fn ratio(x: &Sample, y: &Sample) -> Result<Measurement, EstimatorError> {
    check_non_weighted("x", x)?;
    check_non_weighted("y", y)?;
    let (x, y) = prepare_pair(x, y)?;
    for &v in x.values() {
        if v <= 0.0 {
            return Err(EstimatorError::from(AssumptionError::positivity(
                x.subject(),
            )));
        }
    }
    for &v in y.values() {
        if v <= 0.0 {
            return Err(EstimatorError::from(AssumptionError::positivity(
                y.subject(),
            )));
        }
    }
    let result =
        crate::fast_shift::fast_ratio(x.values(), y.values()).map_err(EstimatorError::from)?;
    Ok(Measurement::new(result, Box::new(RatioUnit)))
}

/// Measures the typical variability when considering both samples together (avg_spread).
///
/// Internal estimator used by disparity. Returns a [`Measurement`] with the finer unit.
#[cfg(test)]
pub(crate) fn avg_spread(x: &Sample, y: &Sample) -> Result<Measurement, EstimatorError> {
    check_non_weighted("x", x)?;
    check_non_weighted("y", y)?;
    let (x, y) = prepare_pair(x, y)?;
    let n = x.size() as f64;
    let m = y.size() as f64;
    let spread_x = crate::fast_spread::fast_spread(x.values()).map_err(EstimatorError::from)?;
    if spread_x <= 0.0 {
        return Err(EstimatorError::from(AssumptionError::sparity(x.subject())));
    }
    let spread_y = crate::fast_spread::fast_spread(y.values()).map_err(EstimatorError::from)?;
    if spread_y <= 0.0 {
        return Err(EstimatorError::from(AssumptionError::sparity(y.subject())));
    }
    Ok(Measurement::new(
        (n * spread_x + m * spread_y) / (n + m),
        x.unit().clone_box(),
    ))
}

/// Measures effect size: a normalized difference between x and y (disparity).
///
/// Returns a [`Measurement`] with [`DisparityUnit`].
///
/// # Assumptions
///
/// - `sparity(x)` - first sample must be non tie-dominant (spread > 0)
/// - `sparity(y)` - second sample must be non tie-dominant (spread > 0)
pub fn disparity(x: &Sample, y: &Sample) -> Result<Measurement, EstimatorError> {
    check_non_weighted("x", x)?;
    check_non_weighted("y", y)?;
    let (x, y) = prepare_pair(x, y)?;
    let n = x.size() as f64;
    let m = y.size() as f64;
    let spread_x = crate::fast_spread::fast_spread(x.values()).map_err(EstimatorError::from)?;
    if spread_x <= 0.0 {
        return Err(EstimatorError::from(AssumptionError::sparity(x.subject())));
    }
    let spread_y = crate::fast_spread::fast_spread(y.values()).map_err(EstimatorError::from)?;
    if spread_y <= 0.0 {
        return Err(EstimatorError::from(AssumptionError::sparity(y.subject())));
    }
    let shift_val =
        crate::fast_shift::fast_shift(x.values(), y.values()).map_err(EstimatorError::from)?;
    let avg_spread_val = (n * spread_x + m * spread_y) / (n + m);
    Ok(Measurement::new(
        shift_val / avg_spread_val,
        Box::new(DisparityUnit),
    ))
}

/// Provides bounds on the shift estimator.
///
/// Returns [`Bounds`] with the finer of x's and y's units.
pub fn shift_bounds(x: &Sample, y: &Sample, misrate: f64) -> Result<Bounds, EstimatorError> {
    check_non_weighted("x", x)?;
    check_non_weighted("y", y)?;
    let (x, y) = prepare_pair(x, y)?;
    let rb = raw::shift_bounds(x.values(), y.values(), misrate)?;
    Ok(Bounds::new(rb.lower, rb.upper, x.unit().clone_box()))
}

/// Provides bounds on the ratio estimator.
///
/// Returns [`Bounds`] with [`RatioUnit`].
pub fn ratio_bounds(x: &Sample, y: &Sample, misrate: f64) -> Result<Bounds, EstimatorError> {
    check_non_weighted("x", x)?;
    check_non_weighted("y", y)?;
    let (x, y) = prepare_pair(x, y)?;
    let rb = raw::ratio_bounds(x.values(), y.values(), misrate)?;
    Ok(Bounds::new(rb.lower, rb.upper, Box::new(RatioUnit)))
}

/// Provides exact distribution-free bounds for center.
///
/// Returns [`Bounds`] with the same unit as the input sample.
pub fn center_bounds(x: &Sample, misrate: f64) -> Result<Bounds, EstimatorError> {
    check_non_weighted("x", x)?;
    let rb = raw::center_bounds(x.values(), misrate)?;
    Ok(Bounds::new(rb.lower, rb.upper, x.unit().clone_box()))
}

/// Provides distribution-free bounds for spread.
///
/// Returns [`Bounds`] with the same unit as the input sample.
pub fn spread_bounds(x: &Sample, misrate: f64) -> Result<Bounds, EstimatorError> {
    check_non_weighted("x", x)?;
    let rb = raw::spread_bounds(x.values(), misrate)?;
    Ok(Bounds::new(rb.lower, rb.upper, x.unit().clone_box()))
}

/// Provides distribution-free spread bounds with a deterministic seed.
pub fn spread_bounds_with_seed(
    x: &Sample,
    misrate: f64,
    seed: &str,
) -> Result<Bounds, EstimatorError> {
    check_non_weighted("x", x)?;
    let rb = raw::spread_bounds_with_seed(x.values(), misrate, seed)?;
    Ok(Bounds::new(rb.lower, rb.upper, x.unit().clone_box()))
}

/// Provides distribution-free bounds for disparity.
///
/// Returns [`Bounds`] with [`DisparityUnit`].
pub fn disparity_bounds(x: &Sample, y: &Sample, misrate: f64) -> Result<Bounds, EstimatorError> {
    check_non_weighted("x", x)?;
    check_non_weighted("y", y)?;
    let (x, y) = prepare_pair(x, y)?;
    let rb = raw::disparity_bounds(x.values(), y.values(), misrate)?;
    Ok(Bounds::new(rb.lower, rb.upper, Box::new(DisparityUnit)))
}

/// Provides distribution-free disparity bounds with a deterministic seed.
pub fn disparity_bounds_with_seed(
    x: &Sample,
    y: &Sample,
    misrate: f64,
    seed: &str,
) -> Result<Bounds, EstimatorError> {
    check_non_weighted("x", x)?;
    check_non_weighted("y", y)?;
    let (x, y) = prepare_pair(x, y)?;
    let rb = raw::disparity_bounds_with_seed(x.values(), y.values(), misrate, seed)?;
    Ok(Bounds::new(rb.lower, rb.upper, Box::new(DisparityUnit)))
}

// Internal avg_spread_bounds functions for tests
#[cfg(test)]
pub(crate) fn avg_spread_bounds(
    x: &Sample,
    y: &Sample,
    misrate: f64,
) -> Result<Bounds, EstimatorError> {
    check_non_weighted("x", x)?;
    check_non_weighted("y", y)?;
    let (x, y) = prepare_pair(x, y)?;
    let rb = raw::avg_spread_bounds(x.values(), y.values(), misrate)?;
    Ok(Bounds::new(rb.lower, rb.upper, x.unit().clone_box()))
}

#[cfg(test)]
pub(crate) fn avg_spread_bounds_with_seed(
    x: &Sample,
    y: &Sample,
    misrate: f64,
    seed: &str,
) -> Result<Bounds, EstimatorError> {
    check_non_weighted("x", x)?;
    check_non_weighted("y", y)?;
    let (x, y) = prepare_pair(x, y)?;
    let rb = raw::avg_spread_bounds_with_seed(x.values(), y.values(), misrate, seed)?;
    Ok(Bounds::new(rb.lower, rb.upper, x.unit().clone_box()))
}
