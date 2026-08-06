"""High-precision standard-normal quantile oracle.

The 395+5 case in `manual/dist-neighbor` needs `Phi^-1((i-0.5)/395)` generated once by a
256-bit oracle and then stored literally in the fixture. This module produces
those values with `decimal.Decimal` at 80 significant digits (~266 bits), using
only the Python standard library, and rounds the final result to the nearest
binary64.

Method
------
`Phi(x) = 1/2 + 1/2 * erf(x / sqrt(2))`, with `erf` evaluated through the
all-positive-terms series

    erf(z) = (2 z / sqrt(pi)) * exp(-z^2) * sum_{k>=0} (2 z^2)^k / (1*3*...*(2k+1))

which has no cancellation for the arguments needed here (|z| < 2.2). The
quantile is then obtained by Newton iteration on `Phi(x) - p`, whose derivative
is the standard normal density. Newton doubles the number of correct digits per
step, so a double-precision start value reaches full working precision in five
steps; the implementation iterates to a fixed residual instead of a fixed count.
"""

from __future__ import annotations

from decimal import Decimal, getcontext, localcontext
from statistics import NormalDist

WORKING_PRECISION = 80

# pi to 100 significant digits (OEIS A000796).
PI_DIGITS = (
    "3.141592653589793238462643383279502884197169399375105820974944592307"
    "8164062862089986280348253421170679"
)


def _pi() -> Decimal:
    return Decimal(PI_DIGITS)


def _erf(z: Decimal) -> Decimal:
    """erf(z) via the exponentially scaled, all-positive series."""
    if z == 0:
        return Decimal(0)
    if z < 0:
        return -_erf(-z)
    with localcontext() as ctx:
        ctx.prec = getcontext().prec + 15
        z2 = z * z
        two_z2 = 2 * z2
        term = Decimal(1)
        total = Decimal(1)
        k = 0
        while True:
            k += 1
            term = term * two_z2 / (2 * k + 1)
            total += term
            if term < total * Decimal(10) ** (-ctx.prec):
                break
        result = 2 * z * (-z2).exp() * total / _pi().sqrt()
    return +result


def _phi_cdf(x: Decimal) -> Decimal:
    """Standard normal CDF.

    Two regimes, because one formula cannot cover both. In the body, `1/2 + 1/2 erf(x/sqrt 2)` is
    accurate and direct. Far into the left tail it is worthless: erf approaches -1, and adding it
    to 1 cancels every significant digit, so `Phi(-21)` came back as a flat zero and the Newton
    iteration above had nothing to descend. There the Laplace continued fraction computes the tail
    mass directly, never forming a difference of nearly equal numbers.
    """
    half = Decimal(1) / 2
    if x < -6:
        return _phi_tail(-x)
    if x > 6:
        return 1 - _phi_tail(x)
    return half + half * _erf(x / Decimal(2).sqrt())


def _phi_tail(x: Decimal) -> Decimal:
    """`Phi(-x)` for `x > 0`, by the Laplace continued fraction.

    Phi(-x) = phi(x) / (x + 1/(x + 2/(x + 3/(x + ...)))), evaluated from the bottom up. The
    fraction converges for every positive x and needs more terms as x shrinks, which is why the
    caller only reaches it past 6, where this depth is ample at the working precision.
    """
    with localcontext() as ctx:
        ctx.prec = getcontext().prec + 15
        depth = 60 + int(400 / float(x))
        frac = Decimal(0)
        for k in range(depth, 0, -1):
            frac = Decimal(k) / (x + frac)
        result = _phi_pdf(x) / (x + frac)
    return +result


def _phi_pdf(x: Decimal) -> Decimal:
    """Standard normal density."""
    with localcontext() as ctx:
        ctx.prec = getcontext().prec + 15
        result = (-(x * x) / 2).exp() / (2 * _pi()).sqrt()
    return +result


def _newton_start(p: Decimal) -> Decimal:
    """A starting point Newton converges from, over the whole open interval.

    The platform's `inv_cdf` is fine in the body of the distribution and useless past roughly
    1e-150: it is a double, so it stops resolving p at all near the subnormals, and Newton then
    steps by residual/density where the density is 1e-300 and lands nowhere. Since the tail
    quantile is exactly what a distribution's Quantile has to answer at extreme cumulances, the
    start is taken from the asymptotic expansion there instead, which needs no float at all.

    For small p, Phi(x) ~ phi(x)/|x|, so -log p ~ x^2/2 + log|x| + log(sqrt(2 pi)). Solving that
    twice by substitution gives a start within a fraction of a unit of the root, which Newton then
    doubles to full precision in a few steps.
    """
    half = Decimal(1) / 2
    lower = min(p, 1 - p)
    if lower > Decimal("1e-15"):
        return Decimal(repr(NormalDist().inv_cdf(float(p))))
    u = -lower.ln()
    x = (2 * u).sqrt()
    for _ in range(3):
        # x^2/2 = u - log(x) - log(sqrt(2 pi)), iterated.
        inner = u - x.ln() - ((2 * _pi()).sqrt()).ln()
        if inner <= 0:
            break
        x = (2 * inner).sqrt()
    return -x if p < half else x


def normal_quantile(p: Decimal, precision: int = WORKING_PRECISION) -> Decimal:
    """`Phi^-1(p)` for `0 < p < 1`, accurate to the requested precision."""
    if not 0 < p < 1:
        raise ValueError("p must lie strictly inside (0, 1)")
    with localcontext() as ctx:
        ctx.prec = precision + 20
        half = Decimal(1) / 2
        if p == half:
            return Decimal(0)
        x = _newton_start(p)
        target = Decimal(10) ** (-(precision + 5))
        for _ in range(64):
            residual = _phi_cdf(x) - p
            step = residual / _phi_pdf(x)
            x = x - step
            if abs(step) < abs(x) * target or abs(step) < target:
                break
        else:  # pragma: no cover - Newton on a convex CDF converges long before
            raise ArithmeticError("normal_quantile failed to converge")
    with localcontext() as ctx:
        ctx.prec = precision
        return +x


def normal_quantile_grid(n: int) -> list[float]:
    """`Phi^-1((i - 0.5) / n)` for `i = 1..n`, rounded to nearest binary64.

    The grid is exactly antisymmetric in exact arithmetic, so only the lower half
    is computed and the upper half is its negation. That keeps the stored
    fixture exactly symmetric instead of symmetric up to two independent
    roundings.
    """
    values: list[float] = [0.0] * n
    for i in range(1, (n + 1) // 2 + 1):
        p = (Decimal(i) - Decimal(1) / 2) / Decimal(n)
        if p == Decimal(1) / 2:
            values[i - 1] = 0.0
            continue
        q = normal_quantile(p)
        values[i - 1] = float(q)
        values[n - i] = -float(q)
    return values
