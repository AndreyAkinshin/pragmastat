//! Standard normal CDF.

use crate::exp_function::exp_function;

/// Computes the standard normal CDF: `P(Z <= x)` for a standard normal `Z`.
///
/// Two Chebyshev-fitted Horner chains and one exponential. The coefficients are produced by
/// `tests/oracles/fit_additive_cumulative.py` against a reference good to 36 digits, so they are
/// reproducible rather than transcribed.
///
/// Every product is written as `p * u + c` and never as `p.mul_add(u, c)`. rustc does not
/// contract on its own, so the two roundings this spells out are the two roundings that run,
/// which is what the other six implementations do. `rs:check:fma` keeps it that way.
pub fn additive_cumulative(z: f64) -> f64 {
    // NaN satisfies neither comparison below, so without this it would leave the tail branch
    // as a finite 0 or 1 and an undefined input would be answered rather than reported.
    if z.is_nan() {
        return z;
    }
    let t = z.abs() / std::f64::consts::SQRT_2;
    if t < 0.5 {
        // erf(t)/t as a polynomial in u = 8*t^2 - 1.
        let s = t * t;
        let u = 8.0 * s - 1.0;
        let mut p = -1.2757552949301143e-19;
        p = p * u + 1.2307154179828511e-17;
        p = p * u - 1.0890239994332592e-15;
        p = p * u + 8.774530700097397e-14;
        p = p * u - 6.3744178527620835e-12;
        p = p * u + 4.1270254211564467e-10;
        p = p * u - 2.347229163519518e-08;
        p = p * u + 1.151603779513705e-06;
        p = p * u - 4.762336934468491e-05;
        p = p * u + 0.0016130716680617086;
        p = p * u - 0.04364205888669792;
        p = p * u + 1.0830752376761712;
        let erf = t * p;
        return if z >= 0.0 {
            0.5 * (1.0 + erf)
        } else {
            0.5 * (1.0 - erf)
        };
    }

    // erfc(t) for 0.5 <= t <= 4.3, as exp(-t^2) times a polynomial in u = (2t - 4.8) / 3.8.
    // Past t = 4.3 the tail is below the resolution of the result and is taken as zero.
    let mut erfc = 0.0;
    if t <= 4.3 {
        let u = (2.0 * t - 4.8) / 3.8;
        let mut p = 2.403093649825437e-09;
        p = p * u - 6.533436159455495e-09;
        p = p * u + 1.334437871983186e-09;
        p = p * u - 2.5055474016226743e-09;
        p = p * u + 5.2376178949357336e-08;
        p = p * u - 1.341394638617228e-07;
        p = p * u + 2.5376572107855777e-07;
        p = p * u - 6.147631059669139e-07;
        p = p * u + 1.561533370779237e-06;
        p = p * u - 3.688982809059467e-06;
        p = p * u + 8.492013869441648e-06;
        p = p * u - 1.9344330869926753e-05;
        p = p * u + 4.3285002216779125e-05;
        p = p * u - 9.489727696113043e-05;
        p = p * u + 0.0002037912849869451;
        p = p * u - 0.0004282777524202283;
        p = p * u + 0.00087969639542425;
        p = p * u - 0.001763698443638436;
        p = p * u + 0.0034462452415540026;
        p = p * u - 0.00655166763664565;
        p = p * u + 0.012094345026186722;
        p = p * u - 0.021629099761798037;
        p = p * u + 0.037371670355588804;
        p = p * u - 0.06218492139115531;
        p = p * u + 0.09925390090168178;
        p = p * u - 0.15121195850373031;
        p = p * u + 0.21849873453703333;
        erfc = exp_function(-(t * t)) * p;
    }

    if z >= 0.0 {
        1.0 - 0.5 * erfc
    } else {
        0.5 * erfc
    }
}

#[cfg(test)]
mod tests {
    use super::additive_cumulative;
    use crate::conformance::bitwise_mismatch;

    /// Payloads, not `==`. Every zero-valued expectation below would pass on a negative zero under
    /// `assert_eq!`, which is the exact divergence class these suites exist to reject, and the
    /// other six ports all use their payload predicate here.
    fn assert_bitwise(what: &str, expected: f64, actual: f64) {
        if let Some(message) = bitwise_mismatch(what, expected, actual) {
            panic!("{message}");
        }
    }

    /// Both of the function's range comparisons are false for a NaN, so without an explicit guard
    /// it leaves the tail branch as a finite 0 or 1: an undefined input answered rather than
    /// reported. The approximation this replaced propagated NaN.
    #[test]
    fn carries_nan_through_and_answers_both_infinities() {
        assert!(additive_cumulative(f64::NAN).is_nan());
        assert_bitwise(
            "additive_cumulative(+inf)",
            1.0,
            additive_cumulative(f64::INFINITY),
        );
        assert_bitwise(
            "additive_cumulative(-inf)",
            0.0,
            additive_cumulative(f64::NEG_INFINITY),
        );
        assert_bitwise("additive_cumulative(+0)", 0.5, additive_cumulative(0.0));
        assert_bitwise("additive_cumulative(-0)", 0.5, additive_cumulative(-0.0));
    }
}
