use pragmastat::estimators::raw;
use serde::Serialize;
use std::fs;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BoundsWidthRow {
    pub n: usize,
    pub center_bounds: Option<f64>,
    pub spread_bounds: Option<f64>,
    pub shift_bounds: Option<f64>,
    pub ratio_bounds: Option<f64>,
    pub disparity_bounds: Option<f64>,
}

const MISRATE: f64 = 1e-3;
const SEED: &str = "bounds-width";

fn linspace(n: usize) -> Vec<f64> {
    (0..n).map(|i| 1.0 + i as f64 / (n - 1) as f64).collect()
}

fn compute_row(n: usize) -> BoundsWidthRow {
    let x = linspace(n);
    let y = x.clone();

    let center_bounds = raw::center_bounds(&x, MISRATE)
        .ok()
        .map(|b| b.upper - b.lower);

    let spread_bounds = raw::spread_bounds_with_seed(&x, MISRATE, SEED)
        .ok()
        .map(|b| b.upper - b.lower);

    let shift_bounds = raw::shift_bounds(&x, &y, MISRATE)
        .ok()
        .map(|b| b.upper - b.lower);

    let ratio_bounds = raw::ratio_bounds(&x, &y, MISRATE)
        .ok()
        .map(|b| b.upper - b.lower);

    let disparity_bounds = raw::disparity_bounds_with_seed(&x, &y, MISRATE, SEED)
        .ok()
        .map(|b| b.upper - b.lower);

    BoundsWidthRow {
        n,
        center_bounds,
        spread_bounds,
        shift_bounds,
        ratio_bounds,
        disparity_bounds,
    }
}

pub fn run(publish: bool) {
    let rows: Vec<BoundsWidthRow> = (2..=10000).map(compute_row).collect();
    let path = super::output_path("bounds-width", publish);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).expect("failed to create output directory");
    }
    let json = serde_json::to_string_pretty(&rows).expect("failed to serialize rows");
    fs::write(&path, json).expect("failed to write bounds-width.json");
    println!("Written {} rows to {}", rows.len(), path.display());
}
