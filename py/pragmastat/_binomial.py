"""Binomial coefficients for the misrate floor and the exact pairwise-margin distribution.

Both call sites want C(n+m, k) for the same n+m: the admissible misrate is
``2 / C(n+m, n)`` and the Loeffler recurrence normalizes its pmf by ``C(n+m, m)``.
They share one entry point so they cannot drift onto different routes, which is
what makes the admissibility check agree with the distribution it guards.
"""

import math

# Below this total, C(n, k) fits the 64-bit integers the other six ports use, so every
# implementation returns the exactly rounded value; at or above it they all switch to the
# binary64 multiplicative recurrence. Python's ints are arbitrary precision and would not
# overflow here, but the threshold is a cross-language contract rather than an
# implementation limit: computing more of the range exactly than the other ports do would
# put this one on a different misrate floor.
MAX_ACCEPTABLE_BINOM_N = 62


def binomial_coefficient(n: int, k: int) -> float:
    """Computes C(n, k) as a binary64, by the route all seven implementations agree on."""
    if n < MAX_ACCEPTABLE_BINOM_N:
        return _exact(n, k)
    return _recurrence(n, k)


def _exact(n: int, k: int) -> float:
    """Computes C(n, k) in exact integer arithmetic, then rounds once."""
    if k > n:
        return 0.0
    if k == 0 or k == n:
        return 1.0

    k = min(k, n - k)  # Take advantage of symmetry
    result = 1  # exact integer arithmetic: each partial product is divisible

    for i in range(k):
        result = result * (n - i) // (i + 1)

    return float(result)


def _recurrence(n: int, k: int) -> float:
    """Computes C(n, k) in binary64 by the multiplicative recurrence
    C(n, k) = prod_{i=1..k} (n-k+i)/i.

    It replaced an exp-of-Stirling formulation, for two reasons. The specification defines
    the admissible misrate as ``misrate >= 2 / C(n+m, n)``, an exact integer quantity;
    measured here against ``math.comb`` over 4 <= n <= 400, Stirling was inexact on 99.0% of
    cases with a worst relative error of 1.6e-8, while this recurrence is inexact on 88.4%
    with a worst relative error of 2.3e-15. And Stirling reached the answer through
    ``math.log`` and ``math.exp``, which every language takes from a different libm: the same
    Stirling code measures 8.1e-9 in Go against 1.6e-8 here, and perturbing those calls by a
    single ulp moved the computed misrate floor on 75579 of 79797 sample-size pairs. This
    form calls nothing, so the same perturbation moves none of them, and all seven
    implementations run the identical sequence of one multiply and one divide per step.

    Normalizing k to the smaller half also makes the function symmetric by construction,
    which matters because the two call sites ask for C(n+m, n) and C(n+m, m). Those are the
    same number, and now they are also the same bits, so the comparison at the misrate floor
    cannot be decided by which of the two rounded higher.

    Every operand is an explicit float. An int operand would promote anyway; spelling it out
    is what keeps ``//`` from ever looking like a plausible edit.
    """
    k = min(k, n - k)
    acc = 1.0
    for i in range(1, k + 1):
        acc = acc * float(n - k + i) / float(i)
        # Once the accumulator reaches infinity the remaining steps cannot bring it back: each
        # one multiplies by a positive integer and divides by a positive integer. Stopping there
        # is the same sequence of roundings, arrived at sooner: at n = m = 100000 it is 89 steps
        # instead of 100000.
        if math.isinf(acc):
            break
    return acc
