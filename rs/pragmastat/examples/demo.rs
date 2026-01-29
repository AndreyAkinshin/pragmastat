use pragmastat::distributions::{Additive, Distribution, Exp, Multiplic, Power, Uniform};
use pragmastat::*;

fn print(result: Result<f64, &str>) {
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

    let mut rng = Rng::from_seed(1729);
    println!("{}", rng.uniform()); // 0.3943034703296536
    println!("{}", rng.uniform()); // 0.5730893757071377

    let mut rng = Rng::from_string("experiment-1");
    println!("{}", rng.uniform()); // 0.9535207726895857

    let mut rng = Rng::from_seed(1729);
    println!(
        "{:?}",
        rng.sample(&[0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0], 3)
    ); // [6, 8, 9]

    let mut rng = Rng::from_seed(1729);
    println!("{:?}", rng.shuffle(&[1.0, 2.0, 3.0, 4.0, 5.0])); // [4, 2, 3, 5, 1]

    // --- Distribution Sampling ---

    let mut rng = Rng::from_seed(1729);
    let dist = Uniform::new(0.0, 10.0);
    println!("{}", dist.sample(&mut rng)); // 3.9430347032965365

    let mut rng = Rng::from_seed(1729);
    let dist = Additive::new(0.0, 1.0);
    println!("{}", dist.sample(&mut rng)); // -1.222932972163442

    let mut rng = Rng::from_seed(1729);
    let dist = Exp::new(1.0);
    println!("{}", dist.sample(&mut rng)); // 0.5013761944646019

    let mut rng = Rng::from_seed(1729);
    let dist = Power::new(1.0, 2.0);
    println!("{}", dist.sample(&mut rng)); // 1.284909255071668

    let mut rng = Rng::from_seed(1729);
    let dist = Multiplic::new(0.0, 1.0);
    println!("{}", dist.sample(&mut rng)); // 0.2943655336550937

    // --- Single-Sample Statistics ---

    let x = vec![0.0, 2.0, 4.0, 6.0, 8.0];

    print(median(&x)); // 4
    print(center(&x)); // 4
    print(spread(&x)); // 4
    print(spread(&add(&x, 10.0))); // 4
    print(spread(&multiply(&x, 2.0))); // 8
    print(rel_spread(&x)); // 1

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

    // --- Confidence Bounds ---

    let x: Vec<f64> = (1..=30).map(|i| i as f64).collect();
    let y: Vec<f64> = (21..=50).map(|i| i as f64).collect();

    println!("{}", pairwise_margin(30, 30, 1e-4).unwrap()); // 390
    print(shift(&x, &y)); // -20

    let bounds = shift_bounds(&x, &y, 1e-4).unwrap(); // [-30, -10]
    println!("{{lower: {}, upper: {}}}", bounds.lower, bounds.upper);
}
