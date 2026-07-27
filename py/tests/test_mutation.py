import numpy as np
import pytest
from binary64 import fmt, identical

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

# Unsorted, 8 elements: large enough that a misrate of 0.3 is valid for the
# bounds estimators.
_BASE_UNSORTED = [5.0, 1.0, 8.0, 3.0, 2.0, 7.0, 4.0, 6.0]
_MISRATE = 0.3

# Two array shapes, each MUTABLE, C-contiguous float64:
#   - "unsorted": the default code path (kernels sort a copy internally).
#   - "sorted+assume_sorted": the no-copy buffer branch of the C kernels, where
#     the kernel may read the caller's buffer directly; it must still not write
#     to it. assume_sorted=True is passed through to every estimator.
_ARRAY_VARIANTS = [
    ("unsorted", _BASE_UNSORTED, {}),
    ("sorted+assume_sorted", sorted(_BASE_UNSORTED), {"assume_sorted": True}),
]


def _mutation_report(actual, original):
    """Describe every element whose raw binary64 payload changed."""
    changed = [i for i in range(original.size) if not identical(actual[i], original[i])]
    return ", ".join(f"[{i}] {fmt(original[i])} -> {fmt(actual[i])}" for i in changed)


def _assert_unchanged(actual, original, what):
    """Assert the caller's buffer is unchanged down to the raw binary64 payloads.

    The comparison is on the bytes, not on ``==``: a kernel that overwrites 0.0
    with -0.0 (or one NaN payload with another) leaves the array numerically
    equal while still having written to memory the caller owns. Only a payload
    comparison calls that what it is (see :mod:`binary64`).
    """
    assert actual.tobytes() == original.tobytes(), f"{what} mutated its input: {_mutation_report(actual, original)}"


@pytest.mark.parametrize(("label", "base", "kwargs"), _ARRAY_VARIANTS, ids=[v[0] for v in _ARRAY_VARIANTS])
def test_raw_api_does_not_mutate_caller_array(label, base, kwargs):
    """The public raw (native-array) API must NOT mutate the caller's array.

    Every point and bounds estimator is fed a MUTABLE, C-contiguous float64 array
    and the array must be byte-for-byte unchanged afterward. Both one- and
    two-sample shapes are covered (the two-sample calls pass the same array as x
    and y). Two array variants are exercised:

    * an UNSORTED array (the kernels sort a copy internally), and
    * a SORTED array with ``assume_sorted=True`` (the no-copy buffer branch of the
      C kernels, where the kernel reads the caller's buffer directly).
    """

    def fresh():
        arr = np.array(base, dtype=np.float64)
        assert arr.flags["C_CONTIGUOUS"]
        assert arr.flags["WRITEABLE"]
        return arr

    # One-sample point estimators.
    for estimator in (center, spread):
        arr = fresh()
        orig = arr.copy()
        estimator(arr, **kwargs)
        _assert_unchanged(arr, orig, f"{estimator.__name__} [{label}]")

    # Two-sample point estimators (same array passed as x and y).
    for estimator in (shift, ratio, disparity):
        arr = fresh()
        orig = arr.copy()
        estimator(arr, arr, **kwargs)
        _assert_unchanged(arr, orig, f"{estimator.__name__} [{label}]")

    # One-sample bounds estimators.
    for bounds in (center_bounds, spread_bounds):
        arr = fresh()
        orig = arr.copy()
        bounds(arr, _MISRATE, **kwargs)
        _assert_unchanged(arr, orig, f"{bounds.__name__} [{label}]")

    # Two-sample bounds estimators (same array passed as x and y).
    for bounds in (shift_bounds, ratio_bounds, disparity_bounds):
        arr = fresh()
        orig = arr.copy()
        bounds(arr, arr, _MISRATE, **kwargs)
        _assert_unchanged(arr, orig, f"{bounds.__name__} [{label}]")


def test_sample_values_are_immutable():
    sample = Sample(np.array([1.0, 2.0, 100.0]))
    with pytest.raises(ValueError, match="read-only"):
        sample.values[2] = 3.0


def test_sample_weights_are_immutable():
    weighted = Sample([1.0, 2.0], weights=[1.0, 2.0])
    with pytest.raises(ValueError, match="read-only"):
        weighted.weights[0] = 5.0
