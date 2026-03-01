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
    // --- One-Sample (Sample-based API) ---

    let x = Sample::new((1..=22).map(|i| i as f64).collect()).unwrap();

    println!("{}", center(&x).unwrap().value); // 11.5
    let bounds = center_bounds(&x, 1e-3).unwrap();
    println!("{{lower: {}, upper: {}}}", bounds.lower, bounds.upper); // {lower: 6, upper: 17}
    println!("{}", spread(&x).unwrap().value); // 7
    let bounds = spread_bounds_with_seed(&x, 1e-3, "demo").unwrap();
    println!("{{lower: {}, upper: {}}}", bounds.lower, bounds.upper); // {lower: 1, upper: 18}

    // --- Two-Sample (Sample-based API) ---

    let x = Sample::new((1..=30).map(|i| i as f64).collect()).unwrap();
    let y = Sample::new((21..=50).map(|i| i as f64).collect()).unwrap();

    println!("{}", shift(&x, &y).unwrap().value); // -20
    let bounds = shift_bounds(&x, &y, 1e-3).unwrap();
    println!("{{lower: {}, upper: {}}}", bounds.lower, bounds.upper); // {lower: -28, upper: -12}
    println!("{}", ratio(&x, &y).unwrap().value); // 0.43669798282695127
    let bounds = ratio_bounds(&x, &y, 1e-3).unwrap();
    println!("{{lower: {}, upper: {}}}", bounds.lower, bounds.upper); // {lower: 0.23255813953488377, upper: 0.6428571428571428}
    println!("{}", disparity(&x, &y).unwrap().value); // -2.2222222222222223
    let bounds = disparity_bounds_with_seed(&x, &y, 1e-3, "demo").unwrap();
    println!("{{lower: {}, upper: {}}}", bounds.lower, bounds.upper); // {lower: -29, upper: -0.4782608695652174}

    // --- Raw slice API (backward-compatible) ---

    let x_raw: Vec<f64> = (1..=22).map(|i| i as f64).collect();
    println!("{}", estimators::raw::center(&x_raw).unwrap()); // 11.5

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

    // --- Unit system ---

    let ms = MeasurementUnit::new("ms", "Time", "ms", "Millisecond", 1_000_000);
    let ns = MeasurementUnit::new("ns", "Time", "ns", "Nanosecond", 1);

    let s = Sample::with_unit(vec![1.0, 2.0, 3.0], ms).unwrap();
    let converted = s.convert_to(&ns).unwrap();
    println!("Converted: {:?}", converted.values()); // [1000000.0, 2000000.0, 3000000.0]

    let registry = UnitRegistry::standard();
    let unit = registry.resolve("number").unwrap();
    println!("Resolved: {}", unit.full_name()); // Number
}
```
