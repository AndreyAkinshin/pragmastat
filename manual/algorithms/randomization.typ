#import "/manual/definitions.typ": *

== Pseudorandom Number Generation <sec-prng>

The toolkit provides deterministic randomization utilities that produce identical sequences across all supported programming languages.
This cross-language reproducibility enables deterministic experiments and cross-validation of statistical analyses.

The core random number generator uses the xoshiro256++ algorithm (see @blackman2021),
  a member of the xoshiro/xoroshiro family developed by David Blackman and Sebastiano Vigna.
This algorithm was selected for several reasons:

- *Quality:* passes all tests in the BigCrush test suite from TestU01
- *Speed:* extremely fast due to simple bitwise operations (shifts, rotations, XORs)
- *Period:* period of $2^(256) - 1$, sufficient for parallel simulations
- *Adoption:* used by .NET 6+, Julia, and Rust's rand crate

The algorithm maintains a 256-bit state ($s_0, s_1, s_2, s_3$) and produces 64-bit outputs.
Each step updates the state through a combination of XOR, shift, and rotate operations.

#v(0.5em)
*Seed Initialization*

Converting a single seed value into the full 256-bit state requires a seeding algorithm.
The toolkit uses SplitMix64 (see @steele2014) for this purpose:

$
x &<- x + "0x9e3779b97f4a7c15" \
z &<- (x xor (x >> 30)) times "0xbf58476d1ce4e5b9" \
z &<- (z xor (z >> 27)) times "0x94d049bb133111eb" \
"output" &<- z xor (z >> 31)
$

Four consecutive outputs from SplitMix64 initialize the xoshiro256++ state ($s_0, s_1, s_2, s_3$).
This approach provides high-quality initial states from simple integer seeds.

#v(0.5em)
*String Seeds*

For named experiments (e.g., $Rng("experiment-1")$), string seeds are converted to integers using FNV-1a hash (see @fowler1991):

$
"hash" &<- "0xcbf29ce484222325" quad "(offset basis)" \
"for each byte" b: quad "hash" &<- ("hash" xor b) times "0x00000100000001b3" quad "(FNV prime)"
$

This enables meaningful experiment identifiers while maintaining determinism.

#v(0.5em)
*Uniform Floating-Point Generation*

To generate uniform values in $[0, 1)$, the upper 53 bits of a 64-bit output are used:

$ "uniform()" = ("next()" >> 11) times 2^(-53) $

The 53-bit mantissa of IEEE 754 double precision ensures all representable values in $[0, 1)$ are reachable.

== Shuffle Algorithm

The $Shuffle$ function uses the Fisher-Yates algorithm (see @fisher1938, @knuth1997),
  also known as the Knuth shuffle:

#block(inset: (left: 1em))[
```
for i from n-1 down to 1:
    j = uniform_int(0, i+1)
    swap(array[i], array[j])
```
]

This produces a uniformly random permutation in $O(n)$ time with $O(1)$ additional space.
The algorithm is unbiased: each of the $n!$ permutations has equal probability.

== Selection Sampling

The $Sample$ function uses selection sampling (see @fan1962) to select $k$ elements from $n$ without replacement:

#block(inset: (left: 1em))[
```
seen = 0, selected = 0
for each element x at position i:
    if uniform() < (k - selected) / (n - seen):
        output x
        selected += 1
    seen += 1
```
]

This algorithm preserves the original order of elements (order of first appearance)
  and requires only a single pass through the data.
Each element is selected independently with the correct marginal probability,
  producing a simple random sample.

== Distribution Sampling

#v(0.5em)
*Box-Muller Transform*

The $Additive$ (normal) distribution uses the Box-Muller transform (see @boxmuller1958)
  to convert uniform random values to normally distributed values.
Given two independent uniform values $U_1, U_2 in [0, 1)$:

$
Z_0 &= sqrt(-2 ln(U_1)) cos(2 pi U_2) \
Z_1 &= sqrt(-2 ln(U_1)) sin(2 pi U_2)
$

Both $Z_0$ and $Z_1$ are independent standard normal values.
The implementation uses only $Z_0$ to maintain cross-language determinism.

#v(0.5em)
*Numerical Constants*

Distribution sampling requires handling edge cases where floating-point operations would be undefined.
Two constants are used across all language implementations:

#block(inset: (left: 1em))[
*Machine Epsilon* ($epsilon_"mach"$): The smallest $epsilon$ such that $1 + epsilon != 1$ in float64 arithmetic.
$ epsilon_"mach" = 2^(-52) approx 2.22 times 10^(-16) $
Used when $U = 1$ to avoid $ln(0)$ or division by zero in inverse transform sampling.

*Smallest Positive Subnormal* ($epsilon_"sub"$): The smallest positive value representable in IEEE 754 binary64.
$ epsilon_"sub" = 2^(-1074) approx 4.94 times 10^(-324) $
Used when $U = 0$ to avoid $ln(0)$ in the Box-Muller transform.
]

All language implementations use the same literal values for these constants (not language-specific
builtins like `Number.EPSILON` or `f64::EPSILON`) to ensure bit-identical outputs across languages.

#v(0.5em)
*Inverse Transform Sampling*

Other distributions use inverse transform sampling.
Given a uniform value $U in [0, 1)$ and the inverse CDF $F^(-1)$:

- *Exponential:* $X = -ln(1 - U) \/ lambda$
- *Pareto (Power):* $X = x_min \/ (1 - U)^(1\/alpha)$
- *Uniform:* $X = a + U(b - a)$

The log-normal ($Multiplic$) distribution samples from $Additive$ and applies the exponential transform:
$X = exp(Additive(mu, sigma))$.
