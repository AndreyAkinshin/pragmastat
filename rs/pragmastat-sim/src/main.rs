mod cli;
mod distributions;
mod estimators;
mod output;
mod progress;
mod runner;
mod sample_sizes;
mod sim;

use clap::Parser;
use cli::{Cli, Command};
use distributions::find_distributions;
use sample_sizes::parse_sample_sizes;
use sim::avg_drift::AvgDriftSim;
use sim::center_bounds::CenterBoundsSim;
use sim::disp_drift::DispDriftSim;
use sim::ratio_bounds::RatioBoundsSim;
use sim::shift_bounds::ShiftBoundsSim;
use sim::spread_bounds::SpreadBoundsSim;

fn parse_names(input: &str) -> Vec<String> {
    input
        .split(',')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect()
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Command::AvgDrift(args) => {
            let dist_names = parse_names(&args.distributions);
            let dists = find_distributions(&dist_names);
            let estimator_names =
                parse_names(args.estimators.as_deref().unwrap_or("Mean,Median,Center"));
            let sizes = parse_sample_sizes(&args.sample_sizes);
            let seed = args.seed.unwrap_or_else(|| "avg-drift".to_string());
            let sim = AvgDriftSim::new(dists, estimator_names, args.sample_count, seed);
            runner::run(&sim, &sizes, args.parallelism, args.overwrite, args.publish);
        }
        Command::DispDrift(args) => {
            let dist_names = parse_names(&args.distributions);
            let dists = find_distributions(&dist_names);
            let estimator_names =
                parse_names(args.estimators.as_deref().unwrap_or("StdDev,MAD,Spread"));
            let sizes = parse_sample_sizes(&args.sample_sizes);
            let seed = args.seed.unwrap_or_else(|| "disp-drift".to_string());
            let sim = DispDriftSim::new(dists, estimator_names, args.sample_count, seed);
            runner::run(&sim, &sizes, args.parallelism, args.overwrite, args.publish);
        }
        Command::CenterBounds(args) => {
            let dist_names = parse_names(&args.distributions);
            let dists = find_distributions(&dist_names);
            let sizes = parse_sample_sizes(&args.sample_sizes);
            let seed = args.seed.unwrap_or_else(|| "center-bounds".to_string());
            let sim = CenterBoundsSim::new(dists, args.sample_count, &args.misrates, seed);
            runner::run(&sim, &sizes, args.parallelism, args.overwrite, args.publish);
        }
        Command::ShiftBounds(args) => {
            let dist_names = parse_names(&args.distributions);
            let dists = find_distributions(&dist_names);
            let sizes = parse_sample_sizes(&args.sample_sizes);
            let seed = args.seed.unwrap_or_else(|| "shift-bounds".to_string());
            let sim = ShiftBoundsSim::new(dists, args.sample_count, &args.misrates, seed);
            runner::run(&sim, &sizes, args.parallelism, args.overwrite, args.publish);
        }
        Command::RatioBounds(args) => {
            let dist_names = parse_names(&args.distributions);
            let dists = find_distributions(&dist_names);
            let sizes = parse_sample_sizes(&args.sample_sizes);
            let seed = args.seed.unwrap_or_else(|| "ratio-bounds".to_string());
            let sim = RatioBoundsSim::new(dists, args.sample_count, &args.misrates, seed);
            runner::run(&sim, &sizes, args.parallelism, args.overwrite, args.publish);
        }
        Command::SpreadBounds(args) => {
            let dist_names = parse_names(&args.distributions);
            let dists = find_distributions(&dist_names);
            let sizes = parse_sample_sizes(&args.sample_sizes);
            let seed = args.seed.unwrap_or_else(|| "spread-bounds".to_string());
            let sim = SpreadBoundsSim::new(dists, args.sample_count, &args.misrates, seed);
            runner::run(&sim, &sizes, args.parallelism, args.overwrite, args.publish);
        }
    }
}
