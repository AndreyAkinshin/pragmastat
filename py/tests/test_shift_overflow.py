"""Regression: shift search bounds can overflow to +-inf on extreme finite input,
turning the midpoint into NaN and returning +-inf instead of the true finite shift.
"""

import sys

from pragmastat import shift

MAX = sys.float_info.max


def test_shift_symmetric_extremes():
    assert shift([-MAX, MAX], [-MAX, MAX], assume_sorted=True) == 0.0


def test_shift_one_sided_extremes():
    assert shift([0.0, MAX], [-MAX, 0.0], assume_sorted=True) == MAX
