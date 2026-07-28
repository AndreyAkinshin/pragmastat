//! No estimator returns a negative zero.
//!
//! A sample holding both `+0.0` and `-0.0` used to make the reported value depend on which of
//! the two the sort placed in the selected position: comparison cannot separate them, so the
//! sorting algorithm decided the payload, and the seven implementations sort their own way. The
//! estimators now normalize the sign away on the way out, which is what the assertions below
//! pin, one estimator at a time.
//!
//! Every check is by payload, through the crate's shared `assert_bits_eq`. `-0.0 == 0.0` holds,
//! so `assert_eq!` proves nothing here: it passes on exactly the value this file exists to
//! reject.
//!
//! The cases were chosen by running the estimators with the normalization removed and keeping
//! the inputs that came back `0x8000000000000000`, so each assertion below fails if its
//! normalization is dropped. The remaining estimators cannot currently reach a negative zero
//! (`spread` and the spread bounds sum absolute differences; `ratio` and the ratio bounds
//! exponentiate), but they carry the same guarantee and `no_estimator_returns_negative_zero`
//! states it for the whole surface rather than for the reachable half.

mod conformance;

use conformance::assert_bits_eq;
use pragmastat::estimators::raw;
use pragmastat::{
    Bounds, Sample, center, center_bounds, disparity, disparity_bounds, disparity_bounds_with_seed,
    ratio, ratio_bounds, shift, shift_bounds, spread, spread_bounds, spread_bounds_with_seed,
};

/// Samples whose center is zero, mixing the two signs of it in every arrangement that a sort can
/// resolve differently: both signs present, one sign only, and a sign carried in by an exact
/// cancellation rather than by the data.
const CENTER_SAMPLES: [&[f64]; 7] = [
    &[0.0, -0.0, 0.0, -0.0, 1.0],
    &[-0.0, -0.0],
    &[-0.0, 0.0],
    &[0.0, -0.0],
    &[-0.0, -0.0, -0.0],
    &[-1.0, 1.0],
    &[-2.0, -0.0, 2.0],
];

const CENTER_BOUNDS_SAMPLE: [f64; 6] = [0.0, -0.0, 0.0, -0.0, 1.0, -1.0];
const CENTER_BOUNDS_MISRATE: f64 = 0.3;

fn sample(values: &[f64]) -> Sample {
    Sample::new(values.to_vec()).expect("sample")
}

#[test]
fn center_never_returns_negative_zero() {
    for x in CENTER_SAMPLES {
        assert_bits_eq(
            &format!("raw::center({x:?})"),
            raw::center(x, false).unwrap(),
            0.0,
        );
        assert_bits_eq(
            &format!("center({x:?})"),
            center(&sample(x)).unwrap().value,
            0.0,
        );
    }
}

#[test]
fn center_bounds_never_returns_negative_zero() {
    let x = &CENTER_BOUNDS_SAMPLE;
    let rb = raw::center_bounds(x, CENTER_BOUNDS_MISRATE, false).unwrap();
    assert_bits_eq("raw::center_bounds.lower", rb.lower, 0.0);
    assert_bits_eq("raw::center_bounds.upper", rb.upper, 0.0);

    let b = center_bounds(&sample(x), CENTER_BOUNDS_MISRATE).unwrap();
    assert_bits_eq("center_bounds.lower", b.lower, 0.0);
    assert_bits_eq("center_bounds.upper", b.upper, 0.0);
}

/// Pairs whose shift is zero: every difference is `-0.0 - 0.0`, which is a negative zero before
/// normalization regardless of which element the selection lands on.
const SHIFT_PAIRS: [(&[f64], &[f64]); 2] = [
    (&[-0.0, -0.0], &[0.0, 0.0]),
    (&[-0.0, -0.0, -0.0], &[0.0, 0.0, 0.0]),
];

#[test]
fn shift_never_returns_negative_zero() {
    for (x, y) in SHIFT_PAIRS {
        assert_bits_eq(
            &format!("raw::shift({x:?},{y:?})"),
            raw::shift(x, y, false).unwrap(),
            0.0,
        );
        assert_bits_eq(
            &format!("shift({x:?},{y:?})"),
            shift(&sample(x), &sample(y)).unwrap().value,
            0.0,
        );
    }
}

#[test]
fn shift_bounds_never_returns_negative_zero() {
    let misrate = 0.9;
    for (x, y) in SHIFT_PAIRS {
        let rb = raw::shift_bounds(x, y, misrate, false).unwrap();
        assert_bits_eq(
            &format!("raw::shift_bounds({x:?},{y:?}).lower"),
            rb.lower,
            0.0,
        );
        assert_bits_eq(
            &format!("raw::shift_bounds({x:?},{y:?}).upper"),
            rb.upper,
            0.0,
        );

        let b = shift_bounds(&sample(x), &sample(y), misrate).unwrap();
        assert_bits_eq(&format!("shift_bounds({x:?},{y:?}).lower"), b.lower, 0.0);
        assert_bits_eq(&format!("shift_bounds({x:?},{y:?}).upper"), b.upper, 0.0);
    }
}

/// Pairs whose shift is a negative zero while both samples still carry a positive spread, so
/// disparity divides one by the other and inherits the sign.
const DISPARITY_PAIRS: [(&[f64], &[f64]); 2] = [
    (&[-0.0, -0.0, 5.0], &[0.0, 0.0, 5.0]),
    (&[-0.0, -0.0, -0.0, 1.0], &[0.0, 0.0, 0.0, 1.0]),
];

