use float_cmp::approx_eq;
use pragmastat::*;

/// Tests mathematical invariance properties of the estimators
#[cfg(test)]
mod invariance_tests {
    use super::*;

    const SEED: u64 = 1729;
    const SAMPLE_SIZES: [usize; 9] = [2, 3, 4, 5, 6, 7, 8, 9, 10];
    const TOLERANCE: f64 = 1e-9;

    /// Simple linear congruential generator for reproducible random numbers
    struct SimpleRng {
        state: u64,
    }

    impl SimpleRng {
        fn new(seed: u64) -> Self {
            SimpleRng { state: seed }
        }

        fn next_f64(&mut self) -> f64 {
            // Simple LCG parameters
            self.state = (self.state.wrapping_mul(1103515245).wrapping_add(12345)) & 0x7fffffff;
            self.state as f64 / 0x7fffffff as f64
        }

        fn next_vec(&mut self, n: usize) -> Vec<f64> {
            (0..n).map(|_| self.next_f64()).collect()
        }
    }

    fn perform_test_one<F1, F2>(expr1: F1, expr2: F2)
    where
        F1: Fn(&[f64]) -> f64,
        F2: Fn(&[f64]) -> f64,
    {
        let mut rng = SimpleRng::new(SEED);
        for &n in &SAMPLE_SIZES {
            let x = rng.next_vec(n);
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
        let mut rng = SimpleRng::new(SEED);
        for &n in &SAMPLE_SIZES {
            let x = rng.next_vec(n);
            let y = rng.next_vec(n);
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
            |x| center(&vec_add_scalar(x, 2.0)).unwrap(),
            |x| center(x).unwrap() + 2.0,
        );
    }

    #[test]
    fn center_scale() {
        perform_test_one(
            |x| center(&vec_mul_scalar(x, 2.0)).unwrap(),
            |x| 2.0 * center(x).unwrap(),
        );
    }

    #[test]
    fn center_negate() {
        perform_test_one(
            |x| center(&vec_mul_scalar(x, -1.0)).unwrap(),
            |x| -1.0 * center(x).unwrap(),
        );
    }

    // Spread invariance tests

    #[test]
    fn spread_shift() {
        perform_test_one(
            |x| spread(&vec_add_scalar(x, 2.0)).unwrap(),
            |x| spread(x).unwrap(),
        );
    }

    #[test]
    fn spread_scale() {
        perform_test_one(
            |x| spread(&vec_mul_scalar(x, 2.0)).unwrap(),
            |x| 2.0 * spread(x).unwrap(),
        );
    }

    #[test]
    fn spread_negate() {
        perform_test_one(
            |x| spread(&vec_mul_scalar(x, -1.0)).unwrap(),
            |x| spread(x).unwrap(),
        );
    }

    // RelSpread invariance tests

    #[test]
    fn rel_spread_scale() {
        perform_test_one(
            |x| rel_spread(&vec_mul_scalar(x, 2.0)).unwrap(),
            |x| rel_spread(x).unwrap(),
        );
    }

    // Shift invariance tests

    #[test]
    fn shift_shift() {
        perform_test_two(
            |x, y| shift(&vec_add_scalar(x, 3.0), &vec_add_scalar(y, 2.0)).unwrap(),
            |x, y| shift(x, y).unwrap() + 1.0,
        );
    }

    #[test]
    fn shift_scale() {
        perform_test_two(
            |x, y| shift(&vec_mul_scalar(x, 2.0), &vec_mul_scalar(y, 2.0)).unwrap(),
            |x, y| 2.0 * shift(x, y).unwrap(),
        );
    }

    #[test]
    fn shift_antisymmetry() {
        perform_test_two(
            |x, y| shift(x, y).unwrap(),
            |x, y| -1.0 * shift(y, x).unwrap(),
        );
    }

    // Ratio invariance tests

    #[test]
    fn ratio_scale() {
        perform_test_two(
            |x, y| ratio(&vec_mul_scalar(x, 2.0), &vec_mul_scalar(y, 3.0)).unwrap(),
            |x, y| (2.0 / 3.0) * ratio(x, y).unwrap(),
        );
    }

    // AvgSpread invariance tests

    #[test]
    fn avg_spread_equal() {
        perform_test_one(|x| avg_spread(x, x).unwrap(), |x| spread(x).unwrap());
    }

    #[test]
    fn avg_spread_symmetry() {
        perform_test_two(
            |x, y| avg_spread(x, y).unwrap(),
            |x, y| avg_spread(y, x).unwrap(),
        );
    }

    #[test]
    fn avg_spread_average() {
        perform_test_one(
            |x| avg_spread(x, &vec_mul_scalar(x, 5.0)).unwrap(),
            |x| 3.0 * spread(x).unwrap(),
        );
    }

    #[test]
    fn avg_spread_scale() {
        perform_test_two(
            |x, y| avg_spread(&vec_mul_scalar(x, -2.0), &vec_mul_scalar(y, -2.0)).unwrap(),
            |x, y| 2.0 * avg_spread(x, y).unwrap(),
        );
    }

    // Disparity invariance tests

    #[test]
    fn disparity_shift() {
        perform_test_two(
            |x, y| disparity(&vec_add_scalar(x, 2.0), &vec_add_scalar(y, 2.0)).unwrap(),
            |x, y| disparity(x, y).unwrap(),
        );
    }

    #[test]
    fn disparity_scale() {
        perform_test_two(
            |x, y| disparity(&vec_mul_scalar(x, 2.0), &vec_mul_scalar(y, 2.0)).unwrap(),
            |x, y| disparity(x, y).unwrap(),
        );
    }

    #[test]
    fn disparity_scale_neg() {
        perform_test_two(
            |x, y| disparity(&vec_mul_scalar(x, -2.0), &vec_mul_scalar(y, -2.0)).unwrap(),
            |x, y| -1.0 * disparity(x, y).unwrap(),
        );
    }

    #[test]
    fn disparity_antisymmetry() {
        perform_test_two(
            |x, y| disparity(x, y).unwrap(),
            |x, y| -1.0 * disparity(y, x).unwrap(),
        );
    }
}
