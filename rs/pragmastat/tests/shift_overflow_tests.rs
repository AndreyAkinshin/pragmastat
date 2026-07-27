//! Regression: shift search bounds x[0]-y[n-1] and x[m-1]-y[0] can overflow to
//! -/+inf on extreme finite input, turning the midpoint into NaN and returning
//! +-inf instead of the true finite shift.
//!
//! Both expectations are exact, so both compare payloads. `assert_eq!` would not:
//! the symmetric case asserts a zero, and `-0.0 == 0.0` holds, so a shift that
//! came back with the wrong sign of zero would be reported as a pass by the very
//! test written to pin the value.

mod conformance;

use conformance::assert_bits_eq;
use pragmastat::estimators::raw;

#[test]
fn shift_symmetric_extremes() {
    let max = f64::MAX;
    let got = raw::shift(&[-max, max], &[-max, max], true).unwrap();
    assert_bits_eq("shift of symmetric extremes", got, 0.0);
}

#[test]
fn shift_one_sided_extremes() {
    let max = f64::MAX;
    let got = raw::shift(&[0.0, max], &[-max, 0.0], true).unwrap();
    assert_bits_eq("shift of one-sided extremes", got, max);
}
