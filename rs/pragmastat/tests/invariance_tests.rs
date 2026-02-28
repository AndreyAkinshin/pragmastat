use float_cmp::approx_eq;
use pragmastat::estimators::raw;
use pragmastat::*;

/// Tests mathematical invariance properties of the estimators
#[cfg(test)]
mod invariance_tests {
    use super::*;

    const SEED: i64 = 1729;
    const SAMPLE_SIZES: [usize; 9] = [2, 3, 4, 5, 6, 7, 8, 9, 10];
    const TOLERANCE: f64 = 1e-9;

    fn perform_test_one<F1, F2>(expr1: F1, expr2: F2)
    where
        F1: Fn(&[f64]) -> f64,
        F2: Fn(&[f64]) -> f64,
    {
        let mut rng = Rng::from_seed(SEED);
        for &n in &SAMPLE_SIZES {
            let x: Vec<f64> = (0..n).map(|_| rng.uniform_f64()).collect();
            let result1 = expr1(&x);
            let result2 = expr2(&x);
            assert!(
                approx_eq!(f64, result1, result2, epsilon = TOLERANCE),
                "Failed for n={}: {} != {}",
                n,
                result1,
                result2
            );
        }
    }

    fn perform_test_two<F1, F2>(expr1: F1, expr2: F2)
    where
        F1: Fn(&[f64], &[f64]) -> f64,
        F2: Fn(&[f64], &[f64]) -> f64,
    {
        let mut rng = Rng::from_seed(SEED);
        for &n in &SAMPLE_SIZES {
            let x: Vec<f64> = (0..n).map(|_| rng.uniform_f64()).collect();
            let y: Vec<f64> = (0..n).map(|_| rng.uniform_f64()).collect();
            let result1 = expr1(&x, &y);
            let result2 = expr2(&x, &y);
            assert!(
                approx_eq!(f64, result1, result2, epsilon = TOLERANCE),
                "Failed for n={}: {} != {}",
                n,
                result1,
                result2
            );
        }
    }

    // Helper functions for vector operations
    fn vec_add_scalar(v: &[f64], scalar: f64) -> Vec<f64> {
        v.iter().map(|&x| x + scalar).collect()
    }

    fn vec_mul_scalar(v: &[f64], scalar: f64) -> Vec<f64> {
        v.iter().map(|&x| x * scalar).collect()
    }

    // Center invariance tests

    #[test]
    fn center_shift() {
        perform_test_one(
            |x| raw::center(&vec_add_scalar(x, 2.0)).unwrap(),
            |x| raw::center(x).unwrap() + 2.0,
        );
    }

    #[test]
    fn center_scale() {
        perform_test_one(
            |x| raw::center(&vec_mul_scalar(x, 2.0)).unwrap(),
            |x| 2.0 * raw::center(x).unwrap(),
        );
    }

    #[test]
    fn center_negate() {
        perform_test_one(
            |x| raw::center(&vec_mul_scalar(x, -1.0)).unwrap(),
            |x| -1.0 * raw::center(x).unwrap(),
        );
    }

    // Spread invariance tests

    #[test]
    fn spread_shift() {
        perform_test_one(
            |x| raw::spread(&vec_add_scalar(x, 2.0)).unwrap(),
            |x| raw::spread(x).unwrap(),
        );
    }

    #[test]
    fn spread_scale() {
        perform_test_one(
            |x| raw::spread(&vec_mul_scalar(x, 2.0)).unwrap(),
            |x| 2.0 * raw::spread(x).unwrap(),
        );
    }

    #[test]
    fn spread_negate() {
        perform_test_one(
            |x| raw::spread(&vec_mul_scalar(x, -1.0)).unwrap(),
            |x| raw::spread(x).unwrap(),
        );
    }

    // RelSpread invariance tests

    #[test]
    #[allow(deprecated)]
    fn rel_spread_scale() {
        perform_test_one(
            |x| raw::rel_spread(&vec_mul_scalar(x, 2.0)).unwrap(),
            |x| raw::rel_spread(x).unwrap(),
        );
    }

    // Shift invariance tests

    #[test]
    fn shift_shift() {
        perform_test_two(
            |x, y| raw::shift(&vec_add_scalar(x, 3.0), &vec_add_scalar(y, 2.0)).unwrap(),
            |x, y| raw::shift(x, y).unwrap() + 1.0,
        );
    }

    #[test]
    fn shift_scale() {
        perform_test_two(
            |x, y| raw::shift(&vec_mul_scalar(x, 2.0), &vec_mul_scalar(y, 2.0)).unwrap(),
            |x, y| 2.0 * raw::shift(x, y).unwrap(),
        );
    }

    #[test]
    fn shift_antisymmetry() {
        perform_test_two(
            |x, y| raw::shift(x, y).unwrap(),
            |x, y| -1.0 * raw::shift(y, x).unwrap(),
        );
    }

    // Ratio invariance tests

