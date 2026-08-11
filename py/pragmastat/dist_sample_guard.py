"""Shared sample preparation and unit plumbing for the distribution estimation models.

:func:`prepare` runs the two assumptions that follow validity in the canonical order:
``domain(x)`` (at least two observations and a finite representable range) and
``distinctness(x)`` (no repeated value; -0.0 and +0.0 count as repeated).
The caller's vector is never retained: the model always owns a private copy.

:func:`to_measurements` and :func:`to_raw` move a query vector between the numeric and the
measured worlds, which is all the measured facade needs on top of the numeric kernels.
"""

from __future__ import annotations

import math
from typing import TYPE_CHECKING

from .assumptions import AssumptionError, Subject
from .measurement import Measurement
from .measurement_unit import MeasurementUnit

if TYPE_CHECKING:
    from collections.abc import Sequence


def check_no_assume_sorted(assume_sorted: bool | None) -> None:
    """Refuses an ``assume_sorted`` promise made about a Sample.

    The static ports give their Sample overload no such parameter, so the combination cannot
    be written at all. A Sample already carries its own ascending promise, and the model never
    reads the flag on that path, so any explicit value -- ``True`` or ``False`` -- is a
    statement about an input that is not read.
    """
    if assume_sorted is not None:
        msg = "assume_sorted does not apply to a Sample, which already keeps its values ascending"
        raise AssumptionError(msg)


def prepare(values: Sequence[float], assume_sorted: bool) -> list[float]:
    """Copies a fitting vector into an ascending list of binary64 values."""
    copy = [float(value) for value in values]
    if not assume_sorted:
        copy.sort()

    n = len(copy)
    if n < 2:
        raise AssumptionError.domain(Subject.X)

    span = copy[n - 1] - copy[0]
    if not math.isfinite(span):
        raise AssumptionError.domain(Subject.X)

    for i in range(1, n):
        if copy[i - 1] == copy[i]:
            raise AssumptionError.distinctness(Subject.X)

    return copy


def to_measurements(values: Sequence[float], unit: MeasurementUnit) -> list[Measurement]:
    """Attaches a unit to every element of a query result."""
    return [Measurement(value, unit) for value in values]


def to_raw(values: Sequence[Measurement], unit: MeasurementUnit) -> list[float]:
    """Converts a measured query vector into the model's own unit.

    A query in a compatible but different unit is converted rather than rejected, matching the
    two-sample estimators, which reconcile their inputs instead of demanding an exact match.
    An incompatible unit is a caller error and is reported as one.

    A bare number is refused rather than assumed to be in the model's unit. The ports with a
    static type system get this from the signature; here it has to be a runtime check, because
    ``Measurement`` converts to ``float`` and the mistake would otherwise pass silently.
    """
    raw = []
    for value in values:
        if not isinstance(value, Measurement):
            msg = f"a measured query vector takes Measurement values, not {type(value).__name__}"
            raise TypeError(msg)
        source = value.unit
        if not source.is_compatible(unit):
            msg = f"can't convert {source.full_name} to {unit.full_name}"
            raise AssumptionError(msg)
        raw.append(value.value * MeasurementUnit.conversion_factor(source, unit))
    return raw
