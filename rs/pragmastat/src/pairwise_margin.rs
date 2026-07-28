//! PairwiseMargin function for computing confidence bound margins
//!
//! Determines how many extreme pairwise differences to exclude when constructing bounds
//! based on the distribution of dominance statistics.

use crate::assumptions::{AssumptionError, Subject};
use crate::binomial::binomial_coefficient;

const MAX_EXACT_SIZE: usize = 400;

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
pub fn pairwise_margin(n: usize, m: usize, misrate: f64) -> Result<u64, AssumptionError> {
    if n == 0 {
        return Err(AssumptionError::domain(Subject::X));
    }
    if m == 0 {
        return Err(AssumptionError::domain(Subject::Y));
    }
    if misrate.is_nan() || !(0.0..=1.0).contains(&misrate) {
        return Err(AssumptionError::domain(Subject::Misrate));
    }

    let min_misrate = crate::min_misrate::min_achievable_misrate_two_sample(n, m)?;
    if misrate < min_misrate {
        return Err(AssumptionError::domain(Subject::Misrate));
    }

    if n + m <= MAX_EXACT_SIZE {
        Ok(pairwise_margin_exact(n, m, misrate))
    } else {
        Ok(pairwise_margin_approx(n, m, misrate))
    }
}

/// Uses the exact distribution based on Loeffler's recurrence
fn pairwise_margin_exact(n: usize, m: usize, misrate: f64) -> u64 {
    pairwise_margin_exact_raw(n, m, misrate / 2.0) as u64 * 2
}

/// Uses Edgeworth approximation for large samples
fn pairwise_margin_approx(n: usize, m: usize, misrate: f64) -> u64 {
    pairwise_margin_approx_raw(n, m, misrate / 2.0) * 2
}

