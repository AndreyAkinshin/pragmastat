"""The natural logarithm this toolkit evaluates, in place of the platform's.

IEEE 754 fixes the result of each arithmetic operation and of the square root, and fixes nothing
about the logarithm. The reason this one has to be ours is stronger than for most transcendentals:
ratio is exp(median(log x - log y)), so a last-bit difference in log moves the median's input and
changes which pairwise element is selected. That is a different answer, not a small error in the
same one.

The constants and the polynomial come from tests/oracles/fit_log.py, which documents the reduction
and what is fitted. Which of the two range endpoints a port names depends on how it splits x: a
library frexp returns a significand in [0.5, 1) and the reduction doubles the low end, while
reading the exponent field gives one in [1, 2) and it halves the high end. Both arrive at the same
[sqrt(2)/2, sqrt(2)) and the seven agree bit for bit; only the endpoint each has to name differs.
"""

import math

# Constants of the range reduction, emitted by tests/oracles/fit_log.py. ln 2 is split so that
# k*LN2_HI is exact: it carries 32 significant bits and |k| needs at most 11, which leaves the
# product inside the 53 available.
LN2_HI = 0.6931471803691238
LN2_LO = 1.9082149292705877e-10
SQRT2_OVER_2 = 0.7071067811865476

# Chebyshev fit of (log((1+s)/(1-s)) - 2*s) / s^3 over the reduced range. The leading terms are
# the recognizable 2/3, 2/5, 2/7 of the odd series doubled, which is where the assembly's missing
# factor of two lives; the tail departs from the series because the fit minimizes the worst error
# rather than matching Taylor.
POLY_P = (
    0.666666666666667,
    0.39999999999899505,
    0.28571428625975487,
    0.2222221113479508,
    0.18182889125261723,
    0.15331721600556042,
    0.14616449685043406,
)


def log_function(x: float) -> float:
    """Returns the natural logarithm of x.

    The special cases follow IEEE 754 and match what every platform returns for them, since they
    are the cases a platform has no freedom about.
    """
    if math.isnan(x):
        return x
    if x < 0:
        return math.nan
    if x == 0:
        return -math.inf
    if math.isinf(x):
        return math.inf
    if x == 1:
        return 0.0

    m, k = math.frexp(x)
    # frexp returns m in [0.5, 1); recenter on [sqrt(2)/2, sqrt(2)) so |f| stays small at both
    # ends. Doubling is exact.
    if m < SQRT2_OVER_2:
        m *= 2
        k -= 1

    f = m - 1
    s = f / (2 + f)
    z = s * s

    acc = POLY_P[-1]
    for coefficient in reversed(POLY_P[:-1]):
        acc = acc * z + coefficient
    r = z * acc
    hfsq = 0.5 * f * f

    if k == 0:
        return f - (hfsq - s * (hfsq + r))
    return k * LN2_HI - ((hfsq - (s * (hfsq + r) + k * LN2_LO)) - f)