#[test]
fn disparity_never_returns_negative_zero() {
    for (x, y) in DISPARITY_PAIRS {
        assert_bits_eq(
            &format!("raw::disparity({x:?},{y:?})"),
            raw::disparity(x, y, false).unwrap(),
            0.0,
        );
        assert_bits_eq(
            &format!("disparity({x:?},{y:?})"),
            disparity(&sample(x), &sample(y)).unwrap().value,
            0.0,
        );
    }
}

/// The unit-carrying constructor is the single place the Sample-level bounds re-attach a unit,
/// so it normalizes too and does not need every wrapper to remember.
#[test]
fn bounds_constructor_normalizes_both_endpoints() {
    let b = Bounds::number(-0.0, -0.0);
    assert_bits_eq("Bounds::number.lower", b.lower, 0.0);
    assert_bits_eq("Bounds::number.upper", b.upper, 0.0);
}

/// A negative zero in the input is data, not a defect: it is accepted and it participates.
#[test]
fn negative_zero_in_the_input_is_preserved_as_a_value() {
    assert_bits_eq("center", raw::center(&[-0.0, 4.0], false).unwrap(), 2.0);
    assert_bits_eq("spread", raw::spread(&[-0.0, 3.0], false).unwrap(), 3.0);
    assert_bits_eq(
        "shift",
        raw::shift(&[-0.0, -0.0], &[-2.0, -2.0], false).unwrap(),
        2.0,
    );
}

/// The guarantee stated over the whole public surface: no estimator, and no endpoint of any
/// bounds estimator, hands back a negative zero for any of these zero-heavy samples.
#[test]
fn no_estimator_returns_negative_zero() {
    let x: &[f64] = &[-0.0, -0.0, -0.0, -0.0, 1.0, 2.0, -1.0, -2.0];
    let y: &[f64] = &[0.0, 0.0, 0.0, 0.0, 1.0, 2.0, -1.0, -2.0];
    let positive_x: &[f64] = &[1.0, 2.0, 4.0, 8.0];
    let positive_y: &[f64] = &[1.0, 2.0, 4.0, 8.0];
    let misrate = 0.5;
    let seed = "negative-zero";

    let (sx, sy) = (sample(x), sample(y));
    let (spx, spy) = (sample(positive_x), sample(positive_y));

    check("center", raw::center(x, false).unwrap());
    check("spread", raw::spread(x, false).unwrap());
    check("shift", raw::shift(x, y, false).unwrap());
    check("ratio", raw::ratio(positive_x, positive_y, false).unwrap());
    check("disparity", raw::disparity(x, y, false).unwrap());

    check_raw_bounds(
        "center_bounds",
        raw::center_bounds(x, misrate, false).unwrap(),
    );
    check_raw_bounds(
        "shift_bounds",
        raw::shift_bounds(x, y, misrate, false).unwrap(),
    );
    check_raw_bounds(
        "ratio_bounds",
        raw::ratio_bounds(positive_x, positive_y, misrate, false).unwrap(),
    );
    check_raw_bounds(
        "spread_bounds",
        raw::spread_bounds(x, misrate, false).unwrap(),
    );
    check_raw_bounds(
        "spread_bounds_with_seed",
        raw::spread_bounds_with_seed(x, misrate, seed, false).unwrap(),
    );
    check_raw_bounds(
        "avg_spread_bounds",
        raw::avg_spread_bounds(x, y, misrate, false).unwrap(),
    );
    check_raw_bounds(
        "disparity_bounds",
        raw::disparity_bounds(x, y, misrate, false).unwrap(),
    );
    check_raw_bounds(
        "disparity_bounds_with_seed",
        raw::disparity_bounds_with_seed(x, y, misrate, seed, false).unwrap(),
    );

    check("Sample center", center(&sx).unwrap().value);
    check("Sample spread", spread(&sx).unwrap().value);
    check("Sample shift", shift(&sx, &sy).unwrap().value);
    check("Sample ratio", ratio(&spx, &spy).unwrap().value);
    check("Sample disparity", disparity(&sx, &sy).unwrap().value);

    check_bounds("Sample center_bounds", center_bounds(&sx, misrate).unwrap());
    check_bounds(
        "Sample shift_bounds",
        shift_bounds(&sx, &sy, misrate).unwrap(),
    );
    check_bounds(
        "Sample ratio_bounds",
        ratio_bounds(&spx, &spy, misrate).unwrap(),
    );
    check_bounds("Sample spread_bounds", spread_bounds(&sx, misrate).unwrap());
    check_bounds(
        "Sample spread_bounds_with_seed",
        spread_bounds_with_seed(&sx, misrate, seed).unwrap(),
    );
    check_bounds(
        "Sample disparity_bounds",
        disparity_bounds(&sx, &sy, misrate).unwrap(),
    );
    check_bounds(
        "Sample disparity_bounds_with_seed",
        disparity_bounds_with_seed(&sx, &sy, misrate, seed).unwrap(),
    );
}

/// Rejects a negative zero without constraining the value: the sweep runs over estimators whose
/// results differ, and only the sign of a zero is under test.
#[track_caller]
fn check(what: &str, value: f64) {
    assert!(
        value.to_bits() != (-0.0f64).to_bits(),
        "{what}: returned a negative zero (0x{:016X})",
        value.to_bits()
    );
}

#[track_caller]
fn check_raw_bounds(what: &str, bounds: raw::RawBounds) {
    check(&format!("{what}.lower"), bounds.lower);
    check(&format!("{what}.upper"), bounds.upper);
}

#[track_caller]
fn check_bounds(what: &str, bounds: Bounds) {
    check(&format!("{what}.lower"), bounds.lower);
    check(&format!("{what}.upper"), bounds.upper);
}
