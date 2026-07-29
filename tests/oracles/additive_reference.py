"""A reference for the normal distribution function, good to more digits than binary64 has.

Used to fit the coefficients in additive_cumulative and to check what they deliver. It depends on nothing
beyond the standard library on purpose: an oracle with a dependency is an oracle that stops
running, and this repository has already had one of those.

Two regimes, each convergent where it is used, and they agree to 36 digits on their overlap:
the Taylor series for erf below 2.5, the continued fraction for erfc above it.
"""
from decimal import Decimal as D, getcontext

getcontext().prec = 80

def _sqrt_pi():
    # pi to 80 digits, then sqrt in Decimal
    pi = D("3.14159265358979323846264338327950288419716939937510582097494459230781640628620899")
    return pi.sqrt()

SQRT_PI = _sqrt_pi()

def erf_series(x: D) -> D:
    s = D(0); term = x; n = 0
    while True:
        add = term / (2 * n + 1)
        s += add if n % 2 == 0 else -add
        n += 1
        term = term * x * x / n
        if abs(term / (2 * n + 1)) < D(10) ** -70:
            break
    return 2 / SQRT_PI * s

def erfc_cf(x: D) -> D:
    """erfc(x) = exp(-x^2)/(x*sqrt(pi)) * 1/(1+ 1/(2x^2)/(1+ 2/(2x^2)/(1+ ...)))"""
    # Lentz, on the continued fraction erfc(x)*sqrt(pi)*x*exp(x^2) = 1/(1+ v1/(1+ v2/(1+ ...)))
    tiny = D(10) ** -70
    f = tiny; c = f; d = D(0)
    for i in range(1, 400):
        a = D(i) / 2 / (x * x) if i > 1 else D(1)
        # standard form: b_i = 1, a_1 = 1, a_i = (i-1)/(2x^2)
        ai = D(1) if i == 1 else D(i - 1) / (2 * x * x)
        d = 1 + ai * d
        if d == 0:
            d = tiny
        c = 1 + ai / c
        if c == 0:
            c = tiny
        d = 1 / d
        delta = c * d
        f = f * delta
        if abs(delta - 1) < D(10) ** -70:
            break
    return f * (-x * x).exp() / (x * SQRT_PI)

def erfc_ref(xf: float) -> D:
    x = D(repr(xf))
    if x < 0:
        return 2 - erfc_ref(-xf)
    if x < D("2.5"):
        return 1 - erf_series(x)
    return erfc_cf(x)

# The two regimes are used on either side of x = 2.5, and both converge on the overlap around it.
# The continued fraction converges faster the larger x is, so the agreement is worst at the low end
# of the overlap: 3e-47 at x = 2.0 against 2.8e-58 at the switch itself. The threshold is set below
# the worst of those with room to spare. It was 1e-60 once, which nothing on the overlap reaches,
# and the check printed a line saying so on every run without failing.
AGREEMENT_THRESHOLD = D(10) ** -45

def check_regimes_agree() -> bool:
    """Cross-check the two regimes on their overlap. Returns True when they agree."""
    ok = True
    for t in ("2.0", "2.2", "2.4", "2.49"):
        a = 1 - erf_series(D(t))
        b = erfc_cf(D(t))
        rel = abs(a - b) / a
        agree = rel < AGREEMENT_THRESHOLD
        ok = ok and agree
        print(f"x={t}: series={str(a)[:24]} cf={str(b)[:24]} relative={rel:.3e} agree={agree}")
    return ok

if __name__ == "__main__":
    import sys

    if not check_regimes_agree():
        print("ERROR: the two regimes disagree on their overlap", file=sys.stderr)
        sys.exit(1)
    print("the two regimes agree on their overlap")
