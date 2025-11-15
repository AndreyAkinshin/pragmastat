from typing import Sequence, Union, NamedTuple
import numpy as np
from numpy.typing import NDArray
from .fast_center import _fast_center
from .fast_spread import _fast_spread
from .fast_shift import _fast_shift
from .pairwise_margin import pairwise_margin


class Bounds(NamedTuple):
    """Represents an interval with lower and upper bounds."""

    lower: float
    upper: float


def center(x: Union[Sequence[float], NDArray]) -> float:
    x = np.asarray(x)
    n = len(x)
    if n == 0:
        raise ValueError("Input array cannot be empty")
    # Use fast O(n log n) algorithm
    return _fast_center(x.tolist())


def spread(x: Union[Sequence[float], NDArray]) -> float:
    x = np.asarray(x)
    n = len(x)
    if n == 0:
        raise ValueError("Input array cannot be empty")
    if n == 1:
        return 0.0
    # Use fast O(n log n) algorithm
    return _fast_spread(x.tolist())


def rel_spread(x: Union[Sequence[float], NDArray]) -> float:
    center_val = center(x)
    if center_val == 0:
        raise ValueError("RelSpread is undefined when Center equals zero")
    return spread(x) / abs(center_val)


def shift(
    x: Union[Sequence[float], NDArray], y: Union[Sequence[float], NDArray]
) -> float:
    x = np.asarray(x)
    y = np.asarray(y)
    if len(x) == 0 or len(y) == 0:
        raise ValueError("Input arrays cannot be empty")
    # Use fast O((m+n) log L) algorithm instead of materializing all m*n differences
    return float(_fast_shift(x, y, p=0.5))


def ratio(
    x: Union[Sequence[float], NDArray], y: Union[Sequence[float], NDArray]
) -> float:
    x = np.asarray(x)
    y = np.asarray(y)
    if len(x) == 0 or len(y) == 0:
        raise ValueError("Input arrays cannot be empty")
    if np.any(y <= 0):
        raise ValueError("All values in y must be strictly positive")
    pairwise_ratios = np.divide.outer(x, y)
    return float(np.median(pairwise_ratios))


def avg_spread(
    x: Union[Sequence[float], NDArray], y: Union[Sequence[float], NDArray]
) -> float:
    x = np.asarray(x)
    y = np.asarray(y)
    n = len(x)
    m = len(y)
    if n == 0 or m == 0:
        raise ValueError("Input arrays cannot be empty")
    spread_x = spread(x)
    spread_y = spread(y)
    return (n * spread_x + m * spread_y) / (n + m)


def disparity(
    x: Union[Sequence[float], NDArray], y: Union[Sequence[float], NDArray]
) -> float:
    avg_spread_val = avg_spread(x, y)
    if avg_spread_val == 0:
        return float("inf")
    return shift(x, y) / avg_spread_val


def shift_bounds(
    x: Union[Sequence[float], NDArray],
    y: Union[Sequence[float], NDArray],
    misrate: float,
) -> Bounds:
    """
    Provides bounds on the Shift estimator with specified misclassification rate.

    The misrate represents the probability that the true shift falls outside
    the computed bounds. This is a pragmatic alternative to traditional confidence
    intervals for the Hodges-Lehmann estimator.

    Args:
        x: First sample
        y: Second sample
        misrate: Misclassification rate (probability that true shift falls outside bounds)

    Returns:
        A Bounds object containing the lower and upper bounds
    """
    x = np.asarray(x)
    y = np.asarray(y)

    if len(x) == 0 or len(y) == 0:
        raise ValueError("Input arrays cannot be empty")

    n = len(x)
    m = len(y)

    # Sort both arrays
    xs = sorted(x.tolist())
    ys = sorted(y.tolist())

    total = n * m

    # Special case: when there's only one pairwise difference, bounds collapse to a single value
    if total == 1:
        value = xs[0] - ys[0]
        return Bounds(value, value)

    margin = pairwise_margin(n, m, misrate)
    half_margin = min(margin // 2, (total - 1) // 2)
    k_left = half_margin
    k_right = (total - 1) - half_margin

    # Compute quantile positions
    denominator = total - 1 if total > 1 else 1
    p = [k_left / denominator, k_right / denominator]

    bounds = _fast_shift(xs, ys, p, assume_sorted=True)

    lower = min(bounds)
    upper = max(bounds)
    return Bounds(lower, upper)
