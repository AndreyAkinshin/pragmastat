//! Shared sample preparation for the distribution estimation models.

use crate::assumptions::{AssumptionError, EstimatorError, Subject, check_distinctness};
use crate::measurement::Measurement;
use crate::measurement_unit::{MeasurementUnit, UnitMismatchError, conversion_factor};

/// Runs the two assumptions that follow validity in the canonical order: `domain(x)` (at least
/// two observations and a finite representable range) and `distinctness(x)` (no repeated value;
/// `-0.0` and `0.0` count as repeated).
///
/// The caller's slice is never retained: the model always owns a private copy.
pub(crate) fn prepare_dist_sample(
    values: &[f64],
    assume_sorted: bool,
) -> Result<Vec<f64>, AssumptionError> {
    let mut x = values.to_vec();
    if !assume_sorted {
        x.sort_unstable_by(|a, b| a.total_cmp(b));
    }

    let n = x.len();
    if n < 2 {
        return Err(AssumptionError::domain(Subject::X));
    }

    let range = x[n - 1] - x[0];
    if !range.is_finite() {
        return Err(AssumptionError::domain(Subject::X));
    }

    check_distinctness(&x, Subject::X)?;

    Ok(x)
}

/// Attaches a unit to every element of a query result.
pub(crate) fn to_measurements(values: &[f64], unit: &MeasurementUnit) -> Vec<Measurement> {
    values
        .iter()
        .map(|&v| Measurement::new(v, unit.clone()))
        .collect()
}

/// Converts a measured query vector into the model's own unit.
///
/// A query in a compatible but different unit is converted rather than rejected, matching the
/// two-sample estimators, which reconcile their inputs instead of demanding an exact match. An
/// incompatible unit is a caller error and is reported as one.
pub(crate) fn to_raw_values(
    values: &[Measurement],
    unit: &MeasurementUnit,
) -> Result<Vec<f64>, EstimatorError> {
    let mut raw = Vec::with_capacity(values.len());
    for m in values {
        if !m.unit.is_compatible(unit) {
            return Err(UnitMismatchError::new(&m.unit, unit).into());
        }
        raw.push(m.value * conversion_factor(&m.unit, unit));
    }
    Ok(raw)
}
