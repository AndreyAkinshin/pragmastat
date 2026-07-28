"""No public estimator ever reports a negative zero.

A sample holding both ``+0.0`` and ``-0.0`` used to make the reported value depend on which
of the two the sort happened to leave in the selected position: comparison cannot separate
them, so the position was settled by the sorting algorithm rather than by the data, and the
seven ports each bring their own sort. ``center([0.0, -0.0, 0.0, -0.0, 1.0])`` came out of
this port as ``-0.0`` and out of Go as ``+0.0``, against an ``exact`` conformance class that
promises identical bits from identical inputs.

Every estimator now sheds the sign on the way out, so these tests compare PAYLOADS: ``-0.0
== 0.0`` is true, and an equality assertion would pass on exactly the results this suite
exists to reject.

The samples below are the ones that reached a negative zero before the fix, plus a sweep
that holds the invariant for the estimators where it is currently unreachable. Those are
unreachable by proof, not by construction (``spread`` is an absolute difference, ``ratio``
is an exponential), and a proof nobody re-checks is how the next divergence gets in.
"""

import math

import pytest
from binary64 import assert_identical, fmt, payload

from pragmastat import (
    Sample,
    center,
    center_bounds,
    disparity,
    disparity_bounds,
    ratio,
    ratio_bounds,
    shift,
    shift_bounds,
    spread,
    spread_bounds,
)
from pragmastat.estimators import _avg_spread, _avg_spread_bounds, _new_bounds, _normalize_zero
from pragmastat.measurement_unit import NUMBER_UNIT

NEGATIVE_ZERO = 0x8000000000000000

# Every one of these estimates to zero, and every one of them selected a -0.0 before the fix.
MIXED_ZERO_SAMPLES = [
    [0.0, -0.0, 0.0, -0.0, 1.0],
    [-0.0, -0.0],
    [-0.0, 0.0],
    [0.0, -0.0],
    [-0.0, -0.0, -0.0],
    [-1.0, 1.0],
    [-2.0, -0.0, 2.0],
]

MIXED = [0.0, -0.0, 0.0, -0.0, 1.0, -1.0]
POSITIVE = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0]


def assert_positive_zero(value, what):
    assert_identical(value, 0.0, what)


@pytest.mark.parametrize("x", MIXED_ZERO_SAMPLES)
def test_center_reports_a_positive_zero(x):
    assert_positive_zero(center(x), f"center({x})")


@pytest.mark.parametrize("x", MIXED_ZERO_SAMPLES)
def test_center_reports_a_positive_zero_for_a_sample(x):
    assert_positive_zero(center(Sample(x)).value, f"center(Sample({x}))")


def test_center_bounds_reports_positive_zeros():
    bounds = center_bounds(MIXED, 0.3)
    assert_positive_zero(bounds.lower, "center_bounds lower")
    assert_positive_zero(bounds.upper, "center_bounds upper")


def test_center_bounds_reports_positive_zeros_for_a_sample():
    bounds = center_bounds(Sample(MIXED), 0.3)
    assert_positive_zero(bounds.lower, "center_bounds lower")
    assert_positive_zero(bounds.upper, "center_bounds upper")


@pytest.mark.parametrize(
    ("x", "y"),
    [
        ([-0.0], [0.0]),
        ([-0.0, 1.0], [0.0, 1.0]),
        ([-0.0, -0.0], [0.0, 0.0]),
    ],
)
def test_shift_reports_a_positive_zero(x, y):
    assert_positive_zero(shift(x, y), f"shift({x}, {y})")


def test_shift_bounds_reports_positive_zeros():
    # A single pair takes the degenerate x[0] - y[0] path, where -0.0 - +0.0 is -0.0.
    bounds = shift_bounds([-0.0], [0.0], 1.0)
    assert_positive_zero(bounds.lower, "shift_bounds lower")
    assert_positive_zero(bounds.upper, "shift_bounds upper")


def test_disparity_reports_a_positive_zero():
    # Shift selects the -0.0 difference; the average spread is positive, so the sign survived.
    assert_positive_zero(disparity([-0.0, 1.0], [0.0, 1.0]), "disparity")


def public_outputs():
    """Every public estimator, evaluated on input that carries a signed zero."""
    yield "center", center(MIXED)
    yield "spread", spread(MIXED)
    yield "shift", shift(MIXED, MIXED)
    yield "ratio", ratio(POSITIVE, POSITIVE)
    yield "disparity", disparity(MIXED, MIXED)
    yield "avg_spread", _avg_spread(Sample(MIXED), Sample(MIXED)).value

    named_bounds = [
        ("center_bounds", center_bounds(MIXED, 0.3)),
        ("spread_bounds", spread_bounds(MIXED, 0.5, "seed")),
        ("shift_bounds", shift_bounds(MIXED, MIXED, 0.5)),
        ("ratio_bounds", ratio_bounds(POSITIVE, POSITIVE, 0.5)),
        ("disparity_bounds", disparity_bounds(MIXED, MIXED, 0.9, "seed")),
        ("avg_spread_bounds", _avg_spread_bounds(Sample(MIXED), Sample(MIXED), 0.9, "seed")),
    ]
    for name, bounds in named_bounds:
        yield f"{name} lower", bounds.lower
        yield f"{name} upper", bounds.upper


@pytest.mark.parametrize(("name", "value"), list(public_outputs()))
def test_no_estimator_reports_a_negative_zero(name, value):
    assert payload(value) != NEGATIVE_ZERO, f"{name} reported a negative zero: {fmt(value)}"


def test_inputs_keep_their_negative_zeros():
    # Only outputs are normalized: a sample must still be able to CONTAIN a -0.0.
    x = [0.0, -0.0, 0.0, -0.0, 1.0]
    center(x)
    assert payload(x[1]) == NEGATIVE_ZERO, f"input was rewritten: {fmt(x[1])}"

    sample = Sample(x)
    center(sample)
    assert payload(sample.values[1]) == NEGATIVE_ZERO, f"sample values were rewritten: {fmt(sample.values[1])}"
    assert NEGATIVE_ZERO in [payload(v) for v in sample.sorted_values], "sorted values were rewritten"


@pytest.mark.parametrize(
    "value",
    [1.0, -1.0, 1e-320, -1e-320, math.inf, -math.inf, math.nan, 5e-324, -5e-324],
)
def test_normalization_leaves_every_non_zero_alone(value):
    assert_identical(_normalize_zero(value), value, f"_normalize_zero({value!r})")


@pytest.mark.parametrize("value", [0.0, -0.0])
def test_normalization_maps_both_zeros_to_the_positive_one(value):
    assert_identical(_normalize_zero(value), 0.0, f"_normalize_zero({value!r})")


def test_new_bounds_normalizes_both_endpoints():
    bounds = _new_bounds(-0.0, -0.0, NUMBER_UNIT)
    assert_positive_zero(bounds.lower, "lower")
    assert_positive_zero(bounds.upper, "upper")


def test_new_bounds_leaves_infinite_endpoints_alone():
    bounds = _new_bounds(-math.inf, math.inf, NUMBER_UNIT)
    assert_identical(bounds.lower, -math.inf, "lower")
    assert_identical(bounds.upper, math.inf, "upper")
