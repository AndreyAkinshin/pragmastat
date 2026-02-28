"""Assumption validation framework for Pragmastat estimators.

This module defines the assumption system that governs which inputs are valid
for each estimator. Assumptions are domain constraints, not statistical models.

Assumption IDs (canonical priority order):
    1. validity - non-empty input with finite defined real values
    2. domain - input parameters within valid domain
    3. positivity - values must be strictly positive
    4. sparity - sample must be non tie-dominant (Spread > 0)

When multiple assumptions are violated, the violation with highest priority
is reported. For two-sample functions, subject X is checked before Y.
"""

from dataclasses import dataclass
from enum import Enum
from typing import Literal

import numpy as np


class AssumptionId(str, Enum):
    """Assumption identifiers in canonical priority order."""

    VALIDITY = "validity"
    DOMAIN = "domain"
    POSITIVITY = "positivity"
    SPARITY = "sparity"


Subject = Literal["x", "y", "misrate"]


@dataclass(frozen=True)
class Violation:
    """Represents a specific assumption violation."""

    id: AssumptionId
    subject: Subject

    def __str__(self) -> str:
        return f"{self.id.value}({self.subject})"


class AssumptionError(Exception):
    """Error type for assumption violations and other estimator errors.

    When constructed with a Violation, ``violation`` is set accordingly.
    When constructed with a plain message string, ``violation`` is None.
    """

    violation: Violation | None

    def __init__(self, violation_or_msg: Violation | str) -> None:
        if isinstance(violation_or_msg, Violation):
            self.violation = violation_or_msg
            super().__init__(str(violation_or_msg))
        else:
            self.violation = None
            super().__init__(violation_or_msg)

    @classmethod
    def validity(cls, subject: Subject) -> "AssumptionError":
        """Creates an error for the validity assumption."""
        return cls(Violation(AssumptionId.VALIDITY, subject))

    @classmethod
    def positivity(cls, subject: Subject) -> "AssumptionError":
        """Creates an error for the positivity assumption."""
        return cls(Violation(AssumptionId.POSITIVITY, subject))

    @classmethod
    def sparity(cls, subject: Subject) -> "AssumptionError":
        """Creates an error for the sparity assumption."""
        return cls(Violation(AssumptionId.SPARITY, subject))

    @classmethod
    def domain(cls, subject: Subject) -> "AssumptionError":
        """Creates an error for the domain assumption."""
        return cls(Violation(AssumptionId.DOMAIN, subject))


def check_validity(values: np.ndarray, subject: Subject) -> None:
    """Checks that a sample is valid (non-empty with finite values)."""
    if len(values) == 0:
        raise AssumptionError.validity(subject)
    if not np.all(np.isfinite(values)):
        raise AssumptionError.validity(subject)


def check_positivity(values: np.ndarray, subject: Subject) -> None:
    """Checks that all values are strictly positive."""
    if np.any(values <= 0):
        raise AssumptionError.positivity(subject)


def log(values: np.ndarray, subject: Subject) -> np.ndarray:
    """Log-transforms an array. Raises AssumptionError if any value is non-positive."""
    if np.any(values <= 0):
        raise AssumptionError.positivity(subject)
    return np.log(values)
