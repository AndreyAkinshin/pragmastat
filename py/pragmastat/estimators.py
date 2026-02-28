from __future__ import annotations

import math

import numpy as np

from ._fast_center_quantiles import fast_center_quantile_bounds
from .assumptions import AssumptionError, check_positivity
from .bounds import Bounds
from .fast_center import _fast_center
from .fast_shift import _fast_shift
from .fast_spread import _fast_spread
from .measurement import Measurement
from .measurement_unit import DISPARITY_UNIT, RATIO_UNIT
from .min_misrate import (
    min_achievable_misrate_one_sample,
    min_achievable_misrate_two_sample,
)
from .pairwise_margin import pairwise_margin
from .rng import Rng
from .sample import Sample, _check_non_weighted, _prepare_pair
from .sign_margin import sign_margin_randomized
from .signed_rank_margin import signed_rank_margin

DEFAULT_MISRATE = 1e-3


def center(x: Sample) -> Measurement:
    """Estimate the central value using Hodges-Lehmann estimator.

    Args:
        x: Input sample.

    Returns:
        Measurement with the center estimate and the sample's unit.

    Raises:
        AssumptionError: If input is empty or contains NaN/Inf.
        AssumptionError: If sample is weighted.
    """
    _check_non_weighted("center", x)
    result = _fast_center(x.values)
    return Measurement(result, x.unit)


def spread(x: Sample) -> Measurement:
    """Estimate data dispersion using Shamos estimator.

    Args:
        x: Input sample.

    Returns:
        Measurement with the spread estimate and the sample's unit.

    Raises:
        AssumptionError: If sample is empty, contains NaN/Inf, or is tie-dominant.
        AssumptionError: If sample is weighted.
    """
    _check_non_weighted("spread", x)
    spread_val = _fast_spread(x.values)
    if spread_val <= 0:
        raise AssumptionError.sparity("x")
    return Measurement(spread_val, x.unit)


def shift(x: Sample, y: Sample) -> Measurement:
    """Measure the typical difference between elements of x and y.

    Args:
        x: First sample.
        y: Second sample.

    Returns:
        Measurement with the shift estimate and the finer unit.

    Raises:
        AssumptionError: If either input is empty or contains NaN/Inf.
        AssumptionError: If either sample is weighted or units are incompatible.
    """
    _check_non_weighted("shift", x)
    _check_non_weighted("shift", y)
    x, y = _prepare_pair(x, y)
    result = float(_fast_shift(x.values, y.values, p=0.5))
    return Measurement(result, x.unit)


def ratio(x: Sample, y: Sample) -> Measurement:
    """Measure how many times larger x is compared to y.

    Args:
        x: First sample.
        y: Second sample (must be strictly positive).

    Returns:
        Measurement with the ratio estimate and RATIO_UNIT.

    Raises:
        AssumptionError: If either sample is empty, contains NaN/Inf, or contains non-positive values.
        AssumptionError: If either sample is weighted or units are incompatible.
    """
    _check_non_weighted("ratio", x)
    _check_non_weighted("ratio", y)
    x, y = _prepare_pair(x, y)
    check_positivity(x.values, "x")
    check_positivity(y.values, "y")
    log_x = np.log(x.values)
    log_y = np.log(y.values)
    log_result = _fast_shift(log_x, log_y, p=0.5)
    return Measurement(float(np.exp(log_result)), RATIO_UNIT)


def _avg_spread(x: Sample, y: Sample) -> Measurement:
    """Measure the typical variability considering both samples together.

    Internal estimator used by Disparity.

    Args:
        x: First sample.
        y: Second sample.

    Returns:
        Measurement with the average spread and the finer unit.

    Raises:
        AssumptionError: If either sample is empty, contains NaN/Inf, or is tie-dominant.
        AssumptionError: If either sample is weighted or units are incompatible.
    """
    _check_non_weighted("avg_spread", x)
    _check_non_weighted("avg_spread", y)
    x, y = _prepare_pair(x, y)
    n = len(x.values)
    m = len(y.values)
    spread_x = _fast_spread(x.values)
    if spread_x <= 0:
        raise AssumptionError.sparity("x")
    spread_y = _fast_spread(y.values)
    if spread_y <= 0:
        raise AssumptionError.sparity("y")
    return Measurement((n * spread_x + m * spread_y) / (n + m), x.unit)


def disparity(x: Sample, y: Sample) -> Measurement:
    """Measure effect size: a normalized difference between x and y.

    Args:
        x: First sample.
        y: Second sample.

    Returns:
        Measurement with the disparity and DISPARITY_UNIT.

    Raises:
        AssumptionError: If either sample is empty, contains NaN/Inf, or is tie-dominant.
        AssumptionError: If either sample is weighted or units are incompatible.
    """
    _check_non_weighted("disparity", x)
    _check_non_weighted("disparity", y)
    x, y = _prepare_pair(x, y)
    n = len(x.values)
    m = len(y.values)
    spread_x = _fast_spread(x.values)
    if spread_x <= 0:
        raise AssumptionError.sparity("x")
    spread_y = _fast_spread(y.values)
    if spread_y <= 0:
        raise AssumptionError.sparity("y")
    shift_val = float(_fast_shift(x.values, y.values, p=0.5))
    avg_spread_val = (n * spread_x + m * spread_y) / (n + m)
    return Measurement(shift_val / avg_spread_val, DISPARITY_UNIT)


