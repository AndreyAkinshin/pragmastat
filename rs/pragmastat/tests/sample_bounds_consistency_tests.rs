//! Consistency tests for the Sample-based bounds API.
//!
//! The shuffle-based spread/disparity bounds are order-dependent: they form
//! random disjoint pairs from the sample. The Sample wrappers must therefore
//! shuffle the ORIGINAL order (not the cached sorted values), so that for a
//! fixed seed they produce exactly the same result as the raw slice API —
//! which is what the cross-language reference fixtures are generated against.
//! The cached sorted view may only feed the order-independent sub-computations
//! (the sparity check and the embedded shift bounds).
//!
//! A wrapper that shuffled `sorted_values()` instead would silently diverge
//! from the raw API and every other language on any unsorted input, and the
//! raw legs of the dual-path reference tests would not catch it.
//! These tests lock the Sample API to the raw API directly.

use pragmastat::estimators::raw;
use pragmastat::{disparity_bounds_with_seed, spread_bounds_with_seed, Sample};

const UNSORTED_X: [f64; 20] = [
    5.0, 3.0, 1.0, 4.0, 2.0, 9.0, 7.0, 6.0, 8.0, 10.0, 15.0, 11.0, 13.0, 12.0, 14.0, 20.0, 18.0,
    16.0, 19.0, 17.0,
];
const UNSORTED_Y: [f64; 20] = [
    25.0, 23.0, 21.0, 24.0, 22.0, 29.0, 27.0, 26.0, 28.0, 30.0, 35.0, 31.0, 33.0, 32.0, 34.0, 40.0,
    38.0, 36.0, 39.0, 37.0,
];

#[test]
fn sample_spread_bounds_matches_raw_on_unsorted_input() {
    let misrate = 0.1;
    let seed = "spread-bounds-consistency";

    let sample = Sample::new(UNSORTED_X.to_vec()).unwrap();
    let via_sample = spread_bounds_with_seed(&sample, misrate, seed).unwrap();
    let via_raw = raw::spread_bounds_with_seed(&UNSORTED_X, misrate, seed, false).unwrap();

    // Guard: this data+seed is genuinely order-dependent (shuffling sorted input
    // yields a different result), so the assert below is meaningful and not
    // vacuous — a wrapper that shuffled the sorted values would fail it.
    let mut sorted_x = UNSORTED_X.to_vec();
    sorted_x.sort_by(f64::total_cmp);
    let via_raw_sorted = raw::spread_bounds_with_seed(&sorted_x, misrate, seed, false).unwrap();
    assert_ne!(
        via_raw.lower, via_raw_sorted.lower,
        "test setup is vacuous: spread bounds are not order-dependent here"
    );

    assert_eq!(
        via_sample.lower, via_raw.lower,
        "spread lower: Sample API diverges from raw API on unsorted input"
    );
    assert_eq!(
        via_sample.upper, via_raw.upper,
        "spread upper: Sample API diverges from raw API on unsorted input"
    );
}

#[test]
fn sample_disparity_bounds_matches_raw_on_unsorted_input() {
    let misrate = 0.5;
    let seed = "disparity-bounds-consistency";

    let x = Sample::new(UNSORTED_X.to_vec()).unwrap();
    let y = Sample::new(UNSORTED_Y.to_vec()).unwrap();
    let via_sample = disparity_bounds_with_seed(&x, &y, misrate, seed).unwrap();
    let via_raw =
        raw::disparity_bounds_with_seed(&UNSORTED_X, &UNSORTED_Y, misrate, seed, false).unwrap();

    // Guard against a vacuous test: the avg-spread component is order-dependent.
    let mut sorted_x = UNSORTED_X.to_vec();
    let mut sorted_y = UNSORTED_Y.to_vec();
    sorted_x.sort_by(f64::total_cmp);
    sorted_y.sort_by(f64::total_cmp);
    let via_raw_sorted =
        raw::disparity_bounds_with_seed(&sorted_x, &sorted_y, misrate, seed, false).unwrap();
    assert_ne!(
        via_raw.lower, via_raw_sorted.lower,
        "test setup is vacuous: disparity bounds are not order-dependent here"
    );

    assert_eq!(
        via_sample.lower, via_raw.lower,
        "disparity lower: Sample API diverges from raw API on unsorted input"
    );
    assert_eq!(
        via_sample.upper, via_raw.upper,
        "disparity upper: Sample API diverges from raw API on unsorted input"
    );
}
