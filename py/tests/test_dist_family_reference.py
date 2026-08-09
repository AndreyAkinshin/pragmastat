"""The five parametric families against their shared dist-* fixtures.

Each family answers three questions about itself, and each answer is assembled out of the
toolkit's own exponential, logarithm, normal cumulative and normal quantile. Nothing on these
paths reaches a platform transcendental, so the seven ports owe each other identical bits and the
suites are declared exact in tests/manifest.json: the comparison below is by binary64 payload. A
tolerance would pass on exactly the divergence these suites exist to catch.

The sibling distributions suite stays approximate, because drawing a sample still goes through the
platform's log, exp, cos and pow. That is a separate change with its own effect on every seeded
stream.
"""

import json
import math
from pathlib import Path

import pytest

from pragmastat.distributions import Additive, Exp, Multiplic, Power, Uniform
from pragmastat.rng import Rng
from tests.binary64 import assert_identical

SUITES = ["dist-uniform", "dist-exp", "dist-additive", "dist-multiplic", "dist-power"]

# How many values the five suites hold between them: for each family, a density and a cumulative
# at every x and a quantile at every p. Pinned rather than derived, because a loader that finds
# four suites out of five satisfies every other assertion here by having less to compare.
DIST_FAMILY_VALUE_COUNT = 7800


def _tests_root():
    root = Path(__file__).resolve()
    while not (root / "CITATION.cff").exists():
        assert root != root.parent, "repository root not found"
        root = root.parent
    return root / "tests"


def _build(suite, parameters):
    """Build the family the suite names, rather than letting the parameter names decide.

    Two families could come to share a spelling, and a fixture read into the wrong family would
    still produce numbers and still be compared.
    """

    def need(name):
        assert name in parameters, f"{suite}: fixture has no parameter named {name}"
        return parameters[name]

    if suite == "dist-uniform":
        return Uniform(need("min"), need("max"))
    if suite == "dist-exp":
        return Exp(need("rate"))
    if suite == "dist-additive":
        return Additive(need("mean"), need("stdDev"))
    if suite == "dist-multiplic":
        return Multiplic(need("logMean"), need("logStdDev"))
    if suite == "dist-power":
        return Power(need("min"), need("shape"))
    raise AssertionError(f"Unknown suite {suite}")


def _cases():
    for suite in SUITES:
        directory = _tests_root() / suite
        files = sorted(directory.glob("*.json"))
        assert files, f"No JSON test files found in {directory}"
        for path in files:
            yield suite, path


@pytest.mark.parametrize(("suite", "path"), list(_cases()), ids=lambda v: getattr(v, "stem", v))
def test_dist_family_reference(suite, path):
    case = json.loads(path.read_text())
    x = case["input"]["x"]
    p = case["input"]["p"]
    output = case["output"]

    # A file that parses into empty lists passes every comparison below, which is the one way this
    # test could report success while checking nothing.
    assert x, f"{path.name}: no x values"
    assert p, f"{path.name}: no p values"
    assert len(output["density"]) == len(x)
    assert len(output["cumulative"]) == len(x)
    assert len(output["quantile"]) == len(p)

    distribution = _build(suite, case["input"]["parameters"])
    for i, value in enumerate(x):
        assert_identical(
            distribution.density_at(value), output["density"][i], f"{suite}/{path.name} density_at({value!r})"
        )
        assert_identical(
            distribution.cumulative_at(value), output["cumulative"][i], f"{suite}/{path.name} cumulative_at({value!r})"
        )
    for i, value in enumerate(p):
        assert_identical(
            distribution.quantile_at(value), output["quantile"][i], f"{suite}/{path.name} quantile_at({value!r})"
        )


def test_the_five_suites_hold_the_number_of_values_they_are_pinned_to():
    total = 0
    for _, path in _cases():
        case = json.loads(path.read_text())
        total += 2 * len(case["input"]["x"]) + len(case["input"]["p"])
    assert total == DIST_FAMILY_VALUE_COUNT


