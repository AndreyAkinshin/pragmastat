//! Statistical estimators for one-sample and two-sample analysis.
//!
//! Public API accepts [`Sample`] and returns [`Measurement`] or [`Bounds`].
//! Raw `&[f64]`-based helpers are available via the `raw` submodule as a
//! lightweight numeric interface and for internal tests.

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
// Raw (slice-based) estimator functions — low-level public slice API
// =============================================================================

/// Low-level public slice API for the estimators.
///
/// This module is a stable, supported entry point for callers that work directly
/// with `&[f64]` rather than the [`Sample`]/[`Measurement`]/[`Bounds`] metrology
/// types. It is a lightweight numeric interface for performance-sensitive or
/// unit-agnostic callers (it also backs the cross-language test suite).
///
/// Functions accept `&[f64]` and return a raw `f64` or the legacy [`RawBounds`]
/// struct (lower/upper without a unit). The `assume_sorted` parameter lets callers
/// that already hold pre-sorted data skip a redundant sort.
///
/// # Safety / contract
///
/// Passing `assume_sorted = true` with input that is NOT actually sorted ascending
/// is a contract violation (undefined behavior): the result is unspecified and may
/// differ from the sorted answer. Termination is nonetheless guaranteed: the
/// selection loops are bounded and fail with a deterministic convergence error on
/// pathological input.
pub mod raw {
    use super::*;

    /// Legacy bounds struct without unit (for backward compatibility).
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct RawBounds {
        pub lower: f64,
        pub upper: f64,
    }

    pub fn center(x: &[f64], assume_sorted: bool) -> Result<f64, EstimatorError> {
        check_validity(x, Subject::X)?;
        crate::center_impl::center_impl(x, assume_sorted).map_err(EstimatorError::from)
    }

    pub fn spread(x: &[f64], assume_sorted: bool) -> Result<f64, EstimatorError> {
        check_validity(x, Subject::X)?;
        let spread_val =
            crate::spread_impl::spread_impl(x, assume_sorted).map_err(EstimatorError::from)?;
        if spread_val <= 0.0 {
            return Err(EstimatorError::from(AssumptionError::sparity(Subject::X)));
        }
        Ok(spread_val)
    }

    pub fn shift(x: &[f64], y: &[f64], assume_sorted: bool) -> Result<f64, EstimatorError> {
        check_validity(x, Subject::X)?;
        check_validity(y, Subject::Y)?;
        Ok(
            crate::shift_impl::shift_quantiles_impl(x, y, &[0.5], assume_sorted)
                .map_err(EstimatorError::from)?[0],
        )
    }

    pub fn ratio(x: &[f64], y: &[f64], assume_sorted: bool) -> Result<f64, EstimatorError> {
        check_validity(x, Subject::X)?;
        check_validity(y, Subject::Y)?;
        check_positivity(x, Subject::X)?;
        check_positivity(y, Subject::Y)?;
        Ok(
            crate::shift_impl::ratio_quantiles_impl(x, y, &[0.5], assume_sorted)
                .map_err(EstimatorError::from)?[0],
        )
    }

    #[cfg(test)]
    pub(crate) fn avg_spread(
        x: &[f64],
        y: &[f64],
        assume_sorted: bool,
    ) -> Result<f64, EstimatorError> {
        check_validity(x, Subject::X)?;
        check_validity(y, Subject::Y)?;
        let n = x.len();
        let m = y.len();
        let spread_x =
            crate::spread_impl::spread_impl(x, assume_sorted).map_err(EstimatorError::from)?;
        if spread_x <= 0.0 {
            return Err(EstimatorError::from(AssumptionError::sparity(Subject::X)));
        }
        let spread_y =
            crate::spread_impl::spread_impl(y, assume_sorted).map_err(EstimatorError::from)?;
        if spread_y <= 0.0 {
            return Err(EstimatorError::from(AssumptionError::sparity(Subject::Y)));
        }
        Ok((n as f64 * spread_x + m as f64 * spread_y) / (n + m) as f64)
    }

