//! Measurement unit system for the metrology layer.
//!
//! Provides the [`MeasurementUnit`] trait and standard unit implementations
//! ([`NumberUnit`], [`RatioUnit`], [`DisparityUnit`]) plus a [`CustomUnit`]
//! for user-defined units.

use std::fmt;

/// A unit of measurement with identity, family membership, and conversion support.
///
/// Units in the same family are compatible and can be converted between each other.
/// The `base_units` value determines the conversion factor relative to the family's
/// base unit: `conversion_factor(from, to) = from.base_units / to.base_units`.
pub trait MeasurementUnit: fmt::Debug + fmt::Display + Send + Sync {
    /// Unique identifier for this unit (e.g., "number", "ratio", "ms").
    fn id(&self) -> &str;

    /// Family this unit belongs to (e.g., "Number", "Time").
    /// Units in the same family are compatible.
    fn family(&self) -> &str;

    /// Short display abbreviation (e.g., "ms", "ns"). Empty for dimensionless units.
    fn abbreviation(&self) -> &str;

    /// Human-readable full name (e.g., "Millisecond", "Number").
    fn full_name(&self) -> &str;

    /// Number of base units this unit represents. Used for conversion:
    /// `conversion_factor(from, to) = from.base_units / to.base_units`.
    fn base_units(&self) -> i64;

    /// Clone this unit into a boxed trait object.
    fn clone_box(&self) -> Box<dyn MeasurementUnit>;
}

impl Clone for Box<dyn MeasurementUnit> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}

/// Returns true if both units belong to the same family.
pub fn is_compatible(a: &dyn MeasurementUnit, b: &dyn MeasurementUnit) -> bool {
    a.family() == b.family()
}

/// Returns the unit with smaller `base_units` (higher precision / finer granularity).
pub fn finer<'a>(
    a: &'a dyn MeasurementUnit,
    b: &'a dyn MeasurementUnit,
) -> &'a dyn MeasurementUnit {
    if a.base_units() <= b.base_units() {
        a
    } else {
        b
    }
}

/// Returns the multiplier to convert a value from one unit to another.
///
/// `converted_value = original_value * conversion_factor(from, to)`
pub fn conversion_factor(from: &dyn MeasurementUnit, to: &dyn MeasurementUnit) -> f64 {
    from.base_units() as f64 / to.base_units() as f64
}

// =============================================================================
// Standard units
// =============================================================================

/// Dimensionless numeric unit. Default unit for raw numeric samples.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NumberUnit;

impl MeasurementUnit for NumberUnit {
    fn id(&self) -> &str {
        "number"
    }
    fn family(&self) -> &str {
        "Number"
    }
    fn abbreviation(&self) -> &str {
        ""
    }
    fn full_name(&self) -> &str {
        "Number"
    }
    fn base_units(&self) -> i64 {
        1
    }
    fn clone_box(&self) -> Box<dyn MeasurementUnit> {
        Box::new(*self)
    }
}

impl fmt::Display for NumberUnit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.abbreviation())
    }
}

/// Dimensionless ratio unit. Used for ratio estimator results.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RatioUnit;

impl MeasurementUnit for RatioUnit {
    fn id(&self) -> &str {
        "ratio"
    }
    fn family(&self) -> &str {
        "Ratio"
    }
    fn abbreviation(&self) -> &str {
        ""
    }
    fn full_name(&self) -> &str {
        "Ratio"
    }
    fn base_units(&self) -> i64 {
        1
    }
    fn clone_box(&self) -> Box<dyn MeasurementUnit> {
        Box::new(*self)
    }
}

impl fmt::Display for RatioUnit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.abbreviation())
    }
}

/// Dimensionless disparity unit. Used for disparity estimator results.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DisparityUnit;

