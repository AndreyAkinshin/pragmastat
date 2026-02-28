from .measurement_unit import DISPARITY_UNIT, NUMBER_UNIT, RATIO_UNIT, MeasurementUnit


class UnitRegistry:
    """Stores measurement units and enables lookup by ID."""

    def __init__(self) -> None:
        self._by_id: dict[str, MeasurementUnit] = {}

    def register(self, unit: MeasurementUnit) -> None:
        """Adds a unit to the registry. Raises if ID already registered."""
        if unit.id in self._by_id:
            msg = f"unit with id '{unit.id}' is already registered"
            raise ValueError(msg)
        self._by_id[unit.id] = unit

    def resolve(self, unit_id: str) -> MeasurementUnit:
        """Looks up a unit by ID. Raises if not found."""
        if unit_id in self._by_id:
            return self._by_id[unit_id]
        msg = f"unknown unit id: '{unit_id}'"
        raise KeyError(msg)

    @staticmethod
    def standard() -> "UnitRegistry":
        """Returns a registry pre-populated with Number, Ratio, and Disparity units."""
        registry = UnitRegistry()
        registry.register(NUMBER_UNIT)
        registry.register(RATIO_UNIT)
        registry.register(DISPARITY_UNIT)
        return registry