    pub fn disparity(x: &[f64], y: &[f64], assume_sorted: bool) -> Result<f64, EstimatorError> {
        check_validity(x, Subject::X)?;
        check_validity(y, Subject::Y)?;
        let n = x.len();
        let m = y.len();
        let spread_x =
            crate::spread_impl::spread_impl(x, assume_sorted).map_err(EstimatorError::from)?;
        if spread_x <= 0.0 {
            return Err(EstimatorError::from(AssumptionError::sparity(Subject::X)));
        }
        let spread_y =
            crate::spread_impl::spread_impl(y, assume_sorted).map_err(EstimatorError::from)?;
        if spread_y <= 0.0 {
            return Err(EstimatorError::from(AssumptionError::sparity(Subject::Y)));
        }
        let shift_val = crate::shift_impl::shift_quantiles_impl(x, y, &[0.5], assume_sorted)
            .map_err(EstimatorError::from)?[0];
        let avg_spread_val = (n as f64 * spread_x + m as f64 * spread_y) / (n + m) as f64;
        Ok(shift_val / avg_spread_val)
    }

    pub fn shift_bounds(
        x: &[f64],
        y: &[f64],
        misrate: f64,
        assume_sorted: bool,
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
        let min_misrate = crate::min_misrate::min_achievable_misrate_two_sample(n, m)
            .map_err(EstimatorError::from)?;
        if misrate < min_misrate {
            return Err(EstimatorError::from(AssumptionError::domain(
                Subject::Misrate,
            )));
        }
        let total = n as u64 * m as u64;
        if total == 1 {
            let (xv, yv) = sorted_pair(x, y, assume_sorted);
            let value = xv[0] - yv[0];
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
        let (xs, ys) = sorted_pair(x, y, assume_sorted);
        let bounds = crate::shift_impl::shift_quantiles_impl(&xs, &ys, &p, true)
            .map_err(EstimatorError::from)?;
        let lower = bounds[0].min(bounds[1]);
        let upper = bounds[0].max(bounds[1]);
        Ok(RawBounds { lower, upper })
    }

    pub fn ratio_bounds(
        x: &[f64],
        y: &[f64],
        misrate: f64,
        assume_sorted: bool,
    ) -> Result<RawBounds, EstimatorError> {
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
        // log is monotonic: sorted positive input → sorted log output
        let log_bounds = shift_bounds(&log_x, &log_y, misrate, assume_sorted)?;
        Ok(RawBounds {
            lower: log_bounds.lower.exp(),
            upper: log_bounds.upper.exp(),
        })
    }

    pub fn center_bounds(
        x: &[f64],
        misrate: f64,
        assume_sorted: bool,
    ) -> Result<RawBounds, EstimatorError> {
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
        let sorted = sorted_one(x, assume_sorted);
        let (lo, hi) =
            crate::center_quantiles_impl::center_quantile_bounds_impl(&sorted, k_left, k_right);
        Ok(RawBounds {
            lower: lo,
            upper: hi,
        })
    }

    pub fn spread_bounds(
        x: &[f64],
        misrate: f64,
        assume_sorted: bool,
    ) -> Result<RawBounds, EstimatorError> {
        let mut rng = crate::rng::Rng::new();
        spread_bounds_with_rng(x, sorted_view(x, assume_sorted), misrate, &mut rng)
    }

    pub fn spread_bounds_with_seed(
        x: &[f64],
        misrate: f64,
        seed: &str,
        assume_sorted: bool,
    ) -> Result<RawBounds, EstimatorError> {
        let mut rng = crate::rng::Rng::from_string(seed);
        spread_bounds_with_rng(x, sorted_view(x, assume_sorted), misrate, &mut rng)
    }

    #[doc(hidden)] // internal estimator, pub only for pragmastat-sim (cross-crate)
    pub fn avg_spread_bounds(
        x: &[f64],
        y: &[f64],
        misrate: f64,
        assume_sorted: bool,
    ) -> Result<RawBounds, EstimatorError> {
        let mut rng_x = crate::rng::Rng::new();
        let mut rng_y = crate::rng::Rng::new();
        avg_spread_bounds_with_rngs(
            x,
            sorted_view(x, assume_sorted),
            y,
            sorted_view(y, assume_sorted),
            misrate,
            &mut rng_x,
            &mut rng_y,
        )
    }

    #[cfg(test)]
    pub(crate) fn avg_spread_bounds_with_seed(
        x: &[f64],
        y: &[f64],
        misrate: f64,
        seed: &str,
        assume_sorted: bool,
    ) -> Result<RawBounds, EstimatorError> {
        let mut rng_x = crate::rng::Rng::from_string(seed);
        let mut rng_y = crate::rng::Rng::from_string(seed);
        avg_spread_bounds_with_rngs(
            x,
            sorted_view(x, assume_sorted),
            y,
            sorted_view(y, assume_sorted),
            misrate,
            &mut rng_x,
            &mut rng_y,
        )
    }

    /// Maps the public `assume_sorted` flag to the internal optional pre-sorted
    /// view: when the caller's slice is already sorted, it doubles as the sorted
    /// view for the order-independent sub-computations (the sparity check, and
    /// for disparity bounds also the embedded shift bounds), skipping a re-sort.
    /// The disjoint-pair shuffle always runs on the caller's slice regardless,
    /// so on a genuinely sorted slice the flag never changes the result.
    fn sorted_view(x: &[f64], assume_sorted: bool) -> Option<&[f64]> {
        if assume_sorted {
            Some(x)
        } else {
            None
        }
    }

    /// Computes weighted-average spread bounds.
    ///
    /// `x`/`y` are always in ORIGINAL order (the disjoint-pair shuffle is
    /// order-dependent). `sorted_x`/`sorted_y`, when provided, are pre-sorted
    /// views used only to speed up the order-independent sparity check.
    pub(crate) fn avg_spread_bounds_with_rngs(
        x: &[f64],
        sorted_x: Option<&[f64]>,
        y: &[f64],
        sorted_y: Option<&[f64]>,
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
        if spread_for_sparity(x, sorted_x).map_err(EstimatorError::from)? <= 0.0 {
            return Err(EstimatorError::from(AssumptionError::sparity(Subject::X)));
        }
        if spread_for_sparity(y, sorted_y).map_err(EstimatorError::from)? <= 0.0 {
            return Err(EstimatorError::from(AssumptionError::sparity(Subject::Y)));
        }
        // The shuffle operates on the ORIGINAL order; sorted views are sparity-only.
        let bounds_x = spread_bounds_with_rng_inner(x, n / 2, alpha, rng_x)?;
        let bounds_y = spread_bounds_with_rng_inner(y, m / 2, alpha, rng_y)?;
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
        assume_sorted: bool,
    ) -> Result<RawBounds, EstimatorError> {
        let mut rng_x = crate::rng::Rng::new();
        let mut rng_y = crate::rng::Rng::new();
        disparity_bounds_with_rngs(
            x,
            sorted_view(x, assume_sorted),
            y,
            sorted_view(y, assume_sorted),
            misrate,
            &mut rng_x,
            &mut rng_y,
        )
    }

    pub fn disparity_bounds_with_seed(
        x: &[f64],
        y: &[f64],
        misrate: f64,
        seed: &str,
        assume_sorted: bool,
    ) -> Result<RawBounds, EstimatorError> {
        let mut rng_x = crate::rng::Rng::from_string(seed);
        let mut rng_y = crate::rng::Rng::from_string(seed);
        disparity_bounds_with_rngs(
            x,
            sorted_view(x, assume_sorted),
            y,
            sorted_view(y, assume_sorted),
            misrate,
            &mut rng_x,
            &mut rng_y,
        )
    }

    /// `x`/`y` are always in ORIGINAL order; `sorted_x`/`sorted_y`, when present,
    /// are pre-sorted views used only for the order-independent sparity and
    /// shift-bounds sub-computations.
    pub(crate) fn disparity_bounds_with_rngs(
        x: &[f64],
        sorted_x: Option<&[f64]>,
        y: &[f64],
        sorted_y: Option<&[f64]>,
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
        // The spread > 0 sparity check is performed by avg_spread_bounds_with_rngs
        // below (identical predicate and Subject::X/Y order). shift_bounds runs
        // first but cannot raise an error for these inputs (alpha_shift >= the
        // two-sample minimum), so it cannot mask that sparity error.
        // shift_bounds is order-independent given sorted input; use sorted views when present.
        let sb = match (sorted_x, sorted_y) {
            (Some(sx), Some(sy)) => shift_bounds(sx, sy, alpha_shift, true)?,
            _ => shift_bounds(x, y, alpha_shift, false)?,
        };
        let ab = avg_spread_bounds_with_rngs(x, sorted_x, y, sorted_y, alpha_avg, rng_x, rng_y)?;
        disparity_bounds_from_components(sb.lower, sb.upper, ab.lower, ab.upper)
    }

    /// `x` is always in ORIGINAL order (the disjoint-pair shuffle is
    /// order-dependent). `sorted_x`, when provided, is a pre-sorted view used
    /// only to speed up the order-independent sparity check.
    pub(crate) fn spread_bounds_with_rng(
        x: &[f64],
        sorted_x: Option<&[f64]>,
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
        if n < 2 {
            return Err(EstimatorError::from(AssumptionError::sparity(Subject::X)));
        }
        let m = n / 2;
        let min_misrate = crate::min_misrate::min_achievable_misrate_one_sample(m)?;
        if misrate < min_misrate {
            return Err(EstimatorError::from(AssumptionError::domain(
                Subject::Misrate,
            )));
        }
        if spread_for_sparity(x, sorted_x).map_err(EstimatorError::from)? <= 0.0 {
            return Err(EstimatorError::from(AssumptionError::sparity(Subject::X)));
        }
        spread_bounds_with_rng_inner(x, m, misrate, rng)
    }

    // =========================================================================
    // Internal algorithmic helpers
    // =========================================================================

    /// Computes the spread value for the sparity check. The result is
    /// order-independent, so a pre-sorted view (when available) is used to skip
    /// re-sorting; otherwise the original slice is sorted internally.
    fn spread_for_sparity(orig: &[f64], sorted: Option<&[f64]>) -> Result<f64, &'static str> {
        match sorted {
            Some(s) => crate::spread_impl::spread_impl(s, true),
            None => crate::spread_impl::spread_impl(orig, false),
        }
    }

