"""Compare1 and Compare2: confirmatory analysis for one-sample and two-sample estimators.

These high-level APIs compare estimates (Center, Spread, Shift, Ratio, Disparity)
against practical thresholds and return verdicts (Less, Greater, or Inconclusive).
"""

from __future__ import annotations

import math
from dataclasses import dataclass
from enum import Enum
from typing import TYPE_CHECKING

from .assumptions import AssumptionError
from .estimators import (
    center as center_estimator,
)
from .estimators import (
    center_bounds,
    disparity_bounds,
    ratio_bounds,
    shift_bounds,
    spread_bounds,
)
from .estimators import (
    disparity as disparity_estimator,
)
from .estimators import (
    ratio as ratio_estimator,
)
from .estimators import (
    shift as shift_estimator,
)
from .estimators import (
    spread as spread_estimator,
)
from .measurement import Measurement
from .measurement_unit import DISPARITY_UNIT, RATIO_UNIT, MeasurementUnit
from .sample import Sample, _check_compatible_units, _check_non_weighted, _convert_to_finer

if TYPE_CHECKING:
    from collections.abc import Callable

    from .bounds import Bounds


class Metric(Enum):
    """Metric types supported by Compare1 and Compare2."""

    CENTER = "center"
    SPREAD = "spread"
    SHIFT = "shift"
    RATIO = "ratio"
    DISPARITY = "disparity"


class ComparisonVerdict(Enum):
    """Verdict from comparing an estimate against a threshold."""

    LESS = "less"
    GREATER = "greater"
    INCONCLUSIVE = "inconclusive"


@dataclass(frozen=True)
class Threshold:
    """A threshold value with a metric type and misrate for comparison.

    Args:
        metric: The metric type (Center, Spread, Shift, Ratio, or Disparity)
        value: The threshold value as a float shorthand or explicit Measurement
        misrate: The per-threshold misclassification rate (must be in (0, 1])

    Raises:
        AssumptionError: If misrate is not in (0, 1] or value is not finite.
    """

    metric: Metric
    value: Measurement | float
    misrate: float

    def __post_init__(self) -> None:
        if not (0.0 < self.misrate <= 1.0):
            raise AssumptionError.domain("misrate")
        if isinstance(self.value, Measurement):
            if not math.isfinite(self.value.value):
                raise AssumptionError("threshold value must be finite")
            return
        try:
            value = float(self.value)
        except (TypeError, ValueError) as exc:
            raise AssumptionError("threshold value must be finite") from exc
        if not math.isfinite(value):
            raise AssumptionError("threshold value must be finite")
        object.__setattr__(self, "value", value)


@dataclass(frozen=True)
class Projection:
    """A projection containing estimate, bounds, and verdict for a single threshold.

    Attributes:
        threshold: The threshold that was evaluated
        estimate: The point estimate
        bounds: The confidence bounds
        verdict: The comparison verdict
    """

    threshold: Threshold
    estimate: Measurement
    bounds: Bounds
    verdict: ComparisonVerdict


@dataclass(frozen=True)
class _MetricSpec:
    """Internal specification for a metric's validation, estimation, and bounds."""

    metric: Metric
    validate_and_normalize: Callable
    estimate: Callable
    bounds: Callable
    seeded_bounds: Callable | None = None


def _as_numeric_threshold(value: Measurement | float) -> float:
    if isinstance(value, Measurement):
        return value.value
    return float(value)


def _normalize_linear_threshold(value: Measurement | float, target_unit: MeasurementUnit) -> float:
    if isinstance(value, Measurement):
        if not value.unit.is_compatible(target_unit):
            raise AssumptionError(
                f"can't convert {value.unit.full_name} to {target_unit.full_name}",
            )
        factor = MeasurementUnit.conversion_factor(value.unit, target_unit)
        return value.value * factor
    return float(value)


def _normalize_center(threshold: Threshold, x: Sample, _y: Sample | None = None) -> float:
    """Validates and normalizes a Center threshold."""
    return _normalize_linear_threshold(threshold.value, x.unit)


def _normalize_spread(threshold: Threshold, x: Sample, _y: Sample | None = None) -> float:
    """Validates and normalizes a Spread threshold."""
    return _normalize_linear_threshold(threshold.value, x.unit)


