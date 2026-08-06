"""Behavioral tests for the deterministic inverse normal CDF.

The stored fixtures compare this function against the other six ports payload by payload. These
tests state what is true of it independently of any stored number: what the definition fixes, what
the textbooks say, and which symmetry is real as opposed to merely plausible.
"""

import math
import struct
import sys
from decimal import Decimal, getcontext
from pathlib import Path

import pytest

from pragmastat.additive_quantile import _CENTRAL_HALF_WIDTH, additive_quantile


def _ulp(value):
    packed = struct.unpack("<Q", struct.pack("<d", abs(value)))[0]
    return struct.unpack("<d", struct.pack("<Q", packed + 1))[0] - abs(value)


def _load_normal_quantile_oracle():
    """The 80-digit reference the fit was made against, loaded from tests/oracles.

    Imported here rather than at module scope: it is a repository-local script, not a package
    dependency, and a test that cannot find it should say so where it is used.
    """
    root = Path(__file__).resolve()
    while not (root / "CITATION.cff").exists():
        assert root != root.parent, "repository root not found"
        root = root.parent
    sys.path.insert(0, str(root / "tests" / "oracles"))
    from normal_quantile import normal_quantile

    return normal_quantile


@pytest.mark.parametrize(
    ("p", "expected"),
    [(0.0, -math.inf), (1.0, math.inf), (-0.1, -math.inf), (1.1, math.inf)],
)
def test_values_the_definition_fixes(p, expected):
    assert additive_quantile(p) == expected


def test_center_is_exactly_zero():
    got = additive_quantile(0.5)
    assert got == 0.0
    assert math.copysign(1.0, got) == 1.0


def test_nan_propagates():
    assert math.isnan(additive_quantile(math.nan))


@pytest.mark.parametrize(
    ("p", "expected"),
    [
        (0.75, 0.674489750196082),
        (0.9, 1.2815515655446004),
        (0.95, 1.6448536269514722),
        (0.975, 1.959963984540054),
        (0.99, 2.3263478740408408),
        (0.995, 2.5758293035489004),
        (0.999, 3.090232306167813),
    ],
)
def test_textbook_values(p, expected):
    """The numbers a reader is most likely to check by hand."""
    assert additive_quantile(p) == pytest.approx(expected, rel=8 * 2.220446049250313e-16)


@pytest.mark.parametrize("q", [0.25, 0.125, 0.0625, 0.4])
def test_central_branch_is_odd_about_the_center(q):
    """The central branch consumes q = p - 1/2 directly, so it is exactly odd.

    The tails are not, and asserting it would assert something false: they work from min(p, 1-p),
    and 1.0 - 0.95 is 0.050000000000000044 rather than 0.05. The band edge is excluded for a
    related reason, since 0.5 + 0.425 - 0.5 returns 0.42500000000000004 and lands its point in the
    other branch.
    """
    assert 0.5 + q - 0.5 == q, "the offset itself does not round-trip"
    assert q <= _CENTRAL_HALF_WIDTH
    assert additive_quantile(0.5 + q) == -additive_quantile(0.5 - q)


def test_monotone_into_the_far_tail():
    """The smallest positive double is the last argument a caller can pass, and it is finite."""
    previous = -math.inf
    p = 5e-324
    while p < 0.5:
        got = additive_quantile(p)
        assert got > previous, f"not monotone at p = {p}"
        assert math.isfinite(got), f"not finite at p = {p}"
        previous = got
        p *= 8.0
    assert additive_quantile(5e-324) == pytest.approx(-38.467405617144, abs=1e-9)


def _error_in_ulp(p, oracle):
    value = additive_quantile(p)
    return float(abs(Decimal(value) - oracle(Decimal(p))) / Decimal(_ulp(value)))


# The worst argument found on each branch by a 320000-point sweep against the 80-digit reference,
# with the error it produces. These are pins, not a bound: the function is fixed bit for bit by the
# shared fixtures, so each of these numbers is a constant, and any change to the coefficients or to
# the order of operations moves it. A sweep small enough to run as a unit test does not reach them
# (600 points reports about 5), which is why the sharp check and the wide one are separate tests.
#
# The sweep that produced them is the one described in the manual under Algorithm. Reproduce it
# before editing these: the arguments are not special, they are simply where a sweep of that size
# happened to land, and a sweep of a different size will find slightly different ones.
#
# Checking that these pins can fail takes more care than it looks. Seven of the far-tail chain's 22
# coefficients change no output at all when moved by one unit in the last place, over 20000
# arguments, so mutating one of those proves nothing about the test. Of the coefficients that are
# observable, one direction of the move often rounds back to the same double while the other does
# not. A mutation that leaves the function's output identical is not a weak test, it is no test.
WORST_KNOWN = {
    "central": (0.452094776420563, 6.304),
    "near tail": (3.959370179526788e-05, 6.383),
    "far tail": (2.2858723302822197e-47, 7.667),
}


