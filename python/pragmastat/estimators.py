from typing import Sequence, Union
import numpy as np
from numpy.typing import NDArray


def center(x: Union[Sequence[float], NDArray]) -> float:
    x = np.asarray(x)
    n = len(x)
    if n == 0:
        raise ValueError("Input array cannot be empty")
    pairwise_averages = np.add.outer(x, x) / 2
    indices = np.triu_indices(n, k=0)
    return float(np.median(pairwise_averages[indices]))


def spread(x: Union[Sequence[float], NDArray]) -> float:
    x = np.asarray(x)
    n = len(x)
    if n == 0:
        raise ValueError("Input array cannot be empty")
    if n == 1:
        return 0.0
    pairwise_diffs = np.subtract.outer(x, x)
    pairwise_abs_diffs = np.abs(pairwise_diffs)
    indices = np.triu_indices(n, k=1)
    return float(np.median(pairwise_abs_diffs[indices]))


def volatility(x: Union[Sequence[float], NDArray]) -> float:
    center_val = center(x)
    if center_val == 0:
        raise ValueError("Volatility is undefined when Center equals zero")
    return spread(x) / abs(center_val)


def precision(x: Union[Sequence[float], NDArray]) -> float:
    x = np.asarray(x)
    n = len(x)
    if n == 0:
        raise ValueError("Input array cannot be empty")
    return 2 * spread(x) / np.sqrt(n)


def med_shift(x: Union[Sequence[float], NDArray], y: Union[Sequence[float], NDArray]) -> float:
    x = np.asarray(x)
    y = np.asarray(y)
    if len(x) == 0 or len(y) == 0:
        raise ValueError("Input arrays cannot be empty")
    pairwise_shifts = np.subtract.outer(x, y)
    return float(np.median(pairwise_shifts))


def med_ratio(x: Union[Sequence[float], NDArray], y: Union[Sequence[float], NDArray]) -> float:
    x = np.asarray(x)
    y = np.asarray(y)
    if len(x) == 0 or len(y) == 0:
        raise ValueError("Input arrays cannot be empty")
    if np.any(y <= 0):
        raise ValueError("All values in y must be strictly positive")
    pairwise_ratios = np.divide.outer(x, y)
    return float(np.median(pairwise_ratios))


def med_spread(x: Union[Sequence[float], NDArray], y: Union[Sequence[float], NDArray]) -> float:
    x = np.asarray(x)
    y = np.asarray(y)
    n = len(x)
    m = len(y)
    if n == 0 or m == 0:
        raise ValueError("Input arrays cannot be empty")
    spread_x = spread(x)
    spread_y = spread(y)
    return (n * spread_x + m * spread_y) / (n + m)


def med_disparity(x: Union[Sequence[float], NDArray], y: Union[Sequence[float], NDArray]) -> float:
    med_spread_val = med_spread(x, y)
    if med_spread_val == 0:
        return float('inf')
    return med_shift(x, y) / med_spread_val