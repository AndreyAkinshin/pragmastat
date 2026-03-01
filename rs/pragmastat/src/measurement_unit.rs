//! Measurement unit system for the metrology layer.
//!
//! Provides [`MeasurementUnit`] — a concrete type for both standard and
//! user-defined units — plus free functions for compatibility, conversion,
//! and precision comparison.

use std::fmt;

/// A unit of measurement with identity, family membership, and conversion support.
///
/// Units in the same family are compatible and can be converted between each other.
/// The `base_units` value determines the conversion factor relative to the family's
/// base unit: `conversion_factor(from, to) = from.base_units / to.base_units`.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MeasurementUnit {
    id: String,
    family: String,
    abbreviation: String,
    full_name: String,
    base_units: i64,
}

impl MeasurementUnit {
    /// Creates a new measurement unit.
    pub fn new(
        id: impl Into<String>,
        family: impl Into<String>,
        abbreviation: impl Into<String>,
        full_name: impl Into<String>,
        base_units: i64,
    ) -> Self {
        Self {
            id: id.into(),
            family: family.into(),
            abbreviation: abbreviation.into(),
            full_name: full_name.into(),
            base_units,
        }
    }

    /// Unique identifier for this unit (e.g., "number", "ratio", "ms").
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Family this unit belongs to (e.g., "Number", "Time").
    /// Units in the same family are compatible.
    pub fn family(&self) -> &str {
        &self.family
    }

    /// Short display abbreviation (e.g., "ms", "ns"). Empty for dimensionless units.
    pub fn abbreviation(&self) -> &str {
        &self.abbreviation
    }

    /// Human-readable full name (e.g., "Millisecond", "Number").
    pub fn full_name(&self) -> &str {
        &self.full_name
    }

    /// Number of base units this unit represents. Used for conversion:
    /// `conversion_factor(from, to) = from.base_units / to.base_units`.
    pub fn base_units(&self) -> i64 {
        self.base_units
    }

    /// Returns true if this unit is compatible (same family) with `other`.
    pub fn is_compatible(&self, other: &Self) -> bool {
        self.family == other.family
    }

    /// Dimensionless numeric unit. Default unit for raw numeric samples.
    pub fn number() -> Self {
        Self::new("number", "Number", "", "Number", 1)
    }

    /// Dimensionless ratio unit. Used for ratio estimator results.
    pub fn ratio() -> Self {
        Self::new("ratio", "Ratio", "", "Ratio", 1)
    }

    /// Dimensionless disparity unit. Used for disparity estimator results.
    pub fn disparity() -> Self {
        Self::new("disparity", "Disparity", "", "Disparity", 1)
    }
}

impl fmt::Display for MeasurementUnit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.abbreviation)
    }
}

/// Returns true if both units belong to the same family.
pub fn is_compatible(a: &MeasurementUnit, b: &MeasurementUnit) -> bool {
    a.family() == b.family()
}

/// Returns the unit with smaller `base_units` (higher precision / finer granularity).
pub fn finer<'a>(a: &'a MeasurementUnit, b: &'a MeasurementUnit) -> &'a MeasurementUnit {
    if a.base_units() <= b.base_units() {
        a
    } else {
        b
    }
}

/// Returns the multiplier to convert a value from one unit to another.
///
/// `converted_value = original_value * conversion_factor(from, to)`
pub fn conversion_factor(from: &MeasurementUnit, to: &MeasurementUnit) -> f64 {
    from.base_units() as f64 / to.base_units() as f64
}

/// Error returned when incompatible units are used together.
#[derive(Debug)]
pub struct UnitMismatchError {
    pub unit1_name: String,
    pub unit2_name: String,
}

impl UnitMismatchError {
    pub fn new(a: &MeasurementUnit, b: &MeasurementUnit) -> Self {
        Self {
            unit1_name: a.full_name().to_string(),
            unit2_name: b.full_name().to_string(),
        }
    }
}

impl fmt::Display for UnitMismatchError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "can't convert {} to {}",
            self.unit1_name, self.unit2_name
        )
    }
}

impl std::error::Error for UnitMismatchError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn standard_units_id() {
        assert_eq!(MeasurementUnit::number().id(), "number");
        assert_eq!(MeasurementUnit::ratio().id(), "ratio");
        assert_eq!(MeasurementUnit::disparity().id(), "disparity");
    }

    #[test]
    fn compatible_same_family() {
        let ms = MeasurementUnit::new("ms", "Time", "ms", "Millisecond", 1_000_000);
        let ns = MeasurementUnit::new("ns", "Time", "ns", "Nanosecond", 1);
        assert!(is_compatible(&ms, &ns));
    }

    #[test]
    fn incompatible_different_family() {
        assert!(!is_compatible(
            &MeasurementUnit::number(),
            &MeasurementUnit::ratio()
        ));
    }

    #[test]
    fn finer_selects_smaller_base() {
        let ms = MeasurementUnit::new("ms", "Time", "ms", "Millisecond", 1_000_000);
        let ns = MeasurementUnit::new("ns", "Time", "ns", "Nanosecond", 1);
        assert_eq!(finer(&ms, &ns).id(), "ns");
        assert_eq!(finer(&ns, &ms).id(), "ns");
    }

    #[test]
    fn conversion_factor_ms_to_ns() {
        let ms = MeasurementUnit::new("ms", "Time", "ms", "Millisecond", 1_000_000);
        let ns = MeasurementUnit::new("ns", "Time", "ns", "Nanosecond", 1);
        let factor = conversion_factor(&ms, &ns);
        assert!((factor - 1_000_000.0).abs() < 1e-9);
    }

    #[test]
    fn conversion_factor_same_unit() {
        let number = MeasurementUnit::number();
        let factor = conversion_factor(&number, &number);
        assert!((factor - 1.0).abs() < 1e-9);
    }

    #[test]
    fn clone_preserves_identity() {
        let unit = MeasurementUnit::number();
        let cloned = unit.clone();
        assert_eq!(cloned.id(), "number");
        assert_eq!(cloned.family(), "Number");
    }

    #[test]
    fn custom_unit_fields() {
        let unit = MeasurementUnit::new("sec", "Time", "s", "Second", 1_000_000_000);
        assert_eq!(unit.id(), "sec");
        assert_eq!(unit.family(), "Time");
        assert_eq!(unit.abbreviation(), "s");
        assert_eq!(unit.full_name(), "Second");
        assert_eq!(unit.base_units(), 1_000_000_000);
    }
}
