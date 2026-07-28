"""Chebyshev interpolation in Decimal: coefficients for a plain Horner chain."""
import sys
from decimal import Decimal as D, getcontext
sys.path.insert(0, sys.path[0] or ".")
getcontext().prec = 80

PI = D("3.14159265358979323846264338327950288419716939937510582097494459230781640628620899")

def dcos(x: D) -> D:
    """cos by Taylor, adequate at this precision for |x| <= pi."""
    s = D(0); term = D(1); n = 0
    while True:
        s += term if n % 2 == 0 else -term
        n += 1
        term = term * x * x / ((2 * n - 1) * (2 * n))
        if abs(term) < D(10) ** -75:
            break
    return s

def cheb_fit(f, a: D, b: D, n: int):
    """Interpolate f on [a, b] at n Chebyshev nodes; return power-basis coefficients."""
    nodes = []
    for k in range(n):
        t = dcos(PI * (2 * k + 1) / (2 * n))
        nodes.append((a + b) / 2 + (b - a) / 2 * t)
    # Chebyshev coefficients of the interpolant
    c = []
    for j in range(n):
        s = D(0)
        for k in range(n):
            t = dcos(PI * (2 * k + 1) / (2 * n))
            s += f(nodes[k]) * dcos(D(j) * PI * (2 * k + 1) / (2 * n))
        c.append(2 * s / n)
    c[0] /= 2
    # Convert to the power basis in u = (2x - a - b) / (b - a)
    poly = [D(0)] * n
    Tprev = [D(1)] + [D(0)] * (n - 1)
    Tcur = [D(0), D(1)] + [D(0)] * (n - 2) if n > 1 else [D(0)]
    for i in range(n):
        T = Tprev if i == 0 else (Tcur if i == 1 else None)
        if i >= 2:
            T = [D(0)] * n
            for d in range(n - 1):
                T[d + 1] += 2 * Tcur[d]
            for d in range(n):
                T[d] -= Tprev[d]
            Tprev, Tcur = Tcur, T
        elif i == 1:
            Tprev, Tcur = Tprev, Tcur
        for d in range(n):
            poly[d] += c[i] * T[d]
    return poly

def horner(poly, u: D) -> D:
    r = D(0)
    for coef in reversed(poly):
        r = r * u + coef
    return r
