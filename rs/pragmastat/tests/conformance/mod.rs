//! How the fixture loaders compare floating-point results.
//!
//! Shared by every integration test that compares one floating-point result
//! against another, whether it comes from `tests/**/*.json` or from a second call
//! into the crate, so that "exact" means one thing in this crate rather than one
//! thing per file. Each test binary links only the parts it uses, which is what
//! the allow below is for: a suite that is exact end to end never constructs
//! `Tolerant`, and that is the intended state, not dead code to clean up.
#![allow(dead_code)]

use float_cmp::approx_eq;

/// How strictly a fixture suite compares a floating-point result.
///
/// `Exact` is the rule, not the exception. A selection-based estimator returns an
/// element of the pairwise set (or the midpoint of two of them), so a divergence
/// is never a small error: either the same element was selected and the answer is
/// bit-identical, or a different one was, and then the gap is data-dependent and
/// no epsilon bounds it. A tolerance there hides exactly the failure it looks like
/// it guards against. This is measured, not assumed: recomputing every estimator
/// with `log`, `exp`, `pow` and `cos` each returning the neighboring
/// representable value (the largest difference two conforming libm
/// implementations may legitimately have) leaves these suites unmoved on every
/// fixture.
///
/// `Tolerant` survives only where that perturbation does move the result: `ratio`
/// is `exp(median(log x - log y))`, which shifts on 94% of the fixtures by up to
/// 16 ULP. `compare2` composes ratio projections next to exact ones, so it picks
/// the class per projection from that projection's threshold metric: the ratio
/// ones are tolerant, and the shift and disparity ones beside them stay exact.
#[derive(Clone, Copy)]
pub enum Conformance {
    Exact,
    Tolerant,
}

impl Conformance {
    pub fn matches(self, actual: f64, expected: f64) -> bool {
        match self {
            // Raw binary64 payloads. Matching infinities carry matching bits, so
            // the tolerant path's infinity special case needs no counterpart here.
            Conformance::Exact => actual.to_bits() == expected.to_bits(),
            Conformance::Tolerant => {
                approx_eq!(f64, actual, expected, epsilon = 1e-9)
                    || (actual.is_infinite()
                        && expected.is_infinite()
                        && actual.signum() == expected.signum())
            }
        }
    }

    /// Renders a mismatch. Under `Exact` the bit patterns are part of the message:
    /// a one-ULP report is only worth having if it is readable, and a sign-of-zero
    /// report is unreadable without them.
    pub fn mismatch(self, what: &str, expected: f64, actual: f64) -> String {
        match self {
            Conformance::Exact => format!(
                "{what}: expected {expected} (0x{:016X}), got {actual} (0x{:016X})",
                expected.to_bits(),
                actual.to_bits()
            ),
            Conformance::Tolerant => format!("{what}: expected {expected}, got {actual}"),
        }
    }
}

/// Asserts that two binary64 values are the same to the last bit.
///
/// The assert form of [`Conformance::Exact`], for the tests that compare a single
/// pair rather than collecting failures across a fixture directory. Every such
/// test routes through here rather than through `assert_eq!`, which compares
/// numbers and not payloads: `-0.0 == 0.0` holds and `NaN == NaN` does not, so
/// `==` both passes a sign-of-zero divergence and fails a pair of identical NaNs.
/// Neither predicate is the stronger one; the one that matches the claim these
/// tests make is bit equality.
#[track_caller]
pub fn assert_bits_eq(what: &str, actual: f64, expected: f64) {
    assert!(
        Conformance::Exact.matches(actual, expected),
        "{}",
        Conformance::Exact.mismatch(what, expected, actual)
    );
}

/// Sequence counterpart of [`assert_bits_eq`], element by element.
///
/// A slice compared with `==` has the same blind spots as a scalar compared with
/// `==`, once per element, so a sequence gets the same predicate and the same
/// message rather than its own.
#[track_caller]
pub fn assert_bits_eq_slice(what: &str, actual: &[f64], expected: &[f64]) {
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

/// The binary32 counterpart of [`assert_bits_eq_slice`].
///
/// `Conformance` is binary64 throughout because every estimator is; the single
/// non-double that crosses a fixture boundary is the RNG's f32 draw stream, whose
/// contract is bit-identical draws in every language. It gets the payload
/// comparison and the hex report for the same reason, in the same place, with the
/// narrower payload spelled out at its own width.
#[track_caller]
pub fn assert_bits_eq_f32_slice(what: &str, actual: &[f32], expected: &[f32]) {
    assert_eq!(
        actual.len(),
        expected.len(),
        "{what}: expected {} values, got {}",
        expected.len(),
        actual.len()
    );
    for (i, (&actual_val, &expected_val)) in actual.iter().zip(expected.iter()).enumerate() {
        assert!(
            actual_val.to_bits() == expected_val.to_bits(),
            "{what}[{i}]: expected {expected_val} (0x{:08X}), got {actual_val} (0x{:08X})",
            expected_val.to_bits(),
            actual_val.to_bits()
        );
    }
}
