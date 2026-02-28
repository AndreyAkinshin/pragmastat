# Rust

Install from crates.io via cargo:

```bash
cargo add pragmastat@10.0.6
```

Install from crates.io via `Cargo.toml`:

```toml
[dependencies]
pragmastat = "10.0.6"
```

Source code: https://github.com/AndreyAkinshin/pragmastat/tree/v10.0.6/rs

Pragmastat on crates.io: https://crates.io/crates/pragmastat

## Demo

```rust
use pragmastat::distributions::{Additive, Distribution, Exp, Multiplic, Power, Uniform};
use pragmastat::*;

fn main() {
    // --- One-Sample ---

    let x: Vec<f64> = (1..=22).map(|i| i as f64).collect();

    println!("{}", center(&x).unwrap()); // 11.5
    let bounds = center_bounds(&x, 1e-3).unwrap();
    println!("{{lower: {}, upper: {}}}", bounds.lower, bounds.upper); // {lower: 6, upper: 17}
    println!("{}", spread(&x).unwrap()); // 7
    let bounds = spread_bounds_with_seed(&x, 1e-3, "demo").unwrap();
    println!("{{lower: {}, upper: {}}}", bounds.lower, bounds.upper); // {lower: 1, upper: 18}

    // --- Two-Sample ---

    let x: Vec<f64> = (1..=30).map(|i| i as f64).collect();
    let y: Vec<f64> = (21..=50).map(|i| i as f64).collect();

    println!("{}", shift(&x, &y).unwrap()); // -20
    let bounds = shift_bounds(&x, &y, 1e-3).unwrap();
    println!("{{lower: {}, upper: {}}}", bounds.lower, bounds.upper); // {lower: -28, upper: -12}
    println!("{}", ratio(&x, &y).unwrap()); // 0.43669798282695127
    let bounds = ratio_bounds(&x, &y, 1e-3).unwrap();
    println!("{{lower: {}, upper: {}}}", bounds.lower, bounds.upper); // {lower: 0.23255813953488377, upper: 0.6428571428571428}
    println!("{}", disparity(&x, &y).unwrap()); // -2.2222222222222223
    let bounds = disparity_bounds_with_seed(&x, &y, 1e-3, "demo").unwrap();
    println!("{{lower: {}, upper: {}}}", bounds.lower, bounds.upper); // {lower: -29, upper: -0.4782608695652174}

    // --- Randomization ---

    let mut rng = Rng::from_string("demo-uniform");
    println!("{}", rng.uniform_f64()); // 0.2640554428629759
    println!("{}", rng.uniform_f64()); // 0.9348534835582796

    let mut rng = Rng::from_string("demo-uniform-int");
    println!("{}", rng.uniform_i64(0, 100)); // 41

    let mut rng = Rng::from_string("demo-sample");
    println!(
        "{:?}",
        rng.sample(&[0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0], 3)
    ); // [3.0, 8.0, 9.0]

    let mut rng = Rng::from_string("demo-resample");
    println!("{:?}", rng.resample(&[1.0, 2.0, 3.0, 4.0, 5.0], 7)); // [3.0, 1.0, 3.0, 2.0, 4.0, 1.0, 2.0]

    let mut rng = Rng::from_string("demo-shuffle");
    println!("{:?}", rng.shuffle(&[1.0, 2.0, 3.0, 4.0, 5.0])); // [4.0, 2.0, 3.0, 5.0, 1.0]

    // --- Distributions ---

    let mut rng = Rng::from_string("demo-dist-additive");
    println!("{}", Additive::new(0.0, 1.0).sample(&mut rng)); // 0.17410448679568188

    let mut rng = Rng::from_string("demo-dist-multiplic");
    println!("{}", Multiplic::new(0.0, 1.0).sample(&mut rng)); // 1.1273244602673853

    let mut rng = Rng::from_string("demo-dist-exp");
    println!("{}", Exp::new(1.0).sample(&mut rng)); // 0.6589065267276553

    let mut rng = Rng::from_string("demo-dist-power");
    println!("{}", Power::new(1.0, 2.0).sample(&mut rng)); // 1.023677535537084

    let mut rng = Rng::from_string("demo-dist-uniform");
    println!("{}", Uniform::new(0.0, 10.0).sample(&mut rng)); // 6.54043657816832
}
```
