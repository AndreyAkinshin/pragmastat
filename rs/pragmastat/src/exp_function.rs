//! The exponential this toolkit evaluates, in place of the platform's.

/// Reciprocal of ln 2, emitted by `tests/oracles/fit_exp.py`.
///
/// clippy notices this is `std::f64::consts::LOG2_E` and would rather the constant were named.
/// It is the same f64 either way, and `inv_ln2_is_log2_e` below is the proof. The literal stays
/// because all seven ports carry the block of constants the fitting script printed, verbatim and
/// in the same order, and a reader checking one against another should not have to know which
/// language happens to name which of them.
#[allow(clippy::approx_constant)]
const INV_LN2: f64 = 1.4426950408889634;

/// High half of ln 2. Split so that `k * LN2_HI` is exact: `LN2_HI` carries 32 significant bits
/// and `|k|` needs at most 11, which leaves the product inside the 53 available. Without the
/// split the reduction would lose the low bits of `r`, and `r` is where the accuracy lives.
const LN2_HI: f64 = 0.6931471803691238;

/// Low half of ln 2.
const LN2_LO: f64 = 1.9082149292705877e-10;

/// Returns `2^n` exactly, for the `n` the reduction below can produce.
///
/// Rust has no `ldexp` in std, so the power of two is assembled from its bit pattern: an
/// exponent field of `n + 1023` with a zero significand is the normal number `2^n`, by the
/// definition of the format rather than by any rounding. The other ports call their library
/// equivalent (`math.Ldexp`, `Math.ScaleB`, `Math.scalb`, `math.ldexp`); this is the same value
/// they get.
///
/// The cutoffs in `exp_function` bound `n` to roughly `[-538, 512]`, comfortably inside the
/// normal range. The assertion is here because a hand-built exponent field, unlike a library
/// call, has no defined behavior outside it: it would silently produce a denormal, an
/// infinity or a NaN rather than a wrong-but-obvious answer.
fn pow2(n: i32) -> f64 {
    debug_assert!(
        (-1022..=1023).contains(&n),
        "pow2 exponent outside the normal range: {n}"
    );
    f64::from_bits(((n + 1023) as u64) << 52)
}

/// Computes `e^y`.
///
/// IEEE 754 fixes the result of each arithmetic operation and of the square root, and fixes
/// nothing about the exponential. Conforming libraries disagree in the last bit and do:
/// measured on one Edgeworth crossover, Go's software exp and glibc's return neighboring
/// values, which moved the reported margin by two. Since a margin selects an order statistic,
/// that is a different confidence interval from the same inputs.
///
/// So the exponential cannot be the platform's. This one is built from operations the standard
/// does fix:
///
/// ```text
/// k = floor(y/ln2 + 1/2),  r = (y - k*ln2Hi) - k*ln2Lo,  exp(y) = 2^k * exp(r)
/// ```
///
/// with `exp(r)` on `|r| <= ln2/2` from the polynomial in `tests/oracles/fit_exp.py`. Fitting
/// `(exp(r) - 1 - r)/r^2` rather than `exp(r)` keeps the two leading terms exact and leaves the
/// polynomial supplying a correction below 0.07, so the assembled result stays within a couple
/// of ulp of the true exponential while being reproducible everywhere.
///
/// `floor(x + 1/2)` rather than a rounding function: Go rounds halves away from zero and R
/// rounds them to even, so naming a rounding is naming a disagreement.
///
/// Every product is written as `a * b + c` and never as `a.mul_add(b, c)`, for the reason given
/// at the top of `lib.rs`. `rs:check:fma` is the guard.
pub(crate) fn exp_function(y: f64) -> f64 {
    if y.is_nan() {
        return y;
    }
    // Past these the answer is not in doubt, and stating the cutoffs keeps the reduction from
    // having to produce a k it cannot scale by.
    if y > 709.79 {
        return f64::INFINITY;
    }
    if y < -745.2 {
        return 0.0;
    }

    // The two halves of the reduction are kept apart until the assembly below, which is where
    // the accuracy is: see the comment there.
    let k = (y * INV_LN2 + 0.5).floor();
    let hi = y - k * LN2_HI;
    let lo = k * LN2_LO;
    let r = hi - lo;

    let mut q = 1.6086622436215554e-10;
    q = q * r + 2.0918129454967065e-09;
    q = q * r + 2.5052071109214447e-08;
    q = q * r + 2.755726330147475e-07;
    q = q * r + 2.755731924720479e-06;
    q = q * r + 2.480158733642132e-05;
    q = q * r + 0.00019841269841263304;
    q = q * r + 0.0013888888888879082;
    q = q * r + 0.008333333333333333;
    q = q * r + 0.04166666666666668;
    q = q * r + 0.16666666666666666;
    q = q * r + 0.5;
    // Assembled from `hi` and `lo` rather than from `r`. Writing `1 + r + r*r*q` discards the low
    // bits of `r`: `r` reaches 0.35, adding it to 1 shifts the mantissa two places, and the tail
    // falls off the end. Reconstructing the same sum from the two halves of the reduction keeps
    // them, and costs nothing, being the same operations in a different order. Measured against a
    // 60-digit reference over the band these estimators reach, it halves the worst relative error,
    // from 2.19e-16 to 1.28e-16, and raises the share of correctly rounded results from 72.6% to
    // 90.3%. That is where fdlibm sits, which reaches it with a division this does not need.
    let p = 1.0 - ((lo - r * r * q) - hi);

    // Two scalings rather than one. Splitting k in half keeps the first factor inside the
    // normal range whatever k is, so only the second can denormalize or overflow, and it does
    // so in a single rounding. The division truncates toward zero, as it does in every port.
    let k_int = k as i32;
    let half = k_int / 2;
    (p * pow2(half)) * pow2(k_int - half)
}

