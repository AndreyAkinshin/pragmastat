# Pragmastat Rust Implementation

[![DOI](https://zenodo.org/badge/doi/10.5281/zenodo.17236778.svg)](https://doi.org/10.5281/zenodo.17236778)

A Rust implementation of the Pragmastat statistical toolkit, providing robust statistical estimators for reliable analysis of real-world data.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
pragmastat = "0.1.0"
```

## Usage

```rust
use pragmastat::{center, spread, rel_spread};
use pragmastat::{shift, ratio, avg_spread, disparity};

fn main() {
    // One-sample estimators
    let data = vec![1.2, 3.4, 2.5, 4.1, 2.8];

    println!("Center: {:.4}", center(&data).unwrap());
    println!("Spread: {:.4}", spread(&data).unwrap());
    println!("RelSpread: {:.2}%", rel_spread(&data).unwrap() * 100.0);

    // Two-sample estimators
    let data_x = vec![5.0, 6.0, 7.0, 8.0];
    let data_y = vec![3.0, 4.0, 5.0, 6.0];

    println!("Shift: {:.4}", shift(&data_x, &data_y).unwrap());
    println!("Ratio: {:.4}", ratio(&data_x, &data_y).unwrap());
    println!("AvgSpread: {:.4}", avg_spread(&data_x, &data_y).unwrap());
    println!("Disparity: {:.4}", disparity(&data_x, &data_y).unwrap());
}
```

## One-Sample Estimators

- **center**: Estimates the central value of the data (Hodges-Lehmann location estimator)
- **spread**: Estimates data dispersion (Shamos scale estimator)
- **rel_spread**: Measures the relative dispersion of a sample

## Two-Sample Estimators

- **shift**: Measures the typical difference between elements of two samples
- **ratio**: Measures how many times larger one sample is compared to another
- **avg_spread**: Measures the typical variability when considering both samples together
- **disparity**: Measures effect size as a normalized absolute difference

## License

MIT