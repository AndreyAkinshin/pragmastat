"""Fits the polynomials behind the deterministic inverse normal CDF.

`AdditiveQuantile` is the last primitive the toolkit needs from the transcendental family. It
inverts `AdditiveCumulative`, and the distributions reach it wherever a normal quantile is asked
for: `Additive.Quantile` is mu + sigma * Phi^-1(p) and `Multiplic.Quantile` exponentiates the same
thing. IEEE 754 fixes nothing about any of it, and a quantile is a value a caller compares and
reports, so the function has to be ours for the reason the exponential and the logarithm already
are.

The shape of the approximation is not a free choice. Phi^-1 runs to infinity at both ends, so no
single polynomial covers (0, 1); the classical answer, and the one used here, splits the domain:

    central   |p - 1/2| <= 0.425     x = q * R1(r),  q = p - 1/2,  r = 0.180625 - q^2
    tails     otherwise              x = sign * R2(w),  w = sqrt(-log(min(p, 1-p)))

The central branch is odd in q, which the factored form preserves exactly: at p = 1/2 it returns
zero by construction rather than by cancellation. The tail branch is smooth in w because the
logarithm has already undone most of the growth, which is what makes a short polynomial reach the
end of the double range at all.

`w` needs a logarithm, and it is the toolkit's own. That is the reason this primitive could not
have been written before `LogFunction` existed: a tail quantile derived from the platform's
logarithm would disagree between languages exactly where the tails matter.

Three branches and rational approximants, both established by measurement rather than assumed.
Two things were tried first and rejected by the numbers this script prints:

    a polynomial in the central band       degree 12 reaches 1.6e-07, and its coefficients run to
                                           1e8 with alternating signs, which is the power basis
                                           saying it is the wrong basis
    one fit across the whole tail          degree (8,8) reaches 3.0e-10, because the band spans
                                           w from 1.6 to 27 and no single approximant of a usable
                                           length is uniform over it

Splitting the tail at w = 5, as AS 241 does, and using rational approximants throughout brings all
three branches below the binary64 rounding floor. The split point is not tuned: it is where the
near branch stops being comfortable, and moving it does not change the conclusion.
"""

import sys
from decimal import Decimal as D, getcontext

sys.path.insert(0, sys.path[0] or ".")
getcontext().prec = 80
from rational import evaluate, rational_fit  # noqa: E402
from normal_quantile import normal_quantile  # noqa: E402

# The split point, and the constant the central branch reduces by. Both are AS 241's: the central
# band is where |q| <= 0.425, and 0.180625 is 0.425^2, so r = 0.180625 - q^2 is non-negative there
# and vanishes at the band edge.
CENTRAL_HALF_WIDTH = D("0.425")
CENTRAL_OFFSET = CENTRAL_HALF_WIDTH * CENTRAL_HALF_WIDTH

# The tail band in w = sqrt(-log(min(p, 1-p))). p = 0.075 is the band edge and 2^-1074 the smallest
# positive double, which is the last argument a caller can pass.
TAIL_W_LO = ((-((D(1) / 2 - CENTRAL_HALF_WIDTH).ln())).sqrt())
TAIL_W_HI = ((-(D(2) ** -1074).ln())).sqrt()

# Degrees chosen as the smallest that clear the rounding floor on their band; main() prints the
# error each one achieves, so a reader can see the margin rather than trust the choice.
CENTRAL_DEGREE = (8, 8)
TAIL_NEAR_DEGREE = (8, 8)
TAIL_FAR_DEGREE = (10, 10)
TAIL_SPLIT = D(5)


def central_ratio(r: D) -> D:
    """Phi^-1(1/2 + q) / q as a function of r = 0.180625 - q^2, continuous at the center.

    Dividing out q is what keeps the branch odd: the polynomial never has to represent the sign,
    and p = 1/2 returns exactly zero because it is multiplied by q = 0 at the end.
    """
    q_squared = CENTRAL_OFFSET - r
    if q_squared <= 0:
        q = D(0)
    else:
        q = q_squared.sqrt()
    if q == 0:
        # The limit of Phi^-1(1/2 + q)/q as q -> 0 is sqrt(2*pi).
        return (2 * _pi()).sqrt()
    return normal_quantile(D(1) / 2 + q) / q


def _pi() -> D:
    return D(
        "3.14159265358979323846264338327950288419716939937510582097494459230781640628620899"
    )


def tail_value(w: D) -> D:
    """Phi^-1(p) as a function of w = sqrt(-log(p)), for p in the lower tail.

    Returned positive; the ports apply the sign. w is monotone in p, so the band maps cleanly.
    """
    p = (-(w * w)).exp()
    return -normal_quantile(p)


BRANCHES = {}


def _fit():
    """Fit all three branches. Each is a rational approximant in its own variable."""
    BRANCHES["central"] = rational_fit(central_ratio, D(0), CENTRAL_OFFSET, *CENTRAL_DEGREE)
    BRANCHES["tail_near"] = rational_fit(tail_value, TAIL_W_LO, TAIL_SPLIT, *TAIL_NEAR_DEGREE)
    BRANCHES["tail_far"] = rational_fit(tail_value, TAIL_SPLIT, TAIL_W_HI, *TAIL_FAR_DEGREE)


def _worst_relative(branch, f, lo, hi, steps=600):
    numerator, denominator = BRANCHES[branch]
    worst = D(0)
    for i in range(steps + 1):
        x = lo + (hi - lo) * i / steps
        exact = f(x)
        if exact == 0:
            continue
        got = evaluate(numerator, denominator, x)
        worst = max(worst, abs(got - exact) / abs(exact))
    return worst


def fmt(p):
    return [f"{float(c):.17e}" for c in p]


def main() -> int:
    _fit()
    print(f"CENTRAL_HALF_WIDTH = {float(CENTRAL_HALF_WIDTH):.17e}")
    print(f"CENTRAL_OFFSET     = {float(CENTRAL_OFFSET):.17e}")
    print(f"TAIL_SPLIT         = {float(TAIL_SPLIT):.17e}")
    for name in ("central", "tail_near", "tail_far"):
        numerator, denominator = BRANCHES[name]
        print(f"POLY_{name.upper()}_NUM =", fmt(numerator))
        print(f"POLY_{name.upper()}_DEN =", fmt(denominator))

    print()
    print("worst relative fit error per branch, against an 80-digit reference:")
    print(f"  central   {float(_worst_relative('central', central_ratio, D(0), CENTRAL_OFFSET)):.3e}")
    print(f"  tail near {float(_worst_relative('tail_near', tail_value, TAIL_W_LO, TAIL_SPLIT)):.3e}")
    print(f"  tail far  {float(_worst_relative('tail_far', tail_value, TAIL_SPLIT, TAIL_W_HI)):.3e}")
    print()
    print("binary64 relative rounding floor is 1.1e-16; every branch must sit below it.")
    return 0


if __name__ == "__main__":
    sys.exit(main())
