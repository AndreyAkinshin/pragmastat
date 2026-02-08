use console::style;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;

/// Shared row type for all coverage-bounds simulations.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BoundsRow {
    pub distribution: String,
    pub sample_size: usize,
    pub requested_misrate: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub observed_misrate: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

impl super::SimulationRow for BoundsRow {
    fn key(&self) -> String {
        format!(
            "{}-{}-{}",
            self.distribution, self.sample_size, self.requested_misrate
        )
    }
}

/// Input for a single bounds simulation task.
pub struct BoundsInput {
    pub distribution_name: String,
    pub sample_count: usize,
    pub sample_size: usize,
    pub misrate: f64,
    pub base_seed: String,
}

/// Minimum achievable misrate for one-sample signed-rank bounds: 2^(1-n).
pub fn min_achievable_misrate_one_sample(n: usize) -> f64 {
    2.0_f64.powf(1.0 - n as f64)
}

/// Minimum achievable misrate for two-sample Mann-Whitney bounds: 2/C(n+m, n).
pub fn min_achievable_misrate_two_sample(n: usize, m: usize) -> f64 {
    let mut binom = 1.0_f64;
    let k = n.min(m);
    for i in 0..k {
        binom = binom * ((n + m - i) as f64) / ((i + 1) as f64);
    }
    2.0 / binom
}

/// Parse misrate strings like "1e-1,5e-2,1e-2,5e-3,1e-3".
pub fn parse_misrates(input: &str) -> Vec<f64> {
    input
        .split(',')
        .filter_map(|s| {
            let trimmed = s.trim();
            if trimmed.is_empty() {
                return None;
            }
            let v: f64 = trimmed.parse().ok()?;
            if v > 0.0 && v < 1.0 {
                Some(v)
            } else {
                None
            }
        })
        .collect()
}

/// Round a BoundsRow's numeric fields.
pub fn round_bounds_row(row: BoundsRow, digits: u32) -> BoundsRow {
    if row.error.is_some() {
        return row;
    }
    let factor = 10.0_f64.powi(digits as i32);
    BoundsRow {
        requested_misrate: (row.requested_misrate * factor).round() / factor,
        observed_misrate: row.observed_misrate.map(|v| (v * factor).round() / factor),
        ..row
    }
}

/// Format a BoundsRow for console output with colors.
pub fn format_bounds_row(row: &BoundsRow) -> String {
    let dist_padded = format!("{:<9}", row.distribution);
    let n_padded = format!("N={:<3}", row.sample_size);
    let requested = format!("{:.4}", row.requested_misrate);

    let req_padded = format!("{requested:<8}");

    if row.error.is_some() {
        let err_msg = row.error.as_deref().unwrap_or("unknown");
        let err_text = format!("Error: {err_msg}");
        return format!(
            "{}  {}   {} {}  {}",
            style(&dist_padded).yellow().bold(),
            style(&n_padded).yellow(),
            style("Req:").cyan(),
            req_padded,
            style(err_text).red(),
        );
    }

    let observed = row.observed_misrate.unwrap_or(0.0);
    let obs_padded = format!("{observed:<8.4}");
    let ratio = if row.requested_misrate > 0.0 {
        observed / row.requested_misrate
    } else {
        0.0
    };
    format!(
        "{}  {}   {} {}  {} {}  {} {ratio:.2}",
        style(&dist_padded).green().bold(),
        style(&n_padded).green(),
        style("Req:").cyan(),
        req_padded,
        style("Obs:").cyan(),
        obs_padded,
        style("Ratio:").cyan(),
    )
}

impl PartialEq for BoundsRow {
    fn eq(&self, other: &Self) -> bool {
        self.distribution == other.distribution
            && self.sample_size == other.sample_size
            && self.requested_misrate.total_cmp(&other.requested_misrate) == Ordering::Equal
    }
}

impl Eq for BoundsRow {}

impl PartialOrd for BoundsRow {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for BoundsRow {
    fn cmp(&self, other: &Self) -> Ordering {
        self.distribution
            .cmp(&other.distribution)
            .then(self.sample_size.cmp(&other.sample_size))
            .then(self.requested_misrate.total_cmp(&other.requested_misrate))
    }
}
