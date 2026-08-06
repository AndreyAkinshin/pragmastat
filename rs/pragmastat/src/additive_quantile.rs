//! The inverse standard normal CDF this toolkit evaluates, in place of the platform's.
//!
//! Nothing calls it yet. It lands with its own fixtures first, and the distributions that need it
//! move to it together, because a sampling stream cannot change in one language ahead of the
//! others.
#![allow(dead_code)]

use crate::log_function::log_function;

/// Half-width of the central band, in `p - 1/2`.
const CENTRAL_HALF_WIDTH: f64 = 0.425;

/// `CENTRAL_HALF_WIDTH^2`, the constant the central branch reduces by. `r = CENTRAL_OFFSET - q*q`
/// is then non-negative across the band and vanishes exactly at its edge.
const CENTRAL_OFFSET: f64 = 0.180625;

/// Where the tail splits. One approximant cannot cover `w` from 1.6 to 27 at a usable length:
/// measured, a single rational fit of degree (8, 8) over the whole tail reaches only 3.0e-10.
const TAIL_SPLIT: f64 = 5.0;

/// Computes `Phi^-1(p)`, the value a standard normal falls below with probability `p`.
///
/// IEEE 754 fixes nothing about this function, and a quantile is a value a caller compares and
/// reports, so it has to be the toolkit's own for the reason the exponential and the logarithm
/// already are. `Additive.Quantile` and `Multiplic.Quantile` are what will be built from it; see
/// the note at the top of this file for why they are not yet.
///
/// Three branches, each a rational approximant produced by `tests/oracles/fit_additive_quantile.py`:
///
/// ```text
/// central   |p - 1/2| <= 0.425     x = q * R1(r),  q = p - 1/2,  r = 0.180625 - q^2
/// near tail w <= 5                 x = +-R2(w),    w = sqrt(-log(min(p, 1-p)))
/// far tail  w > 5                  x = +-R3(w)
/// ```
///
/// The central form is odd in `q` by construction, so `p = 1/2` returns exactly zero rather than a
/// cancellation of two nearly equal numbers. The tails run through the toolkit's logarithm, which
/// is why this primitive could not exist before that one did.
///
/// Rational rather than polynomial, and three branches rather than one, because both were measured:
/// a degree-12 polynomial over the central band reaches 1.6e-07 with coefficients running to 1e8,
/// and one fit across the whole tail reaches 3.0e-10. Each branch here fits below 1.1e-16, the
/// binary64 rounding floor, and the assembled function measures 7.7 ulp worst over a sweep of
/// 320000 arguments, 1.03 on average.
///
/// The distance between those two numbers is the arithmetic, not the approximation, and that is
/// measured rather than assumed: at the argument where the assembled function is 7.7 ulp away, the
/// same coefficients at the same `w` in 60-digit arithmetic land 0.05 ulp from the true value.
/// Improving the fits would therefore buy nothing. The stored fixture is a set of regular grids
/// and reaches only 4.4, because a grid samples a branch evenly and a worst case does not live at
/// evenly spaced arguments.
///
/// A Newton step on `AdditiveCumulative` was tried as a refinement and made it worse rather than
/// better: that function carries about 1e-15 of relative error and is documented accurate only to
/// `|z| <= 6`, so refining a 7e-17 fit against it converges toward the less accurate of the two.
///
/// Every product is written as `a * b + c` and never as `a.mul_add(b, c)`, for the reason given at
/// the top of `lib.rs`. `rs:check:fma` is the guard.
pub(crate) fn additive_quantile(p: f64) -> f64 {
    if p.is_nan() {
        return p;
    }
    if p <= 0.0 {
        return f64::NEG_INFINITY;
    }
    if p >= 1.0 {
        return f64::INFINITY;
    }

    let q = p - 0.5;
    if q.abs() <= CENTRAL_HALF_WIDTH {
        let r = CENTRAL_OFFSET - q * q;
        let mut num = 3907.007639023816;
        num = num * r + 74060.81462563902;
        num = num * r + 214247.3801984238;
        num = num * r + 213550.16248438213;
        num = num * r + 95692.73551269759;
        num = num * r + 21627.48116049056;
        num = num * r + 2548.2248573022625;
        num = num * r + 148.75654077063152;
        num = num * r + 3.3871328727963665;
        let mut den = 9398.46451720659;
        den = den * r + 72342.69749284587;
        den = den * r + 140376.82723612545;
        den = den * r + 109244.70555458817;
        den = den * r + 41169.98166790856;
        den = den * r + 8176.878340636071;
        den = den * r + 871.2838940641986;
        den = den * r + 46.92338689522055;
        den = den * r + 1.0;
        return q * (num / den);
    }

    // Outside the central band the quantile is derived from the smaller tail, and the sign put
    // back afterwards. Working from `min(p, 1-p)` keeps the argument away from 1, where the
    // subtraction would throw away the significant digits the answer depends on.
    let lower = if q < 0.0 { p } else { 1.0 - p };
    let w = (-log_function(lower)).sqrt();

    let value = if w <= TAIL_SPLIT {
        let mut num = 0.054350362850595586;
        num = num * w + 1.5133041619767231;
        num = num * w + 12.302571927534103;
        num = num * w + 43.456109632558785;
        num = num * w + 75.71921394020897;
        num = num * w + 29.56505902784448;
        num = num * w - 57.880330387639276;
        num = num * w - 38.02928654598471;
        num = num * w - 3.5717440640448346;
        let mut den = 2.7706964359226424e-08;
        den = den * w + 0.03842783348289235;
        den = den * w + 1.0704443396791978;
        den = den * w + 8.80416037381848;
        den = den * w + 32.81079285831595;
        den = den * w + 66.49202694561696;
        den = den * w + 59.37032702530251;
        den = den * w + 17.47732083935553;
        den = den * w + 1.0;
        num / den
    } else {
        let mut num = 8.392823453560843e-08;
        num = num * w + 2.1314331124416687e-05;
        num = num * w + 0.0016436514333316493;
        num = num * w + 0.051392875575643406;
        num = num * w + 0.715697208514527;
        num = num * w + 4.497647129161168;
        num = num * w + 12.072411664076757;
        num = num * w + 10.942951231396602;
        num = num * w - 3.792525101710398;
        num = num * w - 10.24280793582052;
        num = num * w - 3.009101693252925;
        let mut den = 8.578649609090119e-17;
        den = den * w + 5.934613897538525e-08;
        den = den * w + 1.5071576669864384e-05;
        den = den * w + 0.0011624636891466385;
        den = den * w + 0.03638643084310481;
        den = den * w + 0.5090592389031116;
        den = den * w + 3.2585635875673855;
        den = den * w + 9.433144981171784;
        den = den * w + 12.233693860928529;
        den = den * w + 6.82391943088949;
        den = den * w + 1.0;
        num / den
    };

    if q < 0.0 { -value } else { value }
}

