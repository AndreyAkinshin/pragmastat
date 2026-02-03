"""Power (Pareto) distribution."""

import math
from typing import TYPE_CHECKING

from .._constants import MACHINE_EPSILON
from .distribution import Distribution

if TYPE_CHECKING:
    from ..rng import Rng


class Power(Distribution):
    """Power (Pareto) distribution with minimum value and shape parameter.

    Follows a power-law distribution where large values are rare but possible.

    Example:
        >>> from pragmastat import Rng
        >>> from pragmastat.distributions import Power
        >>> rng = Rng("demo-dist-power")
        >>> dist = Power(1.0, 2.0)  # min=1, shape=2
        >>> sample = dist.sample(rng)
    """

    def __init__(self, min_val: float, shape: float) -> None:
        """Create a new power (Pareto) distribution.

        Args:
            min_val: Minimum value (lower bound, > 0).
            shape: Shape parameter (alpha > 0, controls tail heaviness).

        Raises:
            ValueError: If min <= 0 or shape <= 0.
        """
        if min_val <= 0:
            raise ValueError("min must be positive")
        if shape <= 0:
            raise ValueError("shape must be positive")
        self.min = min_val
        self.shape = shape

    def sample(self, rng: "Rng") -> float:
        # Inverse CDF method: min / (1 - U)^(1/shape)
        u = rng.uniform()
        # Avoid division by zero - use machine epsilon for cross-language consistency
        if u == 1.0:
            u = 1.0 - MACHINE_EPSILON
        return self.min / math.pow(1.0 - u, 1.0 / self.shape)
