"""Exponential distribution."""

import math
from typing import TYPE_CHECKING

from .._constants import MACHINE_EPSILON
from .distribution import Distribution

if TYPE_CHECKING:
    from ..rng import Rng


class Exp(Distribution):
    """Exponential distribution with given rate parameter.

    The mean of this distribution is 1/rate.

    Example:
        >>> from pragmastat import Rng
        >>> from pragmastat.distributions import Exp
        >>> rng = Rng(1729)
        >>> dist = Exp(1.0)  # rate=1, mean=1
        >>> sample = dist.sample(rng)
    """

    def __init__(self, rate: float) -> None:
        """Create a new exponential distribution with given rate.

        Args:
            rate: Rate parameter (lambda > 0).

        Raises:
            ValueError: If rate <= 0.
        """
        if rate <= 0:
            raise ValueError("rate must be positive")
        self.rate = rate

    def sample(self, rng: "Rng") -> float:
        # Inverse CDF method: -ln(1 - U) / rate
        u = rng.uniform()
        # Avoid log(0) - use machine epsilon for cross-language consistency
        if u == 1.0:
            u = 1.0 - MACHINE_EPSILON
        return -math.log(1.0 - u) / self.rate