    #[test]
    fn ratio_scale() {
        perform_test_two(
            |x, y| raw::ratio(&vec_mul_scalar(x, 2.0), &vec_mul_scalar(y, 3.0)).unwrap(),
            |x, y| (2.0 / 3.0) * raw::ratio(x, y).unwrap(),
        );
    }

    // Disparity invariance tests

    #[test]
    fn disparity_shift() {
        perform_test_two(
            |x, y| raw::disparity(&vec_add_scalar(x, 2.0), &vec_add_scalar(y, 2.0)).unwrap(),
            |x, y| raw::disparity(x, y).unwrap(),
        );
    }

    #[test]
    fn disparity_scale() {
        perform_test_two(
            |x, y| raw::disparity(&vec_mul_scalar(x, 2.0), &vec_mul_scalar(y, 2.0)).unwrap(),
            |x, y| raw::disparity(x, y).unwrap(),
        );
    }

    #[test]
    fn disparity_scale_neg() {
        perform_test_two(
            |x, y| raw::disparity(&vec_mul_scalar(x, -2.0), &vec_mul_scalar(y, -2.0)).unwrap(),
            |x, y| -1.0 * raw::disparity(x, y).unwrap(),
        );
    }

    #[test]
    fn disparity_antisymmetry() {
        perform_test_two(
            |x, y| raw::disparity(x, y).unwrap(),
            |x, y| -1.0 * raw::disparity(y, x).unwrap(),
        );
    }
}

/// Tests randomization invariance properties of Rng operations
#[cfg(test)]
mod rng_invariance {
    use pragmastat::Rng;

    #[test]
    fn shuffle_preserves_multiset() {
        for &n in &[1, 2, 5, 10, 100] {
            let mut rng = Rng::from_seed(42);
            let original: Vec<i64> = (0..n).collect();
            let mut shuffled = rng.shuffle(&original);
            shuffled.sort();
            assert_eq!(
                shuffled, original,
                "Failed for n={}: sorted shuffled != original",
                n
            );
        }
    }

    #[test]
    fn sample_correct_size() {
        let source: Vec<i64> = (0..10).collect();
        let n = source.len();
        for &k in &[1, 3, 5, 10, 15] {
            let mut rng = Rng::from_seed(42);
            let sampled = rng.sample(&source, k);
            let expected_len = k.min(n);
            assert_eq!(
                sampled.len(),
                expected_len,
                "Failed for k={}: expected len {} but got {}",
                k,
                expected_len,
                sampled.len()
            );
        }
    }

    #[test]
    fn sample_elements_from_source() {
        let mut rng = Rng::from_seed(42);
        let source: Vec<i64> = (0..10).collect();
        let sampled = rng.sample(&source, 5);
        for &elem in &sampled {
            assert!(
                source.contains(&elem),
                "Sampled element {} not found in source",
                elem
            );
        }
    }

    #[test]
    fn sample_preserves_order() {
        let mut rng = Rng::from_seed(42);
        let source: Vec<i64> = (0..10).collect();
        let sampled = rng.sample(&source, 5);
        for i in 1..sampled.len() {
            assert!(
                sampled[i - 1] < sampled[i],
                "Elements not in ascending order: sampled[{}]={} >= sampled[{}]={}",
                i - 1,
                sampled[i - 1],
                i,
                sampled[i]
            );
        }
    }

    #[test]
    fn sample_no_duplicates() {
        for &n in &[2, 3, 5, 10, 20] {
            for &k in &[1, n / 2, n] {
                let mut rng = Rng::from_seed(42);
                let source: Vec<i64> = (0..n as i64).collect();
                let sampled = rng.sample(&source, k);
                let mut seen = std::collections::HashSet::new();
                for &elem in &sampled {
                    assert!(
                        seen.insert(elem),
                        "Duplicate element {} in sample(n={}, k={})",
                        elem,
                        n,
                        k
                    );
                }
            }
        }
    }

    #[test]
    fn resample_elements_from_source() {
        let mut rng = Rng::from_seed(42);
        let source: Vec<i64> = (0..5).collect();
        let resampled = rng.resample(&source, 10);
        for &elem in &resampled {
            assert!(
                source.contains(&elem),
                "Resampled element {} not found in source",
                elem
            );
        }
    }

    #[test]
    #[should_panic]
    fn resample_k0_panics() {
        let mut rng = Rng::from_seed(42);
        let source: Vec<i64> = vec![1, 2, 3];
        rng.resample(&source, 0);
    }

    #[test]
    #[should_panic]
    fn shuffle_empty_panics() {
        let mut rng = Rng::from_seed(42);
        let empty: Vec<i64> = vec![];
        rng.shuffle(&empty);
    }

    #[test]
    #[should_panic]
    fn sample_k0_panics() {
        let mut rng = Rng::from_seed(42);
        let source: Vec<i64> = vec![1, 2, 3];
        rng.sample(&source, 0);
    }

    #[test]
    #[should_panic]
    fn sample_empty_panics() {
        let mut rng = Rng::from_seed(42);
        let empty: Vec<i64> = vec![];
        rng.sample(&empty, 1);
    }
}
