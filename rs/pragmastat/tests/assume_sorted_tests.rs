//! Direct coverage for the raw API's `assume_sorted = true` branch.
//!
//! The dual-path reference tests only ever call the raw estimators with
//! `assume_sorted = false`; the `= true` branch is reached only transitively via
//! `Sample` (which passes its cached sorted view). This file exercises the
//! `= true` branch directly.
//!
//! For ORDER-INDEPENDENT estimators, sorting the input ascending and calling
//! with `assume_sorted = true` must equal the call on the unsorted input with
//! `assume_sorted = false`.
//!
//! For SHUFFLE-based bounds, `assume_sorted` only swaps an internal re-sort for
//! the supplied pre-sorted view; it never affects the shuffle (which always runs
//! on the ORIGINAL order). So on a genuinely SORTED slice with a fixed seed the
//! result must be byte-identical for `true` vs `false`.

use pragmastat::estimators::raw;

const MISRATE: f64 = 0.3;
const SEED: &str = "pragmastat";
const EPS: f64 = 1e-9;

fn sorted_copy(x: &[f64]) -> Vec<f64> {
    let mut v = x.to_vec();
    v.sort_unstable_by(|a, b| a.total_cmp(b));
    v
}

fn unsorted_x() -> Vec<f64> {
    vec![3.0, 1.0, 2.0, 5.0, 4.0, 8.0, 6.0, 7.0]
}

fn unsorted_y() -> Vec<f64> {
    vec![9.0, 11.0, 10.0, 13.0, 12.0, 16.0, 14.0, 15.0]
}

// --- Order-independent scalar estimators ---

#[test]
fn center_sorted_true_equals_unsorted_false() {
    let x = unsorted_x();
    let sorted = sorted_copy(&x);
    let want = raw::center(&x, false).unwrap();
    let got = raw::center(&sorted, true).unwrap();
    assert!((got - want).abs() < EPS, "center: {got} != {want}");
}

#[test]
fn spread_sorted_true_equals_unsorted_false() {
    let x = unsorted_x();
    let sorted = sorted_copy(&x);
    let want = raw::spread(&x, false).unwrap();
    let got = raw::spread(&sorted, true).unwrap();
    assert!((got - want).abs() < EPS, "spread: {got} != {want}");
}

#[test]
fn shift_sorted_true_equals_unsorted_false() {
    let x = unsorted_x();
    let y = unsorted_y();
    let want = raw::shift(&x, &y, false).unwrap();
    let got = raw::shift(&sorted_copy(&x), &sorted_copy(&y), true).unwrap();
    assert!((got - want).abs() < EPS, "shift: {got} != {want}");
}

#[test]
fn ratio_sorted_true_equals_unsorted_false() {
    let x = unsorted_x();
    let y = unsorted_y();
    let want = raw::ratio(&x, &y, false).unwrap();
    let got = raw::ratio(&sorted_copy(&x), &sorted_copy(&y), true).unwrap();
    assert!((got - want).abs() < EPS, "ratio: {got} != {want}");
}

#[test]
fn disparity_sorted_true_equals_unsorted_false() {
    let x = unsorted_x();
    let y = unsorted_y();
    let want = raw::disparity(&x, &y, false).unwrap();
    let got = raw::disparity(&sorted_copy(&x), &sorted_copy(&y), true).unwrap();
    assert!((got - want).abs() < EPS, "disparity: {got} != {want}");
}

// --- Order-independent bounds estimators ---

#[test]
fn center_bounds_sorted_true_equals_unsorted_false() {
    let x = unsorted_x();
    let want = raw::center_bounds(&x, MISRATE, false).unwrap();
    let got = raw::center_bounds(&sorted_copy(&x), MISRATE, true).unwrap();
    assert!(
        (got.lower - want.lower).abs() < EPS && (got.upper - want.upper).abs() < EPS,
        "center_bounds: [{},{}] != [{},{}]",
        got.lower,
        got.upper,
        want.lower,
        want.upper
    );
}

