use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "pragmastat-sim", about = "Pragmastat simulations")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    /// Run avg-drift simulation
    #[command(name = "avg-drift")]
    AvgDrift(DriftArgs),

    /// Run disp-drift simulation
    #[command(name = "disp-drift")]
    DispDrift(DriftArgs),

    /// Run center-bounds simulation
    #[command(name = "center-bounds")]
    CenterBounds(BoundsArgs),

    /// Run shift-bounds simulation
    #[command(name = "shift-bounds")]
    ShiftBounds(BoundsArgs),

    /// Run ratio-bounds simulation
    #[command(name = "ratio-bounds")]
    RatioBounds(BoundsArgs),

    /// Run spread-bounds simulation
    #[command(name = "spread-bounds")]
    SpreadBounds(BoundsArgs),
}

#[derive(Parser)]
pub struct DriftArgs {
    /// Sample sizes (e.g. "2..100" or "2,3,4,5,10..20,50..100")
    #[arg(short = 'n', long = "sample-sizes", default_value = "2..100")]
    pub sample_sizes: String,

    /// Number of samples for building sampling distribution
    #[arg(short = 'm', long = "sample-count", default_value = "1000000")]
    pub sample_count: usize,

    /// Comma-separated list of estimators
    #[arg(short = 'e', long = "estimators")]
    pub estimators: Option<String>,

    /// Comma-separated list of distributions
    #[arg(
        short = 'd',
        long = "distributions",
        default_value = "additive,multiplic,exp,uniform"
    )]
    pub distributions: String,

    /// Seed for random number generation (defaults to simulation name)
    #[arg(short = 's', long = "seed")]
    pub seed: Option<String>,

    /// Max parallelism
    #[arg(short = 'p', long = "parallelism", default_value = "8")]
    pub parallelism: usize,

    /// Overwrite existing entries
    #[arg(short = 'o', long = "overwrite")]
    pub overwrite: bool,

    /// Publish results to sim/ root
    #[arg(long = "publish")]
    pub publish: bool,
}

#[derive(Parser)]
pub struct BoundsArgs {
    /// Sample sizes (e.g. "2..50,60,70,80,90,100")
    #[arg(
        short = 'n',
        long = "sample-sizes",
        default_value = "2..50,60,70,80,90,100"
    )]
    pub sample_sizes: String,

    /// Number of samples per combination (default: 100/misrate)
    #[arg(short = 'm', long = "sample-count")]
    pub sample_count: Option<usize>,

    /// Comma-separated list of distributions
    #[arg(
        short = 'd',
        long = "distributions",
        default_value = "additive,uniform,exp,multiplic"
    )]
    pub distributions: String,

    /// Comma-separated list of misrates
    #[arg(
        short = 'r',
        long = "misrates",
        default_value = "1e-2,1e-3,1e-6"
    )]
    pub misrates: String,

    /// Seed for random number generation (defaults to simulation name)
    #[arg(short = 's', long = "seed")]
    pub seed: Option<String>,

    /// Max parallelism
    #[arg(short = 'p', long = "parallelism", default_value = "8")]
    pub parallelism: usize,

    /// Overwrite existing entries
    #[arg(short = 'o', long = "overwrite")]
    pub overwrite: bool,

    /// Publish results to sim/ root
    #[arg(long = "publish")]
    pub publish: bool,
}
