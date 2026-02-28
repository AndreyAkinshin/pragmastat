//! Assumption validation framework for Pragmastat estimators.
//!
//! This module defines the assumption system that governs which inputs are valid
//! for each estimator. Assumptions are domain constraints, not statistical models.
//!
//! # Assumption IDs (canonical priority order)
//!
//! 1. `Validity` - non-empty input with finite defined real values
//! 2. `Domain` - parameter is outside its valid domain
//! 3. `Positivity` - values must be strictly positive
//! 4. `Sparity` - sample must be non tie-dominant (Spread > 0)
//!
//! When multiple assumptions are violated, the violation with highest priority
//! is reported. For two-sample functions, subject `X` is checked before `Y`.

use std::fmt;

/// Assumption identifiers in canonical priority order.
///
/// Lower discriminant values indicate higher priority.
/// When multiple assumptions are violated, report the highest priority violation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum AssumptionId {
    /// Sample must be non-empty with finite, defined real values.
    /// This is the implicit assumption for all functions.
    Validity = 0,
    /// Parameter is outside its valid domain.
    Domain = 1,
    /// All values must be strictly positive (> 0).
    Positivity = 2,
    /// Sample must be non tie-dominant: Spread(x) > 0.
    Sparity = 3,
}

impl AssumptionId {
    /// Returns the string identifier for this assumption.
    pub fn as_str(&self) -> &'static str {
        match self {
            AssumptionId::Validity => "validity",
            AssumptionId::Positivity => "positivity",
            AssumptionId::Sparity => "sparity",
            AssumptionId::Domain => "domain",
        }
    }
}

impl fmt::Display for AssumptionId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// Subject identifier for two-sample functions.
///
/// For two-sample functions, violations are checked in order: X before Y.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Subject {
    /// The first sample (x).
    X,
    /// The second sample (y).
    Y,
    /// The misrate parameter.
    Misrate,
}

impl Subject {
    /// Returns the string identifier for this subject.
    pub fn as_str(&self) -> &'static str {
        match self {
            Subject::X => "x",
            Subject::Y => "y",
            Subject::Misrate => "misrate",
        }
    }
}

impl fmt::Display for Subject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// Represents a specific assumption violation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Violation {
    /// The assumption that was violated.
    pub id: AssumptionId,
    /// The sample that caused the violation.
    pub subject: Subject,
}

impl Violation {
    /// Creates a new violation for the given assumption and subject.
    pub fn new(id: AssumptionId, subject: Subject) -> Self {
        Self { id, subject }
    }
}

impl fmt::Display for Violation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}({})", self.id, self.subject)
    }
}

/// Error type for assumption violations.
#[derive(Debug, Clone)]
pub struct AssumptionError {
    violation: Violation,
}

impl AssumptionError {
    /// Creates a new assumption error.
    pub fn new(violation: Violation) -> Self {
        Self { violation }
    }

    /// Creates an error for the `validity` assumption.
    pub fn validity(subject: Subject) -> Self {
        Self::new(Violation::new(AssumptionId::Validity, subject))
    }

    /// Creates an error for the `positivity` assumption.
    pub fn positivity(subject: Subject) -> Self {
        Self::new(Violation::new(AssumptionId::Positivity, subject))
    }

    /// Creates an error for the `sparity` assumption.
    pub fn sparity(subject: Subject) -> Self {
        Self::new(Violation::new(AssumptionId::Sparity, subject))
    }

    /// Creates an error for a domain violation (e.g., misrate below minimum).
    pub fn domain(subject: Subject) -> Self {
        Self::new(Violation::new(AssumptionId::Domain, subject))
    }

    /// Returns the violation contained in this error.
    pub fn violation(&self) -> Violation {
        self.violation
    }
}

impl fmt::Display for AssumptionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.violation)
    }
}

impl std::error::Error for AssumptionError {}

/// Unified error type for estimator functions.
#[derive(Debug, Clone)]
pub enum EstimatorError {
    /// An assumption violation occurred.
    Assumption(AssumptionError),
    /// A generic error (e.g., parameter out of range).
    Other(String),
}

impl fmt::Display for EstimatorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EstimatorError::Assumption(e) => write!(f, "{}", e),
            EstimatorError::Other(msg) => write!(f, "{}", msg),
        }
    }
}

impl std::error::Error for EstimatorError {}

impl From<AssumptionError> for EstimatorError {
    fn from(e: AssumptionError) -> Self {
        EstimatorError::Assumption(e)
    }
}

impl From<&str> for EstimatorError {
    fn from(msg: &str) -> Self {
        EstimatorError::Other(msg.to_string())
    }
}

// =============================================================================
// Validation helpers
// =============================================================================

/// Checks that a sample is valid (non-empty with finite values).
pub fn check_validity(values: &[f64], subject: Subject) -> Result<(), AssumptionError> {
    if values.is_empty() {
        return Err(AssumptionError::validity(subject));
    }
    if values.iter().any(|v| !v.is_finite()) {
        return Err(AssumptionError::validity(subject));
    }
    Ok(())
}

/// Checks that all values are strictly positive.
pub fn check_positivity(values: &[f64], subject: Subject) -> Result<(), AssumptionError> {
    if values.iter().any(|&v| v <= 0.0) {
        return Err(AssumptionError::positivity(subject));
    }
    Ok(())
}

/// Log-transforms a slice. Returns error if any value is non-positive.
pub fn log(values: &[f64], subject: Subject) -> Result<Vec<f64>, AssumptionError> {
    let mut result = Vec::with_capacity(values.len());
    for &v in values {
        if v <= 0.0 {
            return Err(AssumptionError::positivity(subject));
        }
        result.push(v.ln());
    }
    Ok(result)
}
