//! Minimum achievable misrate functions

use crate::assumptions::{AssumptionError, AssumptionId, Subject, Violation};
use crate::binomial::binomial_coefficient;

/// Computes the minimum achievable misrate for one-sample signed-rank based bounds.
/// Returns 2^(1-n) which is the smallest possible misrate for a sample of size n.
pub fn min_achievable_misrate_one_sample(n: usize) -> Result<f64, AssumptionError> {
    if n == 0 {
        return Err(AssumptionError::new(Violation::new(
            AssumptionId::Domain,
            Subject::X,
        )));
    }
    Ok(0.5_f64.powi((n - 1) as i32))
}

/// Computes the minimum achievable misrate for two-sample Mann-Whitney based bounds.
pub fn min_achievable_misrate_two_sample(n: usize, m: usize) -> Result<f64, AssumptionError> {
    if n == 0 {
        return Err(AssumptionError::domain(Subject::X));
    }
    if m == 0 {
        return Err(AssumptionError::domain(Subject::Y));
    }
    // The same entry point the exact Loeffler pass uses for its own binomial: at this
    // floor the two are compared against each other, so they have to be the same bits.
    Ok(2.0 / binomial_coefficient(n + m, n))
}
