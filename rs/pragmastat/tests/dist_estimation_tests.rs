//! Distribution estimation: the two reconstruction models.
//!
//! Every expected value here is computed by hand against the model's own definition, so the
//! checks double as a derivation, not just a pin.

use pragmastat::{
    AssumptionId, Distribution, EstimatorError, LinearDistModel, Measurement, MeasurementUnit,
    NeighborDistModel, Rng, Sample, Subject,
};

const LINEAR_X: [f64; 4] = [1.0, 2.0, 4.0, 8.0];
const NEIGHBOR_X: [f64; 5] = [0.0, 1.0, 2.0, 4.0, 8.0];

fn assumption(err: EstimatorError) -> (AssumptionId, Subject) {
    match err {
        EstimatorError::Assumption(a) => {
            let v = a.violation();
            (v.id, v.subject)
        }
        other => panic!("expected an assumption error, got {other}"),
    }
}

fn meter() -> MeasurementUnit {
    MeasurementUnit::new("meter", "Length", "m", "Meter", 1)
}

// ── LinearDistModel ──────────────────────────────────────────────────────────

#[test]
fn linear_quantile_at_lands_on_each_observation_at_its_anchor() {
    let dist = LinearDistModel::new(&LINEAR_X, false).unwrap();
    assert_eq!(dist.quantile_at(0.0), 1.0);
    assert_eq!(dist.quantile_at(1.0 / 3.0), 2.0);
    assert_eq!(dist.quantile_at(2.0 / 3.0), 4.0);
    assert_eq!(dist.quantile_at(1.0), 8.0);
}

#[test]
fn linear_quantile_at_interpolates_between_two_observations() {
    let dist = LinearDistModel::new(&LINEAR_X, false).unwrap();
    assert_eq!(dist.quantile_at(0.5), 3.0);
}

#[test]
fn linear_quantile_at_clamps_outside_the_unit_interval() {
    let dist = LinearDistModel::new(&LINEAR_X, false).unwrap();
    assert_eq!(dist.quantile_at(-0.1), 1.0);
    assert_eq!(dist.quantile_at(1.1), 8.0);
}

#[test]
fn linear_cumulative_at_inverts_quantile_at() {
    let dist = LinearDistModel::new(&LINEAR_X, false).unwrap();
    assert_eq!(dist.cumulative_at(3.0), 0.5);
    assert_eq!(dist.cumulative_at(2.0), 1.0 / 3.0);
    assert_eq!(dist.cumulative_at(4.0), 2.0 / 3.0);
}

#[test]
fn linear_cumulative_at_is_zero_below_and_one_above_the_support() {
    let dist = LinearDistModel::new(&LINEAR_X, false).unwrap();
    assert_eq!(dist.cumulative_at(1.0), 0.0);
    assert_eq!(dist.cumulative_at(-5.0), 0.0);
    assert_eq!(dist.cumulative_at(8.0), 1.0);
    assert_eq!(dist.cumulative_at(20.0), 1.0);
}

#[test]
fn linear_density_at_is_mass_over_width_inside_each_cell() {
    let dist = LinearDistModel::new(&LINEAR_X, false).unwrap();
    assert_eq!(dist.density_at(1.5), 1.0 / 3.0);
    assert_eq!(dist.density_at(3.0), 1.0 / 6.0);
    assert_eq!(dist.density_at(6.0), 1.0 / 12.0);
    // The right edge belongs to the last cell.
    assert_eq!(dist.density_at(8.0), 1.0 / 12.0);
}

#[test]
fn linear_density_at_is_zero_outside_the_support() {
    let dist = LinearDistModel::new(&LINEAR_X, false).unwrap();
    assert_eq!(dist.density_at(0.5), 0.0);
    assert_eq!(dist.density_at(9.0), 0.0);
}

#[test]
fn linear_scalar_queries_propagate_nan() {
    let dist = LinearDistModel::new(&LINEAR_X, false).unwrap();
    assert!(dist.quantile_at(f64::NAN).is_nan());
    assert!(dist.cumulative_at(f64::NAN).is_nan());
    assert!(dist.density_at(f64::NAN).is_nan());
}