    /// Shuffles, computes pairwise diffs, returns order-statistic bounds.
    fn spread_bounds_with_rng_inner(
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

    /// Computes disparity bounds from shift bounds (ls, us) and avg-spread bounds (la, ua).
    fn disparity_bounds_from_components(
        ls: f64,
        us: f64,
        la: f64,
        ua: f64,
    ) -> Result<RawBounds, EstimatorError> {
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

    // =========================================================================
    // Sorting helpers
    // =========================================================================

    /// Returns a sorted view of one slice: borrows if assume_sorted, copies+sorts otherwise.
    fn sorted_one<'a>(x: &'a [f64], assume_sorted: bool) -> std::borrow::Cow<'a, [f64]> {
        if assume_sorted {
            std::borrow::Cow::Borrowed(x)
        } else {
            let mut v = x.to_vec();
            v.sort_unstable_by(|a, b| a.total_cmp(b));
            std::borrow::Cow::Owned(v)
        }
    }

    /// Returns sorted views of two slices.
    fn sorted_pair<'a>(
        x: &'a [f64],
        y: &'a [f64],
        assume_sorted: bool,
    ) -> (std::borrow::Cow<'a, [f64]>, std::borrow::Cow<'a, [f64]>) {
        (sorted_one(x, assume_sorted), sorted_one(y, assume_sorted))
    }
}

