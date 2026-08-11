//! Binary64 primitives shared by the distribution estimation models.
//!
//! Every operation is written out explicitly so that all seven language implementations evaluate
//! the same graph and produce identical bits. None of these helpers may be replaced by a fused,
//! vectorized or library variant without a new algorithm identifier.

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
    fn upper_bound_finds_first_gt() {
        let values = [1.0, 2.0, 2.0, 4.0, 8.0];
        assert_eq!(upper_bound(&values, 2.0), 3);
        assert_eq!(upper_bound(&values, 0.0), 0);
        assert_eq!(upper_bound(&values, 8.0), 5);
    }
}