impl MeasurementUnit for DisparityUnit {
    fn id(&self) -> &str {
        "disparity"
    }
    fn family(&self) -> &str {
        "Disparity"
    }
    fn abbreviation(&self) -> &str {
        ""
    }
    fn full_name(&self) -> &str {
        "Disparity"
    }
    fn base_units(&self) -> i64 {
        1
    }
    fn clone_box(&self) -> Box<dyn MeasurementUnit> {
        Box::new(*self)
    }
}

impl fmt::Display for DisparityUnit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.abbreviation())
    }
}

// =============================================================================
// Custom unit
// =============================================================================

/// A user-defined measurement unit.
///
/// Use this for domain-specific units (e.g., milliseconds, nanoseconds)
/// that are not covered by the standard units.
#[derive(Debug, Clone)]
pub struct CustomUnit {
    id: String,
    family: String,
    abbreviation: String,
    full_name: String,
    base_units: i64,
}

impl CustomUnit {
    /// Creates a new custom unit.
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
}

impl MeasurementUnit for CustomUnit {
    fn id(&self) -> &str {
        &self.id
    }
    fn family(&self) -> &str {
        &self.family
    }
    fn abbreviation(&self) -> &str {
        &self.abbreviation
    }
    fn full_name(&self) -> &str {
        &self.full_name
    }
    fn base_units(&self) -> i64 {
        self.base_units
    }
    fn clone_box(&self) -> Box<dyn MeasurementUnit> {
        Box::new(self.clone())
    }
}

impl fmt::Display for CustomUnit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.abbreviation)
    }
}

/// Error returned when incompatible units are used together.
#[derive(Debug)]
pub struct UnitMismatchError {
    pub unit1_name: String,
    pub unit2_name: String,
}

impl UnitMismatchError {
    pub fn new(a: &dyn MeasurementUnit, b: &dyn MeasurementUnit) -> Self {
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
        assert_eq!(NumberUnit.id(), "number");
        assert_eq!(RatioUnit.id(), "ratio");
        assert_eq!(DisparityUnit.id(), "disparity");
    }

    #[test]
    fn compatible_same_family() {
        let ms = CustomUnit::new("ms", "Time", "ms", "Millisecond", 1_000_000);
        let ns = CustomUnit::new("ns", "Time", "ns", "Nanosecond", 1);
        assert!(is_compatible(&ms, &ns));
    }

    #[test]
    fn incompatible_different_family() {
        assert!(!is_compatible(&NumberUnit, &RatioUnit));
    }

    #[test]
    fn finer_selects_smaller_base() {
        let ms = CustomUnit::new("ms", "Time", "ms", "Millisecond", 1_000_000);
        let ns = CustomUnit::new("ns", "Time", "ns", "Nanosecond", 1);
        assert_eq!(finer(&ms, &ns).id(), "ns");
        assert_eq!(finer(&ns, &ms).id(), "ns");
    }

    #[test]
    fn conversion_factor_ms_to_ns() {
        let ms = CustomUnit::new("ms", "Time", "ms", "Millisecond", 1_000_000);
        let ns = CustomUnit::new("ns", "Time", "ns", "Nanosecond", 1);
        let factor = conversion_factor(&ms, &ns);
        assert!((factor - 1_000_000.0).abs() < 1e-9);
    }

    #[test]
    fn conversion_factor_same_unit() {
        let factor = conversion_factor(&NumberUnit, &NumberUnit);
        assert!((factor - 1.0).abs() < 1e-9);
    }

    #[test]
    fn clone_box_preserves_identity() {
        let unit: Box<dyn MeasurementUnit> = Box::new(NumberUnit);
        let cloned = unit.clone_box();
        assert_eq!(cloned.id(), "number");
        assert_eq!(cloned.family(), "Number");
    }

    #[test]
    fn custom_unit_fields() {
        let unit = CustomUnit::new("sec", "Time", "s", "Second", 1_000_000_000);
        assert_eq!(unit.id(), "sec");
        assert_eq!(unit.family(), "Time");
        assert_eq!(unit.abbreviation(), "s");
        assert_eq!(unit.full_name(), "Second");
        assert_eq!(unit.base_units(), 1_000_000_000);
    }
}
