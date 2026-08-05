from dataclasses import dataclass

from .format import format_value
from .measurement_unit import NUMBER_UNIT, MeasurementUnit


@dataclass(frozen=True)
class Measurement:
    """A value paired with its measurement unit."""

    value: float
    unit: MeasurementUnit = NUMBER_UNIT

    def __float__(self) -> float:
        return self.value

    def __str__(self) -> str:
        """Renders the value, with the unit's abbreviation appended when it has one."""
        rendered = format_value(self.value)
        return f"{rendered} {self.unit.abbreviation}" if self.unit.abbreviation else rendered
