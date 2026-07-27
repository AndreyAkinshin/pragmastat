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
    // Repeated halving rather than a power function: this is a power of two, and scaling by an exponent is exact in binary64. A general
    // power function returns the same value in every implementation anyone ships, but the
    // specification does not require it to, and this value is a domain boundary: it decides
    // which misrates the toolkit accepts at all.
    // Rust has no scalb in std, and each halving is exact, so this is the honest form.
    let mut v = 1.0_f64;
    for _ in 1..n {
        v /= 2.0;
    }
    Ok(v)
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
