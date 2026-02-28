from dataclasses import dataclass

from .measurement_unit import MeasurementUnit


@dataclass(frozen=True)
class Measurement:
    """A value paired with its measurement unit."""

    value: float
    unit: MeasurementUnit

    def __float__(self) -> float:
        return self.value
