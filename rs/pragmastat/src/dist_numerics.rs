//! Binary64 primitives shared by the distribution estimation models.
//!
//! Every operation is written out explicitly so that all seven language implementations evaluate
//! the same graph and produce identical bits. None of these helpers may be replaced by a fused,
//! vectorized or library variant without a new algorithm identifier.

use crate::distributions::SMALLEST_POSITIVE_SUBNORMAL;

/// Next representable binary64 value toward positive infinity.
pub(crate) fn next_up(value: f64) -> f64 {
    if value.is_nan() || value == f64::INFINITY {
        return value;
    }
    if value == 0.0 {
        return SMALLEST_POSITIVE_SUBNORMAL;
    }

    let bits = value.to_bits() as i64;
    let bits = if value > 0.0 { bits + 1 } else { bits - 1 };
    f64::from_bits(bits as u64)
}

/// Next representable binary64 value toward negative infinity.
pub(crate) fn next_down(value: f64) -> f64 {
    if value.is_nan() || value == f64::NEG_INFINITY {
        return value;
    }
    if value == 0.0 {
        return -SMALLEST_POSITIVE_SUBNORMAL;
    }

    let bits = value.to_bits() as i64;
    let bits = if value > 0.0 { bits - 1 } else { bits + 1 };
    f64::from_bits(bits as u64)
}

/// Overflow-safe midpoint of two finite values.
pub(crate) fn midpoint(a: f64, b: f64) -> f64 {
    if a < 0.0 && 0.0 < b {
        a / 2.0 + b / 2.0
    } else {
        a + (b - a) / 2.0
    }
}

/// Interpolates between `a` and `b`, reproducing both endpoints exactly.
pub(crate) fn lerp(a: f64, b: f64, t: f64) -> f64 {
    if t == 0.0 {
        return a;
    }
    if t == 1.0 {
        return b;
    }
    if a < 0.0 && 0.0 < b {
        (1.0 - t) * a + t * b
    } else {
        a + t * (b - a)
    }
}

/// Ratio clamped to the unit interval. Callers must have proved `0 <= num <= den` and `den > 0`.
///
/// # Panics
/// Panics if the ratio is NaN. This is reachable only through a broken `assume_sorted` promise
/// (undefined behavior): the sample itself already satisfies domain(x) and distinctness(x), so a
/// NaN here is a numerical-conformance failure rather than an assumption violation.
pub(crate) fn unit_ratio(num: f64, den: f64) -> f64 {
    let t = num / den;
    assert!(
        !t.is_nan(),
        "numerical conformance failure: unit_ratio produced NaN"
    );
    t.clamp(0.0, 1.0)
}

/// Smallest integer k with `k >= ceil(n / k)`, computed without forming `k*k`.
pub(crate) fn ceil_sqrt(n: usize) -> usize {
    let mut k = 1usize;
    while k < n.div_ceil(k) {
        k += 1;
    }
    k
}

/// First index i in `[from, to)` with `values[i] >= key`, over an ascending slice.
pub(crate) fn lower_bound(values: &[f64], from: usize, to: usize, key: f64) -> usize {
    let mut lo = from;
    let mut hi = to;
    while lo < hi {
        let mid = lo + (hi - lo) / 2;
        if values[mid] < key {
            lo = mid + 1;
        } else {
            hi = mid;
        }
    }
    lo
}

/// First index i with `values[i] > key`, over an ascending slice.
pub(crate) fn upper_bound(values: &[f64], key: f64) -> usize {
    let mut lo = 0usize;
    let mut hi = values.len();
    while lo < hi {
        let mid = lo + (hi - lo) / 2;
        if values[mid] <= key {
            lo = mid + 1;
        } else {
            hi = mid;
        }
    }
    lo
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn next_up_basic() {
        assert!(next_up(1.0) > 1.0);
        assert_eq!(next_up(0.0), SMALLEST_POSITIVE_SUBNORMAL);
        assert_eq!(next_up(-0.0), SMALLEST_POSITIVE_SUBNORMAL);
        assert!(next_up(f64::NAN).is_nan());
        assert_eq!(next_up(f64::INFINITY), f64::INFINITY);
    }

    #[test]
    fn next_down_basic() {
        assert!(next_down(1.0) < 1.0);
        assert_eq!(next_down(0.0), -SMALLEST_POSITIVE_SUBNORMAL);
        assert_eq!(next_down(-0.0), -SMALLEST_POSITIVE_SUBNORMAL);
        assert!(next_down(f64::NAN).is_nan());
        assert_eq!(next_down(f64::NEG_INFINITY), f64::NEG_INFINITY);
    }

    #[test]
    fn next_up_down_are_inverses() {
        let v = 1.0;
        assert_eq!(next_down(next_up(v)), v);
    }

    #[test]
    fn midpoint_across_zero() {
        assert_eq!(midpoint(-1.0, 1.0), 0.0);
    }

    #[test]
    fn midpoint_same_sign() {
        assert_eq!(midpoint(2.0, 4.0), 3.0);
    }

    #[test]
    fn lerp_endpoints_exact() {
        assert_eq!(lerp(1.0, 2.0, 0.0), 1.0);
        assert_eq!(lerp(1.0, 2.0, 1.0), 2.0);
        assert_eq!(lerp(1.0, 3.0, 0.5), 2.0);
    }

    #[test]
    fn unit_ratio_clamps() {
        assert_eq!(unit_ratio(0.0, 2.0), 0.0);
        assert_eq!(unit_ratio(2.0, 2.0), 1.0);
        assert_eq!(unit_ratio(1.0, 2.0), 0.5);
    }

    #[test]
    #[should_panic(expected = "numerical conformance failure")]
    fn unit_ratio_panics_on_nan() {
        unit_ratio(0.0, 0.0);
    }

    #[test]
    fn ceil_sqrt_matches_hand_values() {
        assert_eq!(ceil_sqrt(1), 1);
        assert_eq!(ceil_sqrt(4), 2);
        assert_eq!(ceil_sqrt(5), 3);
        assert_eq!(ceil_sqrt(9), 3);
        assert_eq!(ceil_sqrt(10), 4);
    }

    #[test]
    fn lower_bound_finds_first_ge() {
        let values = [1.0, 2.0, 2.0, 4.0, 8.0];
        assert_eq!(lower_bound(&values, 0, 5, 2.0), 1);
        assert_eq!(lower_bound(&values, 0, 5, 3.0), 3);
        assert_eq!(lower_bound(&values, 0, 5, 0.0), 0);
        assert_eq!(lower_bound(&values, 0, 5, 9.0), 5);
    }

    #[test]
    fn upper_bound_finds_first_gt() {
        let values = [1.0, 2.0, 2.0, 4.0, 8.0];
        assert_eq!(upper_bound(&values, 2.0), 3);
        assert_eq!(upper_bound(&values, 0.0), 0);
        assert_eq!(upper_bound(&values, 8.0), 5);
    }
}