// =============================================================================
// Sample-based public API — thin wrappers over raw
// =============================================================================

/// Estimates the central value of the data (center).
///
/// Calculates the median of all pairwise averages (x[i] + x[j])/2.
/// More robust than the mean and more efficient than the median.
///
/// Returns a [`Measurement`] with the same unit as the input sample.
pub fn center(x: &Sample) -> Result<Measurement, EstimatorError> {
    check_non_weighted("x", x)?;
    let result = raw::center(x.sorted_values(), true)?;
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
    let result = raw::spread(x.sorted_values(), true)?;
    Ok(Measurement::new(result, x.unit().clone()))
}

/// Measures the typical difference between elements of x and y (shift).
///
/// Returns a [`Measurement`] with the finer of x's and y's units.
pub fn shift(x: &Sample, y: &Sample) -> Result<Measurement, EstimatorError> {
    check_non_weighted("x", x)?;
    check_non_weighted("y", y)?;
    let (x, y) = prepare_pair(x, y)?;
    let result = raw::shift(x.sorted_values(), y.sorted_values(), true)?;
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
    let result = raw::ratio(x.sorted_values(), y.sorted_values(), true)?;
    Ok(Measurement::new(result, MeasurementUnit::ratio()))
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
    let result = raw::disparity(x.sorted_values(), y.sorted_values(), true)?;
    Ok(Measurement::new(result, MeasurementUnit::disparity()))
}