def shift_bounds(
    x: Sample,
    y: Sample,
    misrate: float = DEFAULT_MISRATE,
) -> Bounds:
    """Provides bounds on the Shift estimator with specified misclassification rate.

    Args:
        x: First sample.
        y: Second sample.
        misrate: Misclassification rate.

    Returns:
        Bounds with lower, upper, and the finer unit.

    Raises:
        AssumptionError: If either sample is empty or contains NaN/Inf, or misrate is out of range.
        AssumptionError: If either sample is weighted or units are incompatible.
    """
    _check_non_weighted("shift_bounds", x)
    _check_non_weighted("shift_bounds", y)
    x, y = _prepare_pair(x, y)

    xv = x.values
    yv = y.values

    if math.isnan(misrate) or misrate < 0 or misrate > 1:
        raise AssumptionError.domain("misrate")

    n = len(xv)
    m = len(yv)

    min_misrate = min_achievable_misrate_two_sample(n, m)
    if misrate < min_misrate:
        raise AssumptionError.domain("misrate")

    xs = sorted(xv)
    ys = sorted(yv)
    total = n * m

    if total == 1:
        value = xs[0] - ys[0]
        return Bounds(value, value, x.unit)

    margin = pairwise_margin(n, m, misrate)
    half_margin = min(margin // 2, (total - 1) // 2)
    k_left = half_margin
    k_right = (total - 1) - half_margin

    denominator = total - 1 if total > 1 else 1
    p = [k_left / denominator, k_right / denominator]

    bounds = _fast_shift(xs, ys, p, assume_sorted=True)

    lower = min(bounds)
    upper = max(bounds)
    return Bounds(lower, upper, x.unit)


def ratio_bounds(
    x: Sample,
    y: Sample,
    misrate: float = DEFAULT_MISRATE,
) -> Bounds:
    """Provides bounds on the Ratio estimator with specified misclassification rate.

    Args:
        x: First sample (must be strictly positive).
        y: Second sample (must be strictly positive).
        misrate: Misclassification rate.

    Returns:
        Bounds with lower, upper, and RATIO_UNIT.

    Raises:
        AssumptionError: If either sample is empty, contains NaN/Inf, non-positive values,
            or misrate is out of range.
        AssumptionError: If either sample is weighted or units are incompatible.
    """
    _check_non_weighted("ratio_bounds", x)
    _check_non_weighted("ratio_bounds", y)
    x, y = _prepare_pair(x, y)

    if math.isnan(misrate) or misrate < 0 or misrate > 1:
        raise AssumptionError.domain("misrate")

    min_misrate = min_achievable_misrate_two_sample(len(x.values), len(y.values))
    if misrate < min_misrate:
        raise AssumptionError.domain("misrate")

    log_x = x.log()
    log_y = y.log()

    log_bounds = shift_bounds(log_x, log_y, misrate)

    return Bounds(np.exp(log_bounds.lower), np.exp(log_bounds.upper), RATIO_UNIT)


def center_bounds(
    x: Sample,
    misrate: float = DEFAULT_MISRATE,
) -> Bounds:
    """Provides exact bounds on the Center with specified misrate.

    Args:
        x: Input sample.
        misrate: Misclassification rate.

    Returns:
        Bounds with lower, upper, and the sample's unit.

    Raises:
        AssumptionError: If sample is empty or contains NaN/Inf, or misrate is below minimum.
        AssumptionError: If sample is weighted.
    """
    _check_non_weighted("center_bounds", x)

    if math.isnan(misrate) or misrate < 0 or misrate > 1:
        raise AssumptionError.domain("misrate")

    n = len(x.values)

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

    sorted_x = sorted(x.values)

    lo, hi = fast_center_quantile_bounds(sorted_x, k_left, k_right)
    return Bounds(lo, hi, x.unit)


def spread_bounds(
    x: Sample,
    misrate: float = DEFAULT_MISRATE,
    seed: str | None = None,
) -> Bounds:
    """Provides distribution-free bounds for Spread using disjoint pairs.

    Args:
        x: Input sample.
        misrate: Misclassification rate.
        seed: Optional string seed for deterministic randomization.

    Returns:
        Bounds with lower, upper, and the sample's unit.

    Raises:
        AssumptionError: If sample is empty, contains NaN/Inf, misrate out of range, or tie-dominant.
        AssumptionError: If sample is weighted.
    """
    _check_non_weighted("spread_bounds", x)

    if math.isnan(misrate) or misrate < 0 or misrate > 1:
        raise AssumptionError.domain("misrate")

    n = len(x.values)
    m = n // 2
    min_misrate_val = min_achievable_misrate_one_sample(m)
    if misrate < min_misrate_val:
        raise AssumptionError.domain("misrate")

    if n < 2:
        raise AssumptionError.sparity("x")
    if _fast_spread(x.values) <= 0:
        raise AssumptionError.sparity("x")

    rng = Rng(seed) if seed is not None else Rng()
    margin = sign_margin_randomized(m, misrate, rng)
    half_margin = margin // 2
    max_half_margin = (m - 1) // 2
    half_margin = min(half_margin, max_half_margin)

    k_left = half_margin + 1
    k_right = m - half_margin

    indices = list(range(n))
    shuffled = rng.shuffle(indices)
    diffs = sorted(abs(float(x.values[shuffled[2 * i]]) - float(x.values[shuffled[2 * i + 1]])) for i in range(m))

    return Bounds(diffs[k_left - 1], diffs[k_right - 1], x.unit)


def disparity_bounds(
    x: Sample,
    y: Sample,
    misrate: float = DEFAULT_MISRATE,
    seed: str | None = None,
) -> Bounds:
    """Provides distribution-free bounds for the Disparity estimator.

    Args:
        x: First input sample.
        y: Second input sample.
        misrate: Misclassification rate.
        seed: Optional string seed for deterministic randomization.

    Returns:
        Bounds with lower, upper, and DISPARITY_UNIT.

    Raises:
        AssumptionError: If inputs are invalid, misrate out of range, or samples are tie-dominant.
        AssumptionError: If either sample is weighted or units are incompatible.
    """
    _check_non_weighted("disparity_bounds", x)
    _check_non_weighted("disparity_bounds", y)
    x, y = _prepare_pair(x, y)

    if math.isnan(misrate) or misrate < 0 or misrate > 1:
        raise AssumptionError.domain("misrate")

    n = len(x.values)
    m = len(y.values)
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

    if _fast_spread(x.values) <= 0:
        raise AssumptionError.sparity("x")
    if _fast_spread(y.values) <= 0:
        raise AssumptionError.sparity("y")

    sb = shift_bounds(x, y, alpha_shift)
    ab = _avg_spread_bounds(x, y, alpha_avg, seed=seed)

    la = ab.lower
    ua = ab.upper
    ls = sb.lower
    us = sb.upper

    unit = DISPARITY_UNIT

    if la > 0.0:
        r1 = ls / la
        r2 = ls / ua
        r3 = us / la
        r4 = us / ua
        return Bounds(min(r1, r2, r3, r4), max(r1, r2, r3, r4), unit)

    if ua <= 0.0:
        if ls == 0.0 and us == 0.0:
            return Bounds(0.0, 0.0, unit)
        if ls >= 0.0:
            return Bounds(0.0, math.inf, unit)
        if us <= 0.0:
            return Bounds(-math.inf, 0.0, unit)
        return Bounds(-math.inf, math.inf, unit)

    # Default: ua > 0 and la <= 0
    if ls > 0.0:
        return Bounds(ls / ua, math.inf, unit)
    if us < 0.0:
        return Bounds(-math.inf, us / ua, unit)
    if ls == 0.0 and us == 0.0:
        return Bounds(0.0, 0.0, unit)
    if ls == 0.0 and us > 0.0:
        return Bounds(0.0, math.inf, unit)
    if ls < 0.0 and us == 0.0:
        return Bounds(-math.inf, 0.0, unit)

    return Bounds(-math.inf, math.inf, unit)


def _avg_spread_bounds(
    x: Sample,
    y: Sample,
    misrate: float = DEFAULT_MISRATE,
    seed: str | None = None,
) -> Bounds:
    """Provides distribution-free bounds for AvgSpread using Bonferroni combination.

    Args:
        x: First input sample.
        y: Second input sample.
        misrate: Misclassification rate.
        seed: Optional string seed for deterministic randomization.

    Returns:
        Bounds with lower, upper, and the finer unit.

    Raises:
        AssumptionError: If sample is empty, contains NaN/Inf, misrate out of range, or tie-dominant.
        AssumptionError: If either sample is weighted or units are incompatible.
    """
    _check_non_weighted("avg_spread_bounds", x)
    _check_non_weighted("avg_spread_bounds", y)
    x, y = _prepare_pair(x, y)

    if math.isnan(misrate) or misrate < 0 or misrate > 1:
        raise AssumptionError.domain("misrate")

    n = len(x.values)
    m = len(y.values)
    if n < 2:
        raise AssumptionError.domain("x")
    if m < 2:
        raise AssumptionError.domain("y")

    alpha = misrate / 2.0
    min_x = min_achievable_misrate_one_sample(n // 2)
    min_y = min_achievable_misrate_one_sample(m // 2)
    if alpha < min_x or alpha < min_y:
        raise AssumptionError.domain("misrate")

    if _fast_spread(x.values) <= 0:
        raise AssumptionError.sparity("x")
    if _fast_spread(y.values) <= 0:
        raise AssumptionError.sparity("y")

    bounds_x = spread_bounds(x, alpha, seed=seed)
    bounds_y = spread_bounds(y, alpha, seed=seed)

    weight_x = n / (n + m)
    weight_y = m / (n + m)

    return Bounds(
        weight_x * bounds_x.lower + weight_y * bounds_y.lower,
        weight_x * bounds_x.upper + weight_y * bounds_y.upper,
        x.unit,
    )