#[test]
fn shift_bounds_sorted_true_equals_unsorted_false() {
    let x = unsorted_x();
    let y = unsorted_y();
    let want = raw::shift_bounds(&x, &y, MISRATE, false).unwrap();
    let got = raw::shift_bounds(&sorted_copy(&x), &sorted_copy(&y), MISRATE, true).unwrap();
    assert!(
        (got.lower - want.lower).abs() < EPS && (got.upper - want.upper).abs() < EPS,
        "shift_bounds: [{},{}] != [{},{}]",
        got.lower,
        got.upper,
        want.lower,
        want.upper
    );
}

#[test]
fn ratio_bounds_sorted_true_equals_unsorted_false() {
    let x = unsorted_x();
    let y = unsorted_y();
    let want = raw::ratio_bounds(&x, &y, MISRATE, false).unwrap();
    let got = raw::ratio_bounds(&sorted_copy(&x), &sorted_copy(&y), MISRATE, true).unwrap();
    assert!(
        (got.lower - want.lower).abs() < EPS && (got.upper - want.upper).abs() < EPS,
        "ratio_bounds: [{},{}] != [{},{}]",
        got.lower,
        got.upper,
        want.lower,
        want.upper
    );
}

// --- Shuffle-based bounds: identical on a SORTED slice with a fixed seed ---
//
// On a genuinely sorted slice the shuffle order is identical for both calls and
// the only difference is whether the (valid) sorted view is reused, so the
// result must be byte-identical.

#[test]
fn spread_bounds_sorted_true_equals_false_byte_identical() {
    let sorted = sorted_copy(&unsorted_x());
    let want = raw::spread_bounds_with_seed(&sorted, MISRATE, SEED, false).unwrap();
    let got = raw::spread_bounds_with_seed(&sorted, MISRATE, SEED, true).unwrap();
    assert_eq!(got.lower, want.lower, "spread_bounds lower");
    assert_eq!(got.upper, want.upper, "spread_bounds upper");
}

#[test]
fn disparity_bounds_sorted_true_equals_false_byte_identical() {
    let sorted_x = sorted_copy(&unsorted_x());
    let sorted_y = sorted_copy(&unsorted_y());
    let want = raw::disparity_bounds_with_seed(&sorted_x, &sorted_y, MISRATE, SEED, false).unwrap();
    let got = raw::disparity_bounds_with_seed(&sorted_x, &sorted_y, MISRATE, SEED, true).unwrap();
    assert_eq!(got.lower, want.lower, "disparity_bounds lower");
    assert_eq!(got.upper, want.upper, "disparity_bounds upper");
}

// NOTE: There is deliberately NO "spread_bounds inert on UNSORTED input" test.
// On unsorted input with assume_sorted = true the sparity check feeds unsorted
// data to the sorted-only spread_impl kernel, which is UNDEFINED BEHAVIOR: it
// may trip the convergence guard (iteration cap or stall detection) and error,
// or pass by luck for a particular input. The only well-defined inertness is on
// a SORTED slice (covered by spread_bounds_sorted_true_equals_false_byte_identical
// above).

// --- n==2 center midpoint order-symmetry ---
//
// With assume_sorted = true the midpoint sees the RAW order (the normalizing
// sort would hide any asymmetry). The 0.5*a + 0.5*b form is exactly symmetric
// in operand order, so reversing the two inputs must yield a BIT-IDENTICAL
// result. An asymmetric midpoint such as a + (b - a) * 0.5 would return
// -3.4000000000000004 for the reversed order below, so this guards an EXACT
// equality, not an approximate one.

#[test]
fn center_n2_midpoint_is_order_symmetric() {
    let forward = raw::center(&[-5.0, -1.8], true).unwrap();
    let reversed = raw::center(&[-1.8, -5.0], true).unwrap();
    assert_eq!(forward, -3.4);
    assert_eq!(reversed, -3.4);
    assert_eq!(forward, reversed); // bit-exact, not approx
}
