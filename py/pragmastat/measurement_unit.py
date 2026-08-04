from dataclasses import dataclass


@dataclass(frozen=True)
class MeasurementUnit:
    """A unit of measurement with identity, family, and conversion support.

    An empty abbreviation is legal: the three dimensionless units have none.
    """

    id: str
    family: str
    abbreviation: str
    full_name: str
    base_units: int

    def __post_init__(self) -> None:
        """Reject malformed unit definitions."""
        # A non-positive base_units poisons every conversion through this unit, and it makes
        # finer pick it over any well-formed one. An empty family makes unrelated units
        # compatible.
        if not self.id:
            raise ValueError("id cannot be empty")
        if not self.family:
            raise ValueError("family cannot be empty")
        if self.base_units <= 0:
            raise ValueError("base_units must be positive")
        # The annotation above says int, and an annotation enforces nothing at runtime. The four
        # ports that type this as a 64-bit integer cannot express a fractional scale at all, so a
        # unit that constructs here and nowhere else breaks the promise that one program reads the
        # same in every port. A whole-valued float is accepted: 1.0 and 1 are the same scale.
        if float(self.base_units) != int(self.base_units):
            raise ValueError("base_units must be a whole number")

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
