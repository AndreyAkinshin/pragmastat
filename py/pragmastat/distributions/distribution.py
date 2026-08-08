"""Base distribution class."""

from abc import ABC, abstractmethod
from typing import TYPE_CHECKING

if TYPE_CHECKING:
    from ..rng import Rng


class Distribution(ABC):
    """Base class for distributions that can generate samples."""

    @abstractmethod
    def sample(self, rng: "Rng") -> float:
        """Generate a single sample from this distribution."""

    @abstractmethod
    def density_at(self, x: float) -> float:
        """Probability density at x. Zero outside the support."""

    @abstractmethod
    def cumulative_at(self, x: float) -> float:
        """Probability of not exceeding x. Zero below the support, one above it."""

    @abstractmethod
    def quantile_at(self, p: float) -> float:
        """The value below which a fraction p of the distribution lies.

        A p outside (0, 1) returns the endpoints of the support, which are infinities for the
        unbounded families.
        """

    def samples(self, rng: "Rng", count: int) -> list[float]:
        """Generate multiple samples from this distribution."""
        return [self.sample(rng) for _ in range(count)]