# The arguments the fixtures cannot carry, because JSON writes neither a NaN nor an infinity.
#
# Without this table the fixtures are the only check the trio has, and they sample the interior of
# each support through the family's own quantile. That left two defects alive: Uniform reported a
# perfectly ordinary density for NaN in six ports, since neither half of `x < min || x >= max`
# holds for a NaN and control fell straight through, and R raised "missing value where TRUE/FALSE
# needed" from thirteen of the fifteen functions, since `NaN < 0` is NA there and `if (NA)` is an
# error. Neither is reachable from a stored fixture.
SPECIAL_VALUES = [
    ("dist-uniform", Uniform(0.0, 1.0), 0.0, 1.0),
    ("dist-exp", Exp(1.0), 0.0, math.inf),
    ("dist-additive", Additive(0.0, 1.0), -math.inf, math.inf),
    ("dist-multiplic", Multiplic(0.0, 1.0), 0.0, math.inf),
    ("dist-power", Power(1.0, 2.0), 1.0, math.inf),
]


@pytest.mark.parametrize(("name", "distribution"), [(n, d) for n, d, _, _ in SPECIAL_VALUES])
def test_nan_propagates_through_all_three(name, distribution):
    assert math.isnan(distribution.density_at(math.nan)), name
    assert math.isnan(distribution.cumulative_at(math.nan)), name
    assert math.isnan(distribution.quantile_at(math.nan)), name


@pytest.mark.parametrize(("name", "distribution"), [(n, d) for n, d, _, _ in SPECIAL_VALUES])
def test_the_infinities_lie_outside_every_support(name, distribution):
    assert distribution.density_at(-math.inf) == 0.0, name
    assert distribution.density_at(math.inf) == 0.0, name
    assert distribution.cumulative_at(-math.inf) == 0.0, name
    assert distribution.cumulative_at(math.inf) == 1.0, name


@pytest.mark.parametrize(("name", "distribution", "lower", "upper"), SPECIAL_VALUES)
def test_a_probability_outside_the_unit_interval_returns_the_endpoints(name, distribution, lower, upper):
    """The contract says so, and the ends differ per family: -inf, 0 or the lower bound."""
    for p in (0.0, -0.1, -math.inf):
        assert distribution.quantile_at(p) == lower, f"{name}: quantile_at({p})"
    for p in (1.0, 1.1, math.inf):
        assert distribution.quantile_at(p) == upper, f"{name}: quantile_at({p})"


# Every family draws by inverting its own quantile.
#
# A fixture says two numbers agreed on the day it was written. This says they cannot disagree: the
# same seed drives a draw and a direct quantile call, and the two must land on the same bits for
# every family. It is what makes "a distribution is three functions" a statement about the code
# rather than about the documentation, and it would fail the moment a family grew a second
# sampling route.
# lifts is True for the two families whose quantile_at(0) is not a sample.
SAMPLING_IDENTITY_FAMILIES = [
    ("Uniform", Uniform(-2.5, 7.5), False),
    ("Exp", Exp(0.5), False),
    ("Additive", Additive(1.0, 2.0), True),
    ("Multiplic", Multiplic(-1.0, 0.5), True),
    ("Power", Power(2.0, 3.0), False),
]


@pytest.mark.parametrize(("name", "distribution", "lifts"), SAMPLING_IDENTITY_FAMILIES)
def test_sampling_is_the_quantile_of_a_drawn_cumulance(name, distribution, lifts):
    drawing = Rng("identity-seed")
    inverting = Rng("identity-seed")
    for i in range(1000):
        drawn = distribution.sample(drawing)
        # The same cumulance the draw consumed, taken from a generator on the same seed.
        u = inverting.uniform_float()
        if lifts and u == 0.0:
            u = 5e-324
        inverted = distribution.quantile_at(u)
        assert_identical(drawn, inverted, f"{name} draw {i}: sample vs quantile_at({u!r})")
