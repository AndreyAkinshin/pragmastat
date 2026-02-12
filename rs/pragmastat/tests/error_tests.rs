//! Tests for error handling and input validation

use pragmastat::assumptions::{AssumptionId, EstimatorError, Subject};
use pragmastat::estimators::{
    avg_spread, center, center_bounds, disparity, median, ratio, rel_spread, shift, shift_bounds,
    spread,
};

#[test]
fn median_empty_input() {
    let result = median(&[]);
    assert!(result.is_err());
    let (id, subject) = unwrap_estimator_error(result.unwrap_err());
    assert_eq!(id, AssumptionId::Validity);
    assert_eq!(subject, Subject::X);
}

#[test]
fn median_nan_input() {
    assert!(median(&[1.0, f64::NAN, 3.0]).is_err());
}

#[test]
fn median_infinite_input() {
    assert!(median(&[1.0, f64::INFINITY, 3.0]).is_err());
}

#[test]
fn center_empty_input() {
    assert!(center(&[]).is_err());
}

#[test]
fn center_nan_input() {
    assert!(center(&[1.0, f64::NAN, 3.0]).is_err());
}

#[test]
fn center_infinite_input() {
    assert!(center(&[1.0, f64::INFINITY, 3.0]).is_err());
}

#[test]
fn spread_empty_input() {
    // spread now requires sparity (Spread > 0), which implies at least 2 elements
    assert!(spread(&[]).is_err());
}

#[test]
fn spread_single_input() {
    // single element fails sparity (Spread = 0)
    assert!(spread(&[5.0]).is_err());
}

#[test]
fn spread_constant_input() {
    // constant values fail sparity (Spread = 0)
    assert!(spread(&[5.0, 5.0, 5.0]).is_err());
}

#[test]
fn spread_nan_input() {
    assert!(spread(&[1.0, f64::NAN, 3.0]).is_err());
}

#[test]
fn spread_infinite_input() {
    assert!(spread(&[1.0, f64::INFINITY, 3.0]).is_err());
}

#[test]
fn rel_spread_empty_input() {
    assert!(rel_spread(&[]).is_err());
}

#[test]
fn rel_spread_zero_center() {
    // Values centered around zero: spread/|center| is undefined
    assert!(rel_spread(&[-1.0, 0.0, 1.0]).is_err());
}

#[test]
fn shift_empty_x() {
    assert!(shift(&[], &[1.0, 2.0]).is_err());
}

#[test]
fn shift_empty_y() {
    assert!(shift(&[1.0, 2.0], &[]).is_err());
}

#[test]
fn shift_nan_x() {
    assert!(shift(&[1.0, f64::NAN], &[1.0, 2.0]).is_err());
}

#[test]
fn shift_nan_y() {
    assert!(shift(&[1.0, 2.0], &[f64::NAN, 2.0]).is_err());
}

#[test]
fn ratio_empty_x() {
    assert!(ratio(&[], &[1.0, 2.0]).is_err());
}

#[test]
fn ratio_empty_y() {
    assert!(ratio(&[1.0, 2.0], &[]).is_err());
}

#[test]
fn ratio_nonpositive_y() {
    // y must be strictly positive for ratio calculation
    assert!(ratio(&[1.0, 2.0], &[0.0, 1.0]).is_err());
    assert!(ratio(&[1.0, 2.0], &[-1.0, 1.0]).is_err());
}

#[test]
fn avg_spread_empty_x() {
    assert!(avg_spread(&[], &[1.0, 2.0]).is_err());
}

#[test]
fn avg_spread_empty_y() {
    assert!(avg_spread(&[1.0, 2.0], &[]).is_err());
}

#[test]
fn disparity_empty_x() {
    assert!(disparity(&[], &[1.0, 2.0]).is_err());
}

#[test]
fn disparity_empty_y() {
    assert!(disparity(&[1.0, 2.0], &[]).is_err());
}

#[test]
fn shift_bounds_empty_x() {
    assert!(shift_bounds(&[], &[1.0, 2.0], 0.05).is_err());
}

#[test]
fn shift_bounds_empty_y() {
    assert!(shift_bounds(&[1.0, 2.0], &[], 0.05).is_err());
}

#[test]
fn shift_bounds_nan_misrate() {
    assert!(shift_bounds(&[1.0, 2.0], &[3.0, 4.0], f64::NAN).is_err());
}

#[test]
fn shift_bounds_negative_misrate() {
    assert!(shift_bounds(&[1.0, 2.0], &[3.0, 4.0], -0.1).is_err());
}

#[test]
fn shift_bounds_misrate_greater_than_one() {
    assert!(shift_bounds(&[1.0, 2.0], &[3.0, 4.0], 1.5).is_err());
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
    // n=2, m=2: min_misrate = 2/C(4,2) = 1/3 ≈ 0.333
    let result = shift_bounds(&[1.0, 2.0], &[3.0, 4.0], 0.05);
    assert!(result.is_err());
    let (id, subject) = unwrap_estimator_error(result.unwrap_err());
    assert_eq!(id, AssumptionId::Domain);
    assert_eq!(subject, Subject::Misrate);
}

// --- center_bounds ---

#[test]
fn center_bounds_single_element() {
    let result = center_bounds(&[1.0], 0.05);
    assert!(result.is_err());
    let (id, subject) = unwrap_estimator_error(result.unwrap_err());
    assert_eq!(id, AssumptionId::Domain);
    assert_eq!(subject, Subject::X);
}

#[test]
fn center_bounds_invalid_misrate() {
    let result = center_bounds(&[1.0, 2.0, 3.0, 4.0, 5.0], 1e-20);
    assert!(result.is_err());
    let (id, subject) = unwrap_estimator_error(result.unwrap_err());
    assert_eq!(id, AssumptionId::Domain);
    assert_eq!(subject, Subject::Misrate);
}
