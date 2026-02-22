"""PairwiseMargin implementation.

Determines how many extreme pairwise differences to exclude when constructing
bounds based on the distribution of dominance statistics. Uses exact calculation
for small samples (n+m <= 400) and Edgeworth approximation for larger samples.
"""

import math
from typing import List

from .assumptions import AssumptionError
from .gauss_cdf import gauss_cdf as _gauss_cdf
from .min_misrate import min_achievable_misrate_two_sample

MAX_EXACT_SIZE = 400
MAX_ACCEPTABLE_BINOM_N = 65


def pairwise_margin(n: int, m: int, misrate: float) -> int:
    """
    PairwiseMargin determines how many extreme pairwise differences to exclude
    when constructing bounds based on the distribution of dominance statistics.

    Uses exact calculation for small samples (n+m <= 400) and Edgeworth
    approximation for larger samples.

    Args:
        n: Sample size of first sample (must be positive)
        m: Sample size of second sample (must be positive)
        misrate: Misclassification rate (must be in [0, 1])

    Returns:
        Integer representing the total margin split between lower and upper tails

    Raises:
        AssumptionError: If n <= 0, m <= 0, or misrate is outside [0, 1]
    """
    if n <= 0:
        raise AssumptionError.domain("x")
    if m <= 0:
        raise AssumptionError.domain("y")
    if misrate < 0 or misrate > 1 or math.isnan(misrate):
        raise AssumptionError.domain("misrate")

    min_misrate = min_achievable_misrate_two_sample(n, m)
    if misrate < min_misrate:
        raise AssumptionError.domain("misrate")

    if n + m <= MAX_EXACT_SIZE:
        return _pairwise_margin_exact(n, m, misrate)
    return _pairwise_margin_approx(n, m, misrate)


def _pairwise_margin_exact(n: int, m: int, misrate: float) -> int:
    """Uses the exact distribution based on Loeffler's recurrence."""
    return _pairwise_margin_exact_raw(n, m, misrate / 2.0) * 2


def _pairwise_margin_approx(n: int, m: int, misrate: float) -> int:
    """Uses Edgeworth approximation for large samples."""
    return _pairwise_margin_approx_raw(n, m, misrate / 2.0) * 2


def _pairwise_margin_exact_raw(n: int, m: int, p: float) -> int:
    """
    Inversed implementation of Andreas Löffler's (1982)
    "Über eine Partition der nat. Zahlen und ihre Anwendung beim U-Test"
    """
    if n + m < MAX_ACCEPTABLE_BINOM_N:
        total = _binomial_coefficient(n + m, m)
    else:
        total = _binomial_coefficient_float(n + m, m)

    pmf: List[float] = [1.0]  # pmf[0] = 1
    sigma: List[float] = [0.0]  # sigma[0] is unused

    u = 0
    cdf = 1.0 / total

    if cdf >= p:
        return 0

    while True:
        u += 1

        # Ensure sigma has entry for u
        if len(sigma) <= u:
            value = 0
            for d in range(1, n + 1):
                if u % d == 0 and u >= d:
                    value += d
            for d in range(m + 1, m + n + 1):
                if u % d == 0 and u >= d:
                    value -= d
            sigma.append(float(value))

        # Compute pmf[u] using Loeffler recurrence
        sum_val = 0.0
        for i in range(u):
            sum_val += pmf[i] * sigma[u - i]
        sum_val /= u
        pmf.append(sum_val)

        cdf += sum_val / total
        if cdf >= p:
            return u
        if sum_val == 0.0:
            break

    return len(pmf) - 1


def _pairwise_margin_approx_raw(n: int, m: int, misrate: float) -> int:
    """Inverse Edgeworth Approximation."""
    a = 0
    b = n * m
    while a < b - 1:
        c = (a + b) // 2
        p = _edgeworth_cdf(n, m, c)
        if p < misrate:
            a = c
        else:
            b = c

    return b if _edgeworth_cdf(n, m, b) < misrate else a


