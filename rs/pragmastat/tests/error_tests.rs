//! Tests for error handling and input validation

use pragmastat::estimators::{
    avg_spread, center, disparity, median, ratio, rel_spread, shift, shift_bounds, spread,
};
use pragmastat::pairwise_margin::pairwise_margin;

#[test]
fn median_empty_input() {
    assert!(median(&[]).is_err());
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
    // spread returns 0.0 for empty input (single element returns 0.0 spread)
    assert!(spread(&[]).is_ok());
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

#[test]
fn pairwise_margin_zero_n() {
    assert!(pairwise_margin(0, 10, 0.05).is_err());
}

#[test]
fn pairwise_margin_zero_m() {
    assert!(pairwise_margin(10, 0, 0.05).is_err());
}

#[test]
fn pairwise_margin_negative_misrate() {
    assert!(pairwise_margin(10, 10, -0.1).is_err());
}

#[test]
fn pairwise_margin_misrate_greater_than_one() {
    assert!(pairwise_margin(10, 10, 1.5).is_err());
}

#[test]
fn pairwise_margin_nan_misrate() {
    assert!(pairwise_margin(10, 10, f64::NAN).is_err());
}
