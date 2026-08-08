"""Multiplicative (Log-Normal) distribution."""

import math
from typing import TYPE_CHECKING

from ..additive_cumulative import additive_cumulative
from ..additive_quantile import additive_quantile
from ..exp_function import exp_function
from ..log_function import log_function
from .additive import Additive
from .constants import SQRT_2PI
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
            ValueError: If log_mean is not finite, or if log_std_dev is not finite
                and positive.
        """
        if not math.isfinite(log_mean):
            raise ValueError("log_mean must be finite")
        if not (math.isfinite(log_std_dev) and log_std_dev > 0):
            raise ValueError("log_std_dev must be finite and positive")
        self.log_mean = log_mean
        self.log_std_dev = log_std_dev
        self._additive = Additive(log_mean, log_std_dev)

    def sample(self, rng: "Rng") -> float:
        return math.exp(self._additive.sample(rng))

    def density_at(self, x: float) -> float:
        """The support is x > 0, and that is the test.

        It used to be x < 1e-9, which is not the edge of the support but an arbitrary point
        inside it: a Multiplic(-30, 1) has its median at 9.4e-14, so that threshold reported zero
        density and zero probability across essentially the whole distribution.
        """
        if x <= 0.0:
            return 0.0
        z = (log_function(x) - self.log_mean) / self.log_std_dev
        return exp_function(-z * z / 2) / (x * self.log_std_dev * SQRT_2PI)

    def cumulative_at(self, x: float) -> float:
        """A lognormal variable's distribution function is the normal one applied to log x.

        So this goes through additive_cumulative rather than spelling the same thing out
        through erf.
        """
        if x <= 0.0:
            return 0.0
        return additive_cumulative((log_function(x) - self.log_mean) / self.log_std_dev)

    def quantile_at(self, p: float) -> float:
        if p <= 0.0:
            return 0.0
        if p >= 1.0:
            return math.inf
        return exp_function(self.log_mean + self.log_std_dev * additive_quantile(p))
