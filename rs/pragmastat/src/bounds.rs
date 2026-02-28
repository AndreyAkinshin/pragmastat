//! Bounds: an interval [lower, upper] paired with a measurement unit.

use crate::measurement_unit::{MeasurementUnit, NumberUnit};
use std::fmt;

/// An interval with lower and upper bounds and an associated measurement unit.
#[derive(Debug, Clone)]
pub struct Bounds {
    pub lower: f64,
    pub upper: f64,
    pub unit: Box<dyn MeasurementUnit>,
}

impl Bounds {
    /// Creates new bounds with the given lower, upper, and unit.
    pub fn new(lower: f64, upper: f64, unit: Box<dyn MeasurementUnit>) -> Self {
        Self { lower, upper, unit }
    }

    /// Creates new bounds with the default [`NumberUnit`].
    pub fn unitless(lower: f64, upper: f64) -> Self {
        Self {
            lower,
            upper,
            unit: Box::new(NumberUnit),
        }
    }

    /// Returns true if `value` is within [lower, upper].
    pub fn contains(&self, value: f64) -> bool {
        self.lower <= value && value <= self.upper
    }
}

impl fmt::Display for Bounds {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let abbr = self.unit.abbreviation();
        if abbr.is_empty() {
            write!(f, "[{};{}]", self.lower, self.upper)
        } else {
            write!(f, "[{};{}] {}", self.lower, self.upper, abbr)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn contains_inside() {
        let b = Bounds::unitless(1.0, 5.0);
        assert!(b.contains(3.0));
    }

    #[test]
    fn contains_boundary() {
        let b = Bounds::unitless(1.0, 5.0);
        assert!(b.contains(1.0));
        assert!(b.contains(5.0));
    }

    #[test]
    fn contains_outside() {
        let b = Bounds::unitless(1.0, 5.0);
        assert!(!b.contains(0.99));
        assert!(!b.contains(5.01));
    }

    #[test]
    fn display_unitless() {
        let b = Bounds::unitless(1.0, 5.0);
        assert_eq!(format!("{b}"), "[1;5]");
    }

    #[test]
    fn display_with_unit() {
        use crate::measurement_unit::CustomUnit;
        let unit = CustomUnit::new("ms", "Time", "ms", "Millisecond", 1_000_000);
        let b = Bounds::new(1.0, 5.0, Box::new(unit));
        assert_eq!(format!("{b}"), "[1;5] ms");
    }
}
