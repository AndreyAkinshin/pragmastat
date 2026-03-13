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
use crate::measurement_unit::MeasurementUnit;
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
        xs.sort_unstable_by(|a, b| a.total_cmp(b));
        ys.sort_unstable_by(|a, b| a.total_cmp(b));
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
        sorted.sort_unstable_by(|a, b| a.total_cmp(b));
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

    #[doc(hidden)] // internal estimator, pub only for pragmastat-sim (cross-crate)
    pub fn avg_spread_bounds(
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
        // Use inner variant to skip redundant fast_spread validation
        let bounds_x = spread_bounds_with_rng_inner(x, n / 2, alpha, rng_x)?;
        let bounds_y = spread_bounds_with_rng_inner(y, m / 2, alpha, rng_y)?;
        let weight_x = n as f64 / (n + m) as f64;
        let weight_y = m as f64 / (n + m) as f64;
        Ok(RawBounds {
            lower: weight_x * bounds_x.lower + weight_y * bounds_y.lower,
            upper: weight_x * bounds_x.upper + weight_y * bounds_y.upper,
        })
    }

    /// Unchecked variant that skips validity/spread checks (caller already verified).
    /// NOTE: misrate validation mirrors avg_spread_bounds_with_rngs — keep in sync
    fn avg_spread_bounds_with_rngs_unchecked(
        x: &[f64],
        y: &[f64],
        n: usize,
        m: usize,
        misrate: f64,
        rng_x: &mut crate::rng::Rng,
        rng_y: &mut crate::rng::Rng,
    ) -> Result<RawBounds, EstimatorError> {
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
        let bounds_x = spread_bounds_with_rng_inner(x, mx, alpha, rng_x)?;
        let bounds_y = spread_bounds_with_rng_inner(y, my, alpha, rng_y)?;
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
        let spread_x = crate::fast_spread::fast_spread(x).map_err(EstimatorError::from)?;
        if spread_x <= 0.0 {
            return Err(EstimatorError::from(AssumptionError::sparity(Subject::X)));
        }
        let spread_y = crate::fast_spread::fast_spread(y).map_err(EstimatorError::from)?;
        if spread_y <= 0.0 {
            return Err(EstimatorError::from(AssumptionError::sparity(Subject::Y)));
        }
        let sb = shift_bounds(x, y, alpha_shift)?;
        // avg_spread_bounds_with_rngs would re-check spreads; call inner directly
        let ab = avg_spread_bounds_with_rngs_unchecked(x, y, n, m, alpha_avg, rng_x, rng_y)?;
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
        spread_bounds_with_rng_inner(x, m, misrate, rng)
    }

    /// Inner implementation that skips the spread validation (already done by caller).
    pub(crate) fn spread_bounds_with_rng_inner(
        x: &[f64],
        m: usize,
        misrate: f64,
        rng: &mut crate::rng::Rng,
    ) -> Result<RawBounds, EstimatorError> {
        let margin = crate::sign_margin::sign_margin_randomized(m, misrate, rng)
            .map_err(EstimatorError::from)?;
        let mut half_margin = margin / 2;
        let max_half_margin = (m - 1) / 2;
        if half_margin > max_half_margin {
            half_margin = max_half_margin;
        }
        let k_left = half_margin + 1;
        let k_right = m - half_margin;
        // Shuffle a copy of x and compute pairwise diffs in-place,
        // avoiding a separate indices allocation and improving cache locality.
        let mut buf = x.to_vec();
        rng.shuffle_mut(&mut buf);
        for i in 0..m {
            buf[i] = (buf[2 * i] - buf[2 * i + 1]).abs();
        }
        buf.truncate(m);
        buf.sort_unstable_by(|a, b| a.total_cmp(b));
        let lower = buf[k_left - 1];
        let upper = buf[k_right - 1];
        Ok(RawBounds { lower, upper })
    }

    // =========================================================================
    // Batch one-sample summary — eliminates redundant sorting, validation,
    // hashing, and spread computation when all four one-sample estimators
    // are needed together.
    // =========================================================================

    /// Combined result of all four one-sample estimators.
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct OneSampleSummary {
        pub center: f64,
        pub spread: f64,
        pub center_bounds: RawBounds,
        pub spread_bounds: RawBounds,
    }

    /// Compute all four one-sample estimators in a single pass, eliminating
    /// redundant sorting, validation, hashing, and spread computation.
    ///
    /// With the `parallel` feature enabled, independent computations run
    /// concurrently via `rayon::join`.
    pub fn one_sample_summary(x: &[f64], misrate: f64) -> Result<OneSampleSummary, EstimatorError> {
        let mut rng = crate::rng::Rng::new();
        one_sample_summary_with_rng(x, misrate, &mut rng)
    }

    /// Compute all four one-sample estimators with a deterministic seed.
    pub fn one_sample_summary_with_seed(
        x: &[f64],
        misrate: f64,
        seed: &str,
    ) -> Result<OneSampleSummary, EstimatorError> {
        let mut rng = crate::rng::Rng::from_string(seed);
        one_sample_summary_with_rng(x, misrate, &mut rng)
    }

    fn one_sample_summary_with_rng(
        x: &[f64],
        misrate: f64,
        rng: &mut crate::rng::Rng,
    ) -> Result<OneSampleSummary, EstimatorError> {
        // --- 1. Validate once ---
        check_validity(x, Subject::X)?;

        let n = x.len();
        // The presorted algorithms (`fast_center_presorted`, `fast_spread_presorted`)
        // require n >= 3. Unlike the wrapper functions (`fast_center`, `fast_spread`)
        // which handle n <= 2 as special cases, we call the presorted variants directly.
        if n < 3 {
            return Err(EstimatorError::from(AssumptionError::domain(Subject::X)));
        }

        // --- 2. Validate misrate domain ---
        if misrate.is_nan() || !(0.0..=1.0).contains(&misrate) {
            return Err(EstimatorError::from(AssumptionError::domain(
                Subject::Misrate,
            )));
        }

        // Check min misrate for both center_bounds and spread_bounds
        let m = n / 2;
        let min_misrate_center = crate::min_misrate::min_achievable_misrate_one_sample(n)?;
        let min_misrate_spread = crate::min_misrate::min_achievable_misrate_one_sample(m)?;
        let min_misrate = min_misrate_center.max(min_misrate_spread);
        if misrate < min_misrate {
            return Err(EstimatorError::from(AssumptionError::domain(
                Subject::Misrate,
            )));
        }

        // --- 3. Hash once ---
        let input_hash = crate::fnv1a::hash_f64_slice(x);

        // --- 4. Sort once ---
        let mut sorted = x.to_vec();
        sorted.sort_unstable_by(|a, b| a.total_cmp(b));

        // --- 5. Compute spread + center (possibly in parallel) ---
        #[cfg(feature = "parallel")]
        let (spread_result, center_result) = rayon::join(
            || crate::fast_spread::fast_spread_presorted(&sorted, input_hash),
            || crate::fast_center::fast_center_presorted(&sorted, input_hash),
        );
        #[cfg(not(feature = "parallel"))]
        let (spread_result, center_result) = (
            crate::fast_spread::fast_spread_presorted(&sorted, input_hash),
            crate::fast_center::fast_center_presorted(&sorted, input_hash),
        );

        let spread_val = spread_result.map_err(EstimatorError::from)?;
        let center_val = center_result.map_err(EstimatorError::from)?;

        // --- 6. Sparity check ---
        if spread_val <= 0.0 {
            return Err(EstimatorError::from(AssumptionError::sparity(Subject::X)));
        }

        // --- 7. Compute center_bounds margins ---
        let margin = crate::signed_rank_margin::signed_rank_margin(n, misrate)?;
        let total_pairs = (n as i64) * (n as i64 + 1) / 2;
        let mut half_margin = (margin / 2) as i64;
        let max_half_margin = (total_pairs - 1) / 2;
        if half_margin > max_half_margin {
            half_margin = max_half_margin;
        }
        let k_left = half_margin + 1;
        let k_right = total_pairs - half_margin;

        // --- 8. Compute center_bounds + spread_bounds (possibly in parallel) ---
        #[cfg(feature = "parallel")]
        let (cb_result, sb_result) = rayon::join(
            || crate::fast_center_quantiles::fast_center_quantile_bounds(&sorted, k_left, k_right),
            // Uses original unsorted `x` because `spread_bounds_with_rng_inner`
            // shuffles internally to form random pairwise differences.
            || spread_bounds_with_rng_inner(x, m, misrate, rng),
        );
        #[cfg(not(feature = "parallel"))]
        let (cb_result, sb_result) = (
            crate::fast_center_quantiles::fast_center_quantile_bounds(&sorted, k_left, k_right),
            // Uses original unsorted `x` — see parallel branch comment above.
            spread_bounds_with_rng_inner(x, m, misrate, rng),
        );

        let (lo, hi) = cb_result;
        let sb = sb_result?;

        Ok(OneSampleSummary {
            center: center_val,
            spread: spread_val,
            center_bounds: RawBounds {
                lower: lo,
                upper: hi,
            },
            spread_bounds: sb,
        })
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
    Ok(Measurement::new(result, x.unit().clone()))
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
    Ok(Measurement::new(spread_val, x.unit().clone()))
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
    Ok(Measurement::new(result, x.unit().clone()))
}

