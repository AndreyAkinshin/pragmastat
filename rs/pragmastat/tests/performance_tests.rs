use pragmastat::estimators::raw;
use std::time::Instant;

#[test]
fn test_center_performance() {
    let n = 100000;
    let x: Vec<f64> = (1..=n).map(|i| i as f64).collect();

    let start = Instant::now();
    let result = raw::center(&x).unwrap();
    let elapsed = start.elapsed();

    println!("\nCenter for n={}: {:.6}", n, result);
    println!("Elapsed time: {:?}", elapsed);

    let expected = 50000.5;
    assert!(
        (result - expected).abs() < 1e-9,
        "Center for n={}: expected {}, got {}",
        n,
        expected,
        result
    );
    assert!(elapsed.as_secs() < 5, "Performance too slow: {:?}", elapsed);
}

#[test]
fn test_spread_performance() {
    let n = 100000;
    let x: Vec<f64> = (1..=n).map(|i| i as f64).collect();

    let start = Instant::now();
    let result = raw::spread(&x).unwrap();
    let elapsed = start.elapsed();

    println!("\nSpread for n={}: {:.6}", n, result);
    println!("Elapsed time: {:?}", elapsed);

    let expected = 29290.0;
    assert!(
        (result - expected).abs() < 1e-9,
        "Spread for n={}: expected {}, got {}",
        n,
        expected,
        result
    );
    assert!(elapsed.as_secs() < 5, "Performance too slow: {:?}", elapsed);
}

#[test]
fn test_shift_performance() {
    let n = 100000;
    let x: Vec<f64> = (1..=n).map(|i| i as f64).collect();
    let y: Vec<f64> = (1..=n).map(|i| i as f64).collect();

    let start = Instant::now();
    let result = raw::shift(&x, &y).unwrap();
    let elapsed = start.elapsed();

    println!("\nShift for n=m={}: {:.6}", n, result);
    println!("Elapsed time: {:?}", elapsed);

    let expected = 0.0;
    assert!(
        (result - expected).abs() < 1e-9,
        "Shift for n=m={}: expected {}, got {}",
        n,
        expected,
        result
    );
    assert!(elapsed.as_secs() < 5, "Performance too slow: {:?}", elapsed);
}
