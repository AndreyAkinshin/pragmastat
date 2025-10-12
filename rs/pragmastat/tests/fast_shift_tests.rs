use float_cmp::approx_eq;
use pragmastat::shift;
use rand::Rng;

const TOLERANCE: f64 = 1e-9;

fn naive_shift(x: &[f64], y: &[f64]) -> f64 {
    let mut diffs = Vec::new();
    for &xi in x {
        for &yj in y {
            diffs.push(xi - yj);
        }
    }
    diffs.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let n = diffs.len();
    if n % 2 == 0 {
        (diffs[n / 2 - 1] + diffs[n / 2]) / 2.0
    } else {
        diffs[n / 2]
    }
}

#[test]
fn test_small_arrays_match_naive() {
    let mut rng = rand::thread_rng();

    for m in 1..=20 {
        for n in 1..=20 {
            for _ in 0..5 {
                let x: Vec<f64> = (0..m).map(|_| rng.gen_range(-10.0..10.0)).collect();
                let y: Vec<f64> = (0..n).map(|_| rng.gen_range(-10.0..10.0)).collect();

                let actual = shift(&x, &y).unwrap();
                let expected = naive_shift(&x, &y);

                assert!(
                    approx_eq!(f64, actual, expected, epsilon = TOLERANCE),
                    "Failed for m={}, n={}: expected {}, got {}",
                    m,
                    n,
                    expected,
                    actual
                );
            }
        }
    }
}

#[test]
fn test_medium_arrays_match_naive() {
    let mut rng = rand::thread_rng();

    for size in (20..=100).step_by(10) {
        for _ in 0..3 {
            let x: Vec<f64> = (0..size).map(|_| rng.gen_range(-50.0..50.0)).collect();
            let y: Vec<f64> = (0..size / 2).map(|_| rng.gen_range(-50.0..50.0)).collect();

            let actual = shift(&x, &y).unwrap();
            let expected = naive_shift(&x, &y);

            assert!(
                approx_eq!(f64, actual, expected, epsilon = TOLERANCE),
                "Failed for size={}: expected {}, got {}",
                size,
                expected,
                actual
            );
        }
    }
}

#[test]
fn test_unsorted_input_matches_sorted() {
    let mut rng = rand::thread_rng();

    for _ in 0..50 {
        let mut x: Vec<f64> = (0..20).map(|_| rng.gen_range(-10.0..10.0)).collect();
        let mut y: Vec<f64> = (0..15).map(|_| rng.gen_range(-10.0..10.0)).collect();

        let result_unsorted = shift(&x, &y).unwrap();

        x.sort_by(|a, b| a.partial_cmp(b).unwrap());
        y.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let result_sorted = shift(&x, &y).unwrap();

        assert!(
            approx_eq!(f64, result_unsorted, result_sorted, epsilon = TOLERANCE),
            "Sorted and unsorted results differ: sorted={}, unsorted={}",
            result_sorted,
            result_unsorted
        );
    }
}

#[test]
fn test_single_element_returns_constant() {
    let mut rng = rand::thread_rng();

    for _ in 0..20 {
        let x = vec![rng.gen_range(-10.0..10.0)];
        let y = vec![rng.gen_range(-10.0..10.0)];

        let result = shift(&x, &y).unwrap();
        let expected = x[0] - y[0];

        assert!(
            approx_eq!(f64, result, expected, epsilon = TOLERANCE),
            "Expected {}, got {}",
            expected,
            result
        );
    }
}

#[test]
fn test_identical_arrays_shift_is_zero() {
    let mut rng = rand::thread_rng();

    for size in 1..=30 {
        for _ in 0..3 {
            let x: Vec<f64> = (0..size).map(|_| rng.gen_range(-10.0..10.0)).collect();

            let result = shift(&x, &x).unwrap();

            assert!(
                approx_eq!(f64, result, 0.0, epsilon = TOLERANCE),
                "Identical arrays should have shift=0, got {}",
                result
            );
        }
    }
}

