"""SignMargin function for one-sample bounds based on Binomial(n, 0.5)."""

import math

from .assumptions import AssumptionError
from .min_misrate import min_achievable_misrate_one_sample


def sign_margin_randomized(n: int, misrate: float, rng) -> int:  # noqa: ANN001
    """Compute randomized sign margin for one-sample bounds.

    Args:
        n: Sample size (must be positive)
        misrate: Misclassification rate
        rng: Rng instance for randomization

    Returns:
        Margin value (even integer)

    Raises:
        AssumptionError: If n <= 0 or misrate is out of valid range.
    """
    if n <= 0:
        raise AssumptionError.domain("x")
    if math.isnan(misrate) or misrate < 0 or misrate > 1:
        raise AssumptionError.domain("misrate")

    min_misrate = min_achievable_misrate_one_sample(n)
    if misrate < min_misrate:
        raise AssumptionError.domain("misrate")

    target = misrate / 2.0
    if target <= 0.0:
        return 0
    if target >= 1.0:
        return n * 2

    r_low, log_cdf_low, log_pmf_high = _binom_cdf_split(n, target)

    log_target = math.log(target)
    log_num = (
        _log_sub_exp(log_target, log_cdf_low)
        if log_target > log_cdf_low
        else float("-inf")
    )

    if math.isfinite(log_pmf_high) and math.isfinite(log_num):
        p = math.exp(log_num - log_pmf_high)
    else:
        p = 0.0
    p = max(0.0, min(1.0, p))

    u = rng.uniform()
    r = r_low + 1 if u < p else r_low
    return r * 2


def _binom_cdf_split(n: int, target: float) -> tuple[int, float, float]:
    """Find the largest r where Binom CDF <= target, returning split info."""
    log_target = math.log(target)
    log_pmf = -n * math.log(2)
    log_cdf = log_pmf
    r_low = 0

    if log_cdf > log_target:
        return (0, log_cdf, log_pmf)

    for k in range(1, n + 1):
        log_pmf_next = log_pmf + math.log(n - k + 1) - math.log(k)
        log_cdf_next = _log_add_exp(log_cdf, log_pmf_next)
        if log_cdf_next > log_target:
            return (r_low, log_cdf, log_pmf_next)
        r_low = k
        log_pmf = log_pmf_next
        log_cdf = log_cdf_next

    return (r_low, log_cdf, float("-inf"))


def _log_add_exp(a: float, b: float) -> float:
    """Compute log(exp(a) + exp(b)) with numerical stability."""
    if a == float("-inf"):
        return b
    if b == float("-inf"):
        return a
    m = max(a, b)
    return m + math.log(math.exp(a - m) + math.exp(b - m))


def _log_sub_exp(a: float, b: float) -> float:
    """Compute log(exp(a) - exp(b)) with numerical stability. Requires a >= b."""
    if b == float("-inf"):
        return a
    diff = math.exp(b - a)
    if diff >= 1.0:
        return float("-inf")
    return a + math.log(1.0 - diff)
