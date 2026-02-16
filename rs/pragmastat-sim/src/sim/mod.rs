pub mod avg_drift;
pub mod avg_spread_bounds;
pub mod bounds;
pub mod center_bounds;
pub mod disp_drift;
pub mod disparity_bounds;
pub mod drift;
pub mod ratio_bounds;
pub mod shift_bounds;
pub mod spread_bounds;

use serde::Serialize;
use std::collections::BTreeMap;
use std::path::PathBuf;

/// A single simulation row that can be keyed, serialized, and sorted.
pub trait SimulationRow: Serialize + Clone + Ord + Send + Sync + 'static {
    fn key(&self) -> String;
}

/// A simulation that produces rows from inputs.
pub trait Simulation: Send + Sync {
    type Input: Send + Sync;
    type Row: SimulationRow;

    fn name(&self) -> &'static str;

    /// Build (new_inputs, reused_rows) from settings.
    fn create_inputs(
        &self,
        sample_sizes: &[usize],
        existing: &BTreeMap<String, Self::Row>,
        overwrite: bool,
    ) -> (Vec<Self::Input>, Vec<Self::Row>);

    /// Run one simulation row.
    fn simulate_row(
        &self,
        input: &Self::Input,
        progress: &dyn Fn(f64),
    ) -> Result<Self::Row, SimError>;

    /// Create an error placeholder row.
    fn create_error_row(&self, input: &Self::Input, error: &str) -> Self::Row;

    /// Format a row for console output.
    fn format_row(&self, row: &Self::Row) -> String;

    /// Round a row's numeric values.
    fn round_row(&self, row: Self::Row, digits: u32) -> Self::Row;
}

#[derive(Debug)]
pub struct SimError(pub String);

impl std::fmt::Display for SimError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

/// Resolve the output path for a simulation.
pub fn output_path(name: &str, publish: bool) -> PathBuf {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let repo_root = PathBuf::from(manifest_dir)
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .to_path_buf();
    let sim_dir = if publish {
        repo_root.join("sim")
    } else {
        repo_root.join("sim").join("rs")
    };
    sim_dir.join(format!("{name}.json"))
}
