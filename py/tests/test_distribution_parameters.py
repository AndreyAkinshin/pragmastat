"""The five parametric families refuse malformed parameters at construction.

Every parameter has to be finite, and every scale or shape parameter has to be positive.

Two gaps this covers. Each family used to check only its scale parameter, so a non-finite
location constructed a family whose every draw is NaN. And the predicate was written negatively
(``std_dev <= 0``), which is not the negation of ``std_dev > 0`` under IEEE 754: ``NaN <= 0`` is
False, so NaN walked past the guard. The predicates are positive now, which refuses NaN without a
branch of its own.
"""

import math

import pytest

from pragmastat import Rng
from pragmastat.distributions import Additive, Exp, Multiplic, Power, Uniform


@pytest.mark.parametrize("min_val", [math.nan, -math.inf, math.inf])
def test_uniform_rejects_a_non_finite_min(min_val):
    with pytest.raises(ValueError, match="min must be finite"):
        Uniform(min_val, 1.0)


@pytest.mark.parametrize("max_val", [math.nan, -math.inf, math.inf])
def test_uniform_rejects_a_non_finite_max(max_val):
    with pytest.raises(ValueError, match="max must be finite"):
        Uniform(0.0, max_val)


@pytest.mark.parametrize("bounds", [(1.0, 1.0), (2.0, 1.0)])
def test_uniform_rejects_an_empty_interval(bounds):
    with pytest.raises(ValueError, match="min must be less than max"):
        Uniform(*bounds)


@pytest.mark.parametrize("mean", [math.nan, math.inf, -math.inf])
def test_additive_rejects_a_non_finite_mean(mean):
    with pytest.raises(ValueError, match="mean must be finite"):
        Additive(mean, 1.0)


@pytest.mark.parametrize("std_dev", [math.nan, math.inf, 0.0, -1.0])
def test_additive_rejects_a_malformed_std_dev(std_dev):
    with pytest.raises(ValueError, match="std_dev must be finite and positive"):
        Additive(0.0, std_dev)


@pytest.mark.parametrize("log_mean", [math.nan, math.inf, -math.inf])
def test_multiplic_rejects_a_non_finite_log_mean(log_mean):
    with pytest.raises(ValueError, match="log_mean must be finite"):
        Multiplic(log_mean, 1.0)


@pytest.mark.parametrize("log_std_dev", [math.nan, math.inf, 0.0, -1.0])
def test_multiplic_rejects_a_malformed_log_std_dev(log_std_dev):
    with pytest.raises(ValueError, match="log_std_dev must be finite and positive"):
        Multiplic(0.0, log_std_dev)


@pytest.mark.parametrize("rate", [math.nan, math.inf, 0.0, -1.0])
def test_exp_rejects_a_malformed_rate(rate):
    with pytest.raises(ValueError, match="rate must be finite and positive"):
        Exp(rate)


@pytest.mark.parametrize("min_val", [math.nan, math.inf, 0.0, -1.0])
def test_power_rejects_a_malformed_min(min_val):
    with pytest.raises(ValueError, match="min must be finite and positive"):
        Power(min_val, 3.0)


@pytest.mark.parametrize("shape", [math.nan, math.inf, 0.0, -1.0])
def test_power_rejects_a_malformed_shape(shape):
    with pytest.raises(ValueError, match="shape must be finite and positive"):
        Power(1.0, shape)


def test_well_formed_families_still_construct_and_draw():
    rng = Rng("distribution-parameters")
    families = [
        Uniform(-2.5, 7.5),
        Additive(10.0, 2.0),
        Multiplic(1.0, 0.5),
        Exp(3.0),
        Power(1.0, 2.0),
    ]
    for family in families:
        assert math.isfinite(family.sample(rng))
