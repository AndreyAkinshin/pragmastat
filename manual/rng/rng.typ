#import "/manual/definitions.typ": *

== Rng <sec-rng>

All randomization in the toolkit is driven by $Rng$ — a deterministic pseudorandom number generator.
Creating a generator from a seed produces an object whose methods (#link(<sec-uniform-float>)[$UniformFloat$], #link(<sec-uniform-int>)[$UniformInt$], #link(<sec-sample>)[$Sample$], #link(<sec-resample>)[$Resample$], #link(<sec-shuffle>)[$Shuffle$])
  yield identical sequences across all seven supported languages.

This chapter starts with the generator object itself, then introduces the primitive draw
  (#link(<sec-uniform-float>)[$UniformFloat$]) and its integer counterpart
  (#link(<sec-uniform-int>)[$UniformInt$]), followed by sampling utilities.

$ r = Rng(s) $

#v(0.3em)
#list(marker: none, tight: true,
  [*Seed types* — integer seed or string seed (hashed via FNV-1a)],
  [*Determinism* — identical sequences across all supported languages],
  [*Period* — $2^(256) - 1$],
)

#v(0.3em)
*Example*

- `r = Rng("experiment-1")` — create generator from string seed
- `r = Rng(42)` — create generator from integer seed

The underlying algorithm is xoshiro256++ seeded via SplitMix64.
See #link(<sec-alg-uniform>)[UniformFloat → Algorithm] for implementation details.

#pagebreak()
=== Implementation

#source-include("cs/Pragmastat/Randomization/Rng.cs", "cs")

#pagebreak()
=== Tests

#include "rng-tests.typ"

=== References

#include "rng-references.typ"
