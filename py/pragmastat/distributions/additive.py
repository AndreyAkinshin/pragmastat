"""Additive (Normal/Gaussian) distribution."""

import math
from typing import TYPE_CHECKING

from .._constants import SMALLEST_POSITIVE_SUBNORMAL
from ..additive_cumulative import additive_cumulative
from ..additive_quantile import additive_quantile
from ..exp_function import exp_function
from .constants import SQRT_2PI
from .distribution import Distribution

if TYPE_CHECKING:
    from ..rng import Rng


class Additive(Distribution):
    """Additive (Normal/Gaussian) distribution with given mean and standard deviation.

    Uses the Box-Muller transform to generate samples.

    Example:
        >>> from pragmastat import Rng
        >>> from pragmastat.distributions import Additive
        >>> rng = Rng("demo-dist-additive")
        >>> dist = Additive(0.0, 1.0)  # Standard normal
        >>> sample = dist.sample(rng)
    """

    def __init__(self, mean: float, std_dev: float) -> None:
        """Create a new additive (normal) distribution.

        Args:
            mean: Location parameter (center of the distribution).
            std_dev: Scale parameter (standard deviation).

        Raises:
            ValueError: If mean is not finite, or if std_dev is not finite and positive.
        """
        if not math.isfinite(mean):
            raise ValueError("mean must be finite")
        if not (math.isfinite(std_dev) and std_dev > 0):
            raise ValueError("std_dev must be finite and positive")
        self.mean = mean
        self.std_dev = std_dev

    def sample(self, rng: "Rng") -> float:
        # Box-Muller transform
        u1 = rng.uniform_float()
        u2 = rng.uniform_float()

        # Avoid log(0) - use smallest positive subnormal for cross-language consistency
        if u1 == 0.0:
            u1 = SMALLEST_POSITIVE_SUBNORMAL

        r = math.sqrt(-2.0 * math.log(u1))
        theta = 2.0 * math.pi * u2

        # Use the first of the two Box-Muller outputs
        z = r * math.cos(theta)

        return self.mean + z * self.std_dev

    def density_at(self, x: float) -> float:
        z = (x - self.mean) / self.std_dev
        return exp_function(-z * z / 2) / (self.std_dev * SQRT_2PI)

    def cumulative_at(self, x: float) -> float:
        return additive_cumulative((x - self.mean) / self.std_dev)

    def quantile_at(self, p: float) -> float:
        """The inverse goes through additive_quantile, which is what that function was built for.

        It replaces two Newton steps taken against an error function accurate to 1.5e-07, which
        had left this quantile eight orders of magnitude coarser than the cumulative beside it.
        The infinities at the ends are the ones additive_quantile already returns.
        """
        return self.mean + self.std_dev * additive_quantile(p)
