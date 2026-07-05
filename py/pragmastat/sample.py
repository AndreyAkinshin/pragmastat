from __future__ import annotations

from functools import cached_property
from typing import TYPE_CHECKING, Sequence

import numpy as np

from .assumptions import AssumptionError, check_validity
from .measurement_unit import NUMBER_UNIT, MeasurementUnit

if TYPE_CHECKING:
    from numpy.typing import NDArray


class Sample:
    """Wraps values with optional weights and a measurement unit."""

    def __init__(
        self,
        values: Sequence[float] | NDArray,
        weights: Sequence[float] | NDArray | None = None,
        unit: MeasurementUnit | None = None,
    ) -> None:
        if unit is None:
            unit = NUMBER_UNIT

        arr = np.array(values, dtype=np.float64)
        # Construction can't know which argument position this sample fills, so
        # construction-time validity errors are always reported under subject "x".
        check_validity(arr, "x")

        _freeze_array(arr)
        self._values: NDArray = arr
        self._unit: MeasurementUnit = unit

        if weights is not None:
            w = np.array(weights, dtype=np.float64)
            if len(w) != len(arr):
                msg = f"weights length ({len(w)}) must match values length ({len(arr)})"
                raise ValueError(msg)
            if np.any(w < 0):
                msg = "all weights must be non-negative"
                raise ValueError(msg)
            total = float(np.sum(w))
            if total < 1e-9:
                msg = "total weight must be positive"
                raise ValueError(msg)
            _freeze_array(w)
            self._weights: NDArray | None = w
            self._is_weighted = True
            self._total_weight = total
            self._weighted_size = float((total * total) / np.sum(w * w))
        else:
            self._weights = None
            self._is_weighted = False
            self._total_weight = 1.0
            self._weighted_size = float(len(arr))

    @property
    def values(self) -> NDArray:
        return self._values

    @property
    def weights(self) -> NDArray | None:
        return self._weights

    @property
    def unit(self) -> MeasurementUnit:
        return self._unit

    @property
    def size(self) -> int:
        return len(self._values)

    @property
    def is_weighted(self) -> bool:
        return self._is_weighted

    @property
    def total_weight(self) -> float:
        return self._total_weight

    @property
    def weighted_size(self) -> float:
        return self._weighted_size

    @cached_property
    def sorted_values(self) -> NDArray:
        return _freeze_array(np.sort(self._values))

    def convert_to(self, target: MeasurementUnit) -> Sample:
        """Converts the sample to a different (compatible) unit."""
        if not self._unit.is_compatible(target):
            msg = f"can't convert {self._unit.full_name} to {target.full_name}"
            raise AssumptionError(msg)
        if self._unit == target:
            return self
        factor = MeasurementUnit.conversion_factor(self._unit, target)
        converted = self._values * factor
        return _build_sample(
            converted,
            target,
            self._is_weighted,
            self._total_weight,
            self._weighted_size,
            self._weights.copy() if self._weights is not None else None,
        )

    # --- Comparison and display ---

    # Unhashable by design: __eq__ compares float array contents element-wise,
    # and no hash can be both cheap and consistent with that value equality.
    __hash__ = None

    def __eq__(self, other: object) -> bool:
        if not isinstance(other, Sample):
            return NotImplemented
        return (
            np.array_equal(self._values, other._values)
            and self._unit == other._unit
            and self._is_weighted == other._is_weighted
            and (
                (self._weights is None and other._weights is None)
                or (
                    self._weights is not None
                    and other._weights is not None
                    and np.array_equal(self._weights, other._weights)
                )
            )
        )

    def __repr__(self) -> str:
        if self._is_weighted:
            return f"Sample(size={self.size}, unit={self._unit.id!r}, weighted=True)"
        return f"Sample(size={self.size}, unit={self._unit.id!r})"

    # --- Arithmetic operators ---

    def __mul__(self, scalar: float) -> Sample:
        if not isinstance(scalar, (int, float)):
            return NotImplemented
        return _build_sample(
            self._values * scalar,
            self._unit,
            self._is_weighted,
            self._total_weight,
            self._weighted_size,
            self._weights,
        )

    def __rmul__(self, scalar: float) -> Sample:
        return self.__mul__(scalar)

    def __add__(self, scalar: float) -> Sample:
        if not isinstance(scalar, (int, float)):
            return NotImplemented
        return _build_sample(
            self._values + scalar,
            self._unit,
            self._is_weighted,
            self._total_weight,
            self._weighted_size,
            self._weights,
        )

    def __radd__(self, scalar: float) -> Sample:
        return self.__add__(scalar)

    def __sub__(self, scalar: float) -> Sample:
        if not isinstance(scalar, (int, float)):
            return NotImplemented
        return self.__add__(-scalar)


def _build_sample(  # noqa: PLR0913
    values: NDArray,
    unit: MeasurementUnit,
    is_weighted: bool,
    total_weight: float,
    weighted_size: float,
    weights: NDArray | None,
) -> Sample:
    """Module-level factory that bypasses validation (values already checked)."""
    obj = Sample.__new__(Sample)
    obj._values = _freeze_array(values)  # noqa: SLF001
    obj._unit = unit  # noqa: SLF001
    obj._is_weighted = is_weighted  # noqa: SLF001
    obj._total_weight = total_weight  # noqa: SLF001
    obj._weighted_size = weighted_size  # noqa: SLF001
    obj._weights = _freeze_array(weights) if weights is not None else None  # noqa: SLF001
    return obj


def _freeze_array(array: NDArray) -> NDArray:
    """Marks an array as read-only so cached sorted values cannot go stale."""
    array.setflags(write=False)
    return array


def _check_non_weighted(name: str, s: Sample) -> None:
    """Raises AssumptionError if the sample is weighted."""
    if s.is_weighted:
        msg = f"weighted samples are not supported for {name}"
        raise AssumptionError(msg)


def _check_compatible_units(a: Sample, b: Sample) -> None:
    """Raises AssumptionError if two samples have incompatible units."""
    if not a.unit.is_compatible(b.unit):
        msg = f"can't convert {a.unit.full_name} to {b.unit.full_name}"
        raise AssumptionError(msg)


def _convert_to_finer(a: Sample, b: Sample) -> tuple[Sample, Sample]:
    """Converts both samples to the finer unit."""
    if a.unit == b.unit:
        return a, b
    target = MeasurementUnit.finer(a.unit, b.unit)
    return a.convert_to(target), b.convert_to(target)


def _prepare_pair(x: Sample, y: Sample) -> tuple[Sample, Sample]:
    """Prepare two samples: check unit compatibility, convert to finer unit.

    The error "subject" (x vs y) is determined positionally by the validating
    function, not stored on the Sample, so no relabeling happens here.

    When both samples already share a unit, they are returned unchanged, so any
    warm sorted-value caches survive. When a unit conversion is required,
    ``convert_to`` builds a fresh Sample with scaled values; that new Sample
    starts with a cold cache (the sorted view is NOT carried over).
    """
    _check_compatible_units(x, y)
    return _convert_to_finer(x, y)
