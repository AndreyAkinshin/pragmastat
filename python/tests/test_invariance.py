import numpy as np
import pytest
from pragmastat import (
    center, spread, volatility, precision,
    med_shift, med_ratio, med_spread, med_disparity
)


class TestInvariance:
    seed = 1729
    sample_sizes = [2, 3, 4, 5, 6, 7, 8, 9, 10]
    tolerance = 1e-9
    
    def perform_test_one(self, expr1_func, expr2_func):
        np.random.seed(self.seed)
        for n in self.sample_sizes:
            x = np.random.uniform(0, 1, n)
            result1 = expr1_func(x)
            result2 = expr2_func(x)
            assert abs(result1 - result2) < self.tolerance, \
                f"Failed for n={n}: {result1} != {result2}"
    
    def perform_test_two(self, expr1_func, expr2_func):
        np.random.seed(self.seed)
        for n in self.sample_sizes:
            x = np.random.uniform(0, 1, n)
            y = np.random.uniform(0, 1, n)
            result1 = expr1_func(x, y)
            result2 = expr2_func(x, y)
            assert abs(result1 - result2) < self.tolerance, \
                f"Failed for n={n}: {result1} != {result2}"
    
    # Center tests
    def test_center_shift(self):
        self.perform_test_one(
            lambda x: center(x + 2),
            lambda x: center(x) + 2
        )
    
    def test_center_scale(self):
        self.perform_test_one(
            lambda x: center(2 * x),
            lambda x: 2 * center(x)
        )
    
    def test_center_negate(self):
        self.perform_test_one(
            lambda x: center(-1 * x),
            lambda x: -1 * center(x)
        )
    
    # Spread tests
    def test_spread_shift(self):
        self.perform_test_one(
            lambda x: spread(x + 2),
            lambda x: spread(x)
        )
    
    def test_spread_scale(self):
        self.perform_test_one(
            lambda x: spread(2 * x),
            lambda x: 2 * spread(x)
        )
    
    def test_spread_negate(self):
        self.perform_test_one(
            lambda x: spread(-1 * x),
            lambda x: spread(x)
        )
    
    # Volatility tests
    def test_volatility_scale(self):
        self.perform_test_one(
            lambda x: volatility(2 * x),
            lambda x: volatility(x)
        )
    
    # Precision tests
    def test_precision_shift(self):
        self.perform_test_one(
            lambda x: precision(x + 2),
            lambda x: precision(x)
        )
    
    def test_precision_scale(self):
        self.perform_test_one(
            lambda x: precision(2 * x),
            lambda x: 2 * precision(x)
        )
    
    def test_precision_scale_negate(self):
        self.perform_test_one(
            lambda x: precision(-2 * x),
            lambda x: 2 * precision(x)
        )
    
    # MedShift tests
    def test_med_shift_shift(self):
        self.perform_test_two(
            lambda x, y: med_shift(x + 3, y + 2),
            lambda x, y: med_shift(x, y) + 1
        )
    
    def test_med_shift_scale(self):
        self.perform_test_two(
            lambda x, y: med_shift(2 * x, 2 * y),
            lambda x, y: 2 * med_shift(x, y)
        )
    
    def test_med_shift_antisymmetry(self):
        self.perform_test_two(
            lambda x, y: med_shift(x, y),
            lambda x, y: -1 * med_shift(y, x)
        )
    
    # MedRatio tests
    def test_med_ratio_scale(self):
        self.perform_test_two(
            lambda x, y: med_ratio(2 * x, 3 * y),
            lambda x, y: (2.0 / 3) * med_ratio(x, y)
        )
    
    # MedSpread tests
    def test_med_spread_equal(self):
        self.perform_test_one(
            lambda x: med_spread(x, x),
            lambda x: spread(x)
        )
    
    def test_med_spread_symmetry(self):
        self.perform_test_two(
            lambda x, y: med_spread(x, y),
            lambda x, y: med_spread(y, x)
        )
    
    def test_med_spread_average(self):
        self.perform_test_one(
            lambda x: med_spread(x, 5 * x),
            lambda x: 3 * spread(x)
        )
    
    def test_med_spread_scale(self):
        self.perform_test_two(
            lambda x, y: med_spread(-2 * x, -2 * y),
            lambda x, y: 2 * med_spread(x, y)
        )
    
    # MedDisparity tests
    def test_med_disparity_shift(self):
        self.perform_test_two(
            lambda x, y: med_disparity(x + 2, y + 2),
            lambda x, y: med_disparity(x, y)
        )
    
    def test_med_disparity_scale(self):
        self.perform_test_two(
            lambda x, y: med_disparity(2 * x, 2 * y),
            lambda x, y: med_disparity(x, y)
        )
    
    def test_med_disparity_scale_neg(self):
        self.perform_test_two(
            lambda x, y: med_disparity(-2 * x, -2 * y),
            lambda x, y: -1 * med_disparity(x, y)
        )
    
    def test_med_disparity_antisymmetry(self):
        self.perform_test_two(
            lambda x, y: med_disparity(x, y),
            lambda x, y: -1 * med_disparity(y, x)
        )