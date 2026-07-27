"""Regressions on the binomial coefficient behind the pairwise margin.

Float accumulation overflowed 2^53 in the partial products for C(56,27), giving
a margin of 784 instead of the correct 782 (go/cs/rs/r) at misrate 1.0, which is
why totals below n+m = 62 go through exact integer arithmetic.
"""

from binary64 import assert_identical

from pragmastat._binomial import binomial_coefficient
from pragmastat.min_misrate import min_achievable_misrate_two_sample
from pragmastat.pairwise_margin import pairwise_margin


def test_pairwise_margin_exact_integer_binomial():
    assert pairwise_margin(29, 27, 1.0) == 782


def test_binomial_coefficient_is_symmetric():
    """C(n+m, n) and C(n+m, m) are one number, so the two call sites must get one float.

    At the misrate floor the comparison ``1/total >= misrate/2`` is an exact tie, and an
    asymmetric binomial would decide it by whichever of the two rounded higher. "One float"
    means one payload, so the assertion compares the bits (see :mod:`binary64`).
    """
    for n in range(1, 80):
        for m in range(1, 80):
            assert_identical(binomial_coefficient(n + m, m), binomial_coefficient(n + m, n), f"C(n+m, m) n={n} m={m}")


def test_min_misrate_saturates_beyond_binary64_range():
    """Past the point where C(n+m, n) leaves binary64 range the floor is 0, not an exception.

    ``math.comb`` used to compute this floor and raised OverflowError converting its exact
    integer to a float, so every two-sample bound on samples of roughly 515 or more failed
    outright while the other six ports returned a margin.
    """
    assert_identical(min_achievable_misrate_two_sample(515, 515), 0.0, "two-sample misrate floor at n=m=515")
    margin = pairwise_margin(600, 600, 1e-3)
    assert margin > 0
    assert margin % 2 == 0
