"""Fits the polynomial behind the deterministic natural logarithm.

IEEE 754 fixes the result of every arithmetic operation and of the square root, and fixes nothing
about the logarithm, exactly as it fixes nothing about the exponential. Two conforming libraries
may return neighboring values for the same argument, and they do.

The logarithm is on a decision path here. `ratio` is exp(median(log x - log y)), so a last-bit
difference in log moves the median's input, and with it which pairwise element gets selected. That
is not a small error in the answer: it is a different answer. The conformance probe recorded the
consequence: while this function came from the platform, perturbing it moved `ratio` on every one
of its probed inputs, and both suites were declared approximate. They are exact now.

The reduction is the classical one, the same shape fdlibm uses:

    x = 2^k * m  with m in [sqrt(2)/2, sqrt(2)),
    f = m - 1,  s = f / (2 + f),
    log(m) = f - (0.5*f*f - s*(0.5*f*f + P(s*s))),
    log(x) = k*ln2_hi + (k*ln2_lo + log(m))

Everything in it is an IEEE operation with a fixed result, so the only freedom left is the
polynomial P on s*s, which this script produces. ln 2 is split so that k*ln2_hi is exact, for the
reason given in fit_exp.py: ln2_hi carries 32 significant bits and |k| needs at most 11.

The fitted quantity is P(z) = (log((1+s)/(1-s)) - 2*s) / s^3 with z = s*s, rather than the
logarithm itself. Writing log(m) through s makes the series odd, so half the terms vanish and a
given accuracy costs half the degree; subtracting the leading 2*s leaves the polynomial supplying
a correction of at most about 0.7, which is what keeps the assembled result inside one ulp.

The division is by s^3 and not by 2*s^3, so the leading coefficients come out as 2/3, 2/5, 2/7
rather than 1/3, 1/5, 1/7. That is deliberate: the ports evaluate R = z * P(z) and reach the
result through f - (hfsq - s*(hfsq + R)), and the factor of two belongs to P rather than to a
multiply the seven implementations would each have to carry.

The ports assemble the result as f - (hfsq - s*(hfsq + R)) rather than the algebraically equal
f - 0.5*f*f + s*hfsq + s*R. Both compute the same expression; the first keeps the cancellation
between f and 0.5*f*f in one subtraction where the two are closest in magnitude, which is where
the low bits of the answer live.
"""

import sys
from decimal import Decimal as D, getcontext

sys.path.insert(0, sys.path[0] or ".")
getcontext().prec = 80
from chebyshev import cheb_fit, horner  # noqa: E402

LN2 = D("0.693147180559945309417232121458176568075500134360255254120680009493393621969694")
SQRT2 = D("1.414213562373095048801688724209698078569671875376948073176679737990732478462107")

# m runs over [sqrt(2)/2, sqrt(2)), so f = m - 1 runs over [sqrt(2)/2 - 1, sqrt(2) - 1) and
# s = f / (2 + f) is bounded by the endpoint of larger magnitude.
S_MAX = (SQRT2 - 1) / (1 + SQRT2)
Z_MAX = S_MAX * S_MAX


def dlog1p_over(s: D) -> D:
    """log((1+s)/(1-s)) by its odd series; |s| < 0.18 here, so it converges immediately."""
    if s == 0:
        return D(0)
    total = D(0)
    term = s
    n = 0
    while True:
        total += term / (2 * n + 1)
        n += 1
        term = term * s * s
        if abs(term) < D(10) ** -75:
            break
    return 2 * total


def p(z: D) -> D:
    """(log((1+s)/(1-s)) - 2*s) / s^3 at z = s*s, continuous at 0 with value 2/3."""
    if z == 0:
        return D(2) / 3
    s = z.sqrt()
    return (dlog1p_over(s) - 2 * s) / (s * s * s)


DEGREE = 7
_normalized = cheb_fit(p, D(0), Z_MAX, DEGREE)
# cheb_fit works in u over [-1, 1] across [0, z_max]; rescaling to z itself removes the change of
# variable from the evaluation and turns the coefficients back into the recognizable 2/3, 2/5,
# 2/7 series, which is worth having in seven files a reader may check by eye.
_half = Z_MAX / 2


def _rescale(normalized):
    """Rewrites a polynomial in u = (z - half)/half as one in z."""
    out = [D(0)] * len(normalized)
    for degree, coefficient in enumerate(normalized):
        # (z/half - 1)^degree expanded by the binomial theorem.
        binomial = D(1)
        for k in range(degree + 1):
            term = coefficient * binomial * D(-1) ** (degree - k) / _half**k
            out[k] += term
            binomial = binomial * (degree - k) / (k + 1)
    return out


poly = _rescale(_normalized)


def fmt(coefficients):
    """Shortest round-trip form, which is what the ports carry.

    A fixed 17-digit format writes more digits than a double distinguishes, and clippy's
    excessive_precision says so. repr gives the shortest string that reads back as the same
    double, so the literal in each port is the value and nothing more."""
    return [repr(float(c)) for c in coefficients]


LN2_HI = float.fromhex("0x1.62e42fee00000p-1")
LN2_LO = float(LN2 - D(LN2_HI))
SQRT2_OVER_2 = float(SQRT2 / 2)

if __name__ == "__main__":
    print(f"LN2_HI       = {LN2_HI!r}   # {LN2_HI.hex()}")
    print(f"LN2_LO       = {LN2_LO!r}   # {LN2_LO.hex()}")
    print(f"SQRT2_OVER_2 = {SQRT2_OVER_2!r}   # {SQRT2_OVER_2.hex()}")
    print("POLY_P =", fmt(poly))

    # Worst error of the fit alone, in exact arithmetic: what the polynomial costs before any
    # binary64 rounding enters.
    worst = D(0)
    steps = 4000
    for i in range(steps + 1):
        z = Z_MAX * i / steps
        err = abs(horner(poly, z) - p(z))
        if err > worst:
            worst = err
    print(f"\nfit error of P over 0 <= z <= {float(Z_MAX):.6f}: {float(worst):.3e}")
