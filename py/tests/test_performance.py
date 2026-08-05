import time

from pragmastat import ratio
from pragmastat.center_impl import _center_impl
from pragmastat.shift_impl import _shift_impl
from pragmastat.spread_impl import _spread_impl


def test_center_performance():
    n = 100000
    x = list(range(1, n + 1))

    start = time.time()
    result = _center_impl(x)
    elapsed = time.time() - start

    print(f"\nCenter for n={n}: {result:.6f}")
    print(f"Elapsed time: {elapsed:.3f}s")

    expected = 50000.5
    assert abs(result - expected) < 1e-9, f"Center for n={n}: expected {expected}, got {result}"
    assert elapsed < 5.0, f"Performance too slow: {elapsed}s"


def test_spread_performance():
    n = 100000
    x = list(range(1, n + 1))

    start = time.time()
    result = _spread_impl(x)
    elapsed = time.time() - start

    print(f"\nSpread for n={n}: {result:.6f}")
    print(f"Elapsed time: {elapsed:.3f}s")

    expected = 29290.0
    assert abs(result - expected) < 1e-9, f"Spread for n={n}: expected {expected}, got {result}"
    assert elapsed < 5.0, f"Performance too slow: {elapsed}s"


def test_shift_performance():
    n = 100000
    x = list(range(1, n + 1))
    y = list(range(1, n + 1))

    start = time.time()
    result = _shift_impl(x, y, p=0.5)
    elapsed = time.time() - start

    print(f"\nShift for n=m={n}: {result:.6f}")
    print(f"Elapsed time: {elapsed:.3f}s")

    expected = 0.0
    assert abs(result - expected) < 1e-9, f"Shift for n=m={n}: expected {expected}, got {result}"
    assert elapsed < 5.0, f"Performance too slow: {elapsed}s"


if __name__ == "__main__":
    test_center_performance()
    print("✓ Center performance test passed")

    test_spread_performance()
    print("✓ Spread performance test passed")

    test_shift_performance()
    print("✓ Shift performance test passed")


def test_ratio_performance():
    # ratio is the one estimator whose cost is dominated by a transcendental rather than by the
    # selection: it log-transforms both samples before the shift. The toolkit evaluates its own
    # logarithm element by element, where the platform's was one vectorized numpy call, so this
    # path carries a real cost that none of the cases above can see. Measured at n = 100000 it is
    # the majority of the total, which is why it gets a budget of its own rather than being
    # assumed to be covered by test_shift_performance.
    n = 100000
    x = list(range(1, n + 1))
    y = list(range(1, n + 1))

    start = time.time()
    result = ratio(x, y)
    elapsed = time.time() - start

    print(f"\nRatio for n=m={n}: {result:.6f}")
    print(f"Elapsed time: {elapsed:.3f}s")

    expected = 1.0
    assert abs(result - expected) < 1e-9, f"Ratio for n=m={n}: expected {expected}, got {result}"
    assert elapsed < 5.0, f"Performance too slow: {elapsed}s"
