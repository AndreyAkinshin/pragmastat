"""
Deterministic random number generator for cross-language reproducibility.

The Rng class provides a deterministic PRNG based on xoshiro256++ that
produces identical sequences across all Pragmastat language implementations.
"""

import time
from typing import List, Optional, Sequence, TypeVar, Union

from .xoshiro256 import Xoshiro256PlusPlus, fnv1a_hash

T = TypeVar("T")


class Rng:
    """
    A deterministic random number generator.

    Rng uses xoshiro256++ internally and guarantees identical output sequences
    across all Pragmastat language implementations when initialized with the same seed.

    Examples
    --------
    >>> rng = Rng("demo-uniform")  # Create from string seed
    >>> rng.uniform()
    0.2640554428629759

    >>> rng = Rng("demo-shuffle")
    >>> rng.shuffle([1.0, 2.0, 3.0, 4.0, 5.0])
    [4.0, 2.0, 3.0, 5.0, 1.0]

    >>> rng = Rng("demo-sample")
    >>> rng.sample([0, 1, 2, 3, 4, 5, 6, 7, 8, 9], 3)
    [3, 8, 9]
    """

    def __init__(self, seed: Optional[Union[int, str]] = None) -> None:
        """
        Create a new Rng.

        Parameters
        ----------
        seed : int, str, or None
            If int: use as integer seed directly.
            If str: hash using FNV-1a to produce a numeric seed.
            If None: use system time for entropy (non-deterministic).
        """
        if seed is None:
            seed_int = int(time.time_ns())
        elif isinstance(seed, str):
            seed_int = fnv1a_hash(seed)
        else:
            seed_int = seed

        self._inner = Xoshiro256PlusPlus(seed_int)

    # ========================================================================
    # Floating Point Methods
    # ========================================================================

    def uniform(self) -> float:
        """
        Generate a uniform random float in [0, 1).

        Uses 53 bits of precision for the mantissa.

        Returns
        -------
        float
            Random value in [0, 1).

        Examples
        --------
        >>> rng = Rng("demo-uniform")
        >>> rng.uniform()
        0.2640554428629759
        """
        return self._inner.uniform()

    def uniform_range(self, min_val: float, max_val: float) -> float:
        """
        Generate a uniform random float in [min, max).

        Parameters
        ----------
        min_val : float
            Minimum value (inclusive).
        max_val : float
            Maximum value (exclusive).

        Returns
        -------
        float
            Random value in [min, max).
            Returns min_val if min_val >= max_val.
        """
        return self._inner.uniform_range(min_val, max_val)

    # ========================================================================
    # Integer Methods
    # ========================================================================

    def uniform_int(self, min_val: int, max_val: int) -> int:
        """
        Generate a uniform random integer in [min, max).

        Uses modulo reduction which introduces slight bias for ranges that don't
        evenly divide 2^64. This bias is negligible for statistical simulations
        but not suitable for cryptographic applications.

        Parameters
        ----------
        min_val : int
            Minimum value (inclusive).
        max_val : int
            Maximum value (exclusive).

        Returns
        -------
        int
            Random integer in [min, max).
            Returns min_val if min_val >= max_val.

        Examples
        --------
        >>> rng = Rng("demo-uniform")
        >>> rng.uniform_int(0, 100)
        26
        """
        return self._inner.uniform_int(min_val, max_val)

    # ========================================================================
    # Boolean Methods
    # ========================================================================

    def uniform_bool(self) -> bool:
        """
        Generate a uniform random boolean with P(True) = 0.5.

        Returns
        -------
        bool
            Random boolean value.
        """
        return self._inner.uniform_bool()

    def shuffle(self, x: Sequence[T]) -> List[T]:
        """
        Return a shuffled copy of the input sequence.

        Uses the Fisher-Yates shuffle algorithm for uniform distribution.
        The original sequence is not modified.

        Parameters
        ----------
        x : Sequence[T]
            Input sequence to shuffle.

        Returns
        -------
        List[T]
            Shuffled copy of the input.

        Examples
        --------
        >>> rng = Rng("demo-shuffle")
        >>> rng.shuffle([1.0, 2.0, 3.0, 4.0, 5.0])
        [4.0, 2.0, 3.0, 5.0, 1.0]
        """
        result = list(x)
        n = len(result)

        # Fisher-Yates shuffle (backwards)
        for i in range(n - 1, 0, -1):
            j = self.uniform_int(0, i + 1)
            result[i], result[j] = result[j], result[i]

        return result

    def sample(self, x: Sequence[T], k: int) -> List[T]:
        """
        Sample k elements from the input sequence without replacement.

        Uses selection sampling to maintain order of first appearance.
        Returns up to k elements; if k >= len(x), returns all elements.

        Parameters
        ----------
        x : Sequence[T]
            Input sequence to sample from.
        k : int
            Number of elements to sample.

        Returns
        -------
        List[T]
            List of k sampled elements.

        Examples
        --------
        >>> rng = Rng("demo-sample")
        >>> rng.sample([0, 1, 2, 3, 4, 5, 6, 7, 8, 9], 3)
        [3, 8, 9]

        Raises
        ------
        ValueError
            If k is negative.
        """
        if k < 0:
            raise ValueError("k must be non-negative")
        n = len(x)
        if k >= n:
            return list(x)

        result: List[T] = []
        remaining = k

        for i, item in enumerate(x):
            available = n - i
            # Probability of selecting this item: remaining / available
            if self.uniform() * available < remaining:
                result.append(item)
                remaining -= 1
                if remaining == 0:
                    break

        return result

    def resample(self, x: Sequence[T], k: int) -> List[T]:
        """
        Resample k elements from the input sequence with replacement.

        Each element in the result is independently drawn from the input sequence.

        Parameters
        ----------
        x : Sequence[T]
            Input sequence to sample from.
        k : int
            Number of elements to sample.

        Returns
        -------
        List[T]
            List of k sampled elements with replacement.

        Raises
        ------
        ValueError
            If k is negative.
        """
        if k < 0:
            raise ValueError("k must be non-negative")
        n = len(x)
        if n == 0:
            raise ValueError("resample: cannot resample from empty sequence")

        result: List[T] = []
        for _ in range(k):
            idx = self.uniform_int(0, n)
            result.append(x[idx])
        return result
