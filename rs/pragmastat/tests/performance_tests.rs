use pragmastat::{center, spread};
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use std::time::Instant;

fn center_simple(x: &[f64]) -> f64 {
    let n = x.len();
    let mut pairwise_averages = Vec::new();
    for i in 0..n {
        for j in i..n {
            pairwise_averages.push((x[i] + x[j]) / 2.0);
        }
    }
    pairwise_averages.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let m = pairwise_averages.len();
    if m % 2 == 0 {
        (pairwise_averages[m / 2 - 1] + pairwise_averages[m / 2]) / 2.0
    } else {
        pairwise_averages[m / 2]
    }
}

fn spread_simple(x: &[f64]) -> f64 {
    let n = x.len();
    if n == 1 {
        return 0.0;
    }
    let mut pairwise_diffs = Vec::new();
    for i in 0..n {
        for j in (i + 1)..n {
            pairwise_diffs.push((x[i] - x[j]).abs());
        }
    }
    pairwise_diffs.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let m = pairwise_diffs.len();
    if m % 2 == 0 {
        (pairwise_diffs[m / 2 - 1] + pairwise_diffs[m / 2]) / 2.0
    } else {
        pairwise_diffs[m / 2]
    }
}

#[test]
fn test_center_correctness() {
    let mut rng = StdRng::seed_from_u64(1729);

    for n in 1..=100 {
        for _ in 0..n {
            let x: Vec<f64> = (0..n).map(|_| rng.gen::<f64>() * 2.0 - 1.0).collect();

            let expected = center_simple(&x);
            let actual = center(&x).unwrap();

            assert!(
                (expected - actual).abs() < 1e-9,
                "Mismatch for n={}: expected={}, actual={}",
                n,
                expected,
                actual
            );
        }
    }
    println!("✓ Center correctness tests passed");
}

#[test]
fn test_spread_correctness() {
    let mut rng = StdRng::seed_from_u64(1729);

    for n in 1..=100 {
        for _ in 0..n {
            let x: Vec<f64> = (0..n).map(|_| rng.gen::<f64>() * 2.0 - 1.0).collect();

            let expected = spread_simple(&x);
            let actual = spread(&x).unwrap();

            assert!(
                (expected - actual).abs() < 1e-9,
                "Mismatch for n={}: expected={}, actual={}",
                n,
                expected,
                actual
            );
        }
    }
    println!("✓ Spread correctness tests passed");
}

#[test]
fn test_center_performance() {
    let mut rng = StdRng::seed_from_u64(1729);
    let n = 100000;
    let x: Vec<f64> = (0..n).map(|_| rng.gen::<f64>() * 2.0 - 1.0).collect();

    let start = Instant::now();
    let result = center(&x).unwrap();
    let elapsed = start.elapsed();

    println!("\nCenter for n={}: {:.6}", n, result);
    println!("Elapsed time: {:?}", elapsed);

    assert!(elapsed.as_secs() < 5, "Performance too slow: {:?}", elapsed);
}

#[test]
fn test_spread_performance() {
    let mut rng = StdRng::seed_from_u64(1729);
    let n = 100000;
    let x: Vec<f64> = (0..n).map(|_| rng.gen::<f64>() * 2.0 - 1.0).collect();

    let start = Instant::now();
    let result = spread(&x).unwrap();
    let elapsed = start.elapsed();

    println!("\nSpread for n={}: {:.6}", n, result);
    println!("Elapsed time: {:?}", elapsed);

    assert!(elapsed.as_secs() < 5, "Performance too slow: {:?}", elapsed);
}
