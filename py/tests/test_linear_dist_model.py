"""Every expected value here is computed by hand against the model's own definition: piecewise
linear interpolation of the sorted sample at anchors (i-1)/(n-1), so the checks double as a
derivation, not just a pin.
"""

import math

import pytest

from pragmastat import LinearDistModel, Measurement, MeasurementUnit, Sample
from pragmastat.assumptions import AssumptionError
from pragmastat.rng import Rng

X = [1.0, 2.0, 4.0, 8.0]


def test_quantile_at_lands_on_each_observation_at_its_anchor():
    model = LinearDistModel(X)
    assert model.quantile_at(0.0) == 1.0
    assert model.quantile_at(1.0 / 3.0) == 2.0
    assert model.quantile_at(2.0 / 3.0) == 4.0
    assert model.quantile_at(1.0) == 8.0


def test_quantile_at_interpolates_between_two_observations():
    model = LinearDistModel(X)
    assert model.quantile_at(0.5) == 3.0


def test_quantile_at_clamps_outside_the_unit_interval():
    model = LinearDistModel(X)
    assert model.quantile_at(-0.1) == 1.0
    assert model.quantile_at(1.1) == 8.0


def test_cumulative_at_inverts_quantile_at():
    model = LinearDistModel(X)
    assert model.cumulative_at(3.0) == 0.5
    assert model.cumulative_at(2.0) == 1.0 / 3.0
    assert model.cumulative_at(4.0) == 2.0 / 3.0


def test_cumulative_at_is_zero_below_and_one_above_the_support():
    model = LinearDistModel(X)
    assert model.cumulative_at(1.0) == 0.0
    assert model.cumulative_at(-5.0) == 0.0
    assert model.cumulative_at(8.0) == 1.0
    assert model.cumulative_at(20.0) == 1.0


def test_density_at_is_mass_over_width_inside_each_cell():
    model = LinearDistModel(X)
    assert model.density_at(1.5) == 1.0 / 3.0
    assert model.density_at(3.0) == 1.0 / 6.0
    assert model.density_at(6.0) == 1.0 / 12.0
    assert model.density_at(8.0) == 1.0 / 12.0  # The right edge belongs to the last cell.


def test_density_at_is_zero_outside_the_support():
    model = LinearDistModel(X)
    assert model.density_at(0.5) == 0.0
    assert model.density_at(9.0) == 0.0


def test_scalar_queries_propagate_nan():
    model = LinearDistModel(X)
    assert math.isnan(model.quantile_at(math.nan))
    assert math.isnan(model.cumulative_at(math.nan))
    assert math.isnan(model.density_at(math.nan))


def test_landscape_agrees_with_the_scalar_queries_bit_for_bit():
    model = LinearDistModel(X)
    landscape = model.landscape()

    assert landscape.boundaries == tuple(X)
    assert landscape.cell_count == 3
    for i, value in enumerate(X):
        assert model.cumulative_at(value) == landscape.cumulative[i]
    for j in range(landscape.cell_count):
        midpoint = (X[j] + X[j + 1]) / 2.0
        assert model.density_at(midpoint) == landscape.densities[j]


def test_landscape_fields_reject_item_mutation():
    model = LinearDistModel(X)
    landscape = model.landscape()
    with pytest.raises(TypeError):
        landscape.boundaries[0] = 999.0
    with pytest.raises(TypeError):
        landscape.cumulative[0] = 999.0
    with pytest.raises(TypeError):
        landscape.densities[0] = 999.0


def test_sample_is_quantile_at_of_a_raw_drawn_cumulance():
    model = LinearDistModel(X)
    drawing = Rng("identity-seed")
    inverting = Rng("identity-seed")
    for _ in range(100):
        drawn = model.sample(drawing)
        u = inverting.uniform_float()
        assert drawn.hex() == model.quantile_at(u).hex()


def test_constructor_rejects_fewer_than_two_observations():
    with pytest.raises(AssumptionError) as exc_info:
        LinearDistModel([1.0])
    assert exc_info.value.violation.id.value == "domain"


def test_constructor_rejects_a_repeated_value():
    with pytest.raises(AssumptionError) as exc_info:
        LinearDistModel([1.0, 1.0, 2.0])
    assert exc_info.value.violation.id.value == "distinctness"


def test_quantile_rejects_a_cumulance_outside_the_unit_interval():
    model = LinearDistModel(X)
    with pytest.raises(AssumptionError) as exc_info:
        model.quantile([1.5])
    assert exc_info.value.violation.id.value == "cumulance"


def test_an_empty_query_vector_is_not_a_violation():
    model = LinearDistModel(X)
    assert model.quantile([]) == []
    assert model.cumulative([]) == []
    assert model.density([]) == []


def test_sample_constructor_takes_the_samples_unit():
    meter = MeasurementUnit("meter", "Length", "m", "Meter", 1)
    sample = Sample(X, unit=meter)
    model = LinearDistModel(sample)
    assert model.unit == meter


def test_sample_constructor_refuses_a_weighted_sample():
    sample = Sample([1.0, 2.0], [1.0, 2.0])
    with pytest.raises(AssumptionError):
        LinearDistModel(sample)


def test_measured_quantile_returns_a_measurement_in_the_models_unit():
    meter = MeasurementUnit("meter", "Length", "m", "Meter", 1)
    model = LinearDistModel(Sample(X, unit=meter))
    result = model.measured.quantile([0.5])
    assert result[0].value == 3.0
    assert result[0].unit == meter


def test_measured_density_returns_a_plain_number():
    meter = MeasurementUnit("meter", "Length", "m", "Meter", 1)
    model = LinearDistModel(Sample(X, unit=meter))
    result = model.measured.density([Measurement(3.0, meter)])
    assert result[0] == 1.0 / 6.0
