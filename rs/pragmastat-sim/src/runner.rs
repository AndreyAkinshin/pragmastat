use crate::output::OutputWriter;
use crate::progress::ProgressTracker;
use crate::sim::{output_path, SimError, Simulation, SimulationRow};
use console::style;
use rayon::prelude::*;
use std::collections::BTreeMap;
use std::fs;
use std::sync::Arc;
use std::time::{Duration, Instant};

/// Load existing rows from a JSON file.
fn load_existing<R: SimulationRow + serde::de::DeserializeOwned>(
    name: &str,
    publish: bool,
) -> BTreeMap<String, R> {
    let path = output_path(name, publish);
    if !path.exists() {
        return BTreeMap::new();
    }
    let content = fs::read_to_string(&path).unwrap_or_default();
    let rows: Vec<R> = serde_json::from_str(&content).unwrap_or_default();
    rows.into_iter().map(|r| (r.key(), r)).collect()
}

fn format_duration(d: Duration) -> String {
    let total_secs = d.as_secs();
    if total_secs >= 60 {
        format!("{}m {:02}s", total_secs / 60, total_secs % 60)
    } else {
        format!("{}s", total_secs)
    }
}

/// Run a simulation with parallel execution.
pub fn run<S>(sim: &S, sample_sizes: &[usize], parallelism: usize, overwrite: bool, publish: bool)
where
    S: Simulation,
    S::Row: serde::de::DeserializeOwned,
{
    let existing = load_existing::<S::Row>(sim.name(), publish);
    let (inputs, reused) = sim.create_inputs(sample_sizes, &existing, overwrite);

    if inputs.is_empty() && reused.is_empty() {
        eprintln!("  No valid simulation combinations found.");
        return;
    }

    let total_new = inputs.len();
    let reused_count = reused.len();
    let total = total_new + reused_count;

    // Header
    let reused_suffix = if reused_count > 0 {
        format!(" {}", style(format!("({reused_count} reused)")).dim())
    } else {
        String::new()
    };
    eprintln!(
        "  {} {} {} {total} tasks{reused_suffix}",
        style("\u{25b6}").cyan().bold(),
        style(sim.name()).white().bold(),
        style("\u{2014}").dim(),
    );

    let path = output_path(sim.name(), publish);
    let writer = Arc::new(OutputWriter::new(path, existing));

    // Print reused rows
    for row in &reused {
        eprintln!("  {}  {}", sim.format_row(row), style("(reused)").dim(),);
    }

    if total_new > 0 {
        let pool = rayon::ThreadPoolBuilder::new()
            .num_threads(parallelism)
            .build()
            .expect("Failed to build rayon thread pool");

        let tracker = Arc::new(ProgressTracker::new(total_new, reused_count));
        let start = Instant::now();

        pool.install(|| {
            inputs.par_iter().enumerate().for_each(|(idx, input)| {
                let progress = |frac: f64| tracker.update(idx, frac);
                let result = sim.simulate_row(input, &progress);

                let row = match result {
                    Ok(row) => row,
                    Err(SimError(msg)) => sim.create_error_row(input, &msg),
                };

                tracker.println(&sim.format_row(&row));
                let rounded = sim.round_row(row, 4);
                writer.write_row(rounded.key(), rounded);
                tracker.complete(idx);
            });
        });

        tracker.finish();

        let elapsed = format_duration(start.elapsed());
        eprintln!(
            "  {} Completed in {}",
            style("\u{2713}").green().bold(),
            style(&elapsed).bold(),
        );
    }

    let saved_path = writer.finalize();
    eprintln!(
        "  {} Results saved: {}",
        style("\u{2713}").green().bold(),
        style(saved_path.display()).dim(),
    );
}