#[cfg(test)]
mod tests {
    use super::{CENTRAL_HALF_WIDTH, additive_quantile};

    /// The values the definition fixes, none of which may reach an approximant.
    #[test]
    fn special_values() {
        assert_eq!(additive_quantile(0.5), 0.0);
        assert!(additive_quantile(0.5).is_sign_positive());
        assert_eq!(additive_quantile(0.0), f64::NEG_INFINITY);
        assert_eq!(additive_quantile(1.0), f64::INFINITY);
        assert_eq!(additive_quantile(-0.1), f64::NEG_INFINITY);
        assert_eq!(additive_quantile(1.1), f64::INFINITY);
        assert!(additive_quantile(f64::NAN).is_nan());
    }

    /// Textbook quantiles, to the accuracy the assembled function holds. These are the numbers a
    /// reader is most likely to check by hand.
    #[test]
    fn textbook_values() {
        for (p, want) in [
            (0.75, 0.674489750196082),
            (0.9, 1.2815515655446004),
            (0.95, 1.6448536269514722),
            (0.975, 1.959963984540054),
            (0.99, 2.3263478740408408),
            (0.995, 2.5758293035489004),
            (0.999, 3.090232306167813),
        ] {
            let got = additive_quantile(p);
            let want: f64 = want;
            let tol = 8.0 * f64::EPSILON * want.abs();
            assert!(
                (got - want).abs() <= tol,
                "Phi^-1({p}) = {got}, expected about {want}"
            );
        }
    }

    /// The central branch is exactly odd about 1/2, because it consumes `q = p - 1/2` directly and
    /// the approximant multiplies it back in.
    ///
    /// The tails cannot be, and that is a property of binary64 rather than of this function: they
    /// work from `min(p, 1-p)`, and `1.0 - 0.95` is `0.050000000000000044` rather than `0.05`, so
    /// the two sides are not handed the same number to begin with. Asserting symmetry there would
    /// be asserting something false.
    ///
    /// The band edge is excluded for the same reason: `0.5 + 0.425` is `0.925`, and subtracting
    /// `0.5` from that returns `0.42500000000000004`, which lands the upper point in the tail
    /// branch while its mirror stays central. Only offsets that survive the round trip are used.
    #[test]
    fn central_branch_is_odd_about_the_center() {
        for q in [0.25, 0.125, 0.0625, 0.4] {
            assert_eq!(
                0.5 + q - 0.5,
                q,
                "the offset itself does not round-trip: {q}"
            );
            assert!(q <= CENTRAL_HALF_WIDTH);
            let hi = additive_quantile(0.5 + q);
            let lo = additive_quantile(0.5 - q);
            assert_eq!(hi, -lo, "asymmetry at q = {q}");
        }
    }

    /// Monotone, and reaching the far tail rather than saturating. The smallest positive double is
    /// the last argument a caller can pass, and its quantile is finite.
    #[test]
    fn monotone_into_the_far_tail() {
        let mut previous = f64::NEG_INFINITY;
        let mut p = 5e-324;
        while p < 0.5 {
            let got = additive_quantile(p);
            assert!(got > previous, "not monotone at p = {p}");
            assert!(got.is_finite(), "not finite at p = {p}");
            previous = got;
            p *= 8.0;
        }
        assert!((additive_quantile(5e-324) - -38.467405617144).abs() < 1e-9);
    }
}
