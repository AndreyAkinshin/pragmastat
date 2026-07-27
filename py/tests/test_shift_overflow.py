"""Regression: shift search bounds can overflow to +-inf on extreme finite input,
turning the midpoint into NaN and returning +-inf instead of the true finite shift.

Both expectations are exact, and exact means the raw binary64 payload: the
symmetric case is a median of pairwise DIFFERENCES, which is precisely where a
-0.0 comes from, and ``-0.0 == 0.0`` would report that divergence as a pass.
"""

import sys

from binary64 import assert_identical

from pragmastat import shift

MAX = sys.float_info.max


def test_shift_symmetric_extremes():
    assert_identical(shift([-MAX, MAX], [-MAX, MAX], assume_sorted=True), 0.0, "shift of symmetric extremes")


def test_shift_one_sided_extremes():
    assert_identical(shift([0.0, MAX], [-MAX, 0.0], assume_sorted=True), MAX, "shift of one-sided extremes")
