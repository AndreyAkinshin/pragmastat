#import "/manual/definitions.typ": *

=== Rng

$ Rng(s) $

Deterministic pseudorandom number generator from seed $s$.

#v(0.3em)
#list(marker: none, tight: true,
  [*Algorithm* — xoshiro256++ with SplitMix64 seeding (see #link(<sec-prng>)[Pseudorandom Number Generation])],
  [*Seed types* — integer seed or string seed (hashed via FNV-1a)],
  [*Determinism* — identical sequences across all supported languages],
  [*Period* — $2^(256) - 1$],
)

#v(0.5em)
*Notation*

#list(marker: none, tight: true,
  [$X$, $Y$ #h(2em) random variables (generators of real measurements)],
)

#v(0.5em)
*Properties*

#list(marker: none, tight: true,
  [*Reproducibility* #h(2em) $Rng(s)$ produces identical sequence for same $s$],
  [*Independence* #h(2em) different seeds produce uncorrelated sequences],
)

#v(0.3em)
*Example*

- `Rng("demo-uniform")` — string seed for reproducible demos
- `Rng("experiment-1")` — string seed for named experiments

#v(0.5em)
$Rng$ provides reproducible random numbers.
The same seed produces exactly the same sequence of values, identical across Python, TypeScript, R, C\#, Kotlin, Rust, and Go.
Passing a descriptive string like `"experiment-1"` makes code self-documenting.
Each draw advances the generator's internal state, so independent random streams require separate generators with different seeds.

