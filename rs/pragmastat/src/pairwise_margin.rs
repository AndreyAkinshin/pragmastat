//! PairwiseMargin function for computing confidence bound margins
//!
//! Determines how many extreme pairwise differences to exclude when constructing bounds
//! based on the distribution of dominance statistics.

use crate::assumptions::{AssumptionError, Subject};

const MAX_EXACT_SIZE: usize = 400;
const MAX_ACCEPTABLE_BINOM_N: usize = 65;

/// PairwiseMargin determines how many extreme pairwise differences to exclude
/// when constructing bounds based on the distribution of dominance statistics.
/// Uses exact calculation for small samples (n+m <= 400) and Edgeworth
/// approximation for larger samples.
///
/// # Arguments
///
/// * `n` - Sample size of first sample (must be positive)
/// * `m` - Sample size of second sample (must be positive)
/// * `misrate` - Misclassification rate (must be in [0, 1])
///
/// # Returns
///
/// Integer representing the total margin split between lower and upper tails,
/// or an error if inputs are invalid.
///
/// # Errors
///
/// Returns an error if n == 0, m == 0, or misrate is outside [0, 1] or is NaN.
pub fn pairwise_margin(n: usize, m: usize, misrate: f64) -> Result<usize, AssumptionError> {
    if n == 0 {
        return Err(AssumptionError::domain(Subject::X));
    }
    if m == 0 {
        return Err(AssumptionError::domain(Subject::Y));
    }
    if misrate.is_nan() || !(0.0..=1.0).contains(&misrate) {
        return Err(AssumptionError::domain(Subject::Misrate));
    }

    if n + m <= MAX_EXACT_SIZE {
        Ok(pairwise_margin_exact(n, m, misrate))
    } else {
        Ok(pairwise_margin_approx(n, m, misrate))
    }
}

/// Uses the exact distribution based on Loeffler's recurrence
fn pairwise_margin_exact(n: usize, m: usize, misrate: f64) -> usize {
    pairwise_margin_exact_raw(n, m, misrate / 2.0) * 2
}

/// Uses Edgeworth approximation for large samples
fn pairwise_margin_approx(n: usize, m: usize, misrate: f64) -> usize {
    pairwise_margin_approx_raw(n, m, misrate / 2.0) * 2
}

/// Inversed implementation of Andreas Löffler's (1982)
/// "Über eine Partition der nat. Zahlen und ihre Anwendung beim U-Test"
fn pairwise_margin_exact_raw(n: usize, m: usize, p: f64) -> usize {
    let total = if n + m < MAX_ACCEPTABLE_BINOM_N {
        binomial_coefficient(n + m, m)
    } else {
        binomial_coefficient_float(n + m, m)
    };

    let mut pmf = vec![1.0]; // pmf[0] = 1
    let mut sigma = vec![0.0]; // sigma[0] is unused

    let mut u: usize = 0;
    let mut cdf = 1.0 / total;

    if cdf >= p {
        return 0;
    }

    loop {
        u += 1;

        // Ensure sigma has entry for u
        if sigma.len() <= u {
            let mut value = 0;
            for d in 1..=n {
                if u.is_multiple_of(d) && u >= d {
                    value += d as i64;
                }
            }
            for d in (m + 1)..=(m + n) {
                if u.is_multiple_of(d) && u >= d {
                    value -= d as i64;
                }
            }
            sigma.push(value as f64);
        }

        // Compute pmf[u] using Loeffler recurrence
        let mut sum = 0.0;
        for i in 0..u {
            sum += pmf[i] * sigma[u - i];
        }
        sum /= u as f64;
        pmf.push(sum);

        cdf += sum / total;
        if cdf >= p {
            return u;
        }
        if sum == 0.0 {
            break;
        }
    }

    pmf.len() - 1
}

/// Inverse Edgeworth Approximation
fn pairwise_margin_approx_raw(n: usize, m: usize, misrate: f64) -> usize {
    let mut a = 0;
    let mut b = n * m;
    while a < b - 1 {
        let c = (a + b) / 2;
        let p = edgeworth_cdf(n, m, c);
        if p < misrate {
            a = c;
        } else {
            b = c;
        }
    }

    if edgeworth_cdf(n, m, b) < misrate {
        b
    } else {
        a
    }
}