#[test]
fn test_asymmetric_sizes() {
    let mut rng = rand::thread_rng();

    let configs = vec![(1, 100), (100, 1), (10, 50), (50, 10), (5, 200)];

    for (m, n) in configs {
        let x: Vec<f64> = (0..m).map(|_| rng.gen_range(-10.0..10.0)).collect();
        let y: Vec<f64> = (0..n).map(|_| rng.gen_range(-10.0..10.0)).collect();

        let actual = shift(&x, &y).unwrap();
        let expected = naive_shift(&x, &y);

        assert!(
            approx_eq!(f64, actual, expected, epsilon = TOLERANCE),
            "Failed for m={}, n={}: expected {}, got {}",
            m,
            n,
            expected,
            actual
        );
    }
}

#[test]
fn test_negative_values() {
    let mut rng = rand::thread_rng();

    for _ in 0..20 {
        let x: Vec<f64> = (0..15).map(|_| rng.gen_range(-100.0..-50.0)).collect();
        let y: Vec<f64> = (0..12).map(|_| rng.gen_range(-100.0..-50.0)).collect();

        let actual = shift(&x, &y).unwrap();
        let expected = naive_shift(&x, &y);

        assert!(
            approx_eq!(f64, actual, expected, epsilon = TOLERANCE),
            "Failed with negative values: expected {}, got {}",
            expected,
            actual
        );
    }
}

#[test]
fn test_duplicate_values() {
    let mut rng = rand::thread_rng();

    for _ in 0..10 {
        let x: Vec<f64> = (0..12)
            .map(|_| {
                let val: f64 = rng.gen_range(-10.0..10.0);
                (val * 2.0).round() / 2.0
            })
            .collect();
        let y: Vec<f64> = (0..10)
            .map(|_| {
                let val: f64 = rng.gen_range(-10.0..10.0);
                (val * 2.0).round() / 2.0
            })
            .collect();

        let actual = shift(&x, &y).unwrap();
        let expected = naive_shift(&x, &y);

        assert!(
            approx_eq!(f64, actual, expected, epsilon = TOLERANCE),
            "Failed with duplicates: expected {}, got {}",
            expected,
            actual
        );
    }
}

#[test]
fn test_very_small_values_numerical_stability() {
    let mut rng = rand::thread_rng();

    for _ in 0..10 {
        let x: Vec<f64> = (0..10).map(|_| rng.gen_range(-1e-8..1e-8)).collect();
        let y: Vec<f64> = (0..10).map(|_| rng.gen_range(-1e-8..1e-8)).collect();

        let result = shift(&x, &y).unwrap();

        assert!(!result.is_nan(), "Result should not be NaN");
        assert!(!result.is_infinite(), "Result should not be infinite");
    }
}

#[test]
fn test_large_values_numerical_stability() {
    let mut rng = rand::thread_rng();

    for _ in 0..10 {
        let x: Vec<f64> = (0..10).map(|_| rng.gen_range(9e5..11e5)).collect();
        let y: Vec<f64> = (0..10).map(|_| rng.gen_range(9e5..11e5)).collect();

        let result = shift(&x, &y).unwrap();

        assert!(!result.is_nan(), "Result should not be NaN");
        assert!(!result.is_infinite(), "Result should not be infinite");
    }
}

#[test]
fn test_zero_spread_all_same() {
    let x = vec![5.0; 10];
    let y = vec![2.0; 8];

    let result = shift(&x, &y).unwrap();

    assert!(
        approx_eq!(f64, result, 3.0, epsilon = TOLERANCE),
        "Expected 3.0, got {}",
        result
    );
}

