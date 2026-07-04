//! Tests for error handling and input validation

use pragmastat::assumptions::{AssumptionId, EstimatorError, Subject};
use pragmastat::estimators::raw;

#[test]
fn center_empty_input() {
    assert!(raw::center(&[], false).is_err());
}

#[test]
fn center_nan_input() {
    assert!(raw::center(&[1.0, f64::NAN, 3.0], false).is_err());
}

#[test]
fn center_infinite_input() {
    assert!(raw::center(&[1.0, f64::INFINITY, 3.0], false).is_err());
}

#[test]
fn spread_empty_input() {
    // spread now requires sparity (Spread > 0), which implies at least 2 elements
    assert!(raw::spread(&[], false).is_err());
}

#[test]
fn spread_single_input() {
    // single element fails sparity (Spread = 0)
    assert!(raw::spread(&[5.0], false).is_err());
}

#[test]
fn spread_constant_input() {
    // constant values fail sparity (Spread = 0)
    assert!(raw::spread(&[5.0, 5.0, 5.0], false).is_err());
}

#[test]
fn spread_nan_input() {
    assert!(raw::spread(&[1.0, f64::NAN, 3.0], false).is_err());
}

#[test]
fn spread_infinite_input() {
    assert!(raw::spread(&[1.0, f64::INFINITY, 3.0], false).is_err());
}

#[test]
fn shift_empty_x() {
    assert!(raw::shift(&[], &[1.0, 2.0], false).is_err());
}

#[test]
fn shift_empty_y() {
    assert!(raw::shift(&[1.0, 2.0], &[], false).is_err());
}

#[test]
fn shift_nan_x() {
    assert!(raw::shift(&[1.0, f64::NAN], &[1.0, 2.0], false).is_err());
}

#[test]
fn shift_nan_y() {
    assert!(raw::shift(&[1.0, 2.0], &[f64::NAN, 2.0], false).is_err());
}

#[test]
fn ratio_empty_x() {
    assert!(raw::ratio(&[], &[1.0, 2.0], false).is_err());
}

#[test]
fn ratio_empty_y() {
    assert!(raw::ratio(&[1.0, 2.0], &[], false).is_err());
}

#[test]
fn ratio_nonpositive_y() {
    // y must be strictly positive for ratio calculation
    assert!(raw::ratio(&[1.0, 2.0], &[0.0, 1.0], false).is_err());
    assert!(raw::ratio(&[1.0, 2.0], &[-1.0, 1.0], false).is_err());
}

#[test]
fn disparity_empty_x() {
    assert!(raw::disparity(&[], &[1.0, 2.0], false).is_err());
}

#[test]
fn disparity_empty_y() {
    assert!(raw::disparity(&[1.0, 2.0], &[], false).is_err());
}

#[test]
fn shift_bounds_empty_x() {
    assert!(raw::shift_bounds(&[], &[1.0, 2.0], 0.05, false).is_err());
}

#[test]
fn shift_bounds_empty_y() {
    assert!(raw::shift_bounds(&[1.0, 2.0], &[], 0.05, false).is_err());
}

#[test]
fn shift_bounds_nan_misrate() {
    assert!(raw::shift_bounds(&[1.0, 2.0], &[3.0, 4.0], f64::NAN, false).is_err());
}

#[test]
fn shift_bounds_negative_misrate() {
    assert!(raw::shift_bounds(&[1.0, 2.0], &[3.0, 4.0], -0.1, false).is_err());
}

#[test]
fn shift_bounds_misrate_greater_than_one() {
    assert!(raw::shift_bounds(&[1.0, 2.0], &[3.0, 4.0], 1.5, false).is_err());
}

// --- Helper functions for error testing ---

fn unwrap_estimator_error(err: EstimatorError) -> (AssumptionId, Subject) {
    match err {
        EstimatorError::Assumption(ae) => (ae.violation().id, ae.violation().subject),
        EstimatorError::Other(msg) => panic!("Expected AssumptionError, got Other: {}", msg),
    }
}

#[test]
fn shift_bounds_misrate_below_min() {
    // n=2, m=2: min_misrate = 2/C(4,2) = 1/3 ~ 0.333
    let result = raw::shift_bounds(&[1.0, 2.0], &[3.0, 4.0], 0.05, false);
    assert!(result.is_err());
    let (id, subject) = unwrap_estimator_error(result.unwrap_err());
    assert_eq!(id, AssumptionId::Domain);
    assert_eq!(subject, Subject::Misrate);
}

// --- center_bounds ---

#[test]
fn center_bounds_single_element() {
    let result = raw::center_bounds(&[1.0], 0.05, false);
    assert!(result.is_err());
    let (id, subject) = unwrap_estimator_error(result.unwrap_err());
    assert_eq!(id, AssumptionId::Domain);
    assert_eq!(subject, Subject::X);
}

#[test]
fn center_bounds_invalid_misrate() {
    let result = raw::center_bounds(&[1.0, 2.0, 3.0, 4.0, 5.0], 1e-20, false);
    assert!(result.is_err());
    let (id, subject) = unwrap_estimator_error(result.unwrap_err());
    assert_eq!(id, AssumptionId::Domain);
    assert_eq!(subject, Subject::Misrate);
}

// --- misrate out-of-[0,1]/NaN domain branch (raw/double path) ---
// Covers the misrate domain branch for the raw bounds API.
// Asserts the RAW (native-array, double misrate) bounds API rejects misrate values
// outside [0,1] and NaN with the domain/misrate violation, for both a one-sample
// (center_bounds) and a two-sample (shift_bounds) entry point.

#[test]
fn center_bounds_misrate_out_of_domain() {
    let x = [1.0, 2.0, 3.0, 4.0, 5.0];
    for &misrate in &[2.0, -0.1, f64::NAN] {
        let result = raw::center_bounds(&x, misrate, false);
        assert!(
            result.is_err(),
            "center_bounds should reject misrate {misrate}"
        );
        let (id, subject) = unwrap_estimator_error(result.unwrap_err());
        assert_eq!(id, AssumptionId::Domain, "misrate {misrate}");
        assert_eq!(subject, Subject::Misrate, "misrate {misrate}");
    }
}

#[test]
fn shift_bounds_misrate_out_of_domain() {
    let x = [1.0, 2.0, 3.0, 4.0];
    let y = [5.0, 6.0, 7.0, 8.0];
    for &misrate in &[2.0, -0.1, f64::NAN] {
        let result = raw::shift_bounds(&x, &y, misrate, false);
        assert!(
            result.is_err(),
            "shift_bounds should reject misrate {misrate}"
        );
        let (id, subject) = unwrap_estimator_error(result.unwrap_err());
        assert_eq!(id, AssumptionId::Domain, "misrate {misrate}");
        assert_eq!(subject, Subject::Misrate, "misrate {misrate}");
    }
}
