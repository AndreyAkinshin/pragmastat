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
/// readable. Matching infinities carry matching bits and need no special case.
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
