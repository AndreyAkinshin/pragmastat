//! Measurement: a value paired with a unit.

use crate::measurement_unit::{MeasurementUnit, NumberUnit};
use std::fmt;

/// A numeric value paired with its measurement unit.
#[derive(Debug, Clone)]
pub struct Measurement {
    pub value: f64,
    pub unit: Box<dyn MeasurementUnit>,
}

impl Measurement {
    /// Creates a new measurement with the given value and unit.
    pub fn new(value: f64, unit: Box<dyn MeasurementUnit>) -> Self {
        Self { value, unit }
    }

    /// Creates a new measurement with the default [`NumberUnit`].
    pub fn unitless(value: f64) -> Self {
        Self {
            value,
            unit: Box::new(NumberUnit),
        }
    }
}

impl From<Measurement> for f64 {
    fn from(m: Measurement) -> f64 {
        m.value
    }
}

impl fmt::Display for Measurement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let abbr = self.unit.abbreviation();
        if abbr.is_empty() {
            write!(f, "{}", self.value)
        } else {
            write!(f, "{} {}", self.value, abbr)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::measurement_unit::CustomUnit;

    #[test]
    fn display_unitless() {
        let m = Measurement::unitless(42.5);
        assert_eq!(format!("{m}"), "42.5");
    }

    #[test]
    fn display_with_abbreviation() {
        let unit = CustomUnit::new("ms", "Time", "ms", "Millisecond", 1_000_000);
        let m = Measurement::new(3.14, Box::new(unit));
        assert_eq!(format!("{m}"), "3.14 ms");
    }

    #[test]
    fn into_f64() {
        let m = Measurement::unitless(2.718);
        let v: f64 = m.into();
        assert!((v - 2.718).abs() < 1e-15);
    }
}
