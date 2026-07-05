"""Statistical estimators for one-sample and two-sample analysis.

Each estimator is implemented exactly once on native ``numpy`` arrays via a
private ``_*_raw`` function that takes an ``assume_sorted`` flag (and, for the
shuffle-based bounds, an optional pre-sorted view). The public estimator
functions are duck-typed: they accept EITHER a :class:`Sample` OR a native
array/sequence.

* When given a :class:`Sample`, they return a unit-aware :class:`Measurement`
  or :class:`Bounds`, and feed the sample's cached sorted values into the raw
  impl (``assume_sorted=True``).
* When given a native array/sequence, they return a UNITLESS plain ``float``
  (point estimators) or a unitless :class:`Bounds` (interval estimators), and
  honor the caller-supplied ``assume_sorted`` flag.

``assume_sorted`` contract (default ``False``):

* Order-INDEPENDENT estimators (``center``, ``spread``, ``shift``, ``ratio``,
  ``disparity``, ``center_bounds``, ``shift_bounds``, ``ratio_bounds``):
  ``assume_sorted=True`` means "the input is already sorted ascending — skip the
  internal sort". This changes the computation path. Passing ``True`` on
  UNSORTED input is a contract violation (undefined behavior): the RESULT is
  unspecified, but TERMINATION is guaranteed — the selection loops are bounded
  and fail with a deterministic convergence error on pathological input.
* SHUFFLE-based bounds (``spread_bounds``, ``disparity_bounds``): the
  disjoint-pair shuffle ALWAYS runs on the passed array's order, so it is
  unaffected by the flag. The same undefined-behavior contract applies if
  ``True`` is passed on unsorted input.

  - ``spread_bounds`` under ``assume_sorted=True`` skips the re-sort in the
    order-independent sparity (spread > 0) check, feeding the passed array
    directly to the sorted-only spread kernel. On genuinely sorted input this
    changes nothing; on UNSORTED input it is a contract violation with an
    unspecified outcome (typically the deterministic convergence error).
  - ``disparity_bounds`` is likewise NOT inert on UNSORTED input. It embeds an
    order-independent shift-bounds computation that consumes the passed slice as
    a sorted view; when the slice is genuinely sorted the flag changes nothing,
    but on UNSORTED input the flag is a contract violation and CAN change the
    result.
"""

from __future__ import annotations

import math
from typing import TYPE_CHECKING, Sequence, Union

import numpy as np

from ._center_quantiles_impl import center_quantile_bounds_impl
from .assumptions import AssumptionError, check_positivity, check_validity, log
from .bounds import Bounds
from .center_impl import _center_impl
from .measurement import Measurement
from .measurement_unit import DISPARITY_UNIT, NUMBER_UNIT, RATIO_UNIT
from .min_misrate import (
    min_achievable_misrate_one_sample,
    min_achievable_misrate_two_sample,
)
from .pairwise_margin import pairwise_margin
from .rng import Rng
from .sample import Sample, _check_non_weighted, _prepare_pair
from .shift_impl import _shift_impl
from .sign_margin import sign_margin_randomized
from .signed_rank_margin import signed_rank_margin
from .spread_impl import _spread_impl

if TYPE_CHECKING:
    from numpy.typing import NDArray

DEFAULT_MISRATE = 1e-3

# A native (unitless) input: any sequence of floats or a numpy array.
ArrayLike = Union[Sequence[float], "NDArray"]


def _as_array(values: ArrayLike) -> NDArray:
    """Coerce a native sequence/array into a float64 numpy array."""
    return np.asarray(values, dtype=np.float64)


def _sorted_view(values: NDArray, assume_sorted: bool) -> NDArray | None:
    """Map the public ``assume_sorted`` flag to an optional pre-sorted view.

    When the caller's array is already sorted, it doubles as the sorted view for
    the order-independent sparity check (skipping a re-sort). The disjoint-pair
    shuffle always runs on the caller's array regardless. Passing
    ``assume_sorted=True`` on an UNSORTED array is a contract violation
    (undefined behavior): the sparity check feeds the unsorted buffer to the
    sorted-only spread kernel, so the result is unspecified — though termination
    is guaranteed, since the bounded selection loop fails with a deterministic
    convergence error on pathological input.
    """
    return values if assume_sorted else None


# =============================================================================
# Raw (native-array) estimator implementations — the single source of truth.
# Each returns a plain float, or a plain (lower, upper) tuple for bounds.
# =============================================================================


