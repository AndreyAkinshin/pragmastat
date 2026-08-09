from dataclasses import dataclass

from ._normalize_zero import normalize_zero
from .format import format_value
from .measurement_unit import MeasurementUnit


@dataclass(frozen=True)
class Bounds:
    """An interval [lower, upper] with an associated measurement unit."""

    lower: float
    upper: float
    unit: MeasurementUnit

    def __post_init__(self) -> None:
        """Normalize -0.0 to 0.0 on both endpoints (see normalize_zero)."""
        object.__setattr__(self, "lower", normalize_zero(self.lower))
        object.__setattr__(self, "upper", normalize_zero(self.upper))

    def contains(self, value: float) -> bool:
        """Returns True if value is within [lower, upper]."""
        return self.lower <= value <= self.upper

    def __str__(self) -> str:
        """Renders the interval as "[lower;upper]", with the unit's abbreviation when it has one."""
        interval = f"[{format_value(self.lower)};{format_value(self.upper)}]"
        return f"{interval} {self.unit.abbreviation}" if self.unit.abbreviation else interval
