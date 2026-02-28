//! Unit registry for looking up measurement units by ID.

use crate::measurement_unit::{DisparityUnit, MeasurementUnit, NumberUnit, RatioUnit};
use std::collections::HashMap;

/// Stores measurement units and enables lookup by ID.
pub struct UnitRegistry {
    units: HashMap<String, Box<dyn MeasurementUnit>>,
}

impl UnitRegistry {
    /// Creates an empty registry.
    pub fn new() -> Self {
        Self {
            units: HashMap::new(),
        }
    }

    /// Registers a unit. Returns an error if a unit with the same ID already exists.
    pub fn register(&mut self, unit: Box<dyn MeasurementUnit>) -> Result<(), String> {
        let id = unit.id().to_string();
        if self.units.contains_key(&id) {
            return Err(format!("unit with id '{id}' is already registered"));
        }
        self.units.insert(id, unit);
        Ok(())
    }

    /// Looks up a unit by ID.
    pub fn resolve(&self, id: &str) -> Result<&dyn MeasurementUnit, String> {
        self.units
            .get(id)
            .map(|u| u.as_ref())
            .ok_or_else(|| format!("unknown unit id: '{id}'"))
    }

    /// Returns a registry pre-populated with the standard units
    /// ([`NumberUnit`], [`RatioUnit`], [`DisparityUnit`]).
    pub fn standard() -> Self {
        let mut r = Self::new();
        // Standard units are guaranteed unique; unwrap is safe.
        r.register(Box::new(NumberUnit)).unwrap();
        r.register(Box::new(RatioUnit)).unwrap();
        r.register(Box::new(DisparityUnit)).unwrap();
        r
    }
}

impl Default for UnitRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::measurement_unit::CustomUnit;

    #[test]
    fn register_and_resolve() {
        let mut r = UnitRegistry::new();
        r.register(Box::new(NumberUnit)).unwrap();
        let unit = r.resolve("number").unwrap();
        assert_eq!(unit.id(), "number");
        assert_eq!(unit.family(), "Number");
    }

    #[test]
    fn duplicate_registration_fails() {
        let mut r = UnitRegistry::new();
        r.register(Box::new(NumberUnit)).unwrap();
        assert!(r.register(Box::new(NumberUnit)).is_err());
    }

    #[test]
    fn resolve_unknown_fails() {
        let r = UnitRegistry::new();
        assert!(r.resolve("nonexistent").is_err());
    }

    #[test]
    fn standard_contains_all_three() {
        let r = UnitRegistry::standard();
        assert!(r.resolve("number").is_ok());
        assert!(r.resolve("ratio").is_ok());
        assert!(r.resolve("disparity").is_ok());
    }

    #[test]
    fn custom_unit_registration() {
        let mut r = UnitRegistry::standard();
        let ms = CustomUnit::new("ms", "Time", "ms", "Millisecond", 1_000_000);
        r.register(Box::new(ms)).unwrap();
        let unit = r.resolve("ms").unwrap();
        assert_eq!(unit.family(), "Time");
        assert_eq!(unit.abbreviation(), "ms");
    }
}