#[cfg(test)]
mod tests {
    use super::{INV_LN2, exp_function, pow2};

    /// The claim the `approx_constant` suppression rests on.
    #[test]
    fn inv_ln2_is_log2_e() {
        assert_eq!(INV_LN2.to_bits(), std::f64::consts::LOG2_E.to_bits());
    }

    /// `pow2` must be exact, not merely close, or the reduction's last step introduces a
    /// rounding the other ports do not have. Checked against repeated halving and doubling of
    /// 1.0, which is exact in binary64 while the result stays normal, and so cannot itself be
    /// wrong over this range.
    #[test]
    fn pow2_is_exact() {
        let mut expected = 1.0f64;
        for n in 0..=1023 {
            assert_eq!(pow2(n), expected, "pow2({n})");
            expected *= 2.0;
        }
        let mut expected = 1.0f64;
        for n in (-1022..=0).rev() {
            assert_eq!(pow2(n), expected, "pow2({n})");
            expected /= 2.0;
        }
    }

    /// Self-consistency proves nothing about the reduction being right. Agreement with a
    /// separately written exponential does.
    #[test]
    fn agrees_with_platform_exp() {
        let mut worst_ulp = 0.0f64;
        let mut worst_at = 0.0f64;
        let steps = 200_000;
        for i in 0..=steps {
            let y = -745.0 + (709.0 - (-745.0)) * (i as f64) / (steps as f64);
            let ours = exp_function(y);
            let theirs = y.exp();
            // ulp of the reference value, from the neighbor above it.
            let next = f64::from_bits(theirs.to_bits() + 1);
            let ulp = next - theirs;
            let diff = ((ours - theirs) / ulp).abs();
            if diff > worst_ulp {
                worst_ulp = diff;
                worst_at = y;
            }
        }
        assert!(
            worst_ulp <= 2.0,
            "worst disagreement with the platform exp: {worst_ulp} ulp at y = {worst_at}"
        );
    }

    /// Both margins binary-search an expansion whose density term is this function, under the
    /// assumption that the expansion is monotone in the index. A single inversion here would let
    /// the search return either neighbor, and the two neighbors are different confidence bounds.
    ///
    /// The sweep walks adjacent doubles rather than a grid: an inversion between two representable
    /// arguments is what the search can hit, and a grid steps straight over it. One port suffices
    /// because all seven return identical bits on every argument, which the shared fixture pins.
    #[test]
    fn monotone_across_adjacent_doubles() {
        for start in [-745.0, -100.0, -1.0, -0.3, 0.0, 0.3, 1.0, 100.0, 709.0] {
            let mut y = start;
            let mut previous = exp_function(y);
            for _ in 0..20_000 {
                y = f64::from_bits(if y >= 0.0 {
                    y.to_bits() + 1
                } else {
                    y.to_bits() - 1
                });
                let current = exp_function(y);
                assert!(
                    current >= previous,
                    "exp_function inverted at y = {y}: {current} < {previous}"
                );
                previous = current;
            }
        }
    }

    #[test]
    fn cutoffs_and_nan() {
        assert!(exp_function(f64::NAN).is_nan());
        assert_eq!(exp_function(709.8), f64::INFINITY);
        assert_eq!(exp_function(f64::INFINITY), f64::INFINITY);
        assert_eq!(exp_function(-745.3), 0.0);
        assert_eq!(exp_function(f64::NEG_INFINITY), 0.0);
        assert_eq!(exp_function(0.0), 1.0);
    }
}
