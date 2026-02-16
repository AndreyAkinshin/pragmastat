#import "/manual/definitions.typ": *

== Shuffle <sec-shuffle>

$ r.Shuffle(vx) $

Uniformly random permutation of sample $vx$ using generator $r$.

#v(0.3em)
#list(marker: none, tight: true,
  [*Algorithm* — Fisher-Yates (Knuth shuffle), see #link(<sec-alg-shuffle>)[Shuffle]],
  [*Complexity* — $O(n)$ time, $O(n)$ space (returns new array)],
  [*Output* — new array (does not modify input)],
)

#v(0.5em)
*Properties*

#list(marker: none, tight: true,
  [*Uniformity* #h(2em) each of $n!$ permutations has equal probability],
  [*Determinism* #h(2em) same generator state produces same permutation],
)

#v(0.3em)
*Example*

- `Rng("demo-shuffle").Shuffle([1, 2, 3, 4, 5])` — shuffled copy
- `r.Shuffle(x)` preserves multiset (same elements, different order)

#v(0.3em)
*Implementation names*

#table(
  columns: 2,
  align: (left, left),
  [*Language*], [*Method*],
  [*C\#*], [`Rng.Shuffle()`],
  [*Go*], [`Shuffle()`],
  [*Kotlin*], [`Rng.shuffle()`],
  [*Rust*], [`Rng::shuffle()`],
  [*Python*], [`Rng.shuffle()`],
  [*R*], [`rng$shuffle()`],
  [*TypeScript*], [`Rng.shuffle()`],
)

#v(0.5em)
$Shuffle$ produces a random reordering of data.
This is essential for permutation tests and useful for eliminating any bias from the original ordering.
Every possible arrangement has exactly equal probability, which is required for valid statistical inference.
The function returns a new shuffled array and leaves the original data unchanged.
For reproducible results, pass a seeded generator: `Shuffle(data, Rng("experiment-1"))` will always produce the same permutation.

#pagebreak()
=== Algorithm <sec-alg-shuffle>

#include "shuffle-algorithm.typ"

#pagebreak()
=== Tests

#include "shuffle-tests.typ"

=== References

#include "shuffle-references.typ"
