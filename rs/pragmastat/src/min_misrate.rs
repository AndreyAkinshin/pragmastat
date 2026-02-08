//! Minimum achievable misrate functions

use crate::assumptions::{AssumptionError, AssumptionId, Subject, Violation};

/// Computes the minimum achievable misrate for one-sample signed-rank based bounds.
/// Returns 2^(1-n) which is the smallest possible misrate for a sample of size n.
pub fn min_achievable_misrate_one_sample(n: usize) -> Result<f64, AssumptionError> {
    if n == 0 {
        return Err(AssumptionError::new(Violation::new(
            AssumptionId::Domain,
            Subject::X,
        )));
    }
    Ok(2.0_f64.powf(1.0 - n as f64))
}

/// Computes the minimum achievable misrate for two-sample Mann-Whitney based bounds.
pub fn min_achievable_misrate_two_sample(n: usize, m: usize) -> Result<f64, AssumptionError> {
    if n == 0 {
        return Err(AssumptionError::domain(Subject::X));
    }
    if m == 0 {
        return Err(AssumptionError::domain(Subject::Y));
    }
    Ok(2.0 / binomial_coefficient(n + m, n))
}

/// Computes binomial coefficient C(n, k) using integer arithmetic.
/// Falls back to logarithmic approximation if overflow is detected.
fn binomial_coefficient(n: usize, k: usize) -> f64 {
    if k > n {
        return 0.0;
    }
    if k == 0 || k == n {
        return 1.0;
    }

    let k = k.min(n - k); // Take advantage of symmetry

    // Try exact computation with overflow detection
    if let Some(result) = binomial_coefficient_exact(n, k) {
        result as f64
    } else {
        // Fall back to logarithmic approximation for large values
        binomial_coefficient_log_approx(n, k)
    }
}

/// Computes binomial coefficient exactly using checked arithmetic.
/// Returns None if overflow is detected.
fn binomial_coefficient_exact(n: usize, k: usize) -> Option<u128> {
    let mut result: u128 = 1;

    for i in 0..k {
        result = result.checked_mul((n - i) as u128)?;
        result /= (i + 1) as u128;
    }

    Some(result)
}

/// Computes binomial coefficient using logarithms for large values.
/// Uses Stirling's approximation for log-gamma.
fn binomial_coefficient_log_approx(n: usize, k: usize) -> f64 {
    let log_result =
        log_gamma(n as f64 + 1.0) - log_gamma(k as f64 + 1.0) - log_gamma((n - k) as f64 + 1.0);
    log_result.exp()
}

/// Computes log(Gamma(x)) using Stirling's approximation with Bernoulli correction.
fn log_gamma(x: f64) -> f64 {
    if x < 1.0 {
        // Use recurrence: Gamma(x) = Gamma(x+1) / x
        return log_gamma(x + 1.0) - x.ln();
    }

    // Stirling's approximation: log(Gamma(x)) ≈ (x - 0.5) * ln(x) - x + 0.5 * ln(2π)
    let mut result = (x - 0.5) * x.ln() - x + 0.5 * std::f64::consts::TAU.ln();

    // Add Bernoulli correction terms for improved accuracy
    let x2 = x * x;
    let x3 = x2 * x;
    let x5 = x3 * x2;
    let x7 = x5 * x2;

    result += 1.0 / (12.0 * x);
    result -= 1.0 / (360.0 * x3);
    result += 1.0 / (1260.0 * x5);
    result -= 1.0 / (1680.0 * x7);

    result
}
