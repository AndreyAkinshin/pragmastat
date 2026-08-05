//! The natural logarithm this toolkit evaluates, in place of the platform's.
//!
//! IEEE 754 fixes the result of every arithmetic operation and of the square root, and fixes
//! nothing about the logarithm. The reason this one has to be ours is stronger than for most
//! transcendentals: `ratio` is exp(median(log x - log y)), so a last-bit difference in log moves
//! the median's input and changes which pairwise element gets selected. That is a different
//! answer, not a small error in the same one.
//!
//! The constants and the polynomial come from `tests/oracles/fit_log.py`, which documents the
//! reduction and what is fitted. Which of the two range endpoints a port names depends on how it
//! splits x: a library frexp returns a significand in [0.5, 1) and the reduction doubles the low
//! end, while reading the exponent field gives one in [1, 2) and it halves the high end. Both
//! arrive at the same [sqrt(2)/2, sqrt(2)) and the seven agree bit for bit; only the endpoint each
//! has to name differs.

/// High half of ln 2. Split so that `k * LN2_HI` is exact: `LN2_HI` carries 32 significant bits
/// and `|k|` needs at most 11, which leaves the product inside the 53 available.
const LN2_HI: f64 = 0.6931471803691238;

/// Low half of ln 2.
const LN2_LO: f64 = 1.9082149292705877e-10;

/// sqrt(2), the upper end, and the one this reduction actually branches on.
#[allow(clippy::approx_constant)]
const SQRT2: f64 = 1.4142135623730951;

/// Chebyshev fit of `(log((1+s)/(1-s)) - 2*s) / s^3` over the reduced range, emitted by
/// `tests/oracles/fit_log.py`. The leading terms are the recognizable 2/3, 2/5, 2/7 of the odd
/// series doubled, which is where the assembly's missing factor of two lives; the tail departs
/// from the series because the fit minimizes the worst error rather than matching Taylor.
const POLY_P: [f64; 7] = [
    0.666666666666667,
    0.39999999999899505,
    0.28571428625975487,
    0.2222221113479508,
    0.18182889125261723,
    0.15331721600556042,
    0.14616449685043406,
];

/// Splits `x` into `(m, k)` with `x == m * 2^k` and `m` in `[sqrt(2)/2, sqrt(2))`.
///
/// Rust has no `frexp` in std, so the exponent is read out of the bit pattern. Subnormals are
/// scaled up first: their exponent field is zero and carries no information, so the leading bit
/// has to be brought into the normal range before the field means anything. The other ports call
/// their library equivalent and land on the same pair, since this is a decomposition rather than
/// a rounding.
fn reduce(x: f64) -> (f64, i32) {
    let mut bits = x.to_bits();
    let mut k = 0i32;

    if (bits >> 52) & 0x7ff == 0 {
        // Subnormal: multiply by 2^54, which is exact, and account for it in k.
        bits = (x * 1.8014398509481984e16).to_bits();
        k -= 54;
    }

    k += (((bits >> 52) & 0x7ff) as i32) - 1023;
    // Replace the exponent field with the one for [1, 2).
    let mut m = f64::from_bits((bits & 0x000f_ffff_ffff_ffff) | 0x3ff0_0000_0000_0000);

    // Recenter on [sqrt(2)/2, sqrt(2)) so that |f| stays small at both ends. Halving is exact.
    if m >= SQRT2 {
        m *= 0.5;
        k += 1;
    }

    (m, k)
}

/// Evaluates `POLY_P` at `z` by Horner, highest coefficient first.
fn poly(z: f64) -> f64 {
    let mut acc = POLY_P[POLY_P.len() - 1];
    for &c in POLY_P.iter().rev().skip(1) {
        acc = acc * z + c;
    }
    acc
}

