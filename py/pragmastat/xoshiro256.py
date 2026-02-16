"""
xoshiro256++ PRNG implementation for cross-language reproducibility.

Reference: https://prng.di.unimi.it/xoshiro256plusplus.c
"""

from typing import List

# Mask for 64-bit unsigned operations
U64_MASK = 0xFFFFFFFFFFFFFFFF


def _rotl(x: int, k: int) -> int:
    """Rotate left for 64-bit unsigned integers."""
    x &= U64_MASK
    return ((x << k) | (x >> (64 - k))) & U64_MASK


class SplitMix64:
    """SplitMix64 PRNG for seed expansion."""

    def __init__(self, seed: int) -> None:
        self._state = seed & U64_MASK

    def next(self) -> int:
        self._state = (self._state + 0x9E3779B97F4A7C15) & U64_MASK
        z = self._state
        z = ((z ^ (z >> 30)) * 0xBF58476D1CE4E5B9) & U64_MASK
        z = ((z ^ (z >> 27)) * 0x94D049BB133111EB) & U64_MASK
        return (z ^ (z >> 31)) & U64_MASK


class Xoshiro256PlusPlus:
    """
    xoshiro256++ PRNG.

    This is the jump-free version of the algorithm. It passes BigCrush
    and is used by .NET 6+, Julia, and Rust's rand crate.
    """

    def __init__(self, seed: int) -> None:
        """Create a new generator from a 64-bit seed."""
        sm = SplitMix64(seed)
        self._state: List[int] = [sm.next(), sm.next(), sm.next(), sm.next()]

    def next_u64(self) -> int:
        """Generate the next 64-bit random value."""
        s = self._state
        result = (_rotl((s[0] + s[3]) & U64_MASK, 23) + s[0]) & U64_MASK

        t = (s[1] << 17) & U64_MASK

        s[2] ^= s[0]
        s[3] ^= s[1]
        s[1] ^= s[2]
        s[0] ^= s[3]

        s[2] ^= t
        s[3] = _rotl(s[3], 45)

        return result

    # ========================================================================
    # Floating Point Methods
    # ========================================================================

    def uniform_float(self) -> float:
        """Generate a uniform float in [0, 1)."""
        return (self.next_u64() >> 11) * (1.0 / (1 << 53))

    def uniform_float_range(self, min_val: float, max_val: float) -> float:
        """Generate a uniform float in [min, max).

        Note: FP rounding in min + (max-min)*u can theoretically yield max
        for extreme values of (max-min). Acceptable for statistical use.
        """
        if min_val >= max_val:
            return min_val
        return min_val + (max_val - min_val) * self.uniform_float()

    # ========================================================================
    # Integer Methods
    # ========================================================================

    def uniform_int(self, min_val: int, max_val: int) -> int:
        """Generate a uniform integer in [min, max).

        Raises:
            OverflowError: If max_val - min_val exceeds u64 range.
        """
        if min_val >= max_val:
            return min_val
        range_size = max_val - min_val
        if range_size > 0xFFFFFFFFFFFFFFFF:
            raise OverflowError("uniform_int: range overflow (max - min exceeds u64)")
        return min_val + (self.next_u64() % range_size)

    # ========================================================================
    # Boolean Methods
    # ========================================================================

    def uniform_bool(self) -> bool:
        """Generate a uniform bool with P(True) = 0.5."""
        return self.uniform_float() < 0.5


# FNV-1a hash constants
FNV_OFFSET_BASIS = 0xCBF29CE484222325
FNV_PRIME = 0x00000100000001B3


def fnv1a_hash(s: str) -> int:
    """Compute FNV-1a 64-bit hash of a string."""
    hash_val = FNV_OFFSET_BASIS
    for byte in s.encode("utf-8"):
        hash_val ^= byte
        hash_val = (hash_val * FNV_PRIME) & U64_MASK
    return hash_val
