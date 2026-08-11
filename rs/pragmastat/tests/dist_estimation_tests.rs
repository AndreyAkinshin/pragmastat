//! Distribution estimation: the piecewise-linear reconstruction model.
//!
//! Every expected value here is computed by hand against the model's own definition, so the
//! checks double as a derivation, not just a pin.

use pragmastat::{
    AssumptionId, Distribution, EstimatorError, LinearDistModel, Measurement, MeasurementUnit, Rng,
    Sample, Subject,
};

const LINEAR_X: [f64; 4] = [1.0, 2.0, 4.0, 8.0];

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
