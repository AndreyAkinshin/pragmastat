//! The measured facade over the distribution estimation models.
//!
//! The metrology path, alongside the numeric one, exactly as every estimator offers both. Only
//! quantile carries a unit -- the sample's own. cumulative answers in probabilities and density
//! is a terminal value read, plotted, or compared against another density of the same model,
//! never converted or combined, so both come back as plain numbers.

use crate::assumptions::EstimatorError;
use crate::dist_sample_guard::{to_measurements, to_raw_values};
use crate::measurement::Measurement;
use crate::measurement_unit::MeasurementUnit;

/// The numeric surface a measured view sits on. Both reconstructions satisfy it, so the view is
/// written once instead of once per model.
pub trait MeasuredModel {
    fn quantile(&self, cumulances: &[f64]) -> Result<Vec<f64>, EstimatorError>;
    fn cumulative(&self, values: &[f64]) -> Result<Vec<f64>, EstimatorError>;
    fn density(&self, values: &[f64]) -> Result<Vec<f64>, EstimatorError>;
    fn unit(&self) -> &MeasurementUnit;
}

/// The measured view of a built reconstruction, obtained from a model's `measured` method.
pub struct MeasuredDist<'a, M: MeasuredModel> {
    model: &'a M,
}

impl<'a, M: MeasuredModel> MeasuredDist<'a, M> {
    pub(crate) fn new(model: &'a M) -> Self {
        Self { model }
    }

    /// Values at the requested cumulances, in the unit of the sample.
    pub fn quantile(&self, cumulances: &[f64]) -> Result<Vec<Measurement>, EstimatorError> {
        let values = self.model.quantile(cumulances)?;
        Ok(to_measurements(&values, self.model.unit()))
    }

    /// Probabilities of not exceeding the requested measurements.
    ///
    /// A query in a compatible but different unit is converted; an incompatible one is rejected
    /// with [`crate::measurement_unit::UnitMismatchError`].
    pub fn cumulative(&self, values: &[Measurement]) -> Result<Vec<f64>, EstimatorError> {
        let raw = to_raw_values(values, self.model.unit())?;
        self.model.cumulative(&raw)
    }

    /// Densities at the requested measurements, as plain numbers.
    ///
    /// A query in a compatible but different unit is converted; an incompatible one is rejected
    /// with [`crate::measurement_unit::UnitMismatchError`].
    pub fn density(&self, values: &[Measurement]) -> Result<Vec<f64>, EstimatorError> {
        let raw = to_raw_values(values, self.model.unit())?;
        self.model.density(&raw)
    }
}
