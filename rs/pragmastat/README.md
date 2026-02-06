# Rust

Install from crates.io via cargo:

```bash
cargo add pragmastat@6.0.1
```

Install from crates.io via `Cargo.toml`:

```toml
[dependencies]
pragmastat = "6.0.1"
```

Source code: https://github.com/AndreyAkinshin/pragmastat/tree/v6.0.1/rs

Pragmastat on crates.io: https://crates.io/crates/pragmastat

## Demo

```rust
use pragmastat::distributions::{Additive, Distribution, Exp, Multiplic, Power, Uniform};
use pragmastat::*;

fn print<E: std::fmt::Debug>(result: Result<f64, E>) {
    println!("{}", result.unwrap());
}

fn add(x: &[f64], val: f64) -> Vec<f64> {
    x.iter().map(|v| v + val).collect()
}

fn multiply(x: &[f64], val: f64) -> Vec<f64> {
    x.iter().map(|v| v * val).collect()
}

fn main() {
    // --- Randomization ---

    let mut rng = Rng::from_string("demo-uniform");
    println!("{}", rng.uniform()); // 0.2640554428629759
    println!("{}", rng.uniform()); // 0.9348534835582796

    let mut rng = Rng::from_string("demo-sample");
    println!(
        "{:?}",
        rng.sample(&[0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0], 3)
    ); // [3, 8, 9]

    let mut rng = Rng::from_string("demo-shuffle");
    println!("{:?}", rng.shuffle(&[1.0, 2.0, 3.0, 4.0, 5.0])); // [4, 2, 3, 5, 1]

    let mut rng = Rng::from_string("demo-resample");
    println!("{:?}", rng.resample(&[1.0, 2.0, 3.0, 4.0, 5.0], 7)); // [5, 1, 1, 3, 3, 4, 5]

    // --- Distribution Sampling ---

    let mut rng = Rng::from_string("demo-dist-uniform");
    let dist = Uniform::new(0.0, 10.0);
    println!("{}", dist.sample(&mut rng)); // 6.54043657816832

    let mut rng = Rng::from_string("demo-dist-additive");
    let dist = Additive::new(0.0, 1.0);
    println!("{}", dist.sample(&mut rng)); // 0.17410448679568188

    let mut rng = Rng::from_string("demo-dist-exp");
    let dist = Exp::new(1.0);
    println!("{}", dist.sample(&mut rng)); // 0.6589065267276553

    let mut rng = Rng::from_string("demo-dist-power");
    let dist = Power::new(1.0, 2.0);
    println!("{}", dist.sample(&mut rng)); // 1.023677535537084

    let mut rng = Rng::from_string("demo-dist-multiplic");
    let dist = Multiplic::new(0.0, 1.0);
    println!("{}", dist.sample(&mut rng)); // 1.1273244602673853

    // --- Single-Sample Statistics ---

    let x = vec![1.0, 3.0, 5.0, 7.0, 9.0];

    print(median(&x)); // 5
    print(center(&x)); // 5
    print(spread(&x)); // 4
    print(spread(&add(&x, 10.0))); // 4
    print(spread(&multiply(&x, 2.0))); // 8
    print(rel_spread(&x)); // 0.8

    // --- Two-Sample Comparison ---

    let x = vec![0.0, 3.0, 6.0, 9.0, 12.0];
    let y = vec![0.0, 2.0, 4.0, 6.0, 8.0];

    print(shift(&x, &y)); // 2
    print(shift(&y, &x)); // -2
    print(avg_spread(&x, &y)); // 5
    print(disparity(&x, &y)); // 0.4
    print(disparity(&y, &x)); // -0.4

    let x = vec![1.0, 2.0, 4.0, 8.0, 16.0];
    let y = vec![2.0, 4.0, 8.0, 16.0, 32.0];
    print(ratio(&x, &y)); // 0.5
    print(ratio(&y, &x)); // 2

    // --- One-Sample Bounds ---

    let x: Vec<f64> = (1..=10).map(|i| i as f64).collect();

    println!("{}", signed_rank_margin(10, 0.05).unwrap()); // 18
    print(center(&x)); // 5.5
    let bounds = center_bounds(&x, 0.05).unwrap(); // {lower: 3.5, upper: 7.5}
    println!("{{lower: {}, upper: {}}}", bounds.lower, bounds.upper);
    let bounds = median_bounds(&x, 0.05).unwrap(); // {lower: 2, upper: 9}
    println!("{{lower: {}, upper: {}}}", bounds.lower, bounds.upper);

    // --- Two-Sample Bounds ---

    let x: Vec<f64> = (1..=30).map(|i| i as f64).collect();
    let y: Vec<f64> = (21..=50).map(|i| i as f64).collect();

    println!("{}", pairwise_margin(30, 30, 1e-4).unwrap()); // 390
    print(shift(&x, &y)); // -20

    let bounds = shift_bounds(&x, &y, 1e-4).unwrap(); // {lower: -30, upper: -10}
    println!("{{lower: {}, upper: {}}}", bounds.lower, bounds.upper);

    let x = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    let y = vec![2.0, 3.0, 4.0, 5.0, 6.0];
    let bounds = ratio_bounds(&x, &y, 0.05).unwrap(); // {lower: 0.333..., upper: 1.5}
    println!("{{lower: {}, upper: {}}}", bounds.lower, bounds.upper);
}
```
