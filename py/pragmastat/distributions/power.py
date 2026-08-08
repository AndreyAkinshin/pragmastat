"""Power (Pareto) distribution."""

import math
from typing import TYPE_CHECKING

from .._constants import MACHINE_EPSILON
from ..exp_function import exp_function
from ..log_function import log_function
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
            ValueError: If min or shape is not finite and positive.
        """
        if not (math.isfinite(min_val) and min_val > 0):
            raise ValueError("min must be finite and positive")
        if not (math.isfinite(shape) and shape > 0):
            raise ValueError("shape must be finite and positive")
        self.min = min_val
        self.shape = shape

    def sample(self, rng: "Rng") -> float:
        # Inverse CDF method: min / (1 - U)^(1/shape)
        u = rng.uniform_float()
        # Avoid division by zero - use machine epsilon for cross-language consistency
        if u == 1.0:
            u = 1.0 - MACHINE_EPSILON
        return self.min / math.pow(1.0 - u, 1.0 / self.shape)

    def density_at(self, x: float) -> float:
        """Shape * min^shape / x^(shape+1), rearranged as (shape / x) * exp(shape * (log min - log x)).

        The toolkit owns an exponential and a logarithm and owns no general power, so a real
        exponent is spelled as the exponential of a logarithm. This family is the reason the
        logarithm had to exist before the trio could.

        The support is [min, infinity), closed at the left, so the density at min is shape / min
        rather than zero, while the cumulative there is zero. Uniform reads the same way at its
        own lower endpoint.
        """
        if x < self.min:
            return 0.0
        return (self.shape / x) * exp_function(self.shape * (log_function(self.min) - log_function(x)))

    def cumulative_at(self, x: float) -> float:
        if x <= self.min:
            return 0.0
        return 1.0 - exp_function(self.shape * (log_function(self.min) - log_function(x)))

    def quantile_at(self, p: float) -> float:
        if p <= 0.0:
            return self.min
        if p >= 1.0:
            return math.inf
        return self.min * exp_function(-log_function(1.0 - p) / self.shape)
