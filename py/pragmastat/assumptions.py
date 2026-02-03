"""Assumption validation framework for Pragmastat estimators.

This module defines the assumption system that governs which inputs are valid
for each estimator. Assumptions are domain constraints, not statistical models.

Assumption IDs (canonical priority order):
    1. validity - non-empty input with finite defined real values
    2. positivity - values must be strictly positive
    3. sparity - sample must be non tie-dominant (Spread > 0)

When multiple assumptions are violated, the violation with highest priority
is reported. For two-sample functions, subject X is checked before Y.
"""

from dataclasses import dataclass
from enum import Enum
from typing import Literal

import numpy as np

from .fast_spread import _fast_spread


class AssumptionId(str, Enum):
    """Assumption identifiers in canonical priority order."""

    VALIDITY = "validity"
    POSITIVITY = "positivity"
    SPARITY = "sparity"


Subject = Literal["x", "y"]


@dataclass(frozen=True)
class Violation:
    """Represents a specific assumption violation."""

    id: AssumptionId
    subject: Subject

    def __str__(self) -> str:
        return f"{self.id.value}({self.subject})"


class AssumptionError(Exception):
    """Error type for assumption violations."""

    def __init__(self, violation: Violation) -> None:
        self.violation = violation
        super().__init__(str(violation))

    @classmethod
    def validity(cls, _function: str, subject: Subject) -> "AssumptionError":
        """Creates an error for the validity assumption."""
        return cls(Violation(AssumptionId.VALIDITY, subject))

    @classmethod
    def positivity(cls, _function: str, subject: Subject) -> "AssumptionError":
        """Creates an error for the positivity assumption."""
        return cls(Violation(AssumptionId.POSITIVITY, subject))

    @classmethod
    def sparity(cls, _function: str, subject: Subject) -> "AssumptionError":
        """Creates an error for the sparity assumption."""
        return cls(Violation(AssumptionId.SPARITY, subject))


def check_validity(values: np.ndarray, subject: Subject, function: str) -> None:
    """Checks that a sample is valid (non-empty with finite values)."""
    if len(values) == 0:
        raise AssumptionError.validity(function, subject)
    if not np.all(np.isfinite(values)):
        raise AssumptionError.validity(function, subject)


def check_positivity(values: np.ndarray, subject: Subject, function: str) -> None:
    """Checks that all values are strictly positive."""
    if np.any(values <= 0):
        raise AssumptionError.positivity(function, subject)


def check_sparity(values: np.ndarray, subject: Subject, function: str) -> None:
    """Checks that a sample is non tie-dominant (Spread > 0)."""
    if len(values) < 2:
        raise AssumptionError.sparity(function, subject)
    spread_val = _fast_spread(values.tolist())
    if spread_val <= 0:
        raise AssumptionError.sparity(function, subject)


def log(values: np.ndarray, subject: Subject, function: str) -> np.ndarray:
    """Log-transforms an array. Raises AssumptionError if any value is non-positive."""
    if np.any(values <= 0):
        raise AssumptionError.positivity(function, subject)
    return np.log(values)
