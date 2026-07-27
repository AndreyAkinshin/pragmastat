"""MinAchievableMisrate functions for bounds validation."""

import math

from ._binomial import binomial_coefficient
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
    # ldexp, not **: this is a power of two, and scaling by an exponent is exact in binary64. A general
    # power function returns the same value in every implementation anyone ships, but the
    # specification does not require it to, and this value is a domain boundary: it decides
    # which misrates the toolkit accepts at all.
    return math.ldexp(1.0, 1 - n)


def min_achievable_misrate_two_sample(n: int, m: int) -> float:
    """
    Computes the minimum achievable misrate for two-sample Mann-Whitney based bounds.

    The floor is ``2 / C(n+m, n)``, and it is computed through the shared binomial rather
    than through ``math.comb``. ``math.comb`` is exact and would give the closer answer, but
    it would give a *different* answer: over 62 <= n+m <= 400 it lands on a different float
    from the recurrence the other six ports use on 76.2% of sample-size pairs, and on 21495
    of them it lands higher, so this port would reject a misrate the other six accept. It
    also raises ``OverflowError`` once C(n+m, n) leaves binary64 range (around n = m = 515),
    where the recurrence reaches +Inf and the floor becomes 0 as everywhere else.

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
    return 2.0 / binomial_coefficient(n + m, n)
