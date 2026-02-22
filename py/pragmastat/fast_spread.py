"""Fast O(n log n) implementation of the Spread (Shamos) estimator.

Based on Monahan's selection algorithm adapted for pairwise differences.
"""

import struct
from typing import List

import numpy as np

from .rng import Rng

# Try to import the C implementation, fall back to pure Python if unavailable
try:
    from . import _fast_spread_c

    _HAS_C_EXTENSION = True
except ImportError:
    _HAS_C_EXTENSION = False


def _derive_seed(values: List[float]) -> int:
    """Derive a deterministic seed from input values using FNV-1a hash."""
    FNV_OFFSET_BASIS = 0xCBF29CE484222325
    FNV_PRIME = 0x00000100000001B3
    MASK64 = (1 << 64) - 1

    hash_val = FNV_OFFSET_BASIS
    for v in values:
        bits = struct.unpack("<Q", struct.pack("<d", v))[0]
        for i in range(8):
            hash_val ^= (bits >> (i * 8)) & 0xFF
            hash_val = (hash_val * FNV_PRIME) & MASK64

    # Convert to signed int64 for consistency with Rust
    if hash_val >= (1 << 63):
        return hash_val - (1 << 64)
    return hash_val


def _fast_spread_python(values: List[float]) -> float:
    """
    Pure Python implementation of fast spread estimator.

    Compute the median of all pairwise absolute differences |xi - xj| efficiently.

    Time complexity: O(n log n) expected
    Space complexity: O(n)

    Args:
        values: A list of numeric values

    Returns:
        The spread estimate (Shamos estimator)
    """
    n = len(values)
    if n <= 1:
        return 0.0
    if n == 2:
        return abs(values[1] - values[0])

    # Create deterministic RNG from input values
    rng = Rng(_derive_seed(values))

    # Sort the values
    a = sorted(values)

    # Total number of pairwise differences with i < j
    N = n * (n - 1) // 2
    k_low = (N + 1) // 2  # 1-based rank of lower middle
    k_high = (N + 2) // 2  # 1-based rank of upper middle

    # Per-row active bounds over columns j (0-based indices)
    # Row i allows j in [i+1, n-1] initially
    L = [min(i + 1, n) for i in range(n)]  # n means empty
    R = [n - 1 for i in range(n)]  # inclusive

    for i in range(n):
        if L[i] > R[i]:
            L[i] = 1
            R[i] = 0  # mark empty

    row_counts = [0] * n

    # Initial pivot: a central gap
    pivot = a[n // 2] - a[(n - 1) // 2]
    prev_count_below = -1

    while True:
        # === PARTITION: count how many differences are < pivot ===
        count_below = 0
        largest_below = float("-inf")
        smallest_at_or_above = float("inf")

        j = 1  # global two-pointer (non-decreasing across rows)
        for i in range(n - 1):
            j = max(j, i + 1)
            while j < n and a[j] - a[i] < pivot:
                j += 1

            cnt_row = max(0, j - (i + 1))
            row_counts[i] = cnt_row
            count_below += cnt_row

            # boundary elements for this row
            if cnt_row > 0:
                cand_below = a[j - 1] - a[i]
                largest_below = max(largest_below, cand_below)

            if j < n:
                cand_at_or_above = a[j] - a[i]
                smallest_at_or_above = min(smallest_at_or_above, cand_at_or_above)

        # === TARGET CHECK ===
        at_target = (count_below == k_low) or (count_below == k_high - 1)

        if at_target:
            if k_low < k_high:
                # Even N: average the two central order stats
                return 0.5 * (largest_below + smallest_at_or_above)
            # Odd N: pick the single middle
            need_largest = count_below == k_low
            return largest_below if need_largest else smallest_at_or_above

        # === STALL HANDLING ===
        if count_below == prev_count_below:
            # Compute min/max remaining difference in the ACTIVE set
            min_active = float("inf")
            max_active = float("-inf")
            active = 0

            for i in range(n - 1):
                Li, Ri = L[i], R[i]
                if Li > Ri:
                    continue

                row_min = a[Li] - a[i]
                row_max = a[Ri] - a[i]
                min_active = min(min_active, row_min)
                max_active = max(max_active, row_max)
                active += Ri - Li + 1

            if active <= 0:
                if k_low < k_high:
                    return 0.5 * (largest_below + smallest_at_or_above)
                return largest_below if count_below >= k_low else smallest_at_or_above

            if max_active <= min_active:
                return min_active

            mid = 0.5 * (min_active + max_active)
            pivot = mid if (mid > min_active and mid <= max_active) else max_active
            prev_count_below = count_below
            continue

        # === SHRINK ACTIVE WINDOW ===
        if count_below < k_low:
            # Need larger differences: discard all strictly below pivot
            for i in range(n - 1):
                new_L = i + 1 + row_counts[i]
                L[i] = max(L[i], new_L)
                if L[i] > R[i]:
                    L[i] = 1
                    R[i] = 0
        else:
            # Too many below: keep only those strictly below pivot
            for i in range(n - 1):
                new_R = i + row_counts[i]
                R[i] = min(R[i], new_R)
                if R[i] < i + 1:
                    L[i] = 1
                    R[i] = 0

        prev_count_below = count_below

        # === CHOOSE NEXT PIVOT FROM ACTIVE SET ===
        active_size = sum(max(0, R[i] - L[i] + 1) for i in range(n - 1) if L[i] <= R[i])

        if active_size <= 2:
            # Few candidates left: return midrange of remaining
            min_rem = float("inf")
            max_rem = float("-inf")
            for i in range(n - 1):
                if L[i] > R[i]:
                    continue
                lo = a[L[i]] - a[i]
                hi = a[R[i]] - a[i]
                min_rem = min(min_rem, lo)
                max_rem = max(max_rem, hi)

            if active_size <= 0:
                if k_low < k_high:
                    return 0.5 * (largest_below + smallest_at_or_above)
                return largest_below if count_below >= k_low else smallest_at_or_above

            if k_low < k_high:
                return 0.5 * (min_rem + max_rem)
            return min_rem if abs((k_low - 1) - count_below) <= abs(count_below - k_low) else max_rem
        # Weighted random row selection
        t = rng.uniform_int(0, active_size)
        acc = 0
        row = 0
        for row in range(n - 1):
            if L[row] > R[row]:
                continue
            size = R[row] - L[row] + 1
            if t < acc + size:
                break
            acc += size

        # Median column of the selected row
        col = (L[row] + R[row]) // 2
        pivot = a[col] - a[row]


def _fast_spread(values) -> float:
    """
    Compute the median of all pairwise absolute differences |xi - xj| efficiently.

    Internal implementation - not part of public API.
    Uses C implementation if available, falls back to pure Python.

    Time complexity: O(n log n) expected
    Space complexity: O(n)

    Args:
        values: A list or numpy array of numeric values

    Returns:
        The spread estimate (Shamos estimator)
    """
    if _HAS_C_EXTENSION:
        arr = np.asarray(values, dtype=np.float64)
        return _fast_spread_c.fast_spread_c(arr)
    # Pure Python fallback requires a list
    if not isinstance(values, list):
        values = list(values)
    return _fast_spread_python(values)
