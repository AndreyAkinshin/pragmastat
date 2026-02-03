"""Uniform distribution."""

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
            ValueError: If min >= max.
        """
        if min_val >= max_val:
            raise ValueError("min must be less than max")
        self.min = min_val
        self.max = max_val

    def sample(self, rng: "Rng") -> float:
        return self.min + rng.uniform() * (self.max - self.min)