#[test]
fn linear_landscape_agrees_with_the_scalar_queries_bit_for_bit() {
    let dist = LinearDistModel::new(&LINEAR_X, false).unwrap();
    let landscape = dist.get_landscape();

    assert_eq!(landscape.boundaries(), LINEAR_X);
    assert_eq!(landscape.cell_count(), 3);
    for (i, &x) in LINEAR_X.iter().enumerate() {
        assert_eq!(dist.cumulative_at(x), landscape.cumulative()[i]);
    }
    for j in 0..landscape.cell_count() {
        let midpoint = (LINEAR_X[j] + LINEAR_X[j + 1]) / 2.0;
        assert_eq!(dist.density_at(midpoint), landscape.densities()[j]);
    }
}

#[test]
fn linear_sample_is_quantile_at_of_a_raw_drawn_cumulance() {
    let dist = LinearDistModel::new(&LINEAR_X, false).unwrap();
    let mut drawing = Rng::from_string("identity-seed");
    let mut inverting = Rng::from_string("identity-seed");
    for _ in 0..100 {
        let drawn = dist.sample(&mut drawing);
        let u = inverting.uniform_f64();
        assert_eq!(drawn.to_bits(), dist.quantile_at(u).to_bits());
    }
}

#[test]
fn linear_constructor_rejects_fewer_than_two_observations() {
    let err = LinearDistModel::new(&[1.0], false).unwrap_err();
    assert_eq!(assumption(err), (AssumptionId::Domain, Subject::X));
}

#[test]
fn linear_constructor_rejects_a_repeated_value() {
    let err = LinearDistModel::new(&[1.0, 1.0, 2.0], false).unwrap_err();
    assert_eq!(assumption(err), (AssumptionId::Distinctness, Subject::X));
}

#[test]
fn linear_quantile_rejects_a_cumulance_outside_the_unit_interval() {
    let dist = LinearDistModel::new(&LINEAR_X, false).unwrap();
    let err = dist.quantile(&[1.5]).unwrap_err();
    assert_eq!(assumption(err), (AssumptionId::Cumulance, Subject::Y));
}

#[test]
fn linear_an_empty_query_vector_is_not_a_violation() {
    let dist = LinearDistModel::new(&LINEAR_X, false).unwrap();
    assert_eq!(dist.quantile(&[]).unwrap().len(), 0);
    assert_eq!(dist.cumulative(&[]).unwrap().len(), 0);
    assert_eq!(dist.density(&[]).unwrap().len(), 0);
}

#[test]
fn linear_sample_constructor_takes_the_samples_unit() {
    let unit = meter();
    let sample = Sample::with_unit(LINEAR_X.to_vec(), unit.clone()).unwrap();
    let dist = LinearDistModel::from_sample(&sample).unwrap();
    assert_eq!(*dist.unit(), unit);
}

#[test]
fn linear_sample_constructor_refuses_a_weighted_sample() {
    let sample =
        Sample::weighted(vec![1.0, 2.0], vec![1.0, 2.0], MeasurementUnit::number()).unwrap();
    assert!(LinearDistModel::from_sample(&sample).is_err());
}

#[test]
fn linear_measured_quantile_returns_a_measurement_in_the_models_unit() {
    let unit = meter();
    let sample = Sample::with_unit(LINEAR_X.to_vec(), unit.clone()).unwrap();
    let dist = LinearDistModel::from_sample(&sample).unwrap();
    let result = dist.measured().quantile(&[0.5]).unwrap();
    assert_eq!(result[0].value, 3.0);
    assert_eq!(result[0].unit, unit);
}

#[test]
fn linear_measured_density_returns_a_plain_number() {
    let unit = meter();
    let sample = Sample::with_unit(LINEAR_X.to_vec(), unit.clone()).unwrap();
    let dist = LinearDistModel::from_sample(&sample).unwrap();
    let result = dist
        .measured()
        .density(&[Measurement::new(3.0, unit.clone())])
        .unwrap();
    assert_eq!(result[0], 1.0 / 6.0);
}

// ── NeighborDistModel ────────────────────────────────────────────────────────

#[test]
fn neighbor_support_matches_the_min_and_max_observation() {
    let dist = NeighborDistModel::new(&NEIGHBOR_X, false).unwrap();
    assert_eq!(dist.support_min(), 0.0);
    assert_eq!(dist.support_max(), 8.0);
}

