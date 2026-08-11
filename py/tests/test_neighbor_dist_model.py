import math
import struct

import pytest

from pragmastat import Measurement, MeasurementUnit, NeighborDistModel, Sample
from pragmastat.assumptions import AssumptionError
from pragmastat.rng import Rng

X = [0.0, 1.0, 2.0, 4.0, 8.0]


def test_support_matches_the_min_and_max_observation():
    model = NeighborDistModel(X)
    assert model.support_min == 0.0
    assert model.support_max == 8.0


def test_cell_count_is_at_most_one_fewer_than_the_observation_count():
    model = NeighborDistModel(X)
    assert model.cell_count == len(X)


def test_cumulative_at_the_support_is_zero_and_one_exactly():
    model = NeighborDistModel(X)
    assert model.cumulative_at(model.support_min) == 0.0
    assert model.cumulative_at(model.support_max) == 1.0


def test_cell_masses_sum_to_exactly_one():
    model = NeighborDistModel(X)
    landscape = model.landscape()
    assert sum(landscape.masses) == 1.0


def test_cumulative_at_is_non_decreasing_across_the_boundaries():
    model = NeighborDistModel(X)
    cumulative = model.landscape().cumulative
    for i in range(1, len(cumulative)):
        assert cumulative[i] > cumulative[i - 1]


def test_quantile_at_inverts_cumulative_at_at_each_boundary():
    model = NeighborDistModel(X)
    landscape = model.landscape()
    for i, boundary in enumerate(landscape.boundaries):
        assert model.quantile_at(landscape.cumulative[i]) == boundary


def test_quantile_at_clamps_outside_the_unit_interval():
    model = NeighborDistModel(X)
    assert model.quantile_at(-0.1) == model.support_min
    assert model.quantile_at(1.1) == model.support_max


def test_density_at_is_zero_outside_the_support():
    model = NeighborDistModel(X)
    assert model.density_at(model.support_min - 1.0) == 0.0
    assert model.density_at(model.support_max + 1.0) == 0.0


def test_scalar_queries_propagate_nan():
    model = NeighborDistModel(X)
    assert math.isnan(model.quantile_at(math.nan))
    assert math.isnan(model.cumulative_at(math.nan))
    assert math.isnan(model.density_at(math.nan))


def test_landscape_agrees_with_the_scalar_queries_bit_for_bit():
    model = NeighborDistModel(X)
    landscape = model.landscape()
    for i, boundary in enumerate(landscape.boundaries):
        assert model.cumulative_at(boundary) == landscape.cumulative[i]
    for j in range(landscape.cell_count):
        midpoint = (landscape.boundaries[j] + landscape.boundaries[j + 1]) / 2.0
        assert model.density_at(midpoint) == landscape.densities[j]


def test_landscape_exposes_scale_index_and_radius():
    model = NeighborDistModel(X)
    landscape = model.landscape()
    # n = 5: ceil_sqrt(5) = 3 (smallest k with k >= ceil(5/k)), radius = 3 // 2 = 1.
    assert landscape.scale_index == 3
    assert landscape.radius == 1


def test_landscape_sources_map_each_cell_to_its_observation():
    model = NeighborDistModel(X)
    assert model.landscape().sources == (0, 1, 2, 3, 4)


def test_a_zero_width_cell_from_a_collapsed_midpoint_is_dropped():
    # The center observation sits exactly one ULP from each neighbor on either side. Both
    # adjacent midpoints round to 1.0 itself under round-to-nearest-even (each is exactly
    # halfway between two representable doubles, and 1.0 has the even mantissa), so the center
    # cell's two boundaries collapse onto the same point and the cell is dropped.
    def bits(value: float) -> int:
        return struct.unpack("<q", struct.pack("<d", value))[0]

    def from_bits(value: int) -> float:
        return struct.unpack("<d", struct.pack("<q", value))[0]

    center = 1.0
    below = from_bits(bits(center) - 1)
    above = from_bits(bits(center) + 1)
    values = [0.0, below, center, above, 10.0]

    model = NeighborDistModel(values)
    landscape = model.landscape()

    assert landscape.cell_count == len(values) - 1
    assert 2 not in landscape.sources


def test_sample_is_quantile_at_of_a_raw_drawn_cumulance():
    model = NeighborDistModel(X)
    drawing = Rng("identity-seed")
    inverting = Rng("identity-seed")
    for _ in range(100):
        drawn = model.sample(drawing)
        u = inverting.uniform_float()
        assert drawn.hex() == model.quantile_at(u).hex()


def test_constructor_rejects_fewer_than_two_observations():
    with pytest.raises(AssumptionError) as exc_info:
        NeighborDistModel([1.0])
    assert exc_info.value.violation.id.value == "domain"


def test_constructor_rejects_a_repeated_value():
    with pytest.raises(AssumptionError) as exc_info:
        NeighborDistModel([1.0, 1.0, 2.0])
    assert exc_info.value.violation.id.value == "distinctness"


def test_quantile_rejects_a_cumulance_outside_the_unit_interval():
    model = NeighborDistModel(X)
    with pytest.raises(AssumptionError) as exc_info:
        model.quantile([-0.1])
    assert exc_info.value.violation.id.value == "cumulance"


def test_an_empty_query_vector_is_not_a_violation():
    model = NeighborDistModel(X)
    assert model.quantile([]) == []
    assert model.cumulative([]) == []
    assert model.density([]) == []


def test_a_broken_assume_sorted_promise_terminates():
    # prepare()'s distinctness check only looks at ADJACENT elements of the (unsorted, since
    # assume_sorted is a promise it trusts) list, so a non-adjacent repeat -- x[2] and x[6] both
    # 5.0 here -- slips through it. That repeat then makes one window's span exactly zero while
    # its cell keeps a nonzero width, so the cell's unnormalized mass divides by zero.
    #
    # The contract only promises termination on a broken promise, not a specific exception: the
    # other six ports divide under IEEE 754 and reach the RuntimeError-raising finite/positive
    # check on an infinite total, but Python raises ZeroDivisionError at the division itself
    # rather than producing inf. Both are compliant terminations of the same undefined behavior.
    scrambled = [0.0, 2.0, 5.0, 8.0, 10.0, 12.0, 5.0, 20.0, 25.0, 30.0]
    with pytest.raises(ZeroDivisionError):
        NeighborDistModel(scrambled, assume_sorted=True)


def test_sample_constructor_takes_the_samples_unit():
    meter = MeasurementUnit("meter", "Length", "m", "Meter", 1)
    model = NeighborDistModel(Sample(X, unit=meter))
    assert model.unit == meter


def test_measured_density_returns_a_plain_number():
    meter = MeasurementUnit("meter", "Length", "m", "Meter", 1)
    model = NeighborDistModel(Sample(X, unit=meter))
    result = model.measured.density([Measurement(3.0, meter)])
    assert result[0] == model.density_at(3.0)