/// Measures how many times larger x is compared to y (ratio).
///
/// Returns a [`Measurement`] with the ratio unit.
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
    Ok(Measurement::new(result, MeasurementUnit::ratio()))
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
        x.unit().clone(),
    ))
}

/// Measures effect size: a normalized difference between x and y (disparity).
///
/// Returns a [`Measurement`] with the disparity unit.
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
        MeasurementUnit::disparity(),
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
    Ok(Bounds::new(rb.lower, rb.upper, x.unit().clone()))
}

/// Provides bounds on the ratio estimator.
///
/// Returns [`Bounds`] with the ratio unit.
pub fn ratio_bounds(x: &Sample, y: &Sample, misrate: f64) -> Result<Bounds, EstimatorError> {
    check_non_weighted("x", x)?;
    check_non_weighted("y", y)?;
    let (x, y) = prepare_pair(x, y)?;
    let rb = raw::ratio_bounds(x.values(), y.values(), misrate)?;
    Ok(Bounds::new(rb.lower, rb.upper, MeasurementUnit::ratio()))
}

/// Provides exact distribution-free bounds for center.
///
/// Returns [`Bounds`] with the same unit as the input sample.
pub fn center_bounds(x: &Sample, misrate: f64) -> Result<Bounds, EstimatorError> {
    check_non_weighted("x", x)?;
    let rb = raw::center_bounds(x.values(), misrate)?;
    Ok(Bounds::new(rb.lower, rb.upper, x.unit().clone()))
}

