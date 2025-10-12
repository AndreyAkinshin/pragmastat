import time
import numpy as np
from pragmastat.fast_center import _fast_center
from pragmastat.fast_spread import _fast_spread
from pragmastat.fast_shift import _fast_shift


def center_naive(x):
    n = len(x)
    pairwise_averages = []
    for i in range(n):
        for j in range(i, n):
            pairwise_averages.append((x[i] + x[j]) / 2)
    return np.median(pairwise_averages)


def spread_naive(x):
    n = len(x)
    if n == 1:
        return 0.0
    pairwise_diffs = []
    for i in range(n):
        for j in range(i + 1, n):
            pairwise_diffs.append(abs(x[i] - x[j]))
    return np.median(pairwise_diffs)


def shift_naive(x, y):
    pairwise_shifts = []
    for xi in x:
        for yj in y:
            pairwise_shifts.append(xi - yj)
    return np.median(pairwise_shifts)


def test_center_correctness():
    np.random.seed(1729)
    for n in range(1, 101):
        for iteration in range(n):
            x = np.random.randn(n).tolist()
            expected = center_naive(x)
            actual = _fast_center(x)
            assert (
                abs(expected - actual) < 1e-9
            ), f"Mismatch for n={n}: expected={expected}, actual={actual}"


def test_spread_correctness():
    np.random.seed(1729)
    for n in range(1, 101):
        for iteration in range(n):
            x = np.random.randn(n).tolist()
            expected = spread_naive(x)
            actual = _fast_spread(x)
            assert (
                abs(expected - actual) < 1e-9
            ), f"Mismatch for n={n}: expected={expected}, actual={actual}"


def test_center_performance():
    np.random.seed(1729)
    x = np.random.randn(100000).tolist()

    start = time.time()
    result = _fast_center(x)
    elapsed = time.time() - start

    print(f"\nCenter for n=100000: {result:.6f}")
    print(f"Elapsed time: {elapsed:.3f}s")
    assert elapsed < 10.0, f"Performance too slow: {elapsed}s"


def test_spread_performance():
    np.random.seed(1729)
    x = np.random.randn(100000).tolist()

    start = time.time()
    result = _fast_spread(x)
    elapsed = time.time() - start

    print(f"\nSpread for n=100000: {result:.6f}")
    print(f"Elapsed time: {elapsed:.3f}s")
    assert elapsed < 10.0, f"Performance too slow: {elapsed}s"


def test_shift_correctness():
    np.random.seed(1729)
    for n in range(2, 51):
        for m in range(2, 51):
            x = np.random.randn(n).tolist()
            y = np.random.randn(m).tolist()
            expected = shift_naive(x, y)
            actual = _fast_shift(x, y, p=0.5)
            assert (
                abs(expected - actual) < 1e-9
            ), f"Mismatch for n={n}, m={m}: expected={expected}, actual={actual}"


def test_shift_performance():
    np.random.seed(1729)
    x = np.random.randn(10000).tolist()
    y = np.random.randn(10000).tolist()

    start = time.time()
    result = _fast_shift(x, y, p=0.5)
    elapsed = time.time() - start

    print(f"\nShift for n=m=10000: {result:.6f}")
    print(f"Elapsed time: {elapsed:.3f}s")
    assert elapsed < 10.0, f"Performance too slow: {elapsed}s"


if __name__ == "__main__":
    test_center_correctness()
    print("✓ Center correctness tests passed")

    test_spread_correctness()
    print("✓ Spread correctness tests passed")

    test_shift_correctness()
    print("✓ Shift correctness tests passed")

    test_center_performance()
    print("✓ Center performance test passed")

    test_spread_performance()
    print("✓ Spread performance test passed")

    test_shift_performance()
    print("✓ Shift performance test passed")
