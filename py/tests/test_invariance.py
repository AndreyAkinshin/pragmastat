import numpy as np
import pytest
from pragmastat import Rng, center, spread, rel_spread, shift, ratio, disparity
from pragmastat.estimators import _avg_spread as avg_spread


class TestInvariance:
    seed = 1729
    sample_sizes = [2, 3, 4, 5, 6, 7, 8, 9, 10]
    tolerance = 1e-9

    def perform_test_one(self, expr1_func, expr2_func):
        rng = Rng(self.seed)
        for n in self.sample_sizes:
            x = np.array([rng.uniform_float() for _ in range(n)])
            result1 = expr1_func(x)
            result2 = expr2_func(x)
            assert abs(result1 - result2) < self.tolerance, (
                f"Failed for n={n}: {result1} != {result2}"
            )

    def perform_test_two(self, expr1_func, expr2_func):
        rng = Rng(self.seed)
        for n in self.sample_sizes:
            x = np.array([rng.uniform_float() for _ in range(n)])
            y = np.array([rng.uniform_float() for _ in range(n)])
            result1 = expr1_func(x, y)
            result2 = expr2_func(x, y)
            assert abs(result1 - result2) < self.tolerance, (
                f"Failed for n={n}: {result1} != {result2}"
            )

    # Center tests
    def test_center_shift(self):
        self.perform_test_one(lambda x: center(x + 2), lambda x: center(x) + 2)

    def test_center_scale(self):
        self.perform_test_one(lambda x: center(2 * x), lambda x: 2 * center(x))

    def test_center_negate(self):
        self.perform_test_one(lambda x: center(-1 * x), lambda x: -1 * center(x))

    # Spread tests
    def test_spread_shift(self):
        self.perform_test_one(lambda x: spread(x + 2), lambda x: spread(x))

    def test_spread_scale(self):
        self.perform_test_one(lambda x: spread(2 * x), lambda x: 2 * spread(x))

    def test_spread_negate(self):
        self.perform_test_one(lambda x: spread(-1 * x), lambda x: spread(x))

    # RelSpread tests
    def test_rel_spread_scale(self):
        import warnings

        with warnings.catch_warnings():
            warnings.simplefilter("ignore", DeprecationWarning)
            self.perform_test_one(lambda x: rel_spread(2 * x), lambda x: rel_spread(x))

    # Shift tests
    def test_shift_shift(self):
        self.perform_test_two(
            lambda x, y: shift(x + 3, y + 2), lambda x, y: shift(x, y) + 1
        )

    def test_shift_scale(self):
        self.perform_test_two(
            lambda x, y: shift(2 * x, 2 * y), lambda x, y: 2 * shift(x, y)
        )

    def test_shift_antisymmetry(self):
        self.perform_test_two(lambda x, y: shift(x, y), lambda x, y: -1 * shift(y, x))

    # Ratio tests
    def test_ratio_scale(self):
        self.perform_test_two(
            lambda x, y: ratio(2 * x, 3 * y), lambda x, y: (2.0 / 3) * ratio(x, y)
        )

    # AvgSpread tests
    def test_avg_spread_equal(self):
        self.perform_test_one(lambda x: avg_spread(x, x), lambda x: spread(x))

    def test_avg_spread_symmetry(self):
        self.perform_test_two(
            lambda x, y: avg_spread(x, y), lambda x, y: avg_spread(y, x)
        )

    def test_avg_spread_average(self):
        self.perform_test_one(lambda x: avg_spread(x, 5 * x), lambda x: 3 * spread(x))

    def test_avg_spread_scale(self):
        self.perform_test_two(
            lambda x, y: avg_spread(-2 * x, -2 * y), lambda x, y: 2 * avg_spread(x, y)
        )

    # Disparity tests
    def test_disparity_shift(self):
        self.perform_test_two(
            lambda x, y: disparity(x + 2, y + 2), lambda x, y: disparity(x, y)
        )

    def test_disparity_scale(self):
        self.perform_test_two(
            lambda x, y: disparity(2 * x, 2 * y), lambda x, y: disparity(x, y)
        )

    def test_disparity_scale_neg(self):
        self.perform_test_two(
            lambda x, y: disparity(-2 * x, -2 * y), lambda x, y: -1 * disparity(x, y)
        )

    def test_disparity_antisymmetry(self):
        self.perform_test_two(
            lambda x, y: disparity(x, y), lambda x, y: -1 * disparity(y, x)
        )


class TestRandomizationInvariance:
    def test_shuffle_preserves_multiset(self):
        for n in [1, 2, 5, 10, 100]:
            x = list(range(n))
            rng = Rng(42)
            shuffled = rng.shuffle(x)
            assert sorted(shuffled) == x

    def test_sample_correct_size(self):
        x = list(range(10))
        for k in [1, 3, 5, 10, 15]:
            rng = Rng(42)
            sampled = rng.sample(x, k)
            assert len(sampled) == min(k, len(x))

    def test_sample_elements_from_source(self):
        x = list(range(10))
        rng = Rng(42)
        sampled = rng.sample(x, 5)
        for elem in sampled:
            assert elem in x

    def test_sample_preserves_order(self):
        x = list(range(10))
        rng = Rng(42)
        sampled = rng.sample(x, 5)
        for i in range(1, len(sampled)):
            assert sampled[i] > sampled[i - 1]

    def test_sample_no_duplicates(self):
        for n in [2, 3, 5, 10, 20]:
            source = list(range(n))
            for k in [1, n // 2, n]:
                rng = Rng(42)
                sampled = rng.sample(source, k)
                assert len(sampled) == len(set(sampled)), (
                    f"Duplicate in sample(n={n}, k={k})"
                )

    def test_resample_negative_k_raises(self):
        rng = Rng(42)
        with pytest.raises(ValueError):
            rng.resample([1, 2, 3], -1)

    def test_resample_elements_from_source(self):
        x = list(range(5))
        rng = Rng(42)
        resampled = rng.resample(x, 10)
        for elem in resampled:
            assert elem in x

    def test_resample_k0_raises(self):
        rng = Rng(42)
        with pytest.raises(ValueError):
            rng.resample([1, 2, 3], 0)

    def test_shuffle_empty_raises(self):
        rng = Rng(42)
        with pytest.raises(ValueError):
            rng.shuffle([])

    def test_sample_k0_raises(self):
        rng = Rng(42)
        with pytest.raises(ValueError):
            rng.sample([1, 2, 3], 0)

    def test_sample_empty_raises(self):
        rng = Rng(42)
        with pytest.raises(ValueError):
            rng.sample([], 1)
