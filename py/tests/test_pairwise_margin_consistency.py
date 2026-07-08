"""Regression: the exact binomial coefficient must use integer arithmetic.

Float accumulation overflowed 2^53 in the partial products for C(56,27), giving
a margin of 784 instead of the correct 782 (go/cs/rs/r) at misrate 1.0.
"""

from pragmastat.pairwise_margin import pairwise_margin


def test_pairwise_margin_exact_integer_binomial():
    assert pairwise_margin(29, 27, 1.0) == 782
