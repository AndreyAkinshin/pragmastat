from dataclasses import dataclass

from .measurement_unit import NUMBER_UNIT, MeasurementUnit


@dataclass(frozen=True)
class Measurement:
    """A value paired with its measurement unit."""

    value: float
    unit: MeasurementUnit = NUMBER_UNIT

    def __float__(self) -> float:
        return self.value
