"""Fits the polynomial behind the deterministic exponential.

IEEE 754 fixes the result of every arithmetic operation and of the square root, and fixes
nothing about the exponential. Two conforming libraries may return neighbouring values for
the same argument, and they do: Go evaluates its own exp in software, V8 carries a third
implementation, and the ports built on a platform libm follow whatever it does. Every
difference is a last-bit one, and a last-bit difference is enough when the value is compared
against a misrate to select an integer.

The exponential therefore has to be ours. The reduction is the classical one:

    k = round(y / ln 2),  r = (y - k*ln2_hi) - k*ln2_lo,  exp(y) = 2^k * exp(r)

with ln 2 split so that k*ln2_hi is exact. Everything in it is an IEEE operation with a fixed
result, so the only freedom left is the polynomial for exp(r) on |r| <= ln2/2, which this
script produces.

The fitted quantity is Q(r) = (exp(r) - 1 - r) / r^2 rather than exp(r) itself. The two leading
terms then carry no fitting error at all and the polynomial only has to supply a correction of
at most 0.07, which is what keeps the assembled result inside one ulp of correctly rounded.

The ports assemble that as 1 - ((lo - r*r*Q) - hi) rather than the obvious 1 + r + r*r*Q. Both
compute the same expression; the first keeps the two halves of the reduction apart until the end,
and the second throws away the low bits of r when it adds r to 1. The difference is a factor of
two in worst relative error and 17 points of correctly-rounded share, for no extra operation.
"""

import sys
from decimal import Decimal as D, getcontext

sys.path.insert(0, sys.path[0] or ".")
getcontext().prec = 80
from chebyshev import cheb_fit, horner  # noqa: E402

LN2 = D("0.693147180559945309417232121458176568075500134360255254120680009493393621969694")
HALF_LN2 = LN2 / 2


def dexp(x: D) -> D:
    """exp by Taylor; |x| <= ln2/2 here, so it converges immediately."""
    s = D(1)
    term = D(1)
    n = 0
    while True:
        n += 1
        term = term * x / n
        s += term
        if abs(term) < D(10) ** -75:
            break
    return s


def q(r: D) -> D:
    """(exp(r) - 1 - r) / r^2, continuous at 0 with value 1/2."""
    if r == 0:
        return D("0.5")
    return (dexp(r) - 1 - r) / (r * r)


DEGREE = 12
_normalised = cheb_fit(q, -HALF_LN2, HALF_LN2, DEGREE)
# cheb_fit works in u = r / (ln2/2); rescaling to r itself removes a multiply from the
# evaluation and turns the coefficients back into the recognisable 1/2, 1/6, 1/24 series,
# which is worth having in seven files a reader may check by eye.
poly = [c / HALF_LN2**d for d, c in enumerate(_normalised)]


def fmt(p):
    return [f"{float(c):.17e}" for c in p]


# ln2 split so that k * LN2_HI is exact for every k the reduction can produce. LN2_HI keeps
# the leading 33 bits and k needs at most 11, so the product fits the 53 available.
LN2_HI = float.fromhex("0x1.62e42fee00000p-1")
LN2_LO = float(LN2 - D(LN2_HI))

if __name__ == "__main__":
    print(f"INV_LN2 = {float(1 / LN2):.17e}")
    print(f"LN2_HI  = {LN2_HI:.17e}   # {LN2_HI.hex()}")
    print(f"LN2_LO  = {LN2_LO:.17e}   # {LN2_LO.hex()}")
    print("POLY_Q =", fmt(poly))

    # Worst error of the fit alone, in exact arithmetic: what the polynomial costs before
    # any binary64 rounding enters.
    worst = D(0)
    steps = 4000
    for i in range(steps + 1):
        r = -HALF_LN2 + 2 * HALF_LN2 * i / steps
        err = abs(horner(poly, r) - q(r))
        if err > worst:
            worst = err
    print(f"\nfit error of Q over |r| <= ln2/2: {float(worst):.3e}")
