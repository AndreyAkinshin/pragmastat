"""Direct coverage of the raw API's ``assume_sorted=True`` branch.

The reference suite exercises the raw estimators only with ``assume_sorted=False``;
the ``=True`` branch is otherwise reached only transitively via :class:`Sample`.
These tests hit it directly through the native-array API.

Two contracts are checked:

* ORDER-INDEPENDENT estimators (center, spread, shift, ratio, disparity,
  center_bounds, shift_bounds, ratio_bounds): sorting the input ascending and
  calling with ``assume_sorted=True`` must equal the call on the UNSORTED input
  with ``assume_sorted=False`` (within ~1e-9).
* SHUFFLE-based bounds (spread_bounds, disparity_bounds): ``assume_sorted`` only
  affects the order-independent sparity check, never the disjoint-pair shuffle,
  so on the SAME array and SAME seed the result must be IDENTICAL for
  ``assume_sorted=True`` vs ``False``. We use an already-SORTED array here so that
  the sparity check receives valid input under BOTH flag values (the kernels treat
  ``assume_sorted=True`` on unsorted input as undefined behavior); the shuffle then
  runs on the identical order regardless, so the two results must be byte-identical.
"""

import numpy as np
import pytest

import pragmastat.center_impl as center_impl_module
import pragmastat.spread_impl as spread_impl_module
from pragmastat import (
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

TOL = 1e-9


@pytest.fixture(params=["c", "python"])
def kernel(request, monkeypatch):
    """Route a test through a specific selection kernel (C extension or pure Python).

    ``_center_impl`` / ``_spread_impl`` dispatch on their module-level
    ``_HAS_C_EXTENSION`` flags at call time, so forcing the flags to ``False``
    sends the public estimators through ``_center_impl_python`` /
    ``_spread_impl_python``. Without this, a build with the C extension present
    would never exercise the pure-Python guard paths.
    """
    if request.param == "python":
        monkeypatch.setattr(center_impl_module, "_HAS_C_EXTENSION", False)
        monkeypatch.setattr(spread_impl_module, "_HAS_C_EXTENSION", False)
    elif not (center_impl_module._HAS_C_EXTENSION and spread_impl_module._HAS_C_EXTENSION):  # noqa: SLF001
        pytest.skip("C extension not built")
    return request.param


# Unsorted, strictly positive (so ratio is defined), 8 elements (large enough for
# the 0.3 misrate used by the bounds estimators).
X_UNSORTED = [5.0, 1.0, 8.0, 3.0, 2.0, 7.0, 4.0, 6.0]
Y_UNSORTED = [12.0, 9.0, 15.0, 10.0, 13.0, 11.0, 16.0, 14.0]
MISRATE = 0.3
SEED = "assume-sorted-roundtrip"


def _sorted(values):
    return np.sort(np.asarray(values, dtype=np.float64))


class TestOrderIndependentRoundtrip:
    """sorted(input)+assume_sorted=True == unsorted+assume_sorted=False."""

    def test_center(self):
        unsorted = center(X_UNSORTED, assume_sorted=False)
        presorted = center(_sorted(X_UNSORTED), assume_sorted=True)
        assert abs(presorted - unsorted) < TOL

    def test_spread(self):
        unsorted = spread(X_UNSORTED, assume_sorted=False)
        presorted = spread(_sorted(X_UNSORTED), assume_sorted=True)
        assert abs(presorted - unsorted) < TOL

    def test_shift(self):
        unsorted = shift(X_UNSORTED, Y_UNSORTED, assume_sorted=False)
        presorted = shift(_sorted(X_UNSORTED), _sorted(Y_UNSORTED), assume_sorted=True)
        assert abs(presorted - unsorted) < TOL

    def test_ratio(self):
        unsorted = ratio(X_UNSORTED, Y_UNSORTED, assume_sorted=False)
        presorted = ratio(_sorted(X_UNSORTED), _sorted(Y_UNSORTED), assume_sorted=True)
        assert abs(presorted - unsorted) < TOL

    def test_disparity(self):
        unsorted = disparity(X_UNSORTED, Y_UNSORTED, assume_sorted=False)
        presorted = disparity(_sorted(X_UNSORTED), _sorted(Y_UNSORTED), assume_sorted=True)
        assert abs(presorted - unsorted) < TOL

    def test_center_bounds(self):
        unsorted = center_bounds(X_UNSORTED, MISRATE, assume_sorted=False)
        presorted = center_bounds(_sorted(X_UNSORTED), MISRATE, assume_sorted=True)
        assert abs(presorted.lower - unsorted.lower) < TOL
        assert abs(presorted.upper - unsorted.upper) < TOL

    def test_shift_bounds(self):
        unsorted = shift_bounds(X_UNSORTED, Y_UNSORTED, MISRATE, assume_sorted=False)
        presorted = shift_bounds(_sorted(X_UNSORTED), _sorted(Y_UNSORTED), MISRATE, assume_sorted=True)
        assert abs(presorted.lower - unsorted.lower) < TOL
        assert abs(presorted.upper - unsorted.upper) < TOL

    def test_ratio_bounds(self):
        unsorted = ratio_bounds(X_UNSORTED, Y_UNSORTED, MISRATE, assume_sorted=False)
        presorted = ratio_bounds(_sorted(X_UNSORTED), _sorted(Y_UNSORTED), MISRATE, assume_sorted=True)
        assert abs(presorted.lower - unsorted.lower) < TOL
        assert abs(presorted.upper - unsorted.upper) < TOL


class TestShuffleBoundsFlagInvariant:
    """assume_sorted never changes shuffle-based bounds (same array, same seed).

    A SORTED array is used so the order-independent sparity check gets valid input
    under both flag values; the disjoint-pair shuffle runs on the identical order
    regardless of the flag, so the two results must be byte-identical.
    """

    X_SORTED = sorted(X_UNSORTED)
    Y_SORTED = sorted(Y_UNSORTED)

    def test_spread_bounds(self):
        false_result = spread_bounds(self.X_SORTED, MISRATE, SEED, assume_sorted=False)
        true_result = spread_bounds(self.X_SORTED, MISRATE, SEED, assume_sorted=True)
        assert true_result.lower == false_result.lower
        assert true_result.upper == false_result.upper

    def test_disparity_bounds(self):
        false_result = disparity_bounds(self.X_SORTED, self.Y_SORTED, MISRATE, SEED, assume_sorted=False)
        true_result = disparity_bounds(self.X_SORTED, self.Y_SORTED, MISRATE, SEED, assume_sorted=True)
        assert true_result.lower == false_result.lower
        assert true_result.upper == false_result.upper


class TestAssumeSortedMisuseTerminates:
    """``assume_sorted=True`` on UNSORTED input must terminate, not wedge.

    The Monahan selection loop in ``center``'s kernel relies on genuine
    sortedness. Handing it UNSORTED data with ``assume_sorted=True`` is undefined
    behavior; without an iteration cap the loop can run forever, wedging the
    process (an unkillable infinite loop inside the C extension that Python-level
    signals cannot interrupt). All kernels bound the loop (256 + 4*n iteration
    cap plus a stall guard on the active set) and raise a deterministic
    convergence error instead. We assert that the call returns quickly with that
    error rather than hanging, via the ``kernel`` fixture for BOTH the C
    extension and the pure-Python fallback.
    """

    # A pathological UNSORTED arrangement that drives the selection loop without
    # progress when (incorrectly) treated as sorted.
    UNSORTED = [3.0, 1.0, 4.0, 1.5, 5.0, 9.0, 2.0, 6.0, 0.5, 8.0, 7.0]

    @pytest.mark.usefixtures("kernel")
    def test_center_unsorted_assume_sorted_raises_quickly(self):
        import time

        start = time.monotonic()
        with pytest.raises(RuntimeError, match=r"[Cc]onvergence failure"):
            center(np.asarray(self.UNSORTED, dtype=np.float64), assume_sorted=True)
        # Must be near-instant; a hang would blow far past this budget.
        assert time.monotonic() - start < 5.0

    # An UNSORTED arrangement that drives spread_impl's selection loop without
    # progress when (incorrectly) treated as sorted. Verified to loop forever in
    # an UNCAPPED spread kernel (an unkillable hang), and to raise the
    # deterministic convergence error with the cap in place. (The center UNSORTED
    # array above does NOT exercise spread's failure mode, hence a dedicated
    # input here.)
    UNSORTED_SPREAD = [5.16, -1.59, -4.82, 0.23, -1.9, 5.68, -3.93, -0.47, 1.67]

    @pytest.mark.usefixtures("kernel")
    def test_spread_unsorted_assume_sorted_raises_quickly(self):
        import time

        start = time.monotonic()
        with pytest.raises(RuntimeError, match=r"[Cc]onvergence failure"):
            spread(np.asarray(self.UNSORTED_SPREAD, dtype=np.float64), assume_sorted=True)
        # Must be near-instant; a hang would blow far past this budget.
        assert time.monotonic() - start < 5.0


# NOTE on the disparity_bounds / spread_bounds UNSORTED behavior
# -------------------------------------------------------------
# There is deliberately NO test asserting that the shuffle-based bounds are
# inert under ``assume_sorted`` on UNSORTED input, because that inertness does
# NOT hold for either estimator:
#
# * ``spread_bounds``: the sparity check skips its re-sort and feeds the passed
#   (unsorted) buffer to the sorted-only spread kernel, so the flag changes the
#   computation path (``test_spread_unsorted_assume_sorted_raises_quickly``
#   shows the kernel can error on such input).
# * ``disparity_bounds``: its embedded order-independent shift-bounds consumes
#   the passed slice as a sorted view, so the flag can change the result.
#
# Passing ``assume_sorted=True`` on UNSORTED input is a contract violation
# (undefined behavior): the RESULT is unspecified, but TERMINATION is
# guaranteed. Without an iteration cap, both ``center``'s Monahan selection AND
# ``spread``'s selection loop can HANG on such input; the pure-Python and C
# extension paths all bound the loop (256 + 4*n iteration cap plus a stall
# guard on the active set) and raise a deterministic convergence error (see
# ``TestAssumeSortedMisuseTerminates`` above). The legitimate path always
# feeds a truly-sorted view (``Sample.sorted_values``, or a native array the
# caller actually sorted), under which ``TestShuffleBoundsFlagInvariant`` covers
# flag-invariance.
