//! Tests for error handling and input validation

use pragmastat::assumptions::{AssumptionId, EstimatorError, Subject};
use pragmastat::estimators::{
    avg_spread, center, disparity, median, ratio, rel_spread, shift, shift_bounds, spread,
};
use pragmastat::pairwise_margin::pairwise_margin;
use pragmastat::signed_rank_margin::signed_rank_margin;

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

fn unwrap_assumption(err: pragmastat::assumptions::AssumptionError) -> (AssumptionId, Subject) {
    (err.violation().id, err.violation().subject)
}

// --- pairwise_margin ---

#[test]
fn pairwise_margin_zero_n() {
    let result = pairwise_margin(0, 10, 0.05);
    assert!(result.is_err());
    let (id, subject) = unwrap_assumption(result.unwrap_err());
    assert_eq!(id, AssumptionId::Domain);
    assert_eq!(subject, Subject::X);
}

#[test]
fn pairwise_margin_zero_m() {
    let result = pairwise_margin(10, 0, 0.05);
    assert!(result.is_err());
    let (id, subject) = unwrap_assumption(result.unwrap_err());
    assert_eq!(id, AssumptionId::Domain);
    assert_eq!(subject, Subject::Y);
}

#[test]
fn pairwise_margin_negative_misrate() {
    let result = pairwise_margin(10, 10, -0.1);
    assert!(result.is_err());
    let (id, subject) = unwrap_assumption(result.unwrap_err());
    assert_eq!(id, AssumptionId::Domain);
    assert_eq!(subject, Subject::Misrate);
}

#[test]
fn pairwise_margin_misrate_greater_than_one() {
    let result = pairwise_margin(10, 10, 1.5);
    assert!(result.is_err());
    let (id, subject) = unwrap_assumption(result.unwrap_err());
    assert_eq!(id, AssumptionId::Domain);
    assert_eq!(subject, Subject::Misrate);
}

#[test]
fn pairwise_margin_nan_misrate() {
    let result = pairwise_margin(10, 10, f64::NAN);
    assert!(result.is_err());
    let (id, subject) = unwrap_assumption(result.unwrap_err());
    assert_eq!(id, AssumptionId::Domain);
    assert_eq!(subject, Subject::Misrate);
}

// --- signed_rank_margin ---

#[test]
fn signed_rank_margin_zero_n() {
    let result = signed_rank_margin(0, 0.05);
    assert!(result.is_err());
    let (id, subject) = unwrap_assumption(result.unwrap_err());
    assert_eq!(id, AssumptionId::Domain);
    assert_eq!(subject, Subject::X);
}

#[test]
fn signed_rank_margin_invalid_misrate() {
    let result = signed_rank_margin(10, -0.1);
    assert!(result.is_err());
    let (id, subject) = unwrap_assumption(result.unwrap_err());
    assert_eq!(id, AssumptionId::Domain);
    assert_eq!(subject, Subject::Misrate);
}

#[test]
fn signed_rank_margin_misrate_below_min() {
    let result = signed_rank_margin(5, 1e-20);
    assert!(result.is_err());
    let (id, subject) = unwrap_assumption(result.unwrap_err());
    assert_eq!(id, AssumptionId::Domain);
    assert_eq!(subject, Subject::Misrate);
}

