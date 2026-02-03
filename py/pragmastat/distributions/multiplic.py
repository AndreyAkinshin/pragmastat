"""Multiplicative (Log-Normal) distribution."""

import math
from typing import TYPE_CHECKING

from .additive import Additive
from .distribution import Distribution

if TYPE_CHECKING:
    from ..rng import Rng


class Multiplic(Distribution):
    """Multiplicative (Log-Normal) distribution.

    The logarithm of samples follows an Additive (Normal) distribution.

    Example:
        >>> from pragmastat import Rng
        >>> from pragmastat.distributions import Multiplic
        >>> rng = Rng("demo-dist-multiplic")
        >>> dist = Multiplic(0.0, 1.0)
        >>> sample = dist.sample(rng)
    """

    def __init__(self, log_mean: float, log_std_dev: float) -> None:
        """Create a new multiplicative (log-normal) distribution.

        Args:
            log_mean: Mean of log values (location parameter).
            log_std_dev: Standard deviation of log values (scale parameter).

        Raises:
            ValueError: If log_std_dev <= 0.
        """
        if log_std_dev <= 0:
            raise ValueError("log_std_dev must be positive")
        self.log_mean = log_mean
        self.log_std_dev = log_std_dev
        self._additive = Additive(log_mean, log_std_dev)

    def sample(self, rng: "Rng") -> float:
        return math.exp(self._additive.sample(rng))
