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
        pass

    def samples(self, rng: "Rng", count: int) -> list[float]:
        """Generate multiple samples from this distribution."""
        return [self.sample(rng) for _ in range(count)]
