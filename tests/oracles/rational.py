"""Rational interpolation at Chebyshev nodes, in Decimal.

A companion to `chebyshev.py`, for the functions a polynomial cannot cover at a length worth
transcribing into seven languages. The inverse normal CDF is the case that needed it: a degree-12
polynomial over the central band reaches only 1.6e-07 and its coefficients run to 1e8 with
alternating signs, which is the power basis telling you it is the wrong basis. A rational function
of degree (8, 8) over the same band reaches 7e-17 with coefficients that stay within a few orders
of one.

R(x) = P(x)/Q(x) with deg P = m, deg Q = n and Q normalized so its constant term is 1. Requiring
R(x_k) = f(x_k) at m + n + 1 points is linear in the unknowns once written as

    f(x_k) * Q(x_k) - P(x_k) = 0

so one dense solve produces every coefficient. Interpolation at Chebyshev nodes rather than a true
minimax fit: it is close enough to minimax for this purpose and needs no Remez iteration, which
for rational approximants is delicate to get right and easy to get subtly wrong.
"""

from decimal import Decimal as D, getcontext

from chebyshev import PI, dcos

getcontext().prec = 80


def _solve(matrix, rhs):
    """Gaussian elimination with partial pivoting, exact to the working precision."""
    size = len(matrix)
    augmented = [row[:] + [rhs[i]] for i, row in enumerate(matrix)]
    for column in range(size):
        pivot = max(range(column, size), key=lambda r: abs(augmented[r][column]))
        if augmented[pivot][column] == 0:
            raise ZeroDivisionError("rational interpolation is singular at this degree")
        augmented[column], augmented[pivot] = augmented[pivot], augmented[column]
        for row in range(size):
            if row == column:
                continue
            factor = augmented[row][column] / augmented[column][column]
            for k in range(column, size + 1):
                augmented[row][k] -= factor * augmented[column][k]
    return [augmented[i][size] / augmented[i][i] for i in range(size)]


def rational_fit(f, a: D, b: D, m: int, n: int):
    """Interpolate f on [a, b] by P/Q of degrees (m, n); return (P, Q) in the variable itself.

    Q[0] is 1 by construction, so the ports evaluate two Horner chains and one divide.
    """
    count = m + n + 1
    nodes = []
    for k in range(count):
        t = dcos(PI * (2 * k + 1) / (2 * count))
        nodes.append((a + b) / 2 + (b - a) / 2 * t)

    matrix, rhs = [], []
    for x in nodes:
        value = f(x)
        row = [x**j for j in range(m + 1)] + [-value * x**j for j in range(1, n + 1)]
        matrix.append(row)
        rhs.append(value)
    solution = _solve(matrix, rhs)
    return solution[: m + 1], [D(1)] + solution[m + 1 :]


def evaluate(numerator, denominator, x: D) -> D:
    """Evaluate P(x)/Q(x) the way the ports do: two Horner chains, then one divide."""
    p = D(0)
    for c in reversed(numerator):
        p = p * x + c
    q = D(0)
    for c in reversed(denominator):
        q = q * x + c
    return p / q
