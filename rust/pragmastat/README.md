# Pragmastat Rust Implementation

A Rust implementation of the Pragmastat statistical toolkit, providing robust statistical estimators for reliable analysis of real-world data.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
pragmastat = "0.1.0"
```

## Usage

```rust
use pragmastat::{center, spread, volatility, precision};
use pragmastat::{med_shift, med_ratio, med_spread, med_disparity};

fn main() {
    // One-sample estimators
    let data = vec![1.2, 3.4, 2.5, 4.1, 2.8];
    
    println!("Center: {}", center(&data));
    println!("Spread: {}", spread(&data));
    println!("Volatility: {}", volatility(&data));
    println!("Precision: {}", precision(&data));
    
    // Two-sample estimators
    let data_x = vec![5.0, 6.0, 7.0, 8.0];
    let data_y = vec![3.0, 4.0, 5.0, 6.0];
    
    println!("Med Shift: {}", med_shift(&data_x, &data_y));
    println!("Med Ratio: {}", med_ratio(&data_x, &data_y));
    println!("Med Spread: {}", med_spread(&data_x, &data_y));
    println!("Med Disparity: {}", med_disparity(&data_x, &data_y));
}
```

## One-Sample Estimators

- **center**: Estimates the central value of the data (Hodges-Lehmann location estimator)
- **spread**: Estimates data dispersion (Shamos scale estimator)
- **volatility**: Measures the relative dispersion of a sample
- **precision**: Measures precision as the distance between two estimations

## Two-Sample Estimators

- **med_shift**: Measures the typical difference between elements of two samples
- **med_ratio**: Measures how many times larger one sample is compared to another
- **med_spread**: Measures the typical variability when considering both samples together
- **med_disparity**: Measures effect size as a normalized absolute difference

## License

MIT