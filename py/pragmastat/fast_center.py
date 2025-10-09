"""Fast O(n log n) implementation of the Center (Hodges-Lehmann) estimator.

Based on Monahan's Algorithm 616 (1984).
"""

from typing import List
import random
import numpy as np

# Try to import the C implementation, fall back to pure Python if unavailable
try:
    from . import _fast_center_c

    _HAS_C_EXTENSION = True
except ImportError:
    _HAS_C_EXTENSION = False


def _fast_center_python(values: List[float]) -> float:
    """
    Pure Python implementation of fast center estimator.

    Compute the median of all pairwise averages (xi + xj)/2 efficiently.

    Time complexity: O(n log n) expected
    Space complexity: O(n)

    Args:
        values: A list of numeric values

    Returns:
        The center estimate (Hodges-Lehmann estimator)
    """
    n = len(values)
    if n == 0:
        raise ValueError("Input array cannot be empty")
    if n == 1:
        return values[0]
    if n == 2:
        return (values[0] + values[1]) / 2

    # Sort the values
    sorted_values = sorted(values)

    # Calculate target median rank(s) among all pairwise sums
    total_pairs = n * (n + 1) // 2
    median_rank_low = (total_pairs + 1) // 2  # 1-based rank
    median_rank_high = (total_pairs + 2) // 2

    # Initialize search bounds for each row (1-based indexing)
    left_bounds = [i + 1 for i in range(n)]  # Row i pairs with columns [i+1..n]
    right_bounds = [n for i in range(n)]

    # Start with a good pivot: sum of middle elements
    pivot = sorted_values[(n - 1) // 2] + sorted_values[n // 2]
    active_set_size = total_pairs
    previous_count = 0

    while True:
        # === PARTITION STEP ===
        # Count pairwise sums less than current pivot
        count_below_pivot = 0
        current_column = n
        partition_counts = []

        for row in range(1, n + 1):  # 1-based
            # Move left from current column until we find sums < pivot
            while (
                current_column >= row
                and sorted_values[row - 1] + sorted_values[current_column - 1] >= pivot
            ):
                current_column -= 1

            # Count elements in this row that are < pivot
            elements_below = max(0, current_column - row + 1)
            partition_counts.append(elements_below)
            count_below_pivot += elements_below

        # === CONVERGENCE CHECK ===
        if count_below_pivot == previous_count:
            # No progress - use midrange strategy
            min_active_sum = float("inf")
            max_active_sum = float("-inf")

            for i in range(n):
                if left_bounds[i] > right_bounds[i]:
                    continue

                row_value = sorted_values[i]
                smallest_in_row = sorted_values[left_bounds[i] - 1] + row_value
                largest_in_row = sorted_values[right_bounds[i] - 1] + row_value

                min_active_sum = min(min_active_sum, smallest_in_row)
                max_active_sum = max(max_active_sum, largest_in_row)

            pivot = (min_active_sum + max_active_sum) / 2
            if pivot <= min_active_sum or pivot > max_active_sum:
                pivot = max_active_sum

            if min_active_sum == max_active_sum or active_set_size <= 2:
                return pivot / 2

            continue

        # === TARGET CHECK ===
        at_target_rank = (
            count_below_pivot == median_rank_low
            or count_below_pivot == median_rank_high - 1
        )

        if at_target_rank:
            # Find boundary values
            largest_below_pivot = float("-inf")
            smallest_at_or_above_pivot = float("inf")

            for i in range(n):
                count_in_row = partition_counts[i]
                row_value = sorted_values[i]
                total_in_row = n - i

                # Find largest sum in this row that's < pivot
                if count_in_row > 0:
                    last_below_index = i + count_in_row
                    last_below_value = row_value + sorted_values[last_below_index - 1]
                    largest_below_pivot = max(largest_below_pivot, last_below_value)

                # Find smallest sum in this row that's >= pivot
                if count_in_row < total_in_row:
                    first_at_or_above_index = i + count_in_row + 1
                    first_at_or_above_value = (
                        row_value + sorted_values[first_at_or_above_index - 1]
                    )
                    smallest_at_or_above_pivot = min(
                        smallest_at_or_above_pivot, first_at_or_above_value
                    )

            # Calculate final result
            if median_rank_low < median_rank_high:
                # Even total: average the two middle values
                return (smallest_at_or_above_pivot + largest_below_pivot) / 4
            else:
                # Odd total: return the single middle value
                need_largest = count_below_pivot == median_rank_low
                return (
                    largest_below_pivot if need_largest else smallest_at_or_above_pivot
                ) / 2

        # === UPDATE BOUNDS ===
        if count_below_pivot < median_rank_low:
            # Too few values below pivot - search higher
            for i in range(n):
                left_bounds[i] = i + partition_counts[i] + 1
        else:
            # Too many values below pivot - search lower
            for i in range(n):
                right_bounds[i] = i + partition_counts[i]

        # === PREPARE NEXT ITERATION ===
        previous_count = count_below_pivot

        # Recalculate active set size
        active_set_size = sum(
            max(0, right_bounds[i] - left_bounds[i] + 1) for i in range(n)
        )

        # Choose next pivot
        if active_set_size > 2:
            # Use randomized row median strategy
            target_index = random.randint(0, active_set_size - 1)
            cumulative_size = 0
            selected_row = 0

            for i in range(n):
                row_size = max(0, right_bounds[i] - left_bounds[i] + 1)
                if target_index < cumulative_size + row_size:
                    selected_row = i
                    break
                cumulative_size += row_size

            # Use median element of the selected row as pivot
            median_column_in_row = (
                left_bounds[selected_row] + right_bounds[selected_row]
            ) // 2
            pivot = (
                sorted_values[selected_row] + sorted_values[median_column_in_row - 1]
            )
        else:
            # Few elements remain - use midrange strategy
            min_remaining_sum = float("inf")
            max_remaining_sum = float("-inf")

            for i in range(n):
                if left_bounds[i] > right_bounds[i]:
                    continue

                row_value = sorted_values[i]
                min_in_row = sorted_values[left_bounds[i] - 1] + row_value
                max_in_row = sorted_values[right_bounds[i] - 1] + row_value

                min_remaining_sum = min(min_remaining_sum, min_in_row)
                max_remaining_sum = max(max_remaining_sum, max_in_row)

            pivot = (min_remaining_sum + max_remaining_sum) / 2
            if pivot <= min_remaining_sum or pivot > max_remaining_sum:
                pivot = max_remaining_sum

            if min_remaining_sum == max_remaining_sum:
                return pivot / 2


def _fast_center(values: List[float]) -> float:
    """
    Compute the median of all pairwise averages (xi + xj)/2 efficiently.

    Internal implementation - not part of public API.
    Uses C implementation if available, falls back to pure Python.

    Time complexity: O(n log n) expected
    Space complexity: O(n)

    Args:
        values: A list of numeric values

    Returns:
        The center estimate (Hodges-Lehmann estimator)
    """
    if _HAS_C_EXTENSION:
        # Convert to numpy array and use C implementation
        arr = np.asarray(values, dtype=np.float64)
        return _fast_center_c.fast_center_c(arr)
    else:
        # Fall back to pure Python implementation
        return _fast_center_python(values)
