//! Exact comparison for the fixture loaders that live inside the crate.
//!
//! The counterpart of `tests/conformance/mod.rs`, which serves the integration
//! tests; a crate-internal test module cannot reach into a `tests/` target, so the
//! predicate exists once on each side rather than once per file.
//!
//! Every suite loaded from `src` (`avg-spread`, `avg-spread-bounds`,
//! `disparity-bounds`) is selection-based: its result is an element of the
//! pairwise set, or the midpoint of two of them. A divergence is therefore never
//! a small error. Either the same element was selected and the answer is
//! bit-identical, or a different one was, and the gap is data-dependent and
//! unbounded by any epsilon. Perturbing every `log`, `exp`, `pow` and `cos` call
//! to the neighbouring representable value (the largest legitimate difference
//! between two conforming libm implementations) does not move any of them.

/// Compares two binary64 payloads. Returns the failure text on a mismatch,
/// including both bit patterns: a one-ULP report is only worth having if it is
/// readable, and a sign-of-zero report is unreadable without them. Matching
/// infinities carry matching bits and need no special case.
///
/// Payloads rather than `==`, which compares numbers: `-0.0 == 0.0` holds and
/// `NaN == NaN` does not, so `==` passes a divergence in the sign of a zero and
/// fails a pair of identical NaNs. Neither predicate is the stronger one; the one
/// that matches the claim these suites make is bit equality.
pub(crate) fn bitwise_mismatch(what: &str, expected: f64, actual: f64) -> Option<String> {
    if actual.to_bits() == expected.to_bits() {
        return None;
    }
    Some(format!(
        "{what}: expected {expected} (0x{:016X}), got {actual} (0x{:016X})",
        expected.to_bits(),
        actual.to_bits()
    ))
}

/// Assert form of [`bitwise_mismatch`], for the tests that check one pair on the
/// spot instead of collecting failures across a fixture directory.
#[track_caller]
pub(crate) fn assert_bits_eq(what: &str, actual: f64, expected: f64) {
    if let Some(mismatch) = bitwise_mismatch(what, expected, actual) {
        panic!("{mismatch}");
    }
}

/// Sequence counterpart of [`assert_bits_eq`], element by element.
///
/// A slice compared with `==` has the same blind spots as a scalar compared with
/// `==`, once per element, so a sequence gets the same predicate and the same
/// message rather than its own.
#[track_caller]
pub(crate) fn assert_bits_eq_slice(what: &str, actual: &[f64], expected: &[f64]) {
    assert_eq!(
        actual.len(),
        expected.len(),
        "{what}: expected {} values, got {}",
        expected.len(),
        actual.len()
    );
    for (i, (&actual_val, &expected_val)) in actual.iter().zip(expected.iter()).enumerate() {
        assert_bits_eq(&format!("{what}[{i}]"), actual_val, expected_val);
    }
}