def _edgeworth_cdf(n: int, m: int, u: int) -> float:
    """Computes the CDF using Edgeworth expansion."""
    nf = float(n)
    mf = float(m)
    uf = float(u)

    mu = (nf * mf) / 2.0
    su = math.sqrt((nf * mf * (nf + mf + 1.0)) / 12.0)
    # -0.5 continuity correction: computing P(U ≥ u) for a right-tail discrete CDF
    z = (uf - mu - 0.5) / su

    # Standard normal PDF and CDF
    phi = math.exp((-z * z) / 2.0) / math.sqrt(2.0 * math.pi)
    big_phi = _gauss_cdf(z)

    # Pre-compute powers of n and m for efficiency
    n2 = nf * nf
    n3 = n2 * nf
    n4 = n2 * n2
    m2 = mf * mf
    m3 = m2 * mf
    m4 = m2 * m2

    # Compute moments
    mu2 = (nf * mf * (nf + mf + 1.0)) / 12.0
    mu4 = (
        nf * mf * (nf + mf + 1.0) * (5.0 * mf * nf * (mf + nf) - 2.0 * (m2 + n2) + 3.0 * mf * nf - 2.0 * (nf + mf))
    ) / 240.0

    mu6 = (
        nf
        * mf
        * (nf + mf + 1.0)
        * (
            35.0 * m2 * n2 * (m2 + n2)
            + 70.0 * m3 * n3
            - 42.0 * mf * nf * (m3 + n3)
            - 14.0 * m2 * n2 * (nf + mf)
            + 16.0 * (n4 + m4)
            - 52.0 * nf * mf * (n2 + m2)
            - 43.0 * n2 * m2
            + 32.0 * (m3 + n3)
            + 14.0 * mf * nf * (nf + mf)
            + 8.0 * (n2 + m2)
            + 16.0 * nf * mf
            - 8.0 * (nf + mf)
        )
    ) / 4032.0

    # Pre-compute powers of mu2 and related terms
    mu2_2 = mu2 * mu2
    mu2_3 = mu2_2 * mu2
    mu4_mu2_2 = mu4 / mu2_2

    # Factorial constants: 4! = 24, 6! = 720, 8! = 40320
    e3 = (mu4_mu2_2 - 3.0) / 24.0
    e5 = (mu6 / mu2_3 - 15.0 * mu4_mu2_2 + 30.0) / 720.0
    e7 = 35.0 * (mu4_mu2_2 - 3.0) * (mu4_mu2_2 - 3.0) / 40320.0

    # Pre-compute powers of z for Hermite polynomials
    z2 = z * z
    z3 = z2 * z
    z5 = z3 * z2
    z7 = z5 * z2

    # Hermite polynomial derivatives: f_n = -phi * H_n(z)
    f3 = -phi * (z3 - 3.0 * z)
    f5 = -phi * (z5 - 10.0 * z3 + 15.0 * z)
    f7 = -phi * (z7 - 21.0 * z5 + 105.0 * z3 - 105.0 * z)

    # Edgeworth expansion
    edgeworth = big_phi + e3 * f3 + e5 * f5 + e7 * f7

    # Clamp to [0, 1]
    return max(0.0, min(1.0, edgeworth))


def _binomial_coefficient(n: int, k: int) -> float:
    """Computes binomial coefficient C(n, k) using integer arithmetic."""
    if k > n:
        return 0.0
    if k == 0 or k == n:
        return 1.0

    k = min(k, n - k)  # Take advantage of symmetry
    result = 1.0

    for i in range(k):
        result = result * (n - i) / (i + 1)

    return result


def _binomial_coefficient_float(n: int, k: int) -> float:
    """Computes binomial coefficient using floating-point logarithms for large values."""
    if k > n:
        return 0.0
    if k == 0 or k == n:
        return 1.0

    k = min(k, n - k)  # Take advantage of symmetry

    # Use log-factorial function: C(n, k) = exp(log(n!) - log(k!) - log((n-k)!))
    log_result = _log_factorial(n) - _log_factorial(k) - _log_factorial(n - k)
    return math.exp(log_result)


def _log_factorial(n: int) -> float:
    """Computes the natural logarithm of n!."""
    if n == 0 or n == 1:
        return 0.0

    x = float(n + 1)  # n! = Gamma(n+1)

    if x < 1e-5:
        return 0.0

    # DONT TOUCH: Stirling's approximation is inaccurate for small x.
    # Use Gamma recurrence: Gamma(x) = Gamma(x+k) / (x*(x+1)*...*(x+k-1))
    # These branches appear unreachable in current usage (n+m >= 65), but
    # are retained for correctness if the function is used in other contexts.
    if x < 1.0:
        return _stirling_approx_log(x + 3.0) - math.log(x * (x + 1.0) * (x + 2.0))
    if x < 2.0:
        return _stirling_approx_log(x + 2.0) - math.log(x * (x + 1.0))
    if x < 3.0:
        return _stirling_approx_log(x + 1.0) - math.log(x)
    return _stirling_approx_log(x)


def _stirling_approx_log(x: float) -> float:
    """Stirling's approximation with Bernoulli correction."""
    result = x * math.log(x) - x + math.log(2.0 * math.pi / x) / 2.0

    # Bernoulli correction series
    B2 = 1.0 / 6.0
    B4 = -1.0 / 30.0
    B6 = 1.0 / 42.0
    B8 = -1.0 / 30.0
    B10 = 5.0 / 66.0

    x2 = x * x
    x3 = x2 * x
    x5 = x3 * x2
    x7 = x5 * x2
    x9 = x7 * x2

    result += B2 / (2.0 * x) + B4 / (12.0 * x3) + B6 / (30.0 * x5) + B8 / (56.0 * x7) + B10 / (90.0 * x9)

    return result
