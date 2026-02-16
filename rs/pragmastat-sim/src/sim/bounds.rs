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
    pub sample_count: usize,
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

/// Minimum achievable misrate for spread-bounds (sign-test on disjoint pairs): 2^(1 - floor(n/2)).
pub fn min_achievable_misrate_spread(n: usize) -> f64 {
    let m = n / 2;
    2.0_f64.powf(1.0 - m as f64)
}

/// Minimum achievable misrate for avg-spread-bounds.
///
/// Uses Bonferroni split: alpha = misrate/2, applied to spread-bounds on each sample.
/// So misrate >= 2 * max(2^(1 - floor(n/2)), 2^(1 - floor(m/2))).
pub fn min_achievable_misrate_avg_spread(n: usize, m: usize) -> f64 {
    let min_x = min_achievable_misrate_spread(n);
    let min_y = min_achievable_misrate_spread(m);
    2.0 * min_x.max(min_y)
}

/// Minimum achievable misrate for disparity-bounds.
///
/// Uses Bonferroni split between shift-bounds and avg-spread-bounds,
/// with each at least its own minimum achievable misrate.
/// So misrate >= min_shift + min_avg.
pub fn min_achievable_misrate_disparity(n: usize, m: usize) -> f64 {
    let min_shift = min_achievable_misrate_two_sample(n, m);
    let min_avg = min_achievable_misrate_avg_spread(n, m);
    min_shift + min_avg
}

/// Resolve sample count: use explicit value or default to max(1_000_000, 100/misrate).
pub fn resolve_sample_count(explicit: Option<usize>, misrate: f64) -> usize {
    explicit.unwrap_or_else(|| ((100.0 / misrate) as usize).max(1_000_000))
}

/// Parse misrate strings like "1e-2,1e-3,1e-6".
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
    let dist = format!("{:<9}", row.distribution);
    let n = format!("N={:<3}", row.sample_size);
    let req = format!("{:e}", row.requested_misrate);
    let digits = (row.sample_count as f64).log10().floor() as usize;

    if row.error.is_some() {
        let err_msg = row.error.as_deref().unwrap_or("unknown");
        return format!(
            "{}  {} {} {} {} Error: {}",
            style(&dist).yellow().bold(),
            style(&n).yellow(),
            style("Req:").cyan(),
            req,
            style("").cyan(),
            style(err_msg).red(),
        );
    }

    let observed = row.observed_misrate.unwrap_or(0.0);
    let obs = format!("{observed:<10.*}", digits);
    let ratio = if row.requested_misrate > 0.0 {
        observed / row.requested_misrate
    } else {
        0.0
    };
    format!(
        "{}  {} {} {} {} {} {} {ratio:.2}",
        style(&dist).green().bold(),
        style(&n).green(),
        style("Req:").cyan(),
        req,
        style("Obs:").cyan(),
        obs,
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

// ---------------------------------------------------------------------------
// Two-sample bounds (avg-spread-bounds)
// ---------------------------------------------------------------------------

/// Row type for two-sample coverage-bounds simulations (different n, m).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TwoSampleBoundsRow {
    pub distribution: String,
    pub sample_size_x: usize,
    pub sample_size_y: usize,
    pub requested_misrate: f64,
    pub sample_count: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub observed_misrate: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

impl super::SimulationRow for TwoSampleBoundsRow {
    fn key(&self) -> String {
        format!(
            "{}-{}-{}-{}",
            self.distribution, self.sample_size_x, self.sample_size_y, self.requested_misrate
        )
    }
}

/// Input for a single two-sample bounds simulation task.
pub struct TwoSampleBoundsInput {
    pub distribution_name: String,
    pub sample_count: usize,
    pub sample_size_x: usize,
    pub sample_size_y: usize,
    pub misrate: f64,
    pub base_seed: String,
}

/// Round a TwoSampleBoundsRow's numeric fields.
pub fn round_two_sample_bounds_row(row: TwoSampleBoundsRow, digits: u32) -> TwoSampleBoundsRow {
    if row.error.is_some() {
        return row;
    }
    let factor = 10.0_f64.powi(digits as i32);
    TwoSampleBoundsRow {
        requested_misrate: (row.requested_misrate * factor).round() / factor,
        observed_misrate: row.observed_misrate.map(|v| (v * factor).round() / factor),
        ..row
    }
}

/// Format a TwoSampleBoundsRow for console output with colors.
pub fn format_two_sample_bounds_row(row: &TwoSampleBoundsRow) -> String {
    let dist = format!("{:<9}", row.distribution);
    let sizes = format!("N={:<3} M={:<3}", row.sample_size_x, row.sample_size_y);
    let req = format!("{:e}", row.requested_misrate);
    let digits = (row.sample_count as f64).log10().floor() as usize;

    if row.error.is_some() {
        let err_msg = row.error.as_deref().unwrap_or("unknown");
        return format!(
            "{}  {} {} {} {} Error: {}",
            style(&dist).yellow().bold(),
            style(&sizes).yellow(),
            style("Req:").cyan(),
            req,
            style("").cyan(),
            style(err_msg).red(),
        );
    }

    let observed = row.observed_misrate.unwrap_or(0.0);
    let obs = format!("{observed:<10.*}", digits);
    let ratio = if row.requested_misrate > 0.0 {
        observed / row.requested_misrate
    } else {
        0.0
    };
    format!(
        "{}  {} {} {} {} {} {} {ratio:.2}",
        style(&dist).green().bold(),
        style(&sizes).green(),
        style("Req:").cyan(),
        req,
        style("Obs:").cyan(),
        obs,
        style("Ratio:").cyan(),
    )
}

impl PartialEq for TwoSampleBoundsRow {
    fn eq(&self, other: &Self) -> bool {
        self.distribution == other.distribution
            && self.sample_size_x == other.sample_size_x
            && self.sample_size_y == other.sample_size_y
            && self.requested_misrate.total_cmp(&other.requested_misrate) == Ordering::Equal
    }
}

impl Eq for TwoSampleBoundsRow {}

impl PartialOrd for TwoSampleBoundsRow {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for TwoSampleBoundsRow {
    fn cmp(&self, other: &Self) -> Ordering {
        self.distribution
            .cmp(&other.distribution)
            .then(self.sample_size_x.cmp(&other.sample_size_x))
            .then(self.sample_size_y.cmp(&other.sample_size_y))
            .then(self.requested_misrate.total_cmp(&other.requested_misrate))
    }
}
