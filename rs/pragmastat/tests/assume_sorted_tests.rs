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
//!
//! Every comparison here is BIT-EXACT on the binary64 payload, not approximate.
//! Both sides are the same kernel fed the same values in the same order; the
//! flag only decides whether that order is re-established by an internal sort or
//! taken on trust. `assume_sorted = false` sorts a copy with `total_cmp`, which
//! yields exactly the slice the `= true` leg is handed, so there is no
//! arithmetic anywhere between the two paths for a rounding to enter. A
//! tolerance would not describe a numerical fact, it would only hide a defect.
//! The `ratio` legs go through log/exp, but BOTH legs go through the same
//! log/exp on the same values, so they are exact for the same reason.

mod conformance;

// Routed through the crate's shared `assert_bits_eq` rather than a private
// comparison so that "exact" means the same thing here as in the fixture suites.
// It compares the raw payloads rather than `==`, which has blind spots:
// `-0.0 == 0.0` holds and `NaN == NaN` does not, and either would misreport a
// genuine divergence between the two paths.
use conformance::assert_bits_eq;
use pragmastat::estimators::raw;

const MISRATE: f64 = 0.3;
const SEED: &str = "pragmastat";

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
    assert_bits_eq("center", got, want);
}

#[test]
fn spread_sorted_true_equals_unsorted_false() {
    let x = unsorted_x();
    let sorted = sorted_copy(&x);
    let want = raw::spread(&x, false).unwrap();
    let got = raw::spread(&sorted, true).unwrap();
    assert_bits_eq("spread", got, want);
}

#[test]
fn shift_sorted_true_equals_unsorted_false() {
    let x = unsorted_x();
    let y = unsorted_y();
    let want = raw::shift(&x, &y, false).unwrap();
    let got = raw::shift(&sorted_copy(&x), &sorted_copy(&y), true).unwrap();
    assert_bits_eq("shift", got, want);
}

#[test]
fn ratio_sorted_true_equals_unsorted_false() {
    let x = unsorted_x();
    let y = unsorted_y();
    let want = raw::ratio(&x, &y, false).unwrap();
    let got = raw::ratio(&sorted_copy(&x), &sorted_copy(&y), true).unwrap();
    assert_bits_eq("ratio", got, want);
}

#[test]
fn disparity_sorted_true_equals_unsorted_false() {
    let x = unsorted_x();
    let y = unsorted_y();
    let want = raw::disparity(&x, &y, false).unwrap();
    let got = raw::disparity(&sorted_copy(&x), &sorted_copy(&y), true).unwrap();
    assert_bits_eq("disparity", got, want);
}

// --- Order-independent bounds estimators ---

#[test]
fn center_bounds_sorted_true_equals_unsorted_false() {
    let x = unsorted_x();
    let want = raw::center_bounds(&x, MISRATE, false).unwrap();
    let got = raw::center_bounds(&sorted_copy(&x), MISRATE, true).unwrap();
    assert_bits_eq("center_bounds lower", got.lower, want.lower);
    assert_bits_eq("center_bounds upper", got.upper, want.upper);
}

#[test]
fn shift_bounds_sorted_true_equals_unsorted_false() {
    let x = unsorted_x();
    let y = unsorted_y();
    let want = raw::shift_bounds(&x, &y, MISRATE, false).unwrap();
    let got = raw::shift_bounds(&sorted_copy(&x), &sorted_copy(&y), MISRATE, true).unwrap();
    assert_bits_eq("shift_bounds lower", got.lower, want.lower);
    assert_bits_eq("shift_bounds upper", got.upper, want.upper);
}

#[test]
fn ratio_bounds_sorted_true_equals_unsorted_false() {
    let x = unsorted_x();
    let y = unsorted_y();
    let want = raw::ratio_bounds(&x, &y, MISRATE, false).unwrap();
    let got = raw::ratio_bounds(&sorted_copy(&x), &sorted_copy(&y), MISRATE, true).unwrap();
    // Both legs log-transform, run the same shift bounds, and exponentiate back;
    // sorting a positive sample and log-transforming it produces exactly the
    // array the other leg sorts, because log is monotonic. Same route, so exact.
    assert_bits_eq("ratio_bounds lower", got.lower, want.lower);
    assert_bits_eq("ratio_bounds upper", got.upper, want.upper);
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
    assert_bits_eq("spread_bounds lower", got.lower, want.lower);
    assert_bits_eq("spread_bounds upper", got.upper, want.upper);
}

#[test]
fn disparity_bounds_sorted_true_equals_false_byte_identical() {
    let sorted_x = sorted_copy(&unsorted_x());
    let sorted_y = sorted_copy(&unsorted_y());
    let want = raw::disparity_bounds_with_seed(&sorted_x, &sorted_y, MISRATE, SEED, false).unwrap();
    let got = raw::disparity_bounds_with_seed(&sorted_x, &sorted_y, MISRATE, SEED, true).unwrap();
    assert_bits_eq("disparity_bounds lower", got.lower, want.lower);
    assert_bits_eq("disparity_bounds upper", got.upper, want.upper);
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
    assert_bits_eq("center n=2 forward", forward, -3.4);
    assert_bits_eq("center n=2 reversed", reversed, -3.4);
    assert_bits_eq("center n=2 order symmetry", reversed, forward);
}
