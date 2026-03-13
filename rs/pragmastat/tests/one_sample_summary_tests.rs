//! Tests for the `one_sample_summary` and `one_sample_summary_with_seed` APIs.
//!
//! Verifies that batch results match individually-called estimators and
//! exercises error paths (small n, non-finite input, misrate out of range,
//! zero-spread data).

use pragmastat::assumptions::{AssumptionId, EstimatorError, Subject};
use pragmastat::estimators::raw;

const TOL: f64 = 1e-9;

fn approx_eq(a: f64, b: f64) -> bool {
    (a - b).abs() < TOL
}

// ---------------------------------------------------------------------------
// Batch vs. individual consistency
// ---------------------------------------------------------------------------

#[test]
fn summary_matches_individual_estimators() {
    let x: Vec<f64> = (1..=30).map(|i| i as f64).collect();
    let misrate = 0.05;
    let seed = "test-seed";

    let summary = raw::one_sample_summary_with_seed(&x, misrate, seed).unwrap();

    let center_val = raw::center(&x).unwrap();
    let spread_val = raw::spread(&x).unwrap();
    let center_bounds = raw::center_bounds(&x, misrate).unwrap();
    let spread_bounds = raw::spread_bounds_with_seed(&x, misrate, seed).unwrap();

    assert!(
        approx_eq(summary.center, center_val),
        "center mismatch: {} vs {}",
        summary.center,
        center_val
    );
    assert!(
        approx_eq(summary.spread, spread_val),
        "spread mismatch: {} vs {}",
        summary.spread,
        spread_val
    );
    assert!(
        approx_eq(summary.center_bounds.lower, center_bounds.lower),
        "center_bounds.lower mismatch: {} vs {}",
        summary.center_bounds.lower,
        center_bounds.lower
    );
    assert!(
        approx_eq(summary.center_bounds.upper, center_bounds.upper),
        "center_bounds.upper mismatch: {} vs {}",
        summary.center_bounds.upper,
        center_bounds.upper
    );
    assert!(
        approx_eq(summary.spread_bounds.lower, spread_bounds.lower),
        "spread_bounds.lower mismatch: {} vs {}",
        summary.spread_bounds.lower,
        spread_bounds.lower
    );
    assert!(
        approx_eq(summary.spread_bounds.upper, spread_bounds.upper),
        "spread_bounds.upper mismatch: {} vs {}",
        summary.spread_bounds.upper,
        spread_bounds.upper
    );
}

#[test]
fn summary_matches_individual_larger_sample() {
    let x: Vec<f64> = (1..=20).map(|i| i as f64).collect();
    let misrate = 0.01;
    let seed = "larger-seed";

    let summary = raw::one_sample_summary_with_seed(&x, misrate, seed).unwrap();

    let center_val = raw::center(&x).unwrap();
    let spread_val = raw::spread(&x).unwrap();
    let center_bounds = raw::center_bounds(&x, misrate).unwrap();
    let spread_bounds = raw::spread_bounds_with_seed(&x, misrate, seed).unwrap();

    assert!(
        approx_eq(summary.center, center_val),
        "center mismatch: {} vs {}",
        summary.center,
        center_val
    );
    assert!(
        approx_eq(summary.spread, spread_val),
        "spread mismatch: {} vs {}",
        summary.spread,
        spread_val
    );
    assert!(
        approx_eq(summary.center_bounds.lower, center_bounds.lower),
        "center_bounds.lower mismatch"
    );
    assert!(
        approx_eq(summary.center_bounds.upper, center_bounds.upper),
        "center_bounds.upper mismatch"
    );
    assert!(
        approx_eq(summary.spread_bounds.lower, spread_bounds.lower),
        "spread_bounds.lower mismatch"
    );
    assert!(
        approx_eq(summary.spread_bounds.upper, spread_bounds.upper),
        "spread_bounds.upper mismatch"
    );
}

#[test]
fn summary_matches_individual_n3_minimum() {
    // n=3 is the smallest valid input for one_sample_summary
    let x = [1.0, 2.0, 4.0];
    let misrate = 1.0; // Use maximum misrate to avoid min_misrate issues

    let summary = raw::one_sample_summary_with_seed(&x, misrate, "n3-seed").unwrap();

    let center_val = raw::center(&x).unwrap();
    let spread_val = raw::spread(&x).unwrap();

    assert!(
        approx_eq(summary.center, center_val),
        "center mismatch at n=3: {} vs {}",
        summary.center,
        center_val
    );
    assert!(
        approx_eq(summary.spread, spread_val),
        "spread mismatch at n=3: {} vs {}",
        summary.spread,
        spread_val
    );
}

// ---------------------------------------------------------------------------
// Error paths: small n
// ---------------------------------------------------------------------------

