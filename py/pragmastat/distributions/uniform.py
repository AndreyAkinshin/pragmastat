"""Uniform distribution."""

import math
from typing import TYPE_CHECKING

from .distribution import Distribution

if TYPE_CHECKING:
    from ..rng import Rng


class Uniform(Distribution):
    """Uniform distribution on [min, max).

    Example:
        >>> from pragmastat import Rng
        >>> from pragmastat.distributions import Uniform
        >>> rng = Rng("demo-dist-uniform")
        >>> dist = Uniform(0.0, 10.0)
        >>> sample = dist.sample(rng)
    """

    def __init__(self, min_val: float, max_val: float) -> None:
        """Create a new uniform distribution on [min, max).

        Args:
            min_val: Lower bound (inclusive).
            max_val: Upper bound (exclusive).

        Raises:
            ValueError: If min or max is not finite, or if min is not less than max.
        """
        if not math.isfinite(min_val):
            raise ValueError("min must be finite")
        if not math.isfinite(max_val):
            raise ValueError("max must be finite")
        if not min_val < max_val:
            raise ValueError("min must be less than max")
        self.min = min_val
        self.max = max_val

    def sample(self, rng: "Rng") -> float:
        return self.quantile_at(rng.uniform_float())

    def density_at(self, x: float) -> float:
        """The support is half-open, so the density is zero at max and positive at min.

        This is the one family assembled from arithmetic alone: no primitive of the toolkit
        appears on any of its three paths.

        NaN is refused before the bounds are compared. This is the only one of the fifteen
        functions whose result does not depend on its argument, so a NaN reaching the comparison
        satisfies neither half of it and falls through to an ordinary density.
        """
        if math.isnan(x):
            return x
        if x < self.min or x >= self.max:
            return 0.0
        return 1.0 / (self.max - self.min)

    def cumulative_at(self, x: float) -> float:
        if x < self.min:
            return 0.0
        if x >= self.max:
            return 1.0
        return (x - self.min) / (self.max - self.min)

    def quantile_at(self, p: float) -> float:
        if p <= 0.0:
            return self.min
        if p >= 1.0:
            return self.max
        return self.min + p * (self.max - self.min)
