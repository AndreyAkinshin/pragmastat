from __future__ import annotations

from functools import cached_property
from typing import TYPE_CHECKING, Sequence

import numpy as np

from .assumptions import AssumptionError, check_positivity, check_validity
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
        _subject: str = "x",
    ) -> None:
        if unit is None:
            unit = NUMBER_UNIT

        arr = np.asarray(values, dtype=np.float64)
        check_validity(arr, _subject)

        self._values: NDArray = arr
        self._unit: MeasurementUnit = unit
        self._subject: str = _subject

        if weights is not None:
            w = np.asarray(weights, dtype=np.float64)
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
        return np.sort(self._values)

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
            self._subject,
            self._is_weighted,
            self._total_weight,
            self._weighted_size,
            self._weights.copy() if self._weights is not None else None,
        )

    def log(self) -> Sample:
        """Returns a new sample with log-transformed values and NumberUnit."""
        check_positivity(self._values, self._subject)
        log_values = np.log(self._values)
        return _build_sample(
            log_values,
            NUMBER_UNIT,
            self._subject,
            self._is_weighted,
            self._total_weight,
            self._weighted_size,
            self._weights.copy() if self._weights is not None else None,
        )

    def _with_subject(self, subject: str) -> Sample:
        """Returns a view of the sample with a different subject label."""
        return _build_sample(
            self._values,
            self._unit,
            subject,
            self._is_weighted,
            self._total_weight,
            self._weighted_size,
            self._weights,
        )

    # --- Comparison and display ---

    __hash__ = None  # mutable numpy arrays inside

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
            self._subject,
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
            self._subject,
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
    subject: str,
    is_weighted: bool,
    total_weight: float,
    weighted_size: float,
    weights: NDArray | None,
) -> Sample:
    """Module-level factory that bypasses validation (values already checked)."""
    obj = Sample.__new__(Sample)
    obj._values = values  # noqa: SLF001
    obj._unit = unit  # noqa: SLF001
    obj._subject = subject  # noqa: SLF001
    obj._is_weighted = is_weighted  # noqa: SLF001
    obj._total_weight = total_weight  # noqa: SLF001
    obj._weighted_size = weighted_size  # noqa: SLF001
    obj._weights = weights  # noqa: SLF001
    return obj


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
    """Prepare two samples: set subjects, check compatibility, convert to finer unit."""
    x = x._with_subject("x")  # noqa: SLF001
    y = y._with_subject("y")  # noqa: SLF001
    _check_compatible_units(x, y)
    return _convert_to_finer(x, y)
