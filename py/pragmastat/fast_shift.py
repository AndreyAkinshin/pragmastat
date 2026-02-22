"""Fast O((m+n) log L) implementation of the Shift estimator.

Computes quantiles of all pairwise differences without materializing them.
Uses binary search in value space with two-pointer counting.
"""

from typing import List, Sequence, Union

import numpy as np
from numpy.typing import NDArray

# Try to import the C implementation, fall back to pure Python if unavailable
try:
    from . import _fast_shift_c

    _HAS_C_EXTENSION = True
except ImportError:
    _HAS_C_EXTENSION = False


def _midpoint(a: float, b: float) -> float:
    """Compute numerically stable midpoint."""
    return a + (b - a) * 0.5


def _count_and_neighbors(x: List[float], y: List[float], threshold: float) -> tuple[int, float, float]:
    """
    Count pairs where x[i] - y[j] <= threshold using two-pointer algorithm.

    Also tracks the closest actual differences on either side of threshold.

    Args:
        x: Sorted array of x values
        y: Sorted array of y values
        threshold: The threshold value

    Returns:
        Tuple of (count_less_or_equal, closest_below, closest_above)
    """
    m = len(x)
    n = len(y)
    count = 0
    max_below = float("-inf")
    min_above = float("inf")

    j = 0
    for i in range(m):
        # Move j forward while x[i] - y[j] > threshold
        while j < n and x[i] - y[j] > threshold:
            j += 1

        # All elements from y[j] to y[n-1] satisfy x[i] - y[j] <= threshold
        count += n - j

        # Track boundary values
        if j < n:
            diff = x[i] - y[j]
            max_below = max(max_below, diff)

        if j > 0:
            diff = x[i] - y[j - 1]
            min_above = min(min_above, diff)

    # Fallback to actual min/max if no boundaries found
    if max_below == float("-inf"):
        max_below = x[0] - y[n - 1]
    if min_above == float("inf"):
        min_above = x[m - 1] - y[0]

    return count, max_below, min_above


def _select_kth_pairwise_diff(x: List[float], y: List[float], k: int) -> float:
    """
    Select the k-th smallest pairwise difference (1-indexed).

    Uses binary search in value space to avoid materializing all differences.

    Args:
        x: Sorted array of x values
        y: Sorted array of y values
        k: The rank to select (1-indexed)

    Returns:
        The k-th smallest pairwise difference
    """
    m = len(x)
    n = len(y)
    total = m * n

    if k < 1 or k > total:
        raise ValueError(f"k must be in [1, {total}], got {k}")

    # Initialize search bounds
    search_min = x[0] - y[n - 1]
    search_max = x[m - 1] - y[0]

    if np.isnan(search_min) or np.isnan(search_max):
        raise ValueError("NaN in input values")

    max_iterations = 128  # Sufficient for double precision convergence
    prev_min = float("-inf")
    prev_max = float("inf")

    for _ in range(max_iterations):
        if search_min == search_max:
            break

        mid = _midpoint(search_min, search_max)
        count_le, closest_below, closest_above = _count_and_neighbors(x, y, mid)

        # Check if we found the exact value
        if closest_below == closest_above:
            return closest_below

        # No progress means we're stuck between two discrete values
        if search_min == prev_min and search_max == prev_max:
            return closest_below if count_le >= k else closest_above

        prev_min = search_min
        prev_max = search_max

        # Narrow the search space
        if count_le >= k:
            search_max = closest_below
        else:
            search_min = closest_above

    if search_min != search_max:
        raise RuntimeError("Convergence failure (pathological input)")

    return search_min


def _fast_shift_python(
    x: List[float],
    y: List[float],
    p: Union[float, List[float]] = 0.5,
    assume_sorted: bool = False,
) -> Union[float, List[float]]:
    """
    Pure Python implementation of fast shift estimator.

    Computes quantiles of all pairwise differences {x_i - y_j} efficiently.

    Time complexity: O((m + n) * log(precision)) per quantile
    Space complexity: O(1)

    Args:
        x: First sample (will be sorted if assume_sorted is False)
        y: Second sample (will be sorted if assume_sorted is False)
        p: Quantile(s) to compute (0.5 for median). Can be a single float or list of floats.
        assume_sorted: If True, assumes x and y are already sorted in ascending order.

    Returns:
        The quantile estimate(s). Returns float if p is float, list if p is list.
    """
    if len(x) == 0 or len(y) == 0:
        raise ValueError("x and y must be non-empty")

    # Handle single probability or list
    return_single = isinstance(p, (float, int))
    probabilities = [p] if return_single else list(p)

    # Validate probabilities
    for pk in probabilities:
        if np.isnan(pk) or pk < 0.0 or pk > 1.0:
            raise ValueError(f"Probabilities must be within [0, 1], got {pk}")

    # Sort the arrays if not already sorted
    if assume_sorted:
        xs = list(x)
        ys = list(y)
    else:
        xs = sorted(x)
        ys = sorted(y)

    m = len(xs)
    n = len(ys)
    total = m * n

    # Type-7 quantile: h = 1 + (n-1)*p, then interpolate between floor(h) and ceil(h)
    required_ranks = set()
    interpolation_params = []

    for pk in probabilities:
        h = 1.0 + (total - 1) * pk
        lower_rank = int(np.floor(h))
        upper_rank = int(np.ceil(h))
        weight = h - lower_rank

        # Clamp to valid range
        lower_rank = max(1, min(total, lower_rank))
        upper_rank = max(1, min(total, upper_rank))

        interpolation_params.append((lower_rank, upper_rank, weight))
        required_ranks.add(lower_rank)
        required_ranks.add(upper_rank)

    # Compute required rank values
    rank_values = {}
    for rank in required_ranks:
        rank_values[rank] = _select_kth_pairwise_diff(xs, ys, rank)

    # Interpolate to get final quantile values
    result = []
    for lower_rank, upper_rank, weight in interpolation_params:
        lower = rank_values[lower_rank]
        upper = rank_values[upper_rank]
        if weight == 0.0:
            result.append(lower)
        else:
            result.append((1.0 - weight) * lower + weight * upper)

    return result[0] if return_single else result


def _fast_shift(
    x: Union[Sequence[float], NDArray],
    y: Union[Sequence[float], NDArray],
    p: Union[float, List[float]] = 0.5,
    assume_sorted: bool = False,
) -> Union[float, List[float]]:
    """
    Compute quantiles of all pairwise differences {x_i - y_j} efficiently.

    Internal implementation - not part of public API.
    Uses C implementation if available, falls back to pure Python.

    Time complexity: O((m + n) * log(precision)) per quantile
    Space complexity: O(1)

    Args:
        x: First sample
        y: Second sample
        p: Quantile(s) to compute (0.5 for median)
        assume_sorted: If True, assumes x and y are already sorted in ascending order.

    Returns:
        The quantile estimate(s)
    """
    if _HAS_C_EXTENSION:
        # Convert to numpy arrays and use C implementation
        # Note: C extension always sorts internally for safety
        x_arr = np.asarray(x, dtype=np.float64)
        y_arr = np.asarray(y, dtype=np.float64)
        return_single = isinstance(p, (float, int))
        p_arr = np.array([p] if return_single else p, dtype=np.float64)
        result = _fast_shift_c.fast_shift_c(x_arr, y_arr, p_arr)
        return float(result[0]) if return_single else result.tolist()
    # Fall back to pure Python implementation
    return _fast_shift_python(x, y, p, assume_sorted)