def _center_raw(x: NDArray, assume_sorted: bool) -> float:
    check_validity(x, "x")
    return float(_center_impl(x, assume_sorted=assume_sorted))


def _spread_raw(x: NDArray, assume_sorted: bool) -> float:
    check_validity(x, "x")
    spread_val = _spread_impl(x, assume_sorted=assume_sorted)
    if spread_val <= 0:
        raise AssumptionError.sparity("x")
    return float(spread_val)


def _shift_raw(x: NDArray, y: NDArray, assume_sorted: bool) -> float:
    check_validity(x, "x")
    check_validity(y, "y")
    return float(_shift_impl(x, y, p=0.5, assume_sorted=assume_sorted))


def _ratio_raw(x: NDArray, y: NDArray, assume_sorted: bool) -> float:
    check_validity(x, "x")
    check_validity(y, "y")
    check_positivity(x, "x")
    check_positivity(y, "y")
    log_x = np.log(x)
    log_y = np.log(y)
    # log is monotonic: sorted positive input -> sorted log output.
    log_result = _shift_impl(log_x, log_y, p=0.5, assume_sorted=assume_sorted)
    return float(np.exp(log_result))


def _disparity_raw(x: NDArray, y: NDArray, assume_sorted: bool) -> float:
    check_validity(x, "x")
    check_validity(y, "y")
    n = len(x)
    m = len(y)
    spread_x = _spread_impl(x, assume_sorted=assume_sorted)
    if spread_x <= 0:
        raise AssumptionError.sparity("x")
    spread_y = _spread_impl(y, assume_sorted=assume_sorted)
    if spread_y <= 0:
        raise AssumptionError.sparity("y")
    shift_val = float(_shift_impl(x, y, p=0.5, assume_sorted=assume_sorted))
    avg_spread_val = (n * spread_x + m * spread_y) / (n + m)
    return shift_val / avg_spread_val


