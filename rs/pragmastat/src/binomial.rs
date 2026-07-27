//! Binomial coefficients in binary64

/// Below this total C(n, k) is computed as an exact integer, at or above it by the
/// recurrence.
///
/// The threshold is where 64-bit signed arithmetic would overflow, not where this
/// crate's would: `exact` accumulates in u128 and stays exact well past n = 100. It
/// switches here anyway because the other six implementations switch here, and a
/// binomial that is exact in one language and 3 ULP off in another is a divergence in
/// the misrate floor. Raising it is not an optimization.
const MAX_ACCEPTABLE_BINOM_N: usize = 62;

/// Computes C(n, k) as a binary64 value: exact integer arithmetic while it fits,
/// the multiplicative recurrence C(n, k) = prod_{i=1..k} (n-k+i)/i beyond it.
///
/// The recurrence replaced an exp-of-Stirling formulation, for two reasons. The
/// specification defines the admissible misrate as `misrate >= 2 / C(n+m, n)`, an exact
/// integer quantity; measured against exact big-integer binomials over the range this
/// path actually serves, 62 <= n <= 400, Stirling was inexact on 99.9% of cases with a
/// worst relative error of 8.1e-9, while the recurrence is inexact on 77.4% with a worst
/// relative error of 2.2e-15. And Stirling reached the answer through `ln` and `exp`,
/// which every language takes from a different libm, so its last bits were never
/// contractual. The recurrence calls nothing: every step is one multiply and one divide,
/// in that order.
///
/// Both call sites in this crate ask for the same number under different names,
/// `C(n+m, n)` from the misrate floor and `C(n+m, m)` from the exact Loeffler pass, and
/// at the floor the comparison `1/total >= misrate/2` is an exact tie between the two.
/// So they have to agree bit for bit: one entry point, one threshold, and k normalized
/// to the smaller half on both paths. When each computed its own binomial its own way,
/// 28412 of the 79800 sample-size pairs with n+m <= 400 returned a non-zero margin at
/// their own misrate floor.
pub(crate) fn binomial_coefficient(n: usize, k: usize) -> f64 {
    if k > n {
        return 0.0;
    }
    if n < MAX_ACCEPTABLE_BINOM_N {
        exact(n, k)
    } else {
        recurrence(n, k)
    }
}

/// Exact C(n, k) in 128-bit integer arithmetic. Every partial product is a binomial
/// coefficient times a factor below it, so each division is exact.
fn exact(n: usize, k: usize) -> f64 {
    let k = k.min(n - k);
    let mut result: u128 = 1;
    for i in 0..k {
        result = result * (n - i) as u128 / (i + 1) as u128;
    }
    result as f64
}

/// C(n, k) by the multiplicative recurrence in binary64.
///
/// One multiply and one divide per step, in that order, and nothing else: no separate
/// numerator and denominator, no reassociation, no fused multiply-add. Seven
/// implementations perform this identical sequence.
///
/// That sequence overflows early, and deliberately so. The intermediate `acc * (n-k+i)`
/// runs up to n/2 times above the final value, so the central binomial returns +Inf from
/// n = 1021 while the exact value stays inside binary64 until n = 1030. Across that
/// window the misrate floor `2/C` collapses from around 7e-307 to 0, which only a
/// misrate below 1e-306 could tell apart, and every one of those is dominated by the
/// one-sample floor anyway. Deferring the divide to keep the intermediate small would be
/// a different sequence of roundings, and only this one is shared.
fn recurrence(n: usize, k: usize) -> f64 {
    let k = k.min(n - k);
    let mut acc = 1.0;
    for i in 1..=k {
        acc = acc * (n - k + i) as f64 / i as f64;
    }
    acc
}

#[cfg(test)]
mod tests {
    use super::binomial_coefficient;
    use crate::conformance::assert_bits_eq;

    /// C(n, k) and C(n, n-k) are the same number, so they have to be the same bits.
    /// The misrate floor compares one against the other and a one-ULP gap decides it.
    #[test]
    fn symmetric_in_k() {
        for n in 0..=400usize {
            for k in 0..=n {
                assert_bits_eq(
                    &format!("C({n}, {}) against C({n}, {k})", n - k),
                    binomial_coefficient(n, n - k),
                    binomial_coefficient(n, k),
                );
            }
        }
    }

    /// The returned f64 is the contract, so these pin payloads rather than numbers:
    /// `assert_eq!` would accept a -0.0 for the k > n case, which is a different
    /// binary64 value than the one the other six implementations return.
    #[test]
    fn small_values() {
        assert_bits_eq("C(0, 0)", binomial_coefficient(0, 0), 1.0);
        assert_bits_eq("C(5, 0)", binomial_coefficient(5, 0), 1.0);
        assert_bits_eq("C(5, 5)", binomial_coefficient(5, 5), 1.0);
        assert_bits_eq("C(5, 2)", binomial_coefficient(5, 2), 10.0);
        assert_bits_eq("C(5, 6)", binomial_coefficient(5, 6), 0.0);
    }

    /// Pins both sides of the switch from exact integers to the recurrence.
    ///
    /// C(62, 31) is 465428353255261088, and the recurrence returns a value 3 ULP above
    /// it. That is not a defect to be tuned away: the returned f64 is the contract, and
    /// seven implementations running this exact sequence of multiplies and divides all
    /// return this one. An accuracy fix here is a cross-language divergence.
    #[test]
    fn across_the_threshold() {
        assert_bits_eq(
            "C(61, 30)",
            binomial_coefficient(61, 30),
            2.3271417662763053e17,
        );
        assert_bits_eq(
            "C(62, 31)",
            binomial_coefficient(62, 31),
            4.6542835325526125e17,
        );
    }
}