/// Provides distribution-free bounds for spread.
///
/// Returns [`Bounds`] with the same unit as the input sample.
pub fn spread_bounds(x: &Sample, misrate: f64) -> Result<Bounds, EstimatorError> {
    check_non_weighted("x", x)?;
    let rb = raw::spread_bounds(x.values(), misrate)?;
    Ok(Bounds::new(rb.lower, rb.upper, x.unit().clone()))
}

/// Provides distribution-free spread bounds with a deterministic seed.
pub fn spread_bounds_with_seed(
    x: &Sample,
    misrate: f64,
    seed: &str,
) -> Result<Bounds, EstimatorError> {
    check_non_weighted("x", x)?;
    let rb = raw::spread_bounds_with_seed(x.values(), misrate, seed)?;
    Ok(Bounds::new(rb.lower, rb.upper, x.unit().clone()))
}

/// Provides distribution-free bounds for disparity.
///
/// Returns [`Bounds`] with the disparity unit.
pub fn disparity_bounds(x: &Sample, y: &Sample, misrate: f64) -> Result<Bounds, EstimatorError> {
    check_non_weighted("x", x)?;
    check_non_weighted("y", y)?;
    let (x, y) = prepare_pair(x, y)?;
    let rb = raw::disparity_bounds(x.values(), y.values(), misrate)?;
    Ok(Bounds::new(
        rb.lower,
        rb.upper,
        MeasurementUnit::disparity(),
    ))
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
    Ok(Bounds::new(
        rb.lower,
        rb.upper,
        MeasurementUnit::disparity(),
    ))
}

/// Combined result of all four one-sample estimators at the [`Sample`] level.
#[derive(Debug, Clone)]
pub struct OneSampleResult {
    pub center: Measurement,
    pub spread: Measurement,
    pub center_bounds: Bounds,
    pub spread_bounds: Bounds,
}

/// Compute all four one-sample estimators in a single pass.
///
/// Eliminates redundant sorting, validation, hashing, and spread computation
/// compared to calling `center`, `spread`, `center_bounds`, and `spread_bounds`
/// individually.
pub fn one_sample_summary(x: &Sample, misrate: f64) -> Result<OneSampleResult, EstimatorError> {
    check_non_weighted("x", x)?;
    let summary = raw::one_sample_summary(x.values(), misrate)?;
    let unit = x.unit().clone();
    Ok(OneSampleResult {
        center: Measurement::new(summary.center, unit.clone()),
        spread: Measurement::new(summary.spread, unit.clone()),
        center_bounds: Bounds::new(
            summary.center_bounds.lower,
            summary.center_bounds.upper,
            unit.clone(),
        ),
        spread_bounds: Bounds::new(
            summary.spread_bounds.lower,
            summary.spread_bounds.upper,
            unit,
        ),
    })
}

/// Compute all four one-sample estimators with a deterministic seed.
pub fn one_sample_summary_with_seed(
    x: &Sample,
    misrate: f64,
    seed: &str,
) -> Result<OneSampleResult, EstimatorError> {
    check_non_weighted("x", x)?;
    let summary = raw::one_sample_summary_with_seed(x.values(), misrate, seed)?;
    let unit = x.unit().clone();
    Ok(OneSampleResult {
        center: Measurement::new(summary.center, unit.clone()),
        spread: Measurement::new(summary.spread, unit.clone()),
        center_bounds: Bounds::new(
            summary.center_bounds.lower,
            summary.center_bounds.upper,
            unit.clone(),
        ),
        spread_bounds: Bounds::new(
            summary.spread_bounds.lower,
            summary.spread_bounds.upper,
            unit,
        ),
    })
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
    Ok(Bounds::new(rb.lower, rb.upper, x.unit().clone()))
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
    Ok(Bounds::new(rb.lower, rb.upper, x.unit().clone()))
}