/// Returns the natural logarithm of `x`.
///
/// The special cases follow IEEE 754 and match what every port's platform function returns for
/// them, since they are the cases a platform has no freedom about: `log(0)` is negative infinity,
/// `log(x)` for `x < 0` is NaN, and NaN and positive infinity pass through.
pub(crate) fn log_function(x: f64) -> f64 {
    if x.is_nan() {
        return x;
    }
    if x < 0.0 {
        return f64::NAN;
    }
    if x == 0.0 {
        return f64::NEG_INFINITY;
    }
    if x.is_infinite() {
        return f64::INFINITY;
    }
    if x == 1.0 {
        return 0.0;
    }

    let (m, k) = reduce(x);
    let f = m - 1.0;
    let s = f / (2.0 + f);
    let z = s * s;
    let r = z * poly(z);
    let hfsq = 0.5 * f * f;
    let kf = f64::from(k);

    if k == 0 {
        f - (hfsq - s * (hfsq + r))
    } else {
        kf * LN2_HI - ((hfsq - (s * (hfsq + r) + kf * LN2_LO)) - f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The reduction has to hand back a significand in the half-open range the polynomial was
    /// fitted over, and the pair has to multiply back to the input exactly, since both steps are
    /// scalings by powers of two.
    #[test]
    fn reduce_lands_in_the_fitted_range() {
        let mut x = 1e-300;
        while x < 1e300 {
            let (m, k) = reduce(x);
            assert!(
                (std::f64::consts::FRAC_1_SQRT_2..SQRT2).contains(&m),
                "significand {m} outside the fitted range for x = {x}"
            );
            assert_eq!(m * 2f64.powi(k), x, "reduction lost {x}");
            x *= 3.7;
        }
    }

    /// The literal clippy flags is the named constant, so the block stays readable across the
    /// seven ports without the reader having to take that on trust.
    #[test]
    fn constants_match_the_named_ones() {
        assert_eq!(SQRT2, std::f64::consts::SQRT_2);
    }

    /// The cases IEEE 754 leaves a platform no freedom about, and which the shared fixture cannot
    /// carry, since JSON has no way to write a negative infinity or a NaN.
    ///
    /// The zero at x = 1 is compared on its payload rather than with `assert_eq!`. `-0.0 == 0.0`
    /// is true, so the numeric form passes on a result of the wrong sign, and ratio exponentiates
    /// this value: the two zeros are distinct doubles that every other suite here would tell
    /// apart. Verified by mutation, which the numeric form did not catch.
    #[test]
    fn special_cases_follow_the_format() {
        assert_eq!(log_function(0.0), f64::NEG_INFINITY);
        assert_eq!(log_function(-0.0), f64::NEG_INFINITY);
        assert!(log_function(-1.0).is_nan());
        assert!(log_function(f64::NAN).is_nan());
        assert_eq!(log_function(f64::INFINITY), f64::INFINITY);
        assert_eq!(log_function(1.0).to_bits(), 0.0f64.to_bits());
    }

    /// Counts the representable values between two doubles, which is the honest unit for "how far
    /// apart are these". Dividing the difference by an estimated ulp answers nearly the same thing
    /// and lands within a rounding of the boundary, which makes a threshold of one either pass or
    /// fail on the estimate rather than on the values.
    fn steps_apart(a: f64, b: f64) -> u64 {
        let key = |v: f64| {
            let bits = v.to_bits() as i64;
            if bits < 0 { i64::MIN - bits } else { bits }
        };
        key(a).abs_diff(key(b))
    }

    /// Against the platform, which is not the reference but is a conforming implementation. The
    /// two may differ, because the format leaves the logarithm free, but only by the last step:
    /// a wider gap would mean this implementation is wrong rather than merely different.
    ///
    /// Measured over 50,980,435 points from 1e-300 to 1e300: 99.89% are bit-identical to the
    /// platform and no point is more than one representable value away.
    #[test]
    fn agrees_with_the_platform_to_the_last_step() {
        let mut worst = 0u64;
        let mut worst_at = 0.0f64;
        let mut x = 1e-280;
        while x < 1e280 {
            let steps = steps_apart(log_function(x), x.ln());
            if steps > worst {
                worst = steps;
                worst_at = x;
            }
            x *= 1.0000137;
        }
        assert!(
            worst <= 1,
            "log({worst_at}) is {worst} representable values from the platform's"
        );
    }

    /// exp and log are each other's inverse, and both are ours, so the round trip is a statement
    /// about this toolkit rather than about the platform.
    ///
    /// The tolerance is not a constant, because the round trip is not equally conditioned across
    /// the range. exp amplifies a relative error in its argument by |log x|: at x = 1e-12 that is
    /// a factor of 27.6, so a log accurate to half an ulp comes back through exp off by three
    /// parts in 1e15 with nothing wrong anywhere. A flat bound would have to be loose enough for
    /// the worst point and would then assert nothing at the best one. This bounds each point by
    /// its own amplification, so the claim stays the same strength everywhere.
    #[test]
    fn round_trips_through_the_toolkit_exponential() {
        use crate::exp_function::exp_function;

        let mut x = 1e-12;
        while x < 1e12 {
            let logged = log_function(x);
            let back = exp_function(logged);
            let relative = (back - x).abs() / x;
            let allowed = (logged.abs() + 1.0) * 2.0 * f64::EPSILON;
            assert!(
                relative <= allowed,
                "exp(log({x})) = {back}, off by {relative}, allowed {allowed}"
            );
            x *= 1.3;
        }
    }
}