#[test]
fn neighbor_cell_count_is_at_most_one_fewer_than_the_observation_count() {
    let dist = NeighborDistModel::new(&NEIGHBOR_X, false).unwrap();
    assert_eq!(dist.cell_count(), NEIGHBOR_X.len());
}

#[test]
fn neighbor_cumulative_at_the_support_is_zero_and_one_exactly() {
    let dist = NeighborDistModel::new(&NEIGHBOR_X, false).unwrap();
    assert_eq!(dist.cumulative_at(dist.support_min()), 0.0);
    assert_eq!(dist.cumulative_at(dist.support_max()), 1.0);
}

#[test]
fn neighbor_cell_masses_sum_to_exactly_one() {
    let dist = NeighborDistModel::new(&NEIGHBOR_X, false).unwrap();
    let landscape = dist.get_landscape();
    let sum: f64 = landscape.masses().iter().sum();
    assert_eq!(sum, 1.0);
}

#[test]
fn neighbor_cumulative_at_is_non_decreasing_across_the_boundaries() {
    let dist = NeighborDistModel::new(&NEIGHBOR_X, false).unwrap();
    let cumulative = dist.get_landscape().cumulative().to_vec();
    for i in 1..cumulative.len() {
        assert!(cumulative[i] > cumulative[i - 1]);
    }
}

#[test]
fn neighbor_quantile_at_inverts_cumulative_at_at_each_boundary() {
    let dist = NeighborDistModel::new(&NEIGHBOR_X, false).unwrap();
    let landscape = dist.get_landscape();
    for i in 0..landscape.boundaries().len() {
        assert_eq!(
            dist.quantile_at(landscape.cumulative()[i]),
            landscape.boundaries()[i]
        );
    }
}

#[test]
fn neighbor_quantile_at_clamps_outside_the_unit_interval() {
    let dist = NeighborDistModel::new(&NEIGHBOR_X, false).unwrap();
    assert_eq!(dist.quantile_at(-0.1), dist.support_min());
    assert_eq!(dist.quantile_at(1.1), dist.support_max());
}

#[test]
fn neighbor_density_at_is_zero_outside_the_support() {
    let dist = NeighborDistModel::new(&NEIGHBOR_X, false).unwrap();
    assert_eq!(dist.density_at(dist.support_min() - 1.0), 0.0);
    assert_eq!(dist.density_at(dist.support_max() + 1.0), 0.0);
}

#[test]
fn neighbor_scalar_queries_propagate_nan() {
    let dist = NeighborDistModel::new(&NEIGHBOR_X, false).unwrap();
    assert!(dist.quantile_at(f64::NAN).is_nan());
    assert!(dist.cumulative_at(f64::NAN).is_nan());
    assert!(dist.density_at(f64::NAN).is_nan());
}

#[test]
fn neighbor_landscape_agrees_with_the_scalar_queries_bit_for_bit() {
    let dist = NeighborDistModel::new(&NEIGHBOR_X, false).unwrap();
    let landscape = dist.get_landscape();
    for i in 0..landscape.boundaries().len() {
        assert_eq!(
            dist.cumulative_at(landscape.boundaries()[i]),
            landscape.cumulative()[i]
        );
    }
    for j in 0..landscape.cell_count() {
        let midpoint = (landscape.boundaries()[j] + landscape.boundaries()[j + 1]) / 2.0;
        assert_eq!(dist.density_at(midpoint), landscape.densities()[j]);
    }
}

#[test]
fn neighbor_landscape_exposes_scale_index_and_radius() {
    let dist = NeighborDistModel::new(&NEIGHBOR_X, false).unwrap();
    let landscape = dist.get_landscape();
    // n = 5: ceil_sqrt(5) = 3 (smallest k with k >= ceil(5/k)), radius = 3 / 2 = 1.
    assert_eq!(landscape.scale_index(), 3);
    assert_eq!(landscape.radius(), 1);
}

#[test]
fn neighbor_landscape_sources_map_each_cell_to_its_observation() {
    let dist = NeighborDistModel::new(&NEIGHBOR_X, false).unwrap();
    assert_eq!(dist.get_landscape().sources(), [0, 1, 2, 3, 4]);
}

