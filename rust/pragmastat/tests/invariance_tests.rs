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

    // Volatility invariance tests

    #[test]
    fn volatility_scale() {
        perform_test_one(
            |x| volatility(&vec_mul_scalar(x, 2.0)).unwrap(),
            |x| volatility(x).unwrap(),
        );
    }

    // Precision invariance tests

    #[test]
    fn precision_shift() {
        perform_test_one(
            |x| precision(&vec_add_scalar(x, 2.0)).unwrap(),
            |x| precision(x).unwrap(),
        );
    }

    #[test]
    fn precision_scale() {
        perform_test_one(
            |x| precision(&vec_mul_scalar(x, 2.0)).unwrap(),
            |x| 2.0 * precision(x).unwrap(),
        );
    }

    #[test]
    fn precision_scale_negate() {
        perform_test_one(
            |x| precision(&vec_mul_scalar(x, -2.0)).unwrap(),
            |x| 2.0 * precision(x).unwrap(),
        );
    }

    // MedShift invariance tests

    #[test]
    fn med_shift_shift() {
        perform_test_two(
            |x, y| med_shift(&vec_add_scalar(x, 3.0), &vec_add_scalar(y, 2.0)).unwrap(),
            |x, y| med_shift(x, y).unwrap() + 1.0,
        );
    }

    #[test]
    fn med_shift_scale() {
        perform_test_two(
            |x, y| med_shift(&vec_mul_scalar(x, 2.0), &vec_mul_scalar(y, 2.0)).unwrap(),
            |x, y| 2.0 * med_shift(x, y).unwrap(),
        );
    }

    #[test]
    fn med_shift_antisymmetry() {
        perform_test_two(
            |x, y| med_shift(x, y).unwrap(),
            |x, y| -1.0 * med_shift(y, x).unwrap(),
        );
    }

    // MedRatio invariance tests

    #[test]
    fn med_ratio_scale() {
        perform_test_two(
            |x, y| med_ratio(&vec_mul_scalar(x, 2.0), &vec_mul_scalar(y, 3.0)).unwrap(),
            |x, y| (2.0 / 3.0) * med_ratio(x, y).unwrap(),
        );
    }

    // MedSpread invariance tests

    #[test]
    fn med_spread_equal() {
        perform_test_one(|x| med_spread(x, x).unwrap(), |x| spread(x).unwrap());
    }

    #[test]
    fn med_spread_symmetry() {
        perform_test_two(
            |x, y| med_spread(x, y).unwrap(),
            |x, y| med_spread(y, x).unwrap(),
        );
    }

    #[test]
    fn med_spread_average() {
        perform_test_one(
            |x| med_spread(x, &vec_mul_scalar(x, 5.0)).unwrap(),
            |x| 3.0 * spread(x).unwrap(),
        );
    }

    #[test]
    fn med_spread_scale() {
        perform_test_two(
            |x, y| med_spread(&vec_mul_scalar(x, -2.0), &vec_mul_scalar(y, -2.0)).unwrap(),
            |x, y| 2.0 * med_spread(x, y).unwrap(),
        );
    }

    // MedDisparity invariance tests

    #[test]
    fn med_disparity_shift() {
        perform_test_two(
            |x, y| med_disparity(&vec_add_scalar(x, 2.0), &vec_add_scalar(y, 2.0)).unwrap(),
            |x, y| med_disparity(x, y).unwrap(),
        );
    }

    #[test]
    fn med_disparity_scale() {
        perform_test_two(
            |x, y| med_disparity(&vec_mul_scalar(x, 2.0), &vec_mul_scalar(y, 2.0)).unwrap(),
            |x, y| med_disparity(x, y).unwrap(),
        );
    }

    #[test]
    fn med_disparity_scale_neg() {
        perform_test_two(
            |x, y| med_disparity(&vec_mul_scalar(x, -2.0), &vec_mul_scalar(y, -2.0)).unwrap(),
            |x, y| -1.0 * med_disparity(x, y).unwrap(),
        );
    }

    #[test]
    fn med_disparity_antisymmetry() {
        perform_test_two(
            |x, y| med_disparity(x, y).unwrap(),
            |x, y| -1.0 * med_disparity(y, x).unwrap(),
        );
    }
}