#[test]
fn test_shift_invariance_x_shift() {
    let mut rng = rand::thread_rng();

    for _ in 0..10 {
        let x: Vec<f64> = (0..15).map(|_| rng.gen_range(-10.0..10.0)).collect();
        let y: Vec<f64> = (0..12).map(|_| rng.gen_range(-10.0..10.0)).collect();
        let shift_amount = rng.gen_range(-100.0..100.0);

        let result1 = shift(&x, &y).unwrap();

        let x_shifted: Vec<f64> = x.iter().map(|v| v + shift_amount).collect();
        let result2 = shift(&x_shifted, &y).unwrap();

        assert!(
            approx_eq!(f64, result1 + shift_amount, result2, epsilon = TOLERANCE),
            "Shift invariance failed: result1={}, result2={}, shift={}",
            result1,
            result2,
            shift_amount
        );
    }
}

#[test]
fn test_shift_invariance_y_shift() {
    let mut rng = rand::thread_rng();

    for _ in 0..10 {
        let x: Vec<f64> = (0..15).map(|_| rng.gen_range(-10.0..10.0)).collect();
        let y: Vec<f64> = (0..12).map(|_| rng.gen_range(-10.0..10.0)).collect();
        let shift_amount = rng.gen_range(-100.0..100.0);

        let result1 = shift(&x, &y).unwrap();

        let y_shifted: Vec<f64> = y.iter().map(|v| v + shift_amount).collect();
        let result2 = shift(&x, &y_shifted).unwrap();

        assert!(
            approx_eq!(f64, result1 - shift_amount, result2, epsilon = TOLERANCE),
            "Shift invariance failed: result1={}, result2={}, shift={}",
            result1,
            result2,
            shift_amount
        );
    }
}

#[test]
fn test_scale_invariance() {
    let mut rng = rand::thread_rng();

    for _ in 0..10 {
        let x: Vec<f64> = (0..15).map(|_| rng.gen_range(-10.0..10.0)).collect();
        let y: Vec<f64> = (0..12).map(|_| rng.gen_range(-10.0..10.0)).collect();
        let scale = 2.0;

        let result1 = shift(&x, &y).unwrap();

        let x_scaled: Vec<f64> = x.iter().map(|v| v * scale).collect();
        let y_scaled: Vec<f64> = y.iter().map(|v| v * scale).collect();
        let result2 = shift(&x_scaled, &y_scaled).unwrap();

        assert!(
            approx_eq!(f64, result1 * scale, result2, epsilon = 1e-6),
            "Scale invariance failed: result1={}, result2={}, scale={}",
            result1,
            result2,
            scale
        );
    }
}

#[test]
fn test_empty_x_returns_error() {
    let x: Vec<f64> = vec![];
    let y = vec![1.0, 2.0];

    assert!(shift(&x, &y).is_err());
}

#[test]
fn test_empty_y_returns_error() {
    let x = vec![1.0, 2.0];
    let y: Vec<f64> = vec![];

    assert!(shift(&x, &y).is_err());
}

#[test]
fn test_performance_large_arrays() {
    let mut rng = rand::thread_rng();
    let x: Vec<f64> = (0..500).map(|_| rng.gen_range(-100.0..100.0)).collect();
    let y: Vec<f64> = (0..500).map(|_| rng.gen_range(-100.0..100.0)).collect();

    let start = std::time::Instant::now();
    let result = shift(&x, &y).unwrap();
    let duration = start.elapsed();

    println!("500x500 arrays: {:?}", duration);
    assert!(duration.as_secs() < 5);
    assert!(!result.is_nan());
}

#[test]
fn test_performance_very_large_arrays() {
    let mut rng = rand::thread_rng();
    let x: Vec<f64> = (0..1000).map(|_| rng.gen_range(-100.0..100.0)).collect();
    let y: Vec<f64> = (0..1000).map(|_| rng.gen_range(-100.0..100.0)).collect();

    let start = std::time::Instant::now();
    let result = shift(&x, &y).unwrap();
    let duration = start.elapsed();

    println!("1000x1000 arrays (1M pairs): {:?}", duration);
    assert!(duration.as_secs() < 10);
    assert!(!result.is_nan());
}
