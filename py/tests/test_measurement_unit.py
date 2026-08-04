"""MeasurementUnit refuses malformed unit definitions.

A non-positive ``base_units`` poisons every conversion through the unit and makes ``finer``
prefer it over any well-formed unit; an empty ``family`` makes two unrelated units compatible,
because ``is_compatible`` compares families. An empty ``abbreviation`` stays legal: the three
dimensionless units have none.
"""

import pytest

from pragmastat.measurement_unit import (
    DISPARITY_UNIT,
    NUMBER_UNIT,
    RATIO_UNIT,
    MeasurementUnit,
)


def test_rejects_empty_id():
    with pytest.raises(ValueError, match="id cannot be empty"):
        MeasurementUnit("", "Time", "ms", "Millisecond", 1_000_000)


def test_rejects_empty_family():
    with pytest.raises(ValueError, match="family cannot be empty"):
        MeasurementUnit("ms", "", "ms", "Millisecond", 1_000_000)


def test_rejects_zero_base_units():
    with pytest.raises(ValueError, match="base_units must be positive"):
        MeasurementUnit("ms", "Time", "ms", "Millisecond", 0)


def test_rejects_negative_base_units():
    with pytest.raises(ValueError, match="base_units must be positive"):
        MeasurementUnit("ms", "Time", "ms", "Millisecond", -1)


def test_accepts_well_formed_unit():
    ms = MeasurementUnit("ms", "Time", "ms", "Millisecond", 1_000_000)
    assert ms.id == "ms"
    assert ms.family == "Time"
    assert ms.abbreviation == "ms"
    assert ms.full_name == "Millisecond"
    assert ms.base_units == 1_000_000


def test_dimensionless_units_keep_their_empty_abbreviation():
    for unit_id, unit in [
        ("number", NUMBER_UNIT),
        ("ratio", RATIO_UNIT),
        ("disparity", DISPARITY_UNIT),
    ]:
        assert unit.id == unit_id
        assert unit.abbreviation == ""
        assert unit.base_units == 1
