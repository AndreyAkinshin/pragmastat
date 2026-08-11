"""Binary64 primitives shared by the distribution estimation models.

Every operation here is written out explicitly so that all seven language
implementations evaluate the same graph and produce identical bits. None of
these helpers may be replaced by a fused, vectorized or library variant
without a new algorithm identifier.
"""

from __future__ import annotations

import math
import struct
from typing import TYPE_CHECKING

from ._constants import SMALLEST_POSITIVE_SUBNORMAL

if TYPE_CHECKING:
    from collections.abc import Sequence


def to_bits(value: float) -> int:
    """Reinterprets a binary64 value as a signed 64-bit integer."""
    return struct.unpack("<q", struct.pack("<d", value))[0]


def from_bits(bits: int) -> float:
    """Reinterprets a signed 64-bit integer as a binary64 value."""
    return struct.unpack("<d", struct.pack("<q", bits))[0]


def next_up(value: float) -> float:
    """Next representable binary64 value toward positive infinity.

    Implemented on the bit pattern rather than through ``math.nextafter`` so that
    every language implementation walks the same lattice by the same rule.
    """
    if math.isnan(value) or value == math.inf:
        return value
    if value == 0.0:
        return SMALLEST_POSITIVE_SUBNORMAL
    return from_bits(to_bits(value) + (1 if value > 0.0 else -1))


def next_down(value: float) -> float:
    """Next representable binary64 value toward negative infinity."""
    if math.isnan(value) or value == -math.inf:
        return value
    if value == 0.0:
        return -SMALLEST_POSITIVE_SUBNORMAL
    return from_bits(to_bits(value) + (-1 if value > 0.0 else 1))


def midpoint(a: float, b: float) -> float:
    """Overflow-safe midpoint of two finite values."""
    if a < 0.0 < b:
        return a / 2.0 + b / 2.0
    return a + (b - a) / 2.0


def lerp(a: float, b: float, t: float) -> float:
    """Interpolates between a and b, reproducing both endpoints exactly."""
    if t == 0.0:
        return a
    if t == 1.0:
        return b
    if a < 0.0 < b:
        return (1.0 - t) * a + t * b
    return a + t * (b - a)


def unit_ratio(num: float, den: float) -> float:
    """Ratio clamped to the unit interval. Callers must have proved 0 <= num <= den and den > 0."""
    t = num / den
    if math.isnan(t):
        raise RuntimeError("Numerical conformance failure: unit_ratio produced NaN")
    return min(1.0, max(0.0, t))


def ceil_sqrt(n: int) -> int:
    """Smallest integer k with k >= ceil(n / k), computed without forming k*k."""
    k = 1
    while k < (n + k - 1) // k:
        k += 1
    return k


def lower_bound(values: Sequence[float], start: int, stop: int, key: float) -> int:
    """First index i in [start, stop) with values[i] >= key, over an ascending sequence."""
    lo, hi = start, stop
    while lo < hi:
        mid = lo + (hi - lo) // 2
        if values[mid] < key:
            lo = mid + 1
        else:
            hi = mid
    return lo


def upper_bound(values: Sequence[float], key: float) -> int:
    """First index i with values[i] > key, over an ascending sequence."""
    lo, hi = 0, len(values)
    while lo < hi:
        mid = lo + (hi - lo) // 2
        if values[mid] <= key:
            lo = mid + 1
        else:
            hi = mid
    return lo
