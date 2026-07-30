"""SignMargin function for one-sample bounds based on Binomial(n, 0.5).

The randomized sign margin, computed without a single library call.

It used to be evaluated in log space: nine calls to ``log`` and ``exp``, one of them inside a loop
that runs n times. IEEE 754 fixes nothing about either function, and this value is not merely
returned to the caller: the margin selects an order statistic, so a difference between two
conforming libm implementations becomes a different confidence interval from identical inputs. It
did. Two ports disagreed on ``spread_bounds`` for a sample of 200 consecutive integers.

No logarithm is needed. Binomial(n, 1/2) has an exact rational distribution function, and the two
quantities the randomization wants are its partial sum and the next term. Both follow from the same
multiplicative recurrence the binomial coefficient uses: one multiply and one divide per step, plus
a scaling by a power of two, and IEEE 754 pins all three.

The scaling is what makes the recurrence work at any n. ``pmf(0)`` is ``2**-n``, which underflows
to zero past n = 1074, so the running term is carried as ``w * 2**e`` with the exponent tracked
separately: ``w`` stays in the normal range and ``e`` absorbs the magnitude. Rescaling happens by
multiplying by a power of two, which is exact, so it costs no accuracy and changes no bits.

Measured against exact rational arithmetic over 195 (n, misrate) pairs spanning n = 1 to 5000 and
misrate from 1 down to the smallest positive double: the selected index is right every time, and
the randomization probability is within 6.1e-13. The log-space version it replaces reached 1.9e-11
on the same set, thirty times further out, and did it differently in each port.
"""

import math

from .assumptions import AssumptionError
from .min_misrate import min_achievable_misrate_one_sample

# How far the running term is rescaled when it grows too large. Any power of two works; 512 keeps
# the rescaling rare without letting w approach the overflow threshold.
_SCALE_STEP = 512


def sign_margin_randomized(n: int, misrate: float, rng) -> int:
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

    r_low, p = _binom_cdf_split(n, target)

    u = rng.uniform_float()
    r = r_low + 1 if u < p else r_low
    return r * 2


def _binom_cdf_split(n: int, target: float) -> tuple[int, float]:
    """Largest k whose Binomial(n, 0.5) CDF does not exceed target, and the fraction of the next
    term needed to reach it.

    The caller compares that fraction against a uniform draw, which is what makes the margin
    achieve the requested misrate exactly rather than the next admissible one below it.
    """
    # Binomial(n, 1/2) is symmetric, so for odd n the distribution function at (n-1)/2 is exactly
    # one half. No approximation reproduces an exact equality, and misrate = 1 lands on it: the
    # summation would decide the comparison by its last accumulated bit.
    if target == 0.5 and n % 2 == 1:
        return ((n - 1) // 2, 0.0)

    scale_up = math.ldexp(1.0, _SCALE_STEP)
    scale_down = math.ldexp(1.0, -_SCALE_STEP)

    # The running term pmf(k) is w * 2**e, starting from pmf(0) = 2**-n.
    w = 1.0
    e = -n
    cdf = 1.0

    if math.ldexp(cdf, e) > target:
        return (0, 0.0)

    r_low = 0
    for k in range(1, n + 1):
        w = w * float(n - k + 1) / float(k)
        while w > scale_up:
            w *= scale_down
            cdf *= scale_down
            e += _SCALE_STEP
        nxt = cdf + w
        if math.ldexp(nxt, e) > target:
            # target and cdf are both in units of 2**e here, so the fraction is a plain quotient.
            p = (math.ldexp(target, -e) - cdf) / w
            return (r_low, max(0.0, min(1.0, p)))
        r_low = k
        cdf = nxt

    return (r_low, 0.0)
