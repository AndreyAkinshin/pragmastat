#import "/manual/definitions.typ": *

$ Resample("seed", vx, k) $

The $Resample$ test suite contains 19 test cases validating sampling with replacement (bootstrap resampling).
Given a seed, input array $vx$ of size $n$, and draw count $k$, $Resample$ returns $k$ elements drawn independently and uniformly from $vx$ (with replacement, so duplicates are possible).
All tests verify reproducibility: the same seed, input, and $k$ must produce the same output across all language implementations.

*Seed variation* --- 6 tests with different seeds:

- `seed-0-n10-k3`: seed $= 0$, $n = 10$, $k = 3$
- `seed-42-n10-k5`: seed $= 42$, $n = 10$, $k = 5$
- `seed-123-n10-k3`: seed $= 123$, $n = 10$, $k = 3$
- `seed-314-n10-k10`: seed $= 314$, $n = 10$, $k = 10$
- `seed-999-n10-k3`: seed $= 999$, $n = 10$, $k = 3$
- `seed-2718-n100-k25`: seed $= 2718$, $n = 100$, $k = 25$

These tests validate that different seeds produce different bootstrap samples from the same input.

*Parameter variation* (seed $= 1729$) --- 13 tests exploring $n$ and $k$:

- `seed-1729-n1-k1`: $n = 1$, $k = 1$ (trivial case)
- `seed-1729-n2-k1`: $n = 2$, $k = 1$ (single draw from two)
- `seed-1729-n5-k3`: $n = 5$, $k = 3$ (standard draw)
- `seed-1729-n5-k7`: $n = 5$, $k = 7$ ($k > n$, valid for resampling)
- `seed-1729-n10-k1`: $n = 10$, $k = 1$ (single draw from many)
- `seed-1729-n10-k3`: $n = 10$, $k = 3$ (standard draw)
- `seed-1729-n10-k5`: $n = 10$, $k = 5$ (half draw)
- `seed-1729-n10-k10`: $n = 10$, $k = 10$ (draw equal to pool size)
- `seed-1729-n10-k15`: $n = 10$, $k = 15$ ($k > n$, exercises repeated sampling)
- `seed-1729-n20-k5`: $n = 20$, $k = 5$
- `seed-1729-n20-k10`: $n = 20$, $k = 10$
- `seed-1729-n100-k10`: $n = 100$, $k = 10$ (large pool, small draw)
- `seed-1729-n100-k25`: $n = 100$, $k = 25$ (large pool, moderate draw)

Unlike $Sample$ (without replacement), $Resample$ allows $k > n$ since each draw is independent.
The $k > n$ cases (`n5-k7`, `n10-k15`) are unique to resampling and validate that the output can contain repeated values from the input.
