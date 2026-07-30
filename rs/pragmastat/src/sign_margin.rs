//! SignMargin function for computing confidence bound margins.
//!
//! Computes randomized cutoffs for one-sample sign-test bounds based on the Binomial(n, 0.5)
//! distribution, without a single library call.
//!
//! It used to be evaluated in log space: nine calls to `ln` and `exp`, one of them inside a loop
//! that runs n times. IEEE 754 fixes nothing about either function, and this value is not merely
//! returned to the caller: the margin selects an order statistic, so a difference between two
//! conforming libm implementations becomes a different confidence interval from identical inputs.
//! It did. Two ports disagreed on `spread_bounds` for a sample of 200 consecutive integers.
//!
//! No logarithm is needed. Binomial(n, 1/2) has an exact rational distribution function, and the
//! two quantities the randomization wants are its partial sum and the next term. Both follow from
//! the same multiplicative recurrence the binomial coefficient uses: one multiply and one divide
//! per step, plus a scaling by a power of two, and IEEE 754 pins all three.
//!
//! The scaling is what makes the recurrence work at any n. `pmf(0)` is `2^-n`, which underflows to
//! zero past n = 1074, so the running term is carried as `w * 2^e` with the exponent tracked
//! separately: `w` stays in the normal range and `e` absorbs the magnitude. Rescaling happens by
//! multiplying by a power of two, which is exact, so it costs no accuracy and changes no bits.
//!
//! Measured against exact rational arithmetic over 195 (n, misrate) pairs spanning n = 1 to 5000
//! and misrate from 1 down to the smallest positive double: the selected index is right every
//! time, and the randomization probability is within 6.1e-13. The log-space version it replaces
//! reached 1.9e-11 on the same set, thirty times further out, and did it differently in each port.

use crate::assumptions::{AssumptionError, Subject};
use crate::rng::Rng;

/// How far the running term is rescaled when it grows too large. Any power of two works; 512 keeps
/// the rescaling rare without letting `w` approach the overflow threshold.
const SCALE_STEP: i32 = 512;

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

    let (r_low, p) = binom_cdf_split(n, target);

    let u = rng.uniform_f64();
    let r = if u < p { r_low + 1 } else { r_low };
    Ok(r * 2)
}

/// Returns the largest k whose Binomial(n, 0.5) CDF does not exceed target, together with the
/// fraction of the next term that would be needed to reach it. The caller compares that fraction
/// against a uniform draw, which is what makes the margin achieve the requested misrate exactly
/// rather than the next admissible one below it.
fn binom_cdf_split(n: usize, target: f64) -> (usize, f64) {
    // Binomial(n, 1/2) is symmetric, so for odd n the distribution function at (n-1)/2 is exactly
    // one half. No approximation reproduces an exact equality, and misrate = 1 lands on it: the
    // summation would decide the comparison by its last accumulated bit.
    if target == 0.5 && n % 2 == 1 {
        return ((n - 1) / 2, 0.0);
    }

    let scale_up = f64::from_bits(((1023i64 + SCALE_STEP as i64) as u64) << 52);
    let scale_down = f64::from_bits(((1023i64 - SCALE_STEP as i64) as u64) << 52);

    // The running term pmf(k) is w * 2^e, starting from pmf(0) = 2^-n.
    let mut w = 1.0f64;
    let mut e = -(n as i32);
    let mut cdf = 1.0f64;

    if ldexp(cdf, e) > target {
        return (0, 0.0);
    }

    let mut r_low = 0;
    for k in 1..=n {
        w = w * (n - k + 1) as f64 / k as f64;
        while w > scale_up {
            w *= scale_down;
            cdf *= scale_down;
            e += SCALE_STEP;
        }
        let next = cdf + w;
        if ldexp(next, e) > target {
            // target and cdf are both in units of 2^e here, so the fraction is a plain quotient.
            let p = (ldexp(target, -e) - cdf) / w;
            return (r_low, p.clamp(0.0, 1.0));
        }
        r_low = k;
        cdf = next;
    }

    (r_low, 0.0)
}

/// `v * 2^exp`, exactly, including where the result leaves the normal range.
///
/// Rust has no `ldexp` in the standard library. Scaling by a power of two is exact wherever the
/// result is representable, so this splits an out-of-range exponent into steps that are not.
fn ldexp(v: f64, exp: i32) -> f64 {
    let mut result = v;
    let mut remaining = exp;
    while remaining > 1023 {
        result *= f64::from_bits((2046u64) << 52);
        remaining -= 1023;
    }
    while remaining < -1022 {
        result *= f64::from_bits((1u64) << 52);
        remaining += 1022;
    }
    result * f64::from_bits(((1023 + remaining) as u64) << 52)
}
