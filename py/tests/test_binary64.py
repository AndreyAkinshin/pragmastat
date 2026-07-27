"""The exactness predicate itself: it must be payload equality, not ``==``.

Every suite that claims to compare bit for bit routes through :mod:`binary64`,
so these are the tests that keep that claim honest. Both directions where the
two predicates disagree are pinned, because neither is uniformly stronger.
"""

import math

import pytest
from binary64 import assert_identical, assert_sequence_identical, fmt, identical, payload

NAN = float("nan")
ONE_ULP_ABOVE_ONE = math.nextafter(1.0, 2.0)


def test_payload_separates_the_two_zeros():
    assert payload(0.0) == 0x0000000000000000
    assert payload(-0.0) == 0x8000000000000000


def test_identical_rejects_sign_of_zero():
    # -0.0 == 0.0 is true; the payloads differ, and the claim is about payloads.
    assert not identical(-0.0, 0.0)


def test_identical_accepts_matching_nan_payloads():
    # NaN == NaN is false; identical NaN payloads are identical results.
    assert identical(NAN, NAN)


def test_identical_rejects_one_ulp():
    assert not identical(ONE_ULP_ABOVE_ONE, 1.0)


def test_assert_identical_reports_both_payloads_in_hex():
    with pytest.raises(AssertionError) as exc_info:
        assert_identical(-0.0, 0.0, "sign of zero")
    message = str(exc_info.value)
    assert "sign of zero" in message
    assert "0x0000000000000000" in message
    assert "0x8000000000000000" in message
    # The decimal values stay in the message next to the payloads.
    assert "-0.0" in message


def test_fmt_pairs_the_decimal_value_with_its_payload():
    assert fmt(1.0) == "1.0 (0x3FF0000000000000)"


def test_assert_sequence_identical_reports_the_offending_index():
    with pytest.raises(AssertionError, match=r"index 1"):
        assert_sequence_identical([1.0, -0.0], [1.0, 0.0], "draws")


def test_assert_sequence_identical_rejects_a_length_mismatch():
    with pytest.raises(AssertionError, match=r"length mismatch"):
        assert_sequence_identical([1.0], [1.0, 2.0], "draws")
