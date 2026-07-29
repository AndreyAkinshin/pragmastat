"""``additive_cumulative`` has no answer for a NaN, and must say so rather than inventing one.

Both of the function's range comparisons are false for a NaN, so without an explicit guard it
leaves the tail branch as a finite 0 or 1: an undefined input answered rather than reported. The
approximation it replaced propagated NaN, so losing that is a regression on behavior every port
shares, and it is invisible to the margin fixtures because none of them passes a NaN.

The assertions compare PAYLOADS: ``==`` reads the two zeros as equal and every NaN as unequal,
and neither reading is the claim being made here.
"""

import math

import pytest
from binary64 import assert_identical

from pragmastat.additive_cumulative import additive_cumulative


def test_carries_nan_through():
    assert math.isnan(additive_cumulative(math.nan)), "additive_cumulative(nan) should be nan"


@pytest.mark.parametrize(
    ("z", "expected"),
    [
        (math.inf, 1.0),
        (-math.inf, 0.0),
        (0.0, 0.5),
        (-0.0, 0.5),
    ],
)
def test_answers_the_defined_boundaries(z, expected):
    assert_identical(additive_cumulative(z), expected, f"additive_cumulative({z!r})")


def test_exp_function_cutoffs():
    """The values outside the reduction band, which the shared fixture cannot carry.

    JSON has no way to express an infinity, so the generated suite stops at 709.78 and these
    arguments are covered here or nowhere.
    """
    from pragmastat.exp_function import exp_function

    assert math.isnan(exp_function(math.nan))
    assert exp_function(709.8) == math.inf
    assert exp_function(math.inf) == math.inf
    assert_identical(exp_function(-745.3), 0.0, "exp_function(-745.3)")
    assert_identical(exp_function(-math.inf), 0.0, "exp_function(-inf)")
    assert_identical(exp_function(0.0), 1.0, "exp_function(0)")
