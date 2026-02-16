#import "/manual/definitions.typ": *

$ Shuffle("seed", vx) $

The $Shuffle$ test suite contains 12 test cases validating random permutation.
Given a seed and input array $vx$, $Shuffle$ returns a permutation of $vx$ using the Fisher--Yates algorithm.
All tests verify reproducibility: the same seed and input must produce the same permutation across all language implementations.

*Seed variation* ($n = 5$, $vx = (1, 2, 3, 4, 5)$) --- 3 tests with different seeds:

- `seed-0-n5-basic`: seed $= 0$
- `seed-123-n5-basic`: seed $= 123$
- `seed-999-n5-basic`: seed $= 999$

These tests validate that different seeds produce different permutations of the same input.

*Fixed seed* (seed $= 1729$) --- 9 tests exploring different input sizes and content:

- `seed-1729-n1-single`: $vx = (1)$ (trivial case, single element)
- `seed-1729-n2-basic`: $vx = (1, 2)$ (minimum non-trivial case)
- `seed-1729-n5-basic`: $vx = (1, 2, 3, 4, 5)$ (standard case)
- `seed-1729-n5-zeros`: $vx = (0, 0, 0, 0, 0)$ (all identical, permutation preserves content)
- `seed-1729-n6-neg`: $vx = (-5, -3, -1, 1, 3, 5)$ (negative and positive values)
- `seed-1729-n10-seq`: $vx = (0, 1, ..., 9)$ (10-element sequential)
- `seed-1729-n20-seq`: $vx = (0, 1, ..., 19)$ (20-element sequential)
- `seed-1729-n100-seq`: $vx = (0, 1, ..., 99)$ (large array)
- `seed-123-n10-seq`: seed $= 123$, $vx = (0, 1, ..., 9)$ (different seed, same size as n10-seq)

The progression from $n = 1$ to $n = 100$ validates that the Fisher--Yates implementation scales correctly.
The zero-valued and negative-valued tests verify that shuffling operates on positions, not values.
