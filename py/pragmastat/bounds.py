from dataclasses import dataclass

from .measurement_unit import MeasurementUnit


@dataclass(frozen=True)
class Bounds:
    """An interval [lower, upper] with an associated measurement unit."""

    lower: float
    upper: float
    unit: MeasurementUnit

    def contains(self, value: float) -> bool:
        """Returns True if value is within [lower, upper]."""
        return self.lower <= value <= self.upper
