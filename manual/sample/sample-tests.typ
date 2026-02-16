#import "/manual/definitions.typ": *

$ Sample("seed", vx, k) $

The $Sample$ test suite contains 15 test cases validating sampling without replacement.
Given a seed, input array $vx$ of size $n$, and draw count $k$, $Sample$ returns $k$ distinct elements from $vx$, preserving their original order.
All tests verify reproducibility: the same seed, input, and $k$ must produce the same output across all language implementations.

*Seed variation* ($n = 10$, $k = 3$) --- 3 tests with different seeds:

- `seed-0-n10-k3`: seed $= 0$
- `seed-123-n10-k3`: seed $= 123$
- `seed-999-n10-k3`: seed $= 999$

These tests validate that different seeds produce different samples from the same input.

*Parameter variation* (seed $= 1729$) --- 12 tests exploring $n$ and $k$:

- `seed-1729-n1-k1`: $n = 1$, $k = 1$ (trivial case, single element)
- `seed-1729-n2-k1`: $n = 2$, $k = 1$ (draw one from two)
- `seed-1729-n5-k3`: $n = 5$, $k = 3$ (standard draw)
- `seed-1729-n10-k1`: $n = 10$, $k = 1$ (single draw from many)
- `seed-1729-n10-k3`: $n = 10$, $k = 3$ (standard draw)
- `seed-1729-n10-k5`: $n = 10$, $k = 5$ (half draw)
- `seed-1729-n10-k10`: $n = 10$, $k = 10$ (full permutation)
- `seed-1729-n10-k15`: $n = 10$, $k = 15$ ($k > n$, clamped to $n$)
- `seed-1729-n20-k5`: $n = 20$, $k = 5$
- `seed-1729-n20-k10`: $n = 20$, $k = 10$
- `seed-1729-n100-k10`: $n = 100$, $k = 10$ (large pool, small draw)
- `seed-1729-n100-k25`: $n = 100$, $k = 25$ (large pool, moderate draw)

The progression from $k = 1$ to $k = n$ to $k > n$ validates boundary handling.
When $k >= n$, the result is a copy of $vx$ in its original order.

*Seed-based validation* ---
All seed $= 1729$ tests share the same underlying RNG state.
Cross-seed tests ($0$, $123$, $999$) confirm that different seeds yield different permutation sequences.
