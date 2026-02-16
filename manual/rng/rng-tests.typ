#import "/manual/definitions.typ": *

The $Rng$ test suite contains 55 test cases validating the deterministic pseudo-random number generator across seven output categories.
All tests verify reproducibility: given the same seed, every language implementation must produce identical sequences.
Seeds can be integers or strings (string seeds are hashed to produce an integer seed).

*uniform-seed* (10 tests) --- base $Uniform(0, 1)$ generation from integer seeds:

- Seeds: $-2147483648$, $-42$, $-1$, $0$, $1$, $123$, $999$, $1729$, $12345$, $2147483647$
- Each generates 20 values in $[0, 1)$
- Covers int32 boundary values (min, max), negative seeds, and common seeds

*uniform-f32* (7 tests) --- single-precision $Uniform(0, 1)$ generation:

- Seeds: $-42$, $-1$, $0$, $1$, $123$, $999$, $1729$
- Each generates 20 values in $[0, 1)$ at f32 precision
- Validates that f32 output matches the truncated f64 sequence

*uniform-bool* (7 tests) --- boolean generation ($Uniform < 0.5$):

- Seeds: $-42$, $-1$, $0$, $1$, $123$, $999$, $1729$
- Each generates 100 boolean values
- Validates the threshold-based boolean conversion

*uniform-range* (7 tests) --- $Uniform(min, max)$ generation with real-valued bounds:

- Seeds and ranges: seed $0$ with $[-1, 1]$; seed $123$ with $[0, 1]$; seed $999$ with $[0, 100]$; seed $1729$ with $[-1, 1]$, $[-50, 50]$, $[0, 1]$, $[0, 100]$
- Each generates 20 values in $[min, max)$
- Validates affine transformation of base uniform

*uniform-int* (8 tests) --- uniform integer generation in $[min, max)$:

- Seeds and ranges: seed $-42$ with $[0, 100]$; seed $0$ with $[0, 100]$; seed $123$ with $[0, 100]$; seed $999$ with $[-100, 100]$; seed $1729$ with $[-50, 50]$, $[0, 10]$, $[0, 100]$, $[1000, 2000]$
- Each generates 20 integer values
- Validates modulo reduction of raw 64-bit output

*uniform-i32* (5 tests) --- 32-bit signed integer generation:

- Seeds and ranges: seed $0$ with $[-500, 500]$; seed $123$ with $[0, 1000]$; seed $999$ with $[0, 100]$; seed $1729$ with $[-500, 500]$ and $[0, 1000]$
- Each generates 20 values
- Validates i32-specific truncation behavior

*uniform-string* (11 tests) --- string-seeded $Uniform(0, 1)$ generation:

- Seeds: `""` (empty), `"a"`, `"abc"`, `"test"`, `"hello_world"`, `"pragmastat"`, `"Rng"`, `"experiment-1"`, plus 3 UTF-8 seeds ($pi$, `"hello"` in Chinese, `"hello"` in German)
- Each generates 20 values in $[0, 1)$
- Validates string-to-seed hashing, including empty strings, case sensitivity, and multi-byte UTF-8