def _shift_bounds_raw(
    x: NDArray,
    y: NDArray,
    misrate: float,
    assume_sorted: bool,
) -> tuple[float, float]:
    check_validity(x, "x")
    check_validity(y, "y")

    if math.isnan(misrate) or misrate < 0 or misrate > 1:
        raise AssumptionError.domain("misrate")

    n = len(x)
    m = len(y)

    min_misrate = min_achievable_misrate_two_sample(n, m)
    if misrate < min_misrate:
        raise AssumptionError.domain("misrate")

    xs = x if assume_sorted else np.sort(x)
    ys = y if assume_sorted else np.sort(y)

    total = n * m
    if total == 1:
        value = float(xs[0] - ys[0])
        return value, value

    margin = pairwise_margin(n, m, misrate)
    half_margin = min(margin // 2, (total - 1) // 2)
    k_left = half_margin
    k_right = (total - 1) - half_margin

    # total >= 2 here (the total == 1 case returned early above), so total-1 >= 1.
    denominator = total - 1
    p = [k_left / denominator, k_right / denominator]

    bounds = _shift_impl(xs, ys, p, assume_sorted=True)
    return float(min(bounds)), float(max(bounds))


def _ratio_bounds_raw(
    x: NDArray,
    y: NDArray,
    misrate: float,
    assume_sorted: bool,
) -> tuple[float, float]:
    check_validity(x, "x")
    check_validity(y, "y")

    if math.isnan(misrate) or misrate < 0 or misrate > 1:
        raise AssumptionError.domain("misrate")

    min_misrate = min_achievable_misrate_two_sample(len(x), len(y))
    if misrate < min_misrate:
        raise AssumptionError.domain("misrate")

    log_x = log(x, "x")
    log_y = log(y, "y")
    # log is monotonic: sorted positive input -> sorted log output.
    lower, upper = _shift_bounds_raw(log_x, log_y, misrate, assume_sorted)
    return float(np.exp(lower)), float(np.exp(upper))


def _center_bounds_raw(
    x: NDArray,
    misrate: float,
    assume_sorted: bool,
) -> tuple[float, float]:
    check_validity(x, "x")

    if math.isnan(misrate) or misrate < 0 or misrate > 1:
        raise AssumptionError.domain("misrate")

    n = len(x)
    if n < 2:
        raise AssumptionError.domain("x")

    min_misrate = min_achievable_misrate_one_sample(n)
    if misrate < min_misrate:
        raise AssumptionError.domain("misrate")

    total_pairs = n * (n + 1) // 2

    margin = signed_rank_margin(n, misrate)
    half_margin = min(margin // 2, (total_pairs - 1) // 2)

    k_left = half_margin + 1
    k_right = total_pairs - half_margin

    sorted_x = x if assume_sorted else np.sort(x)
    lo, hi = center_quantile_bounds_impl(sorted_x, k_left, k_right)
    return float(lo), float(hi)


def _spread_for_sparity(original: NDArray, sorted_view: NDArray | None) -> float:
    """Compute the spread value for the sparity check.

    The result is order-independent, so a pre-sorted view (when available) is
    used to skip re-sorting; otherwise the original array is sorted internally.
    """
    if sorted_view is not None:
        return float(_spread_impl(sorted_view, assume_sorted=True))
    return float(_spread_impl(original, assume_sorted=False))


def _spread_bounds_shuffle(
    x: NDArray,
    m: int,
    misrate: float,
    rng: Rng,
) -> tuple[float, float]:
    """Shuffle the ORIGINAL order into disjoint pairs and return order-statistic bounds.

    The caller is responsible for validity/domain/sparity checks, so
    ``_avg_spread_bounds_raw`` can reuse it without re-running ``_spread_impl``.
    """
    n = len(x)
    margin = sign_margin_randomized(m, misrate, rng)
    half_margin = margin // 2
    max_half_margin = (m - 1) // 2
    half_margin = min(half_margin, max_half_margin)

    k_left = half_margin + 1
    k_right = m - half_margin

    indices = list(range(n))
    shuffled = rng.shuffle(indices)
    diffs = sorted(abs(float(x[shuffled[2 * i]]) - float(x[shuffled[2 * i + 1]])) for i in range(m))

    return diffs[k_left - 1], diffs[k_right - 1]


def _spread_bounds_raw(
    x: NDArray,
    sorted_view: NDArray | None,
    misrate: float,
    seed: str | None,
) -> tuple[float, float]:
    """Compute distribution-free spread bounds.

    ``x`` is always in ORIGINAL order (the disjoint-pair shuffle is
    order-dependent). ``sorted_view``, when provided, is a pre-sorted view used
    only to speed up the order-independent sparity check.
    """
    check_validity(x, "x")

    if math.isnan(misrate) or misrate < 0 or misrate > 1:
        raise AssumptionError.domain("misrate")

    n = len(x)
    if n < 2:
        raise AssumptionError.sparity("x")
    m = n // 2
    min_misrate_val = min_achievable_misrate_one_sample(m)
    if misrate < min_misrate_val:
        raise AssumptionError.domain("misrate")
    if _spread_for_sparity(x, sorted_view) <= 0:
        raise AssumptionError.sparity("x")

    rng = Rng(seed) if seed is not None else Rng()
    return _spread_bounds_shuffle(x, m, misrate, rng)


def _avg_spread_bounds_raw(  # noqa: PLR0913
    x: NDArray,
    sorted_x: NDArray | None,
    y: NDArray,
    sorted_y: NDArray | None,
    misrate: float,
    seed: str | None,
) -> tuple[float, float]:
    """Compute distribution-free average-spread bounds via Bonferroni combination.

    ``x``/``y`` are always in ORIGINAL order (the disjoint-pair shuffle is
    order-dependent). ``sorted_x``/``sorted_y``, when provided, are pre-sorted
    views used only to speed up the order-independent sparity check.
    """
    check_validity(x, "x")
    check_validity(y, "y")

    if math.isnan(misrate) or misrate < 0 or misrate > 1:
        raise AssumptionError.domain("misrate")

    n = len(x)
    m = len(y)
    if n < 2:
        raise AssumptionError.domain("x")
    if m < 2:
        raise AssumptionError.domain("y")

    alpha = misrate / 2.0
    min_x = min_achievable_misrate_one_sample(n // 2)
    min_y = min_achievable_misrate_one_sample(m // 2)
    if alpha < min_x or alpha < min_y:
        raise AssumptionError.domain("misrate")

    if _spread_for_sparity(x, sorted_x) <= 0:
        raise AssumptionError.sparity("x")
    if _spread_for_sparity(y, sorted_y) <= 0:
        raise AssumptionError.sparity("y")

    # Validity/domain/sparity already checked; reuse the inner shuffle directly.
    # The shuffle operates on the ORIGINAL order; sorted views are sparity-only.
    rng_x = Rng(seed) if seed is not None else Rng()
    rng_y = Rng(seed) if seed is not None else Rng()
    lower_x, upper_x = _spread_bounds_shuffle(x, n // 2, alpha, rng_x)
    lower_y, upper_y = _spread_bounds_shuffle(y, m // 2, alpha, rng_y)

    weight_x = n / (n + m)
    weight_y = m / (n + m)
    return (
        weight_x * lower_x + weight_y * lower_y,
        weight_x * upper_x + weight_y * upper_y,
    )


def _disparity_bounds_from_components(
    ls: float,
    us: float,
    la: float,
    ua: float,
) -> tuple[float, float]:
    """Compute disparity bounds from shift bounds (ls, us) and avg-spread bounds (la, ua)."""
    if la > 0.0:
        r1 = ls / la
        r2 = ls / ua
        r3 = us / la
        r4 = us / ua
        return min(r1, r2, r3, r4), max(r1, r2, r3, r4)

    if ua <= 0.0:
        if ls == 0.0 and us == 0.0:
            return 0.0, 0.0
        if ls >= 0.0:
            return 0.0, math.inf
        if us <= 0.0:
            return -math.inf, 0.0
        return -math.inf, math.inf

    # Default: ua > 0 and la <= 0
    if ls > 0.0:
        return ls / ua, math.inf
    if us < 0.0:
        return -math.inf, us / ua
    if ls == 0.0 and us == 0.0:
        return 0.0, 0.0
    if ls == 0.0 and us > 0.0:
        return 0.0, math.inf
    if ls < 0.0 and us == 0.0:
        return -math.inf, 0.0

    return -math.inf, math.inf


def _disparity_bounds_raw(  # noqa: PLR0913
    x: NDArray,
    sorted_x: NDArray | None,
    y: NDArray,
    sorted_y: NDArray | None,
    misrate: float,
    seed: str | None,
) -> tuple[float, float]:
    """Compute distribution-free disparity bounds.

    ``x``/``y`` are always in ORIGINAL order; ``sorted_x``/``sorted_y``, when
    present, are pre-sorted views used only for the order-independent sparity and
    shift-bounds sub-computations.
    """
    check_validity(x, "x")
    check_validity(y, "y")

    if math.isnan(misrate) or misrate < 0 or misrate > 1:
        raise AssumptionError.domain("misrate")

    n = len(x)
    m = len(y)
    if n < 2:
        raise AssumptionError.domain("x")
    if m < 2:
        raise AssumptionError.domain("y")

    min_shift = min_achievable_misrate_two_sample(n, m)
    min_x = min_achievable_misrate_one_sample(n // 2)
    min_y = min_achievable_misrate_one_sample(m // 2)
    min_avg = 2.0 * max(min_x, min_y)

    if misrate < min_shift + min_avg:
        raise AssumptionError.domain("misrate")

    extra = misrate - (min_shift + min_avg)
    alpha_shift = min_shift + extra / 2.0
    alpha_avg = min_avg + extra / 2.0

    # The spread > 0 sparity check is performed by _avg_spread_bounds_raw below
    # (identical predicate and "x"/"y" order). shift_bounds runs first but cannot
    # raise for these inputs (alpha_shift >= the two-sample minimum), so it cannot
    # mask that sparity error. shift_bounds is order-independent given sorted
    # input; use sorted views when present.
    if sorted_x is not None and sorted_y is not None:
        ls, us = _shift_bounds_raw(sorted_x, sorted_y, alpha_shift, assume_sorted=True)
    else:
        ls, us = _shift_bounds_raw(x, y, alpha_shift, assume_sorted=False)
    la, ua = _avg_spread_bounds_raw(x, sorted_x, y, sorted_y, alpha_avg, seed)

    return _disparity_bounds_from_components(ls, us, la, ua)


# =============================================================================
# Public estimators — duck-typed over Sample | native array.
# =============================================================================


def center(x: Sample | ArrayLike, *, assume_sorted: bool = False) -> Measurement | float:
    """Estimate the central value using the Hodges-Lehmann estimator.

    Args:
        x: A :class:`Sample`, or a native array/sequence of values.
        assume_sorted: For native input, skip the internal sort when the input
            is already sorted ascending. Ignored for :class:`Sample` input
            (its cached sorted values are always used). See module docstring.

    Returns:
        For :class:`Sample` input, a unit-aware :class:`Measurement`.
        For native input, a plain unitless ``float``.

    Raises:
        AssumptionError: If input is empty or contains NaN/Inf, or sample weighted.
    """
    if isinstance(x, Sample):
        _check_non_weighted("x", x)
        return Measurement(_center_raw(x.sorted_values, assume_sorted=True), x.unit)
    return _center_raw(_as_array(x), assume_sorted)


def spread(x: Sample | ArrayLike, *, assume_sorted: bool = False) -> Measurement | float:
    """Estimate data dispersion using the Shamos estimator.

    Args:
        x: A :class:`Sample`, or a native array/sequence of values.
        assume_sorted: For native input, skip the internal sort. See module docstring.

    Returns:
        For :class:`Sample` input, a unit-aware :class:`Measurement`.
        For native input, a plain unitless ``float``.

    Raises:
        AssumptionError: If sample is empty, contains NaN/Inf, tie-dominant, or weighted.
    """
    if isinstance(x, Sample):
        _check_non_weighted("x", x)
        return Measurement(_spread_raw(x.sorted_values, assume_sorted=True), x.unit)
    return _spread_raw(_as_array(x), assume_sorted)


def shift(
    x: Sample | ArrayLike,
    y: Sample | ArrayLike,
    *,
    assume_sorted: bool = False,
) -> Measurement | float:
    """Measure the typical difference between elements of x and y.

    Args:
        x: First :class:`Sample` or native array/sequence.
        y: Second :class:`Sample` or native array/sequence.
        assume_sorted: For native input, skip the internal sort. See module docstring.

    Returns:
        For :class:`Sample` input, a unit-aware :class:`Measurement` (finer unit).
        For native input, a plain unitless ``float``.

    Raises:
        AssumptionError: If either input is empty or contains NaN/Inf, weighted,
            or units are incompatible.
    """
    if isinstance(x, Sample) or isinstance(y, Sample):
        sx, sy = _coerce_pair(x, y)
        return Measurement(_shift_raw(sx.sorted_values, sy.sorted_values, assume_sorted=True), sx.unit)
    return _shift_raw(_as_array(x), _as_array(y), assume_sorted)


def ratio(
    x: Sample | ArrayLike,
    y: Sample | ArrayLike,
    *,
    assume_sorted: bool = False,
) -> Measurement | float:
    """Measure how many times larger x is compared to y.

    Args:
        x: First :class:`Sample` or native array/sequence (strictly positive).
        y: Second :class:`Sample` or native array/sequence (strictly positive).
        assume_sorted: For native input, skip the internal sort. See module docstring.

    Returns:
        For :class:`Sample` input, a :class:`Measurement` with RATIO_UNIT.
        For native input, a plain unitless ``float``.

    Raises:
        AssumptionError: If either sample is empty, contains NaN/Inf, non-positive,
            weighted, or units are incompatible.
    """
    if isinstance(x, Sample) or isinstance(y, Sample):
        sx, sy = _coerce_pair(x, y)
        return Measurement(_ratio_raw(sx.sorted_values, sy.sorted_values, assume_sorted=True), RATIO_UNIT)
    return _ratio_raw(_as_array(x), _as_array(y), assume_sorted)


def _avg_spread(x: Sample, y: Sample) -> Measurement:
    """Measure the typical variability considering both samples together.

    Internal estimator used by Disparity. Sample-only (no public raw entry).
    """
    _check_non_weighted("x", x)
    _check_non_weighted("y", y)
    x, y = _prepare_pair(x, y)
    n = len(x.values)
    m = len(y.values)
    spread_x = _spread_impl(x.sorted_values, assume_sorted=True)
    if spread_x <= 0:
        raise AssumptionError.sparity("x")
    spread_y = _spread_impl(y.sorted_values, assume_sorted=True)
    if spread_y <= 0:
        raise AssumptionError.sparity("y")
    return Measurement((n * spread_x + m * spread_y) / (n + m), x.unit)


def disparity(
    x: Sample | ArrayLike,
    y: Sample | ArrayLike,
    *,
    assume_sorted: bool = False,
) -> Measurement | float:
    """Measure effect size: a normalized difference between x and y.

    Args:
        x: First :class:`Sample` or native array/sequence.
        y: Second :class:`Sample` or native array/sequence.
        assume_sorted: For native input, skip the internal sort. See module docstring.

    Returns:
        For :class:`Sample` input, a :class:`Measurement` with DISPARITY_UNIT.
        For native input, a plain unitless ``float``.

    Raises:
        AssumptionError: If either sample is empty, contains NaN/Inf, tie-dominant,
            weighted, or units are incompatible.
    """
    if isinstance(x, Sample) or isinstance(y, Sample):
        sx, sy = _coerce_pair(x, y)
        return Measurement(_disparity_raw(sx.sorted_values, sy.sorted_values, assume_sorted=True), DISPARITY_UNIT)
    return _disparity_raw(_as_array(x), _as_array(y), assume_sorted)


def shift_bounds(
    x: Sample | ArrayLike,
    y: Sample | ArrayLike,
    misrate: float = DEFAULT_MISRATE,
    *,
    assume_sorted: bool = False,
) -> Bounds:
    """Provide bounds on the Shift estimator with a specified misclassification rate.

    Args:
        x: First :class:`Sample` or native array/sequence.
        y: Second :class:`Sample` or native array/sequence.
        misrate: Misclassification rate.
        assume_sorted: For native input, skip the internal sort. See module docstring.

    Returns:
        :class:`Bounds`. Unit-aware (finer unit) for Sample input; unitless for native input.

    Raises:
        AssumptionError: If either sample is empty or contains NaN/Inf, misrate is
            out of range, samples are weighted, or units are incompatible.
    """
    if isinstance(x, Sample) or isinstance(y, Sample):
        sx, sy = _coerce_pair(x, y)
        lower, upper = _shift_bounds_raw(sx.sorted_values, sy.sorted_values, misrate, assume_sorted=True)
        return Bounds(lower, upper, sx.unit)
    lower, upper = _shift_bounds_raw(_as_array(x), _as_array(y), misrate, assume_sorted)
    return Bounds(lower, upper, NUMBER_UNIT)


def ratio_bounds(
    x: Sample | ArrayLike,
    y: Sample | ArrayLike,
    misrate: float = DEFAULT_MISRATE,
    *,
    assume_sorted: bool = False,
) -> Bounds:
    """Provide bounds on the Ratio estimator with a specified misclassification rate.

    Args:
        x: First :class:`Sample` or native array/sequence (strictly positive).
        y: Second :class:`Sample` or native array/sequence (strictly positive).
        misrate: Misclassification rate.
        assume_sorted: For native input, skip the internal sort. See module docstring.

    Returns:
        :class:`Bounds`. RATIO_UNIT for Sample input; unitless for native input.

    Raises:
        AssumptionError: If either sample is empty, contains NaN/Inf, non-positive,
            misrate is out of range, weighted, or units are incompatible.
    """
    if isinstance(x, Sample) or isinstance(y, Sample):
        sx, sy = _coerce_pair(x, y)
        lower, upper = _ratio_bounds_raw(sx.sorted_values, sy.sorted_values, misrate, assume_sorted=True)
        return Bounds(lower, upper, RATIO_UNIT)
    lower, upper = _ratio_bounds_raw(_as_array(x), _as_array(y), misrate, assume_sorted)
    return Bounds(lower, upper, NUMBER_UNIT)


def center_bounds(
    x: Sample | ArrayLike,
    misrate: float = DEFAULT_MISRATE,
    *,
    assume_sorted: bool = False,
) -> Bounds:
    """Provide exact bounds on the Center with a specified misrate.

    Args:
        x: A :class:`Sample` or native array/sequence.
        misrate: Misclassification rate.
        assume_sorted: For native input, skip the internal sort. See module docstring.

    Returns:
        :class:`Bounds`. Unit-aware for Sample input; unitless for native input.

    Raises:
        AssumptionError: If sample is empty or contains NaN/Inf, misrate below minimum,
            or sample is weighted.
    """
    if isinstance(x, Sample):
        _check_non_weighted("x", x)
        lower, upper = _center_bounds_raw(x.sorted_values, misrate, assume_sorted=True)
        return Bounds(lower, upper, x.unit)
    lower, upper = _center_bounds_raw(_as_array(x), misrate, assume_sorted)
    return Bounds(lower, upper, NUMBER_UNIT)


def spread_bounds(
    x: Sample | ArrayLike,
    misrate: float = DEFAULT_MISRATE,
    seed: str | None = None,
    *,
    assume_sorted: bool = False,
) -> Bounds:
    """Provide distribution-free bounds for Spread using disjoint pairs.

    Args:
        x: A :class:`Sample` or native array/sequence.
        misrate: Misclassification rate.
        seed: Optional string seed for deterministic randomization.
        assume_sorted: For native input, skips the sparity-check re-sort (the
            shuffle always runs on the passed order). Passing ``True`` on
            unsorted input is a contract violation: the result is unspecified,
            but termination is guaranteed. See module docstring.

    Returns:
        :class:`Bounds`. Unit-aware for Sample input; unitless for native input.

    Raises:
        AssumptionError: If sample is empty, contains NaN/Inf, misrate out of range,
            tie-dominant, or weighted.
    """
    if isinstance(x, Sample):
        _check_non_weighted("x", x)
        # Shuffle runs on the original order; the cached sorted view is sparity-only.
        lower, upper = _spread_bounds_raw(x.values, x.sorted_values, misrate, seed)
        return Bounds(lower, upper, x.unit)
    arr = _as_array(x)
    lower, upper = _spread_bounds_raw(arr, _sorted_view(arr, assume_sorted), misrate, seed)
    return Bounds(lower, upper, NUMBER_UNIT)


def disparity_bounds(
    x: Sample | ArrayLike,
    y: Sample | ArrayLike,
    misrate: float = DEFAULT_MISRATE,
    seed: str | None = None,
    *,
    assume_sorted: bool = False,
) -> Bounds:
    """Provide distribution-free bounds for the Disparity estimator.

    Args:
        x: First :class:`Sample` or native array/sequence.
        y: Second :class:`Sample` or native array/sequence.
        misrate: Misclassification rate.
        seed: Optional string seed for deterministic randomization.
        assume_sorted: For native input, skips re-sorting when the input is
            already sorted ascending. The inertness holds ONLY on genuinely
            sorted input; on UNSORTED input the flag is undefined behavior and
            CAN change the result (the embedded order-independent shift-bounds
            consumes the slice as a sorted view). See module docstring.

    Returns:
        :class:`Bounds`. DISPARITY_UNIT for Sample input; unitless for native input.

    Raises:
        AssumptionError: If inputs are invalid, misrate out of range, tie-dominant,
            weighted, or units are incompatible.
    """
    if isinstance(x, Sample) or isinstance(y, Sample):
        sx, sy = _coerce_pair(x, y)
        # Shuffle runs on the original order; the cached sorted views are sparity-only.
        lower, upper = _disparity_bounds_raw(sx.values, sx.sorted_values, sy.values, sy.sorted_values, misrate, seed)
        return Bounds(lower, upper, DISPARITY_UNIT)
    ax = _as_array(x)
    ay = _as_array(y)
    lower, upper = _disparity_bounds_raw(
        ax, _sorted_view(ax, assume_sorted), ay, _sorted_view(ay, assume_sorted), misrate, seed
    )
    return Bounds(lower, upper, NUMBER_UNIT)


def _avg_spread_bounds(
    x: Sample,
    y: Sample,
    misrate: float = DEFAULT_MISRATE,
    seed: str | None = None,
) -> Bounds:
    """Provide distribution-free bounds for AvgSpread (internal; Sample-only).

    Thin wrapper over :func:`_avg_spread_bounds_raw` that passes the cached
    sorted views so the sparity check reuses them.
    """
    _check_non_weighted("x", x)
    _check_non_weighted("y", y)
    x, y = _prepare_pair(x, y)
    lower, upper = _avg_spread_bounds_raw(x.values, x.sorted_values, y.values, y.sorted_values, misrate, seed)
    return Bounds(lower, upper, x.unit)


def _coerce_pair(x: Sample | ArrayLike, y: Sample | ArrayLike) -> tuple[Sample, Sample]:
    """Validate non-weighted and prepare a two-sample pair (unit-coerced).

    If only one argument is a :class:`Sample` (mixed input), the native one is
    wrapped in a unitless :class:`Sample` so both go through the same path. The
    error "subject" (x vs y) is supplied positionally by the raw impl, not stored
    on the Sample.
    """
    sx = x if isinstance(x, Sample) else Sample(x)
    sy = y if isinstance(y, Sample) else Sample(y)
    _check_non_weighted("x", sx)
    _check_non_weighted("y", sy)
    return _prepare_pair(sx, sy)
