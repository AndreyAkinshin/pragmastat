"""Fast algorithm for finding quantiles of pairwise averages (x[i] + x[j])/2.

Uses binary search with counting function to find exact quantiles in O(n log(range)) time.
"""

from typing import List, Tuple

# Relative epsilon for floating-point comparisons in binary search convergence.
RELATIVE_EPSILON = 1e-14


def _count_pairs_less_or_equal(sorted_vals: List[float], threshold: float) -> int:
    """
    Counts how many pairwise averages (sorted[i] + sorted[j])/2 where i <= j are <= threshold.

    Args:
        sorted_vals: Sorted list of values
        threshold: Threshold to count against

    Returns:
        Number of pairwise averages <= threshold
    """
    n = len(sorted_vals)
    count = 0
    # j is not reset: as i increases, threshold decreases monotonically
    j = n - 1

    for i in range(n):
        target = 2 * threshold - sorted_vals[i]
        while j >= 0 and sorted_vals[j] > target:
            j -= 1
        if j >= i:
            count += j - i + 1

    return count


def _find_exact_quantile(sorted_vals: List[float], k: int) -> float:
    """
    Finds the k-th smallest pairwise average using binary search.

    Args:
        sorted_vals: Sorted list of values
        k: 1-based rank of the desired quantile

    Returns:
        The k-th smallest pairwise average
    """
    n = len(sorted_vals)
    total_pairs = n * (n + 1) // 2

    # Early-return edge cases
    if n == 1:
        return sorted_vals[0]
    if k == 1:
        return sorted_vals[0]
    if k == total_pairs:
        return sorted_vals[n - 1]

    min_val = sorted_vals[0]
    max_val = sorted_vals[n - 1]

    # Binary search on value range
    lo = min_val
    hi = max_val

    while hi - lo > RELATIVE_EPSILON * max(1.0, abs(lo), abs(hi)):
        mid = (lo + hi) / 2
        count = _count_pairs_less_or_equal(sorted_vals, mid)
        if count < k:
            lo = mid
        else:
            hi = mid

    target = (lo + hi) / 2

    # Extract candidates that are close to the target
    candidates: List[float] = []
    for i in range(n):
        threshold = 2 * target - sorted_vals[i]

        # Find left boundary using binary search
        left = i
        right = n
        while left < right:
            m = (left + right) // 2
            if sorted_vals[m] < threshold - RELATIVE_EPSILON:
                left = m + 1
            else:
                right = m

        if (
            left < n
            and left >= i
            and abs(sorted_vals[left] - threshold)
            < RELATIVE_EPSILON * max(1.0, abs(threshold))
        ):
            candidates.append((sorted_vals[i] + sorted_vals[left]) / 2)

        if left > i:
            avg_before = (sorted_vals[i] + sorted_vals[left - 1]) / 2
            if avg_before <= target + RELATIVE_EPSILON:
                candidates.append(avg_before)

    if not candidates:
        return target

    candidates.sort()

    # Return the candidate that gives exactly k pairs <= it
    for c in candidates:
        if _count_pairs_less_or_equal(sorted_vals, c) >= k:
            return c

    return target


def fast_center_quantile_bounds(
    sorted_vals: List[float], k_lo: int, k_hi: int
) -> Tuple[float, float]:
    """
    Finds both lower and upper quantile bounds for pairwise averages.

    Args:
        sorted_vals: Sorted list of values
        k_lo: 1-based rank for lower bound
        k_hi: 1-based rank for upper bound

    Returns:
        Tuple of (lower bound, upper bound)
    """
    n = len(sorted_vals)
    total_pairs = n * (n + 1) // 2

    # Clamp margins to valid range
    k_lo = max(1, min(k_lo, total_pairs))
    k_hi = max(1, min(k_hi, total_pairs))

    lower = _find_exact_quantile(sorted_vals, k_lo)
    upper = _find_exact_quantile(sorted_vals, k_hi)

    # Ensure lower <= upper
    return (min(lower, upper), max(lower, upper))
