"""Assumption validation framework for Pragmastat estimators.

This module defines the assumption system that governs which inputs are valid
for each estimator. Assumptions are domain constraints, not statistical models.

Assumption IDs (canonical priority order):
    1. validity - non-empty input with finite defined real values
    2. domain - input parameters within valid domain
    3. positivity - values must be strictly positive
    4. sparity - sample must be non tie-dominant (Spread > 0)
    4. distinctness - sample values must be pairwise distinct
    5. cumulance - every queried cumulance must lie in the closed unit interval

sparity and distinctness share a priority level: no function checks both.

When multiple assumptions are violated, the violation with highest priority
is reported. For two-sample functions, subject X is checked before Y.
"""

import math
from dataclasses import dataclass
from enum import StrEnum
from typing import TYPE_CHECKING

import numpy as np

from .log_function import log_function

if TYPE_CHECKING:
    from collections.abc import Sequence


class AssumptionId(StrEnum):
    """Assumption identifiers in canonical priority order."""

    VALIDITY = "validity"
    DOMAIN = "domain"
    POSITIVITY = "positivity"
    SPARITY = "sparity"
    DISTINCTNESS = "distinctness"
    CUMULANCE = "cumulance"


class Subject(StrEnum):
    """Identifies which input caused an assumption violation."""

    X = "x"
    Y = "y"
    MISRATE = "misrate"


@dataclass(frozen=True)
class Violation:
    """Represents a specific assumption violation."""

    id: AssumptionId
    subject: Subject

    def __str__(self) -> str:
        return f"{self.id}({self.subject})"


class AssumptionError(Exception):
    """Error type for assumption violations and other estimator errors.

    When constructed with a Violation, ``violation`` is set accordingly.
    When constructed with a plain message string, ``violation`` is None.
    """

    violation: Violation | None

    def __init__(self, violation_or_msg: Violation | str) -> None:
        """Builds an error from a Violation, or from a plain message with no violation."""
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

    @classmethod
    def distinctness(cls, subject: Subject) -> "AssumptionError":
        """Creates an error for the distinctness assumption."""
        return cls(Violation(AssumptionId.DISTINCTNESS, subject))

    @classmethod
    def cumulance(cls, subject: Subject) -> "AssumptionError":
        """Creates an error for the cumulance assumption."""
        return cls(Violation(AssumptionId.CUMULANCE, subject))


def check_validity(values: np.ndarray, subject: Subject) -> None:
    """Checks that a sample is valid (non-empty with finite values)."""
    if len(values) == 0:
        raise AssumptionError.validity(subject)
    if not np.all(np.isfinite(values)):
        raise AssumptionError.validity(subject)


def check_query_validity(values: "Sequence[float]") -> None:
    """Checks the validity assumption for a Distribution Estimation query vector.

    A query vector is a list of questions asked of a model, not a sample, so an
    empty vector satisfies the assumption vacuously. Every element that is
    present must be finite.
    """
    for value in values:
        if not math.isfinite(value):
            raise AssumptionError.validity(Subject.Y)


def check_cumulance(values: "Sequence[float]") -> None:
    """Checks that every queried cumulance lies in the closed unit interval."""
    for value in values:
        if not 0.0 <= value <= 1.0:
            raise AssumptionError.cumulance(Subject.Y)


def check_positivity(values: np.ndarray, subject: Subject) -> None:
    """Checks that all values are strictly positive."""
    if np.any(values <= 0):
        raise AssumptionError.positivity(subject)


def log(values: np.ndarray, subject: Subject) -> np.ndarray:
    """Log-transforms an array. Raises AssumptionError if any value is non-positive."""
    if np.any(values <= 0):
        raise AssumptionError.positivity(subject)
    return np.array([log_function(float(v)) for v in values], dtype=np.float64)
