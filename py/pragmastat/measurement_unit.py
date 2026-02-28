from dataclasses import dataclass


@dataclass(frozen=True)
class MeasurementUnit:
    """A unit of measurement with identity, family, and conversion support."""

    id: str
    family: str
    abbreviation: str
    full_name: str
    base_units: int

    def is_compatible(self, other: "MeasurementUnit") -> bool:
        """Returns True if both units belong to the same family."""
        return self.family == other.family

    @staticmethod
    def finer(a: "MeasurementUnit", b: "MeasurementUnit") -> "MeasurementUnit":
        """Returns the unit with smaller base_units (higher precision)."""
        return a if a.base_units <= b.base_units else b

    @staticmethod
    def conversion_factor(from_unit: "MeasurementUnit", to_unit: "MeasurementUnit") -> float:
        """Returns the multiplier to convert from one unit to another."""
        return from_unit.base_units / to_unit.base_units


NUMBER_UNIT = MeasurementUnit("number", "Number", "", "Number", 1)
RATIO_UNIT = MeasurementUnit("ratio", "Ratio", "", "Ratio", 1)
DISPARITY_UNIT = MeasurementUnit("disparity", "Disparity", "", "Disparity", 1)