/// Computes the CDF using Edgeworth expansion
fn edgeworth_cdf(n: usize, m: usize, u: usize) -> f64 {
    let n_f64 = n as f64;
    let m_f64 = m as f64;
    let u_f64 = u as f64;

    let mu = (n_f64 * m_f64) / 2.0;
    let su = ((n_f64 * m_f64 * (n_f64 + m_f64 + 1.0)) / 12.0).sqrt();
    // -0.5 continuity correction: computing P(U ≥ u) for a right-tail discrete CDF
    let z = (u_f64 - mu - 0.5) / su;

    // Standard normal PDF and CDF
    let phi = (-z * z / 2.0).exp() / (2.0 * std::f64::consts::PI).sqrt();
    let big_phi = crate::gauss_cdf::gauss_cdf(z);

    // Pre-compute powers of n and m for efficiency
    let n2 = n_f64 * n_f64;
    let n3 = n2 * n_f64;
    let n4 = n2 * n2;
    let m2 = m_f64 * m_f64;
    let m3 = m2 * m_f64;
    let m4 = m2 * m2;

    // Compute moments
    let mu2 = (n_f64 * m_f64 * (n_f64 + m_f64 + 1.0)) / 12.0;
    let mu4 = (n_f64
        * m_f64
        * (n_f64 + m_f64 + 1.0)
        * (5.0 * m_f64 * n_f64 * (m_f64 + n_f64) - 2.0 * (m2 + n2) + 3.0 * m_f64 * n_f64
            - 2.0 * (n_f64 + m_f64)))
        / 240.0;

    let mu6 = (n_f64
        * m_f64
        * (n_f64 + m_f64 + 1.0)
        * (35.0 * m2 * n2 * (m2 + n2) + 70.0 * m3 * n3
            - 42.0 * m_f64 * n_f64 * (m3 + n3)
            - 14.0 * m2 * n2 * (n_f64 + m_f64)
            + 16.0 * (n4 + m4)
            - 52.0 * n_f64 * m_f64 * (n2 + m2)
            - 43.0 * n2 * m2
            + 32.0 * (m3 + n3)
            + 14.0 * m_f64 * n_f64 * (n_f64 + m_f64)
            + 8.0 * (n2 + m2)
            + 16.0 * n_f64 * m_f64
            - 8.0 * (n_f64 + m_f64)))
        / 4032.0;

    // Pre-compute powers of mu2 and related terms
    let mu2_2 = mu2 * mu2;
    let mu2_3 = mu2_2 * mu2;
    let mu4_mu2_2 = mu4 / mu2_2;

    // Factorial constants: 4! = 24, 6! = 720, 8! = 40320
    let e3 = (mu4_mu2_2 - 3.0) / 24.0;
    let e5 = (mu6 / mu2_3 - 15.0 * mu4_mu2_2 + 30.0) / 720.0;
    let e7 = 35.0 * (mu4_mu2_2 - 3.0) * (mu4_mu2_2 - 3.0) / 40320.0;

    // Pre-compute powers of z for Hermite polynomials
    let z2 = z * z;
    let z3 = z2 * z;
    let z5 = z3 * z2;
    let z7 = z5 * z2;

    // Hermite polynomial derivatives: f_n = -phi * H_n(z)
    let f3 = -phi * (z3 - 3.0 * z);
    let f5 = -phi * (z5 - 10.0 * z3 + 15.0 * z);
    let f7 = -phi * (z7 - 21.0 * z5 + 105.0 * z3 - 105.0 * z);

    // Edgeworth expansion
    let edgeworth = big_phi + e3 * f3 + e5 * f5 + e7 * f7;

    // Clamp to [0, 1]
    edgeworth.clamp(0.0, 1.0)
}

/// Computes binomial coefficient C(n, k) using integer arithmetic
fn binomial_coefficient(n: usize, k: usize) -> f64 {
    if k > n {
        return 0.0;
    }
    if k == 0 || k == n {
        return 1.0;
    }

    let k = k.min(n - k); // Take advantage of symmetry
    let mut result = 1u128;

    for i in 0..k {
        result = result * (n - i) as u128 / (i + 1) as u128;
    }

    result as f64
}

/// Computes binomial coefficient using floating-point logarithms for large values
fn binomial_coefficient_float(n: usize, k: usize) -> f64 {
    if k > n {
        return 0.0;
    }
    if k == 0 || k == n {
        return 1.0;
    }

    let k = k.min(n - k); // Take advantage of symmetry

    // Use log-factorial function: C(n, k) = exp(log(n!) - log(k!) - log((n-k)!))
    let log_result = log_factorial(n) - log_factorial(k) - log_factorial(n - k);
    log_result.exp()
}

/// Computes the natural logarithm of n!
fn log_factorial(n: usize) -> f64 {
    if n == 0 || n == 1 {
        return 0.0;
    }

    let x = (n + 1) as f64; // n! = Gamma(n+1)

    if x < 1e-5 {
        return 0.0;
    }

    // DONT TOUCH: Stirling's approximation is inaccurate for small x.
    // Use Gamma recurrence: Gamma(x) = Gamma(x+k) / (x*(x+1)*...*(x+k-1))
    // These branches appear unreachable in current usage (n+m >= 65), but
    // are retained for correctness if the function is used in other contexts.
    if x < 1.0 {
        return stirling_approx_log(x + 3.0) - (x * (x + 1.0) * (x + 2.0)).ln();
    }
    if x < 2.0 {
        return stirling_approx_log(x + 2.0) - (x * (x + 1.0)).ln();
    }
    if x < 3.0 {
        return stirling_approx_log(x + 1.0) - x.ln();
    }

    stirling_approx_log(x)
}

/// Stirling's approximation with Bernoulli correction
fn stirling_approx_log(x: f64) -> f64 {
    let mut result = x * x.ln() - x + (2.0 * std::f64::consts::PI / x).ln() / 2.0;

    // Bernoulli correction series
    const B2: f64 = 1.0 / 6.0;
    const B4: f64 = -1.0 / 30.0;
    const B6: f64 = 1.0 / 42.0;
    const B8: f64 = -1.0 / 30.0;
    const B10: f64 = 5.0 / 66.0;

    let x2 = x * x;
    let x3 = x2 * x;
    let x5 = x3 * x2;
    let x7 = x5 * x2;
    let x9 = x7 * x2;

    result +=
        B2 / (2.0 * x) + B4 / (12.0 * x3) + B6 / (30.0 * x5) + B8 / (56.0 * x7) + B10 / (90.0 * x9);

    result
}