def _normalize_shift(threshold: Threshold, x: Sample, y: Sample | None = None) -> float:
    """Validates and normalizes a Shift threshold."""
    if y is None:
        raise AssumptionError("shift threshold normalization requires both samples")
    target_unit = MeasurementUnit.finer(x.unit, y.unit)
    return _normalize_linear_threshold(threshold.value, target_unit)


def _validate_ratio(threshold: Threshold, _x: Sample | None = None, _y: Sample | None = None) -> float:
    """Validates and normalizes a Ratio threshold."""
    if isinstance(threshold.value, Measurement):
        unit = threshold.value.unit
        if unit.id not in ("ratio", "number"):
            raise AssumptionError(f"can't convert {unit.full_name} to Ratio")
    value = _as_numeric_threshold(threshold.value)
    if value <= 0.0:
        raise AssumptionError("ratio threshold value must be positive")
    return value


def _validate_disparity(threshold: Threshold, _x: Sample | None = None, _y: Sample | None = None) -> float:
    """Validates and normalizes a Disparity threshold."""
    if isinstance(threshold.value, Measurement):
        unit = threshold.value.unit
        if unit.id not in ("disparity", "number"):
            raise AssumptionError(f"can't convert {unit.full_name} to Disparity")
    return _as_numeric_threshold(threshold.value)


# Compare1 metric specifications
_COMPARE1_SPECS: list[_MetricSpec] = [
    _MetricSpec(
        metric=Metric.CENTER,
        validate_and_normalize=_normalize_center,
        estimate=lambda x, _: Measurement(center_estimator(x).value, x.unit),
        bounds=lambda x, _, misrate: center_bounds(x, misrate),
        seeded_bounds=None,
    ),
    _MetricSpec(
        metric=Metric.SPREAD,
        validate_and_normalize=_normalize_spread,
        estimate=lambda x, _: Measurement(spread_estimator(x).value, x.unit),
        bounds=lambda x, _, misrate: spread_bounds(x, misrate),
        seeded_bounds=lambda x, _, misrate, seed: spread_bounds(x, misrate, seed=seed),
    ),
]

# Compare2 metric specifications
_COMPARE2_SPECS: list[_MetricSpec] = [
    _MetricSpec(
        metric=Metric.SHIFT,
        validate_and_normalize=_normalize_shift,
        estimate=lambda x, y: Measurement(shift_estimator(x, y).value, x.unit),
        bounds=lambda x, y, misrate: shift_bounds(x, y, misrate),
        seeded_bounds=None,
    ),
    _MetricSpec(
        metric=Metric.RATIO,
        validate_and_normalize=_validate_ratio,
        estimate=lambda x, y: Measurement(ratio_estimator(x, y).value, RATIO_UNIT),
        bounds=lambda x, y, misrate: ratio_bounds(x, y, misrate),
        seeded_bounds=None,
    ),
    _MetricSpec(
        metric=Metric.DISPARITY,
        validate_and_normalize=_validate_disparity,
        estimate=lambda x, y: Measurement(disparity_estimator(x, y).value, DISPARITY_UNIT),
        bounds=lambda x, y, misrate: disparity_bounds(x, y, misrate),
        seeded_bounds=lambda x, y, misrate, seed: disparity_bounds(x, y, misrate, seed=seed),
    ),
]


def _get_spec(specs: list[_MetricSpec], metric: Metric) -> _MetricSpec | None:
    """Find a metric specification by metric type."""
    for spec in specs:
        if spec.metric == metric:
            return spec
    return None


def _compute_verdict(bounds: Bounds, threshold_value: float) -> ComparisonVerdict:
    """Computes the verdict by comparing bounds against a threshold value."""
    if bounds.lower > threshold_value:
        return ComparisonVerdict.GREATER
    if bounds.upper < threshold_value:
        return ComparisonVerdict.LESS
    return ComparisonVerdict.INCONCLUSIVE