/// Provides bounds on the shift estimator.
///
/// Returns [`Bounds`] with the finer of x's and y's units.
pub fn shift_bounds(x: &Sample, y: &Sample, misrate: f64) -> Result<Bounds, EstimatorError> {
    check_non_weighted("x", x)?;
    check_non_weighted("y", y)?;
    let (x, y) = prepare_pair(x, y)?;
    let rb = raw::shift_bounds(x.sorted_values(), y.sorted_values(), misrate, true)?;
    Ok(Bounds::new(rb.lower, rb.upper, x.unit().clone()))
}

/// Provides bounds on the ratio estimator.
///
/// Returns [`Bounds`] with the ratio unit.
pub fn ratio_bounds(x: &Sample, y: &Sample, misrate: f64) -> Result<Bounds, EstimatorError> {
    check_non_weighted("x", x)?;
    check_non_weighted("y", y)?;
    let (x, y) = prepare_pair(x, y)?;
    let rb = raw::ratio_bounds(x.sorted_values(), y.sorted_values(), misrate, true)?;
    Ok(Bounds::new(rb.lower, rb.upper, MeasurementUnit::ratio()))
}

/// Provides exact distribution-free bounds for center.
///
/// Returns [`Bounds`] with the same unit as the input sample.
pub fn center_bounds(x: &Sample, misrate: f64) -> Result<Bounds, EstimatorError> {
    check_non_weighted("x", x)?;
    let rb = raw::center_bounds(x.sorted_values(), misrate, true)?;
    Ok(Bounds::new(rb.lower, rb.upper, x.unit().clone()))
}

/// Provides distribution-free bounds for spread.
///
/// Returns [`Bounds`] with the same unit as the input sample.
pub fn spread_bounds(x: &Sample, misrate: f64) -> Result<Bounds, EstimatorError> {
    check_non_weighted("x", x)?;
    let mut rng = crate::rng::Rng::new();
    // Shuffle runs on the original order; the cached sorted view is sparity-only.
    let rb = raw::spread_bounds_with_rng(x.values(), Some(x.sorted_values()), misrate, &mut rng)?;
    Ok(Bounds::new(rb.lower, rb.upper, x.unit().clone()))
}

/// Provides distribution-free spread bounds with a deterministic seed.
pub fn spread_bounds_with_seed(
    x: &Sample,
    misrate: f64,
    seed: &str,
) -> Result<Bounds, EstimatorError> {
    check_non_weighted("x", x)?;
    let mut rng = crate::rng::Rng::from_string(seed);
    // Shuffle runs on the original order; the cached sorted view is sparity-only.
    let rb = raw::spread_bounds_with_rng(x.values(), Some(x.sorted_values()), misrate, &mut rng)?;
    Ok(Bounds::new(rb.lower, rb.upper, x.unit().clone()))
}

/// Provides distribution-free bounds for disparity.
///
/// Returns [`Bounds`] with the disparity unit.
pub fn disparity_bounds(x: &Sample, y: &Sample, misrate: f64) -> Result<Bounds, EstimatorError> {
    check_non_weighted("x", x)?;
    check_non_weighted("y", y)?;
    let (x, y) = prepare_pair(x, y)?;
    let mut rng_x = crate::rng::Rng::new();
    let mut rng_y = crate::rng::Rng::new();
    let rb = raw::disparity_bounds_with_rngs(
        x.values(),
        Some(x.sorted_values()),
        y.values(),
        Some(y.sorted_values()),
        misrate,
        &mut rng_x,
        &mut rng_y,
    )?;
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
    let mut rng_x = crate::rng::Rng::from_string(seed);
    let mut rng_y = crate::rng::Rng::from_string(seed);
    let rb = raw::disparity_bounds_with_rngs(
        x.values(),
        Some(x.sorted_values()),
        y.values(),
        Some(y.sorted_values()),
        misrate,
        &mut rng_x,
        &mut rng_y,
    )?;
    Ok(Bounds::new(
        rb.lower,
        rb.upper,
        MeasurementUnit::disparity(),
    ))
}
