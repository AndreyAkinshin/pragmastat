//! SignMargin function for computing confidence bound margins.
//!
//! Computes randomized cutoffs for one-sample sign-test bounds based on
//! the Binomial(n, 0.5) distribution.

use crate::assumptions::{AssumptionError, Subject};
use crate::rng::Rng;

/// Randomized version of SignMargin.
/// Randomizes the cutoff between adjacent ranks to match the requested misrate.
pub fn sign_margin_randomized(
    n: usize,
    misrate: f64,
    rng: &mut Rng,
) -> Result<usize, AssumptionError> {
    if n == 0 {
        return Err(AssumptionError::domain(Subject::X));
    }
    if misrate.is_nan() || !(0.0..=1.0).contains(&misrate) {
        return Err(AssumptionError::domain(Subject::Misrate));
    }

    let min_misrate = crate::min_misrate::min_achievable_misrate_one_sample(n)?;
    if misrate < min_misrate {
        return Err(AssumptionError::domain(Subject::Misrate));
    }

    let target = misrate / 2.0;
    if target <= 0.0 {
        return Ok(0);
    }
    if target >= 1.0 {
        return Ok(n * 2);
    }

    let (r_low, log_cdf, log_pmf_high) = binom_cdf_split(n, target);

    // If we are already at the boundary, no need to randomize.
    let log_target = target.ln();
    let log_num = if log_target > log_cdf {
        log_sub_exp(log_target, log_cdf)
    } else {
        f64::NEG_INFINITY
    };

    let mut p = if log_pmf_high.is_finite() && log_num.is_finite() {
        (log_num - log_pmf_high).exp()
    } else {
        0.0
    };
    p = p.clamp(0.0, 1.0);

    let u = rng.uniform();
    let r = if u < p { r_low + 1 } else { r_low };
    Ok(r * 2)
}

/// Small helper for log-sum-exp in base-e.
fn log_add_exp(a: f64, b: f64) -> f64 {
    if a.is_infinite() && a.is_sign_negative() {
        return b;
    }
    if b.is_infinite() && b.is_sign_negative() {
        return a;
    }
    let m = a.max(b);
    m + ((a - m).exp() + (b - m).exp()).ln()
}

/// Small helper for log(exp(a) - exp(b)) with a >= b.
fn log_sub_exp(a: f64, b: f64) -> f64 {
    if b.is_infinite() && b.is_sign_negative() {
        return a;
    }
    let diff = (b - a).exp();
    if diff >= 1.0 {
        f64::NEG_INFINITY
    } else {
        a + (1.0 - diff).ln()
    }
}

/// Returns (r_low, log_cdf, log_pmf_high) where:
/// - r_low is the largest r such that CDF(r) <= target
/// - log_cdf = log(CDF(r_low))
/// - log_pmf_high = log(PMF(r_low + 1))
///
/// Special case: when CDF(0) > target, returns (0, log(PMF(0)), log(PMF(0)))
/// because r_low = 0 and the "next" PMF is also PMF(0).
fn binom_cdf_split(n: usize, target: f64) -> (usize, f64, f64) {
    let log_target = target.ln();

    // pmf(0) = 2^-n
    let mut log_pmf = -(n as f64) * std::f64::consts::LN_2;
    let mut log_cdf = log_pmf;

    let mut r_low = 0;

    if log_cdf > log_target {
        return (0, log_cdf, log_pmf);
    }

    for k in 1..=n {
        let log_pmf_next = log_pmf + ((n - k + 1) as f64).ln() - (k as f64).ln();
        let log_cdf_next = log_add_exp(log_cdf, log_pmf_next);

        if log_cdf_next > log_target {
            return (r_low, log_cdf, log_pmf_next);
        }

        r_low = k;
        log_pmf = log_pmf_next;
        log_cdf = log_cdf_next;
    }

    (r_low, log_cdf, f64::NEG_INFINITY)
}
