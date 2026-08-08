"""Exponential distribution."""

import math
from typing import TYPE_CHECKING

from .._constants import MACHINE_EPSILON
from ..exp_function import exp_function
from ..log_function import log_function
from .distribution import Distribution

if TYPE_CHECKING:
    from ..rng import Rng


class Exp(Distribution):
    """Exponential distribution with given rate parameter.

    The mean of this distribution is 1/rate.

    Example:
        >>> from pragmastat import Rng
        >>> from pragmastat.distributions import Exp
        >>> rng = Rng("demo-dist-exp")
        >>> dist = Exp(1.0)  # rate=1, mean=1
        >>> sample = dist.sample(rng)
    """

    def __init__(self, rate: float) -> None:
        """Create a new exponential distribution with given rate.

        Args:
            rate: Rate parameter (lambda > 0).

        Raises:
            ValueError: If rate is not finite and positive.
        """
        if not (math.isfinite(rate) and rate > 0):
            raise ValueError("rate must be finite and positive")
        self.rate = rate

    def sample(self, rng: "Rng") -> float:
        # Inverse CDF method: -ln(1 - U) / rate
        u = rng.uniform_float()
        # Avoid log(0) - use machine epsilon for cross-language consistency
        if u == 1.0:
            u = 1.0 - MACHINE_EPSILON
        return -math.log(1.0 - u) / self.rate

    def density_at(self, x: float) -> float:
        if x < 0.0:
            return 0.0
        return self.rate * exp_function(-self.rate * x)

    def cumulative_at(self, x: float) -> float:
        if x < 0.0:
            return 0.0
        return 1.0 - exp_function(-self.rate * x)

    def quantile_at(self, p: float) -> float:
        if p <= 0.0:
            return 0.0
        if p >= 1.0:
            return math.inf
        return -log_function(1.0 - p) / self.rate
