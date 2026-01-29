"""Additive (Normal/Gaussian) distribution."""

import math
from typing import TYPE_CHECKING

from .._constants import SMALLEST_POSITIVE_SUBNORMAL
from .distribution import Distribution

if TYPE_CHECKING:
    from ..rng import Rng


class Additive(Distribution):
    """Additive (Normal/Gaussian) distribution with given mean and standard deviation.

    Uses the Box-Muller transform to generate samples.

    Example:
        >>> from pragmastat import Rng
        >>> from pragmastat.distributions import Additive
        >>> rng = Rng(1729)
        >>> dist = Additive(0.0, 1.0)  # Standard normal
        >>> sample = dist.sample(rng)
    """

    def __init__(self, mean: float, std_dev: float) -> None:
        """Create a new additive (normal) distribution.

        Args:
            mean: Location parameter (center of the distribution).
            std_dev: Scale parameter (standard deviation).

        Raises:
            ValueError: If std_dev <= 0.
        """
        if std_dev <= 0:
            raise ValueError("std_dev must be positive")
        self.mean = mean
        self.std_dev = std_dev

    def sample(self, rng: "Rng") -> float:
        # Box-Muller transform
        u1 = rng.uniform()
        u2 = rng.uniform()

        # Avoid log(0) - use smallest positive subnormal for cross-language consistency
        if u1 == 0.0:
            u1 = SMALLEST_POSITIVE_SUBNORMAL

        r = math.sqrt(-2.0 * math.log(u1))
        theta = 2.0 * math.pi * u2

        # Use the first of the two Box-Muller outputs
        z = r * math.cos(theta)

        return self.mean + z * self.std_dev