#[test]
fn summary_empty_input() {
    let err = raw::one_sample_summary(&[], 0.05).unwrap_err();
    assert!(
        matches!(err, EstimatorError::Assumption(ref ae) if ae.violation().id == AssumptionId::Validity)
    );
}

#[test]
fn summary_single_element() {
    let err = raw::one_sample_summary(&[1.0], 0.05).unwrap_err();
    assert!(
        matches!(err, EstimatorError::Assumption(ref ae) if ae.violation().id == AssumptionId::Domain)
    );
}

#[test]
fn summary_n2_rejected() {
    // n=2 must be rejected because the presorted algorithms require n >= 3
    let err = raw::one_sample_summary(&[1.0, 2.0], 1.0).unwrap_err();
    assert!(
        matches!(err, EstimatorError::Assumption(ref ae) if ae.violation().id == AssumptionId::Domain
            && ae.violation().subject == Subject::X),
        "n=2 should produce domain error for x, got: {err:?}"
    );
}

// ---------------------------------------------------------------------------
// Error paths: non-finite input
// ---------------------------------------------------------------------------

#[test]
fn summary_nan_input() {
    let err = raw::one_sample_summary(&[1.0, f64::NAN, 3.0], 0.05).unwrap_err();
    assert!(
        matches!(err, EstimatorError::Assumption(ref ae) if ae.violation().id == AssumptionId::Validity)
    );
}

#[test]
fn summary_infinite_input() {
    let err = raw::one_sample_summary(&[1.0, f64::INFINITY, 3.0], 0.05).unwrap_err();
    assert!(
        matches!(err, EstimatorError::Assumption(ref ae) if ae.violation().id == AssumptionId::Validity)
    );
}

// ---------------------------------------------------------------------------
// Error paths: misrate
// ---------------------------------------------------------------------------

#[test]
fn summary_nan_misrate() {
    let x = [1.0, 2.0, 3.0, 4.0, 5.0];
    let err = raw::one_sample_summary(&x, f64::NAN).unwrap_err();
    assert!(
        matches!(err, EstimatorError::Assumption(ref ae) if ae.violation().id == AssumptionId::Domain
        && ae.violation().subject == Subject::Misrate)
    );
}

#[test]
fn summary_negative_misrate() {
    let x = [1.0, 2.0, 3.0, 4.0, 5.0];
    let err = raw::one_sample_summary(&x, -0.1).unwrap_err();
    assert!(
        matches!(err, EstimatorError::Assumption(ref ae) if ae.violation().id == AssumptionId::Domain
        && ae.violation().subject == Subject::Misrate)
    );
}

#[test]
fn summary_misrate_too_large() {
    let x = [1.0, 2.0, 3.0, 4.0, 5.0];
    let err = raw::one_sample_summary(&x, 1.5).unwrap_err();
    assert!(
        matches!(err, EstimatorError::Assumption(ref ae) if ae.violation().id == AssumptionId::Domain
        && ae.violation().subject == Subject::Misrate)
    );
}

#[test]
fn summary_misrate_below_minimum() {
    // n=3 has a relatively high minimum misrate; a very small misrate should fail
    let x = [1.0, 2.0, 3.0];
    let err = raw::one_sample_summary(&x, 1e-10).unwrap_err();
    assert!(
        matches!(err, EstimatorError::Assumption(ref ae) if ae.violation().id == AssumptionId::Domain
        && ae.violation().subject == Subject::Misrate)
    );
}

// ---------------------------------------------------------------------------
// Error paths: sparity (zero-spread data)
// ---------------------------------------------------------------------------

#[test]
fn summary_constant_data() {
    let x = [5.0, 5.0, 5.0, 5.0, 5.0];
    let err = raw::one_sample_summary(&x, 1.0).unwrap_err();
    assert!(
        matches!(err, EstimatorError::Assumption(ref ae) if ae.violation().id == AssumptionId::Sparity),
        "constant data should produce sparity error, got: {err:?}"
    );
}

// ---------------------------------------------------------------------------
// Seeded vs. unseeded API
// ---------------------------------------------------------------------------

#[test]
fn summary_with_seed_is_deterministic() {
    let x: Vec<f64> = (1..=30).map(|i| i as f64).collect();
    let misrate = 0.05;
    let seed = "determinism-check";

    let s1 = raw::one_sample_summary_with_seed(&x, misrate, seed).unwrap();
    let s2 = raw::one_sample_summary_with_seed(&x, misrate, seed).unwrap();

    assert_eq!(s1.center, s2.center);
    assert_eq!(s1.spread, s2.spread);
    assert_eq!(s1.center_bounds.lower, s2.center_bounds.lower);
    assert_eq!(s1.center_bounds.upper, s2.center_bounds.upper);
    assert_eq!(s1.spread_bounds.lower, s2.spread_bounds.lower);
    assert_eq!(s1.spread_bounds.upper, s2.spread_bounds.upper);
}
