//! Regression: shift search bounds x[0]-y[n-1] and x[m-1]-y[0] can overflow to
//! -/+inf on extreme finite input, turning the midpoint into NaN and returning
//! +-inf instead of the true finite shift.

use pragmastat::estimators::raw;

#[test]
fn shift_symmetric_extremes() {
    let max = f64::MAX;
    assert_eq!(raw::shift(&[-max, max], &[-max, max], true).unwrap(), 0.0);
}

#[test]
fn shift_one_sided_extremes() {
    let max = f64::MAX;
    assert_eq!(raw::shift(&[0.0, max], &[-max, 0.0], true).unwrap(), max);
}
