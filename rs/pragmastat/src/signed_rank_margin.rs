//! SignedRankMargin function for one-sample bounds
//!
//! One-sample analog of PairwiseMargin using Wilcoxon signed-rank distribution.

use crate::assumptions::AssumptionError;
use crate::gauss_cdf::gauss_cdf;
use crate::min_misrate::min_achievable_misrate_one_sample;

/// Maximum n for exact computation. Limited to 63 because 2^n must fit in a 64-bit integer.
const SIGNED_RANK_MAX_EXACT_SIZE: usize = 63;

/// SignedRankMargin computes the margin for one-sample signed-rank bounds.
/// Uses Wilcoxon signed-rank distribution to determine the margin that achieves
/// the specified misrate.
///
/// # Arguments
///
/// * `n` - Sample size (must be positive)
/// * `misrate` - Misclassification rate (must be in [0, 1])
///
/// # Returns
///
/// Integer margin, or an error if inputs are invalid.
pub fn signed_rank_margin(n: usize, misrate: f64) -> Result<usize, AssumptionError> {
    if n == 0 {
        return Err(AssumptionError::domain(crate::assumptions::Subject::X));
    }
    if misrate.is_nan() || !(0.0..=1.0).contains(&misrate) {
        return Err(AssumptionError::domain(
            crate::assumptions::Subject::Misrate,
        ));
    }

    let min_misrate = min_achievable_misrate_one_sample(n)?;
    if misrate < min_misrate {
        return Err(AssumptionError::domain(
            crate::assumptions::Subject::Misrate,
        ));
    }

    if n <= SIGNED_RANK_MAX_EXACT_SIZE {
        Ok(signed_rank_margin_exact(n, misrate))
    } else {
        signed_rank_margin_approx(n, misrate)
    }
}

/// Computes one-sided margin using exact Wilcoxon signed-rank distribution.
/// Uses dynamic programming to compute the CDF.
fn signed_rank_margin_exact(n: usize, misrate: f64) -> usize {
    signed_rank_margin_exact_raw(n, misrate / 2.0) * 2
}

fn signed_rank_margin_exact_raw(n: usize, p: f64) -> usize {
    let total = 1_u64 << n;
    let max_w = n * (n + 1) / 2;

    let mut count = vec![0_u64; max_w + 1];
    count[0] = 1;

    for i in 1..=n {
        let max_wi = i * (i + 1) / 2;
        let max_wi = max_wi.min(max_w);
        for w in (i..=max_wi).rev() {
            count[w] += count[w - i];
        }
    }

    let mut cumulative: u64 = 0;
    for (w, &c) in count.iter().enumerate().take(max_w + 1) {
        cumulative += c;
        let cdf = cumulative as f64 / total as f64;
        if cdf >= p {
            return w;
        }
    }

    max_w
}

/// Computes one-sided margin using Edgeworth approximation for large n.
fn signed_rank_margin_approx(n: usize, misrate: f64) -> Result<usize, AssumptionError> {
    let raw = signed_rank_margin_approx_raw(n, misrate / 2.0);
    raw.checked_mul(2)
        .ok_or_else(|| AssumptionError::domain(crate::assumptions::Subject::X))
}

fn signed_rank_margin_approx_raw(n: usize, misrate: f64) -> usize {
    let max_w = n * (n + 1) / 2;
    let mut a: usize = 0;
    let mut b = max_w;

    while a < b - 1 {
        let c = (a + b) / 2;
        let cdf = signed_rank_edgeworth_cdf(n, c);
        if cdf < misrate {
            a = c;
        } else {
            b = c;
        }
    }

    if signed_rank_edgeworth_cdf(n, b) < misrate {
        b
    } else {
        a
    }
}

/// Edgeworth expansion for Wilcoxon signed-rank distribution CDF.
fn signed_rank_edgeworth_cdf(n: usize, w: usize) -> f64 {
    let n_f64 = n as f64;
    let mu = n_f64 * (n_f64 + 1.0) / 4.0;
    let sigma2 = n_f64 * (n_f64 + 1.0) * (2.0 * n_f64 + 1.0) / 24.0;
    let sigma = sigma2.sqrt();

    // +0.5 continuity correction: computing P(W ≤ w) for a left-tail discrete CDF
    let z = (w as f64 - mu + 0.5) / sigma;
    let phi = (-z * z / 2.0).exp() / (2.0 * std::f64::consts::PI).sqrt();
    let big_phi = gauss_cdf(z);

    let kappa4 =
        -n_f64 * (n_f64 + 1.0) * (2.0 * n_f64 + 1.0) * (3.0 * n_f64 * n_f64 + 3.0 * n_f64 - 1.0)
            / 240.0;

    let e3 = kappa4 / (24.0 * sigma2 * sigma2);

    let z2 = z * z;
    let z3 = z2 * z;
    let f3 = -phi * (z3 - 3.0 * z);

    let edgeworth = big_phi + e3 * f3;
    edgeworth.clamp(0.0, 1.0)
}