#[test]
fn neighbor_a_zero_width_cell_from_a_collapsed_midpoint_is_dropped() {
    // The center observation sits exactly one ULP from each neighbor on either side. Both
    // adjacent midpoints round to 1.0 itself under round-to-nearest-even (each is exactly
    // halfway between two representable doubles, and 1.0 has the even mantissa), so the center
    // cell's two boundaries collapse onto the same point and the cell is dropped.
    let center = 1.0f64;
    let below = f64::from_bits(center.to_bits() - 1);
    let above = f64::from_bits(center.to_bits() + 1);
    let values = [0.0, below, center, above, 10.0];

    let dist = NeighborDistModel::new(&values, false).unwrap();
    let landscape = dist.get_landscape();

    assert_eq!(landscape.cell_count(), values.len() - 1);
    assert!(!landscape.sources().contains(&2));
}

#[test]
fn neighbor_sample_is_quantile_at_of_a_raw_drawn_cumulance() {
    let dist = NeighborDistModel::new(&NEIGHBOR_X, false).unwrap();
    let mut drawing = Rng::from_string("identity-seed");
    let mut inverting = Rng::from_string("identity-seed");
    for _ in 0..100 {
        let drawn = dist.sample(&mut drawing);
        let u = inverting.uniform_f64();
        assert_eq!(drawn.to_bits(), dist.quantile_at(u).to_bits());
    }
}

#[test]
fn neighbor_constructor_rejects_fewer_than_two_observations() {
    let err = NeighborDistModel::new(&[1.0], false).unwrap_err();
    assert_eq!(assumption(err), (AssumptionId::Domain, Subject::X));
}

#[test]
fn neighbor_constructor_rejects_a_repeated_value() {
    let err = NeighborDistModel::new(&[1.0, 1.0, 2.0], false).unwrap_err();
    assert_eq!(assumption(err), (AssumptionId::Distinctness, Subject::X));
}

#[test]
fn neighbor_quantile_rejects_a_cumulance_outside_the_unit_interval() {
    let dist = NeighborDistModel::new(&NEIGHBOR_X, false).unwrap();
    let err = dist.quantile(&[-0.1]).unwrap_err();
    assert_eq!(assumption(err), (AssumptionId::Cumulance, Subject::Y));
}

#[test]
fn neighbor_an_empty_query_vector_is_not_a_violation() {
    let dist = NeighborDistModel::new(&NEIGHBOR_X, false).unwrap();
    assert_eq!(dist.quantile(&[]).unwrap().len(), 0);
    assert_eq!(dist.cumulative(&[]).unwrap().len(), 0);
    assert_eq!(dist.density(&[]).unwrap().len(), 0);
}

#[test]
#[should_panic(expected = "numerical conformance failure")]
fn neighbor_a_broken_assume_sorted_promise_terminates() {
    // prepare_dist_sample's distinctness check only looks at ADJACENT elements of the
    // (unsorted, since assume_sorted is a promise it trusts) slice, so a non-adjacent repeat --
    // x[2] and x[6] both 5.0 here -- slips through it. That repeat then makes one window's span
    // exactly zero while its cell keeps a nonzero width, so the cell's unnormalized mass divides
    // by zero and the total the whole reconstruction divides by is not a usable number.
    let scrambled = [0.0, 2.0, 5.0, 8.0, 10.0, 12.0, 5.0, 20.0, 25.0, 30.0];
    let _ = NeighborDistModel::new(&scrambled, true);
}

#[test]
fn neighbor_sample_constructor_takes_the_samples_unit() {
    let unit = meter();
    let sample = Sample::with_unit(NEIGHBOR_X.to_vec(), unit.clone()).unwrap();
    let dist = NeighborDistModel::from_sample(&sample).unwrap();
    assert_eq!(*dist.unit(), unit);
}

#[test]
fn neighbor_measured_density_returns_a_plain_number() {
    let unit = meter();
    let sample = Sample::with_unit(NEIGHBOR_X.to_vec(), unit.clone()).unwrap();
    let dist = NeighborDistModel::from_sample(&sample).unwrap();
    let result = dist
        .measured()
        .density(&[Measurement::new(3.0, unit.clone())])
        .unwrap();
    assert_eq!(result[0], dist.density_at(3.0));
}
