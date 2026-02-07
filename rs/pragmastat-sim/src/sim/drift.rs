use console::style;
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;

/// Shared row type for drift simulations.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DriftRow {
    pub distribution: String,
    pub sample_size: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub drifts: Option<IndexMap<String, f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

impl super::SimulationRow for DriftRow {
    fn key(&self) -> String {
        format!("{}-{}", self.distribution, self.sample_size)
    }
}

/// Input for a single drift simulation task.
pub struct DriftInput {
    pub distribution_name: String,
    pub estimator_names: Vec<String>,
    pub sample_count: usize,
    pub sample_size: usize,
    pub base_seed: String,
}

/// Round a DriftRow's numeric fields.
pub fn round_drift_row(row: DriftRow, digits: u32) -> DriftRow {
    if row.error.is_some() {
        return row;
    }
    let factor = 10.0_f64.powi(digits as i32);
    let drifts = row.drifts.map(|d| {
        d.into_iter()
            .map(|(k, v)| (k, (v * factor).round() / factor))
            .collect()
    });
    DriftRow { drifts, ..row }
}

/// Format a DriftRow for console output with colors.
pub fn format_drift_row(row: &DriftRow) -> String {
    let dist_padded = format!("{:<9}", row.distribution);
    let n_padded = format!("N={:<3}", row.sample_size);

    if row.error.is_some() {
        let err_msg = row.error.as_deref().unwrap_or("unknown");
        let err_text = format!("Error: {err_msg}");
        return format!(
            "{}  {}   {}",
            style(&dist_padded).yellow().bold(),
            style(&n_padded).yellow(),
            style(err_text).red(),
        );
    }

    if let Some(ref drifts) = row.drifts {
        let parts: Vec<String> = drifts
            .iter()
            .map(|(k, v)| {
                let label = format!("{k}:");
                format!("{} {v:.4}", style(label).cyan())
            })
            .collect();
        format!(
            "{}  {}   {}",
            style(&dist_padded).green().bold(),
            style(&n_padded).green(),
            parts.join("  "),
        )
    } else {
        format!(
            "{}  {}   (no data)",
            style(&dist_padded).green().bold(),
            style(&n_padded).green(),
        )
    }
}

impl PartialEq for DriftRow {
    fn eq(&self, other: &Self) -> bool {
        self.distribution == other.distribution && self.sample_size == other.sample_size
    }
}

impl Eq for DriftRow {}

impl PartialOrd for DriftRow {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for DriftRow {
    fn cmp(&self, other: &Self) -> Ordering {
        self.distribution
            .cmp(&other.distribution)
            .then(self.sample_size.cmp(&other.sample_size))
    }
}
