import time
from pragmastat.fast_center import _fast_center
from pragmastat.fast_spread import _fast_spread
from pragmastat.fast_shift import _fast_shift


def test_center_performance():
    n = 100000
    x = list(range(1, n + 1))

    start = time.time()
    result = _fast_center(x)
    elapsed = time.time() - start

    print(f"\nCenter for n={n}: {result:.6f}")
    print(f"Elapsed time: {elapsed:.3f}s")

    expected = 50000.5
    assert abs(result - expected) < 1e-9, (
        f"Center for n={n}: expected {expected}, got {result}"
    )
    assert elapsed < 5.0, f"Performance too slow: {elapsed}s"


def test_spread_performance():
    n = 100000
    x = list(range(1, n + 1))

    start = time.time()
    result = _fast_spread(x)
    elapsed = time.time() - start

    print(f"\nSpread for n={n}: {result:.6f}")
    print(f"Elapsed time: {elapsed:.3f}s")

    expected = 29290.0
    assert abs(result - expected) < 1e-9, (
        f"Spread for n={n}: expected {expected}, got {result}"
    )
    assert elapsed < 5.0, f"Performance too slow: {elapsed}s"


def test_shift_performance():
    n = 100000
    x = list(range(1, n + 1))
    y = list(range(1, n + 1))

    start = time.time()
    result = _fast_shift(x, y, p=0.5)
    elapsed = time.time() - start

    print(f"\nShift for n=m={n}: {result:.6f}")
    print(f"Elapsed time: {elapsed:.3f}s")

    expected = 0.0
    assert abs(result - expected) < 1e-9, (
        f"Shift for n=m={n}: expected {expected}, got {result}"
    )
    assert elapsed < 5.0, f"Performance too slow: {elapsed}s"


if __name__ == "__main__":
    test_center_performance()
    print("✓ Center performance test passed")

    test_spread_performance()
    print("✓ Spread performance test passed")

    test_shift_performance()
    print("✓ Shift performance test passed")
