//! How the fixture loaders compare floating-point results.
//!
//! Shared by every integration test that reads `tests/**/*.json`, so that "exact"
//! means one thing in this crate rather than one thing per file. Each test binary
//! links only the parts it uses, which is what the allow below is for: a
//! suite that is exact end to end never constructs `Tolerant`, and that is the
//! intended state, not dead code to clean up.
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
/// with `log`, `exp`, `pow` and `cos` each returning the neighbouring
/// representable value (the largest difference two conforming libm
/// implementations may legitimately have) leaves these suites unmoved on every
/// fixture.
///
/// `Tolerant` survives only where that perturbation does move the result: `ratio`
/// is `exp(median(log x - log y))`, which shifts on 94% of the fixtures by up to
/// 16 ULP. `compare2` composes ratio projections next to exact ones, and a
/// per-suite mode cannot express "exact for shift, approximate for ratio", so it
/// stays tolerant as a whole.
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
    /// a one-ULP report is only worth having if it is readable.
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
