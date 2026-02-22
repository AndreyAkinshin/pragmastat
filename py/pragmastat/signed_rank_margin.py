"""SignedRankMargin function for one-sample bounds.

One-sample analog of PairwiseMargin using Wilcoxon signed-rank distribution.
"""

import math

from .assumptions import AssumptionError
from .gauss_cdf import gauss_cdf
from .min_misrate import min_achievable_misrate_one_sample

# Maximum n for exact computation. Limited to 63 because 2^n must fit in a 64-bit integer.
SIGNED_RANK_MAX_EXACT_SIZE = 63


def signed_rank_margin(n: int, misrate: float) -> int:
    """
    SignedRankMargin computes the margin for one-sample signed-rank bounds.
    Uses Wilcoxon signed-rank distribution to determine the margin that achieves
    the specified misrate.

    Args:
        n: Sample size (must be positive)
        misrate: Misclassification rate (must be in [0, 1])

    Returns:
        Integer margin

    Raises:
        ValueError: If inputs are invalid or misrate is below minimum achievable
    """
    if n <= 0:
        raise AssumptionError.domain("x")
    if math.isnan(misrate) or misrate < 0 or misrate > 1:
        raise AssumptionError.domain("misrate")

    min_misrate = min_achievable_misrate_one_sample(n)
    if misrate < min_misrate:
        raise AssumptionError.domain("misrate")

    if n <= SIGNED_RANK_MAX_EXACT_SIZE:
        return _signed_rank_margin_exact(n, misrate)
    return _signed_rank_margin_approx(n, misrate)


def _signed_rank_margin_exact(n: int, misrate: float) -> int:
    """Computes one-sided margin using exact Wilcoxon signed-rank distribution."""
    return _signed_rank_margin_exact_raw(n, misrate / 2.0) * 2


def _signed_rank_margin_exact_raw(n: int, p: float) -> int:
    """Uses dynamic programming to compute the CDF."""
    total = 1 << n
    max_w = n * (n + 1) // 2

    count = [0] * (max_w + 1)
    count[0] = 1

    for i in range(1, n + 1):
        max_wi = min(i * (i + 1) // 2, max_w)
        for w in range(max_wi, i - 1, -1):
            count[w] = count[w] + count[w - i]

    cumulative = 0
    for w in range(max_w + 1):
        cumulative = cumulative + count[w]
        cdf = float(cumulative) / float(total)
        if cdf >= p:
            return w

    return max_w


def _signed_rank_margin_approx(n: int, misrate: float) -> int:
    """Computes one-sided margin using Edgeworth approximation for large n."""
    return _signed_rank_margin_approx_raw(n, misrate / 2.0) * 2


def _signed_rank_margin_approx_raw(n: int, misrate: float) -> int:
    """Binary search using Edgeworth CDF."""
    max_w = n * (n + 1) // 2
    a = 0
    b = max_w

    while a < b - 1:
        c = (a + b) // 2
        cdf = _signed_rank_edgeworth_cdf(n, c)
        if cdf < misrate:
            a = c
        else:
            b = c

    return b if _signed_rank_edgeworth_cdf(n, b) < misrate else a


def _signed_rank_edgeworth_cdf(n: int, w: int) -> float:
    """Edgeworth expansion for Wilcoxon signed-rank distribution CDF."""
    n_f64 = float(n)
    mu = n_f64 * (n_f64 + 1.0) / 4.0
    sigma2 = n_f64 * (n_f64 + 1.0) * (2.0 * n_f64 + 1.0) / 24.0
    sigma = math.sqrt(sigma2)

    # +0.5 continuity correction: computing P(W ≤ w) for a left-tail discrete CDF
    z = (float(w) - mu + 0.5) / sigma
    phi = math.exp(-z * z / 2.0) / math.sqrt(2.0 * math.pi)
    big_phi = gauss_cdf(z)

    kappa4 = -n_f64 * (n_f64 + 1.0) * (2.0 * n_f64 + 1.0) * (3.0 * n_f64 * n_f64 + 3.0 * n_f64 - 1.0) / 240.0

    e3 = kappa4 / (24.0 * sigma2 * sigma2)

    z2 = z * z
    z3 = z2 * z
    f3 = -phi * (z3 - 3.0 * z)

    edgeworth = big_phi + e3 * f3
    return max(0.0, min(1.0, edgeworth))