def compare1(x: Sample, thresholds: list[Threshold], seed: str | None = None) -> list[Projection]:
    """One-sample confirmatory analysis: compares Center/Spread against practical thresholds.

    Args:
        x: The sample to analyze
        thresholds: List of thresholds to compare against
        seed: Optional seed for reproducibility (used for Spread bounds only)

    Returns:
        List of Projections in the same order as the input thresholds

    Raises:
        AssumptionError: If the sample is weighted, thresholds list is empty,
            any threshold has an unsupported metric, or any threshold value is invalid.
    """
    _check_non_weighted("x", x)

    if not thresholds:
        raise AssumptionError("thresholds list cannot be empty")

    for threshold in thresholds:
        if threshold.metric not in (Metric.CENTER, Metric.SPREAD):
            raise AssumptionError(
                f"metric {threshold.metric.value} is not supported by compare1. Use compare2 instead."
            )

    normalized_values: list[float] = []
    for threshold in thresholds:
        spec = _get_spec(_COMPARE1_SPECS, threshold.metric)
        if spec is None:
            raise AssumptionError(f"no spec found for metric {threshold.metric.value}")
        normalized = spec.validate_and_normalize(threshold, x, None)
        normalized_values.append(normalized)

    results: list[Projection | None] = [None] * len(thresholds)

    for spec in _COMPARE1_SPECS:
        entries = [(i, t, n) for i, (t, n) in enumerate(zip(thresholds, normalized_values)) if t.metric == spec.metric]

        if not entries:
            continue

        estimate = spec.estimate(x, None)

        for input_idx, threshold, normalized_value in entries:
            if seed is not None and spec.seeded_bounds is not None:
                bounds = spec.seeded_bounds(x, None, threshold.misrate, seed)
            else:
                bounds = spec.bounds(x, None, threshold.misrate)
            verdict = _compute_verdict(bounds, normalized_value)
            results[input_idx] = Projection(threshold, estimate, bounds, verdict)

    return [r for r in results if r is not None]


def compare2(x: Sample, y: Sample, thresholds: list[Threshold], seed: str | None = None) -> list[Projection]:
    """Two-sample confirmatory analysis: compares Shift/Ratio/Disparity against practical thresholds.

    Args:
        x: The first sample
        y: The second sample
        thresholds: List of thresholds to compare against
        seed: Optional seed for reproducibility (used for Disparity bounds only)

    Returns:
        List of Projections in the same order as the input thresholds

    Raises:
        AssumptionError: If either sample is weighted, samples have incompatible units,
            thresholds list is empty, any threshold has an unsupported metric,
            or any threshold value is invalid.
    """
    _check_non_weighted("x", x)
    _check_non_weighted("y", y)
    _check_compatible_units(x, y)

    if not thresholds:
        raise AssumptionError("thresholds list cannot be empty")

    for threshold in thresholds:
        if threshold.metric not in (Metric.SHIFT, Metric.RATIO, Metric.DISPARITY):
            raise AssumptionError(
                f"metric {threshold.metric.value} is not supported by compare2. Use compare1 instead."
            )

    x_conv, y_conv = _convert_to_finer(x, y)

    normalized_values: list[float] = []
    for threshold in thresholds:
        spec = _get_spec(_COMPARE2_SPECS, threshold.metric)
        if spec is None:
            raise AssumptionError(f"no spec found for metric {threshold.metric.value}")
        normalized = spec.validate_and_normalize(threshold, x_conv, y_conv)
        normalized_values.append(normalized)

    results: list[Projection | None] = [None] * len(thresholds)

    for spec in _COMPARE2_SPECS:
        entries = [(i, t, n) for i, (t, n) in enumerate(zip(thresholds, normalized_values)) if t.metric == spec.metric]

        if not entries:
            continue

        estimate = spec.estimate(x_conv, y_conv)

        for input_idx, threshold, normalized_value in entries:
            if seed is not None and spec.seeded_bounds is not None:
                bounds = spec.seeded_bounds(x_conv, y_conv, threshold.misrate, seed)
            else:
                bounds = spec.bounds(x_conv, y_conv, threshold.misrate)
            verdict = _compute_verdict(bounds, normalized_value)
            results[input_idx] = Projection(threshold, estimate, bounds, verdict)

    return [r for r in results if r is not None]