@pytest.mark.parametrize(("branch", "argument", "expected"), [(k, *v) for k, v in WORST_KNOWN.items()])
def test_the_known_worst_argument_of_each_branch_has_not_moved(branch, argument, expected):
    """A sharp check where a bound is blunt.

    The wide sweep below has to allow room for a future sweep to find a slightly worse point, so
    its bound sits well above anything observed. That leaves it unable to notice a change that
    doubles the error at one argument without crossing the bound. These pins notice.
    """
    getcontext().prec = 80
    got = _error_in_ulp(argument, _load_normal_quantile_oracle())
    assert got == pytest.approx(expected, abs=0.01), f"the {branch} worst case moved from {expected} to {got:.4f} ulp"


def test_additive_quantile_is_faithful_over_the_whole_open_interval():
    """The accuracy check the shared fixture cannot make.

    tests/check_function_oracles.py compares the fixture's 546 outputs against an 80-digit
    reference, and the fixture is four regular grids plus the seams. Regular grids are the wrong
    instrument for a worst case: they sample the branches evenly, and a worst case does not live at
    evenly spaced arguments. The fixture reaches 4.4 ulp; a 320000-point sweep drawn log-uniformly
    reaches 7.7, on the branches:

        central     6.30 ulp
        near tail   6.38 ulp
        far tail    7.67 ulp

    with a mean of 1.03 and 58% of results inside one ulp. The error is arithmetic rather than
    approximation, and that is measured: at the far-tail argument above, the same coefficients at
    the same w evaluated in 60-digit arithmetic land 0.05 ulp from the true value.

    The bound is 16 rather than 8. A bound has to sit above what a longer sweep would find, not
    merely above what this one did, and the observed worst case climbed with every enlargement of
    the sweep: 5.1 at 600 arguments, 6.8 at 10000, 7.7 at 320000. Set at 8 it would have been a
    gate with a 4% margin against its own measurement, which is a gate that fails on the day
    someone lengthens the sweep rather than on the day someone breaks the function.
    """
    getcontext().prec = 80
    oracle = _load_normal_quantile_oracle()

    worst, worst_at = 0.0, None
    for index in range(600):
        # A low-discrepancy sequence rather than a pseudo-random one: the golden-ratio rotation
        # spreads points more evenly than sampling does, needs no seed to be reproducible, and
        # avoids arguing with a linter about a generator nothing here is using for randomness.
        position = (index * 0.6180339887498949) % 1.0
        band = index % 3
        if band == 0:
            p = 0.075 + position * 0.85
        elif band == 1:
            p = 10 ** (-(1.2 + position * 9.6))
        else:
            p = 10 ** (-(11 + position * 289))
        value = additive_quantile(p)
        if value == 0.0 or not math.isfinite(value):
            continue
        error = _error_in_ulp(p, oracle)
        if error > worst:
            worst, worst_at = error, p

    assert worst < 16.0, f"additive_quantile({worst_at}) is {worst:.4f} ulp from the true value"


def test_additive_quantile_is_exactly_antisymmetric():
    """Phi^-1(1/2 + q) = -Phi^-1(1/2 - q), exactly, whenever both arguments are representable.

    The central branch is written as q * R(r) with r even in q, so the sign rides on the factor and
    the approximant never has to represent it. That makes the identity exact rather than
    approximate, and it is worth pinning because a rewrite that folded q into the polynomial would
    keep every other test green while losing it.

    The pairs are filtered rather than assumed: for most p, (1 - p) - 1/2 is not -(p - 1/2), and a
    test that skipped that filter would be measuring binary64 representation rather than this
    function. It reports 42% "asymmetry" if you let it.
    """
    pairs = 0
    for index in range(20000):
        q = ((index * 0.6180339887498949) % 1.0) * 0.425
        upper, lower = 0.5 + q, 0.5 - q
        if (upper - 0.5) != q or (0.5 - lower) != q:
            continue
        pairs += 1
        assert additive_quantile(upper) == -additive_quantile(lower), f"asymmetric at q={q!r}"
    assert pairs > 5000, f"only {pairs} exactly representable pairs; the filter is too strict"
