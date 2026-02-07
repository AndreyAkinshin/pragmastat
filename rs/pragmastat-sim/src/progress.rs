use indicatif::{ProgressBar, ProgressStyle};
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Duration;

const SCALE: u64 = 1000;

/// Tracks progress across parallel simulation tasks using an indicatif progress bar.
pub struct ProgressTracker {
    bar: ProgressBar,
    /// Per-task progress scaled to 0..SCALE
    fractions: Vec<AtomicU64>,
    total_tasks: usize,
}

impl ProgressTracker {
    pub fn new(new_tasks: usize, reused: usize) -> Self {
        let total = new_tasks + reused;
        let bar = ProgressBar::new((total as u64) * SCALE);
        bar.set_style(
            ProgressStyle::with_template(
                "  {spinner:.cyan} [{elapsed_precise}] [{bar:40.green/dim}] {percent:>3}%  {msg}",
            )
            .unwrap()
            .progress_chars("\u{2501}\u{2578}\u{2500}")
            .tick_chars(
                "\u{280b}\u{2819}\u{2839}\u{2838}\u{283c}\u{2834}\u{2826}\u{2827}\u{2807}\u{280f}",
            ),
        );
        // Account for reused tasks as already complete
        bar.inc(reused as u64 * SCALE);
        bar.enable_steady_tick(Duration::from_millis(80));

        let mut fractions = Vec::with_capacity(new_tasks);
        for _ in 0..new_tasks {
            fractions.push(AtomicU64::new(0));
        }

        Self {
            bar,
            fractions,
            total_tasks: total,
        }
    }

    /// Update fractional progress for a task (0.0..1.0).
    pub fn update(&self, index: usize, fraction: f64) {
        let new_scaled = ((fraction * SCALE as f64) as u64).min(SCALE);
        let old_scaled = self.fractions[index].swap(new_scaled, Ordering::Relaxed);
        if new_scaled > old_scaled {
            self.bar.inc(new_scaled - old_scaled);
        }
    }

    /// Mark a task as complete.
    pub fn complete(&self, index: usize) {
        let old_scaled = self.fractions[index].swap(SCALE, Ordering::Relaxed);
        if SCALE > old_scaled {
            self.bar.inc(SCALE - old_scaled);
        }
        let done = self.bar.position() / SCALE;
        self.bar
            .set_message(format!("{done}/{} completed", self.total_tasks));
    }

    /// Print a message above the progress bar.
    pub fn println(&self, msg: &str) {
        self.bar.println(format!("  {msg}"));
    }

    /// Finish and clear the progress bar.
    pub fn finish(&self) {
        self.bar.finish_and_clear();
    }
}
