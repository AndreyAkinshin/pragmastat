"""MinAchievableMisrate functions for bounds validation."""

import math

from .assumptions import AssumptionError


def min_achievable_misrate_one_sample(n: int) -> float:
    """
    Computes the minimum achievable misrate for one-sample bounds.

    For a sample of size n, the minimum achievable misrate is 2^(1-n),
    which corresponds to the probability of the most extreme configuration
    in the Wilcoxon signed-rank distribution.

    Args:
        n: Sample size (must be positive)

    Returns:
        Minimum achievable misrate

    Raises:
        AssumptionError: If n is not positive
    """
    if n <= 0:
        raise AssumptionError.domain("x")
    return 2.0 ** (1 - n)


def min_achievable_misrate_two_sample(n: int, m: int) -> float:
    """
    Computes the minimum achievable misrate for two-sample Mann-Whitney based bounds.

    Args:
        n: Size of first sample (must be positive)
        m: Size of second sample (must be positive)

    Returns:
        Minimum achievable misrate

    Raises:
        AssumptionError: If n or m is not positive
    """
    if n <= 0:
        raise AssumptionError.domain("x")
    if m <= 0:
        raise AssumptionError.domain("y")
    return 2.0 / math.comb(n + m, n)