/// Inversed implementation of Andreas Löffler's (1982)
/// "Über eine Partition der nat. Zahlen und ihre Anwendung beim U-Test"
fn pairwise_margin_exact_raw(n: usize, m: usize, p: f64) -> usize {
    let total = binomial_coefficient(n + m, m);

    let capacity = n * m + 1;
    let mut pmf = Vec::with_capacity(capacity);
    pmf.push(1.0); // pmf[0] = 1
    let mut sigma = Vec::with_capacity(capacity);
    sigma.push(0.0); // sigma[0] is unused

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
fn pairwise_margin_approx_raw(n: usize, m: usize, misrate: f64) -> u64 {
    let mut a: u64 = 0;
    let mut b: u64 = n as u64 * m as u64;
    while a < b - 1 {
        let c = u64::midpoint(a, b);
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
fn edgeworth_cdf(n: usize, m: usize, u: u64) -> f64 {
    let n_f64 = n as f64;
    let m_f64 = m as f64;
    let u_f64 = u as f64;

    let mu = (n_f64 * m_f64) / 2.0;
    let su = ((n_f64 * m_f64 * (n_f64 + m_f64 + 1.0)) / 12.0).sqrt();
    // -0.5 continuity correction: computing P(U ≥ u) for a right-tail discrete CDF
    let z = (u_f64 - mu - 0.5) / su;

    // Standard normal PDF and CDF
    let phi = crate::portable_exp::portable_exp(-z * z / 2.0) / (2.0 * std::f64::consts::PI).sqrt();
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

#[cfg(test)]
mod tests {
    use super::{MAX_EXACT_SIZE, binomial_coefficient, pairwise_margin};
    use serde::Deserialize;
    use std::fs;
    use std::path::PathBuf;

    #[derive(Debug, Deserialize)]
    struct Input {
        n: usize,
        m: usize,
        misrate: f64,
    }

    #[derive(Debug, Deserialize)]
    struct TestCase {
        input: Input,
        output: Option<u64>,
        expected_error: Option<serde_json::Value>,
    }

    fn repo_root() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .parent()
            .unwrap()
            .to_path_buf()
    }

    #[test]
    fn reference() {
        let test_data_dir = repo_root().join("tests").join("pairwise-margin");
        assert!(
            test_data_dir.exists(),
            "Test data directory not found: {test_data_dir:?}"
        );

        let json_files: Vec<_> = fs::read_dir(&test_data_dir)
            .unwrap()
            .filter_map(|entry| {
                let path = entry.unwrap().path();
                if path.extension()?.to_str()? == "json" {
                    Some(path)
                } else {
                    None
                }
            })
            .collect();

        assert!(
            !json_files.is_empty(),
            "No JSON test files found in {test_data_dir:?}"
        );

        let mut failures = Vec::new();

        for json_file in &json_files {
            let content = fs::read_to_string(json_file).unwrap();
            let test_case: TestCase = serde_json::from_str(&content).unwrap();
            let file_name = json_file.file_name().unwrap();

            if let Some(ref expected_error) = test_case.expected_error {
                let result = pairwise_margin(
                    test_case.input.n,
                    test_case.input.m,
                    test_case.input.misrate,
                );
                match result {
                    Ok(_) => failures.push(format!("{file_name:?}: expected error, got Ok")),
                    Err(err) => {
                        if let Some(expected_id) = expected_error.get("id").and_then(|v| v.as_str())
                        {
                            if err.violation().id.as_str() != expected_id {
                                failures.push(format!(
                                    "{file_name:?}: expected violation id {expected_id}, got {}",
                                    err.violation().id.as_str()
                                ));
                            }
                        }
                    }
                }
                continue;
            }

            let actual_output = match pairwise_margin(
                test_case.input.n,
                test_case.input.m,
                test_case.input.misrate,
            ) {
                Ok(val) => val,
                Err(e) => {
                    failures.push(format!("{file_name:?}: unexpected error {e:?}"));
                    continue;
                }
            };
            let expected_output = test_case.output.expect("Test case must have output");

            if actual_output != expected_output {
                failures.push(format!(
                    "{file_name:?}: expected {expected_output}, got {actual_output}"
                ));
            }
        }

        assert!(
            failures.is_empty(),
            "Failed tests:\n{}",
            failures.join("\n")
        );
    }

    use crate::assumptions::{AssumptionId, Subject};

    #[test]
    fn zero_n() {
        let result = pairwise_margin(0, 10, 0.05);
        assert!(result.is_err());
        let v = result.unwrap_err().violation();
        assert_eq!(v.id, AssumptionId::Domain);
        assert_eq!(v.subject, Subject::X);
    }

    #[test]
    fn zero_m() {
        let result = pairwise_margin(10, 0, 0.05);
        assert!(result.is_err());
        let v = result.unwrap_err().violation();
        assert_eq!(v.id, AssumptionId::Domain);
        assert_eq!(v.subject, Subject::Y);
    }

    #[test]
    fn negative_misrate() {
        let result = pairwise_margin(10, 10, -0.1);
        assert!(result.is_err());
        let v = result.unwrap_err().violation();
        assert_eq!(v.id, AssumptionId::Domain);
        assert_eq!(v.subject, Subject::Misrate);
    }

    #[test]
    fn misrate_greater_than_one() {
        let result = pairwise_margin(10, 10, 1.5);
        assert!(result.is_err());
        let v = result.unwrap_err().violation();
        assert_eq!(v.id, AssumptionId::Domain);
        assert_eq!(v.subject, Subject::Misrate);
    }

    #[test]
    fn nan_misrate() {
        let result = pairwise_margin(10, 10, f64::NAN);
        assert!(result.is_err());
        let v = result.unwrap_err().violation();
        assert_eq!(v.id, AssumptionId::Domain);
        assert_eq!(v.subject, Subject::Misrate);
    }

    /// The specification admits `misrate >= 2 / C(n+m, n)`, and at that floor the whole
    /// sample is the interval: `pairwise_margin` excludes nothing and returns 0.
    ///
    /// It only does so if the binomial behind the floor and the binomial behind the
    /// exact pass are the same f64, because `1/total >= misrate/2` is then an exact tie.
    /// When the two were computed by different code, 28412 of these 79800 pairs came
    /// back non-zero: the function rejected nothing, then acted as if the caller had
    /// asked for less than it did. This is the cheap analytic form of the comparison,
    /// run over every pair the exact path serves; `margin_is_zero_at_misrate_floor`
    /// checks the same thing end to end.
    #[test]
    fn misrate_floor_is_attainable() {
        for n in 1..MAX_EXACT_SIZE {
            for m in 1..=(MAX_EXACT_SIZE - n) {
                let floor = crate::min_misrate::min_achievable_misrate_two_sample(n, m).unwrap();
                let total = binomial_coefficient(n + m, m);
                assert!(
                    1.0 / total >= floor / 2.0,
                    "n={n} m={m}: 1/{total} < {floor}/2"
                );
            }
        }
    }

    #[test]
    fn margin_is_zero_at_misrate_floor() {
        for (n, m) in [
            (1, 1),
            (2, 2),
            (5, 5),
            (30, 31),
            (1, 61),
            (31, 31),
            (1, 62),
            (31, 32),
            (60, 66),
            (64, 64),
        ] {
            let floor = crate::min_misrate::min_achievable_misrate_two_sample(n, m).unwrap();
            assert_eq!(
                pairwise_margin(n, m, floor).unwrap(),
                0,
                "n={n} m={m} floor={floor}"
            );
        }
    }

    #[test]
    fn misrate_below_min() {
        // n=2, m=2: min_misrate = 2/C(4,2) = 1/3 ≈ 0.333
        let result = pairwise_margin(2, 2, 0.05);
        assert!(result.is_err());
        let v = result.unwrap_err().violation();
        assert_eq!(v.id, AssumptionId::Domain);
        assert_eq!(v.subject, Subject::Misrate);
    }
}
