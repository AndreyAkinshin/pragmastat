#import "/manual/definitions.typ": *

== Shuffle

$ r.Shuffle(vx) $

Uniformly random permutation of sample $vx$ using generator $r$.

#v(0.3em)
#list(marker: none, tight: true,
  [*Algorithm* — Fisher-Yates (Knuth shuffle), see #link(<sec-prng>)[Pseudorandom Number Generation]],
  [*Complexity* — $O(n)$ time, $O(1)$ additional space],
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

- `Shuffle([1, 2, 3, 4, 5], Rng("demo-shuffle"))` — shuffled copy
- `Shuffle(x, r)` preserves multiset (same elements, different order)

#v(0.5em)
Use $Shuffle$ when you need a random reordering of your data.
This is essential for permutation tests and useful whenever you want to eliminate any bias that might come from the original ordering.
Every possible arrangement has exactly equal probability, which is required for valid statistical inference.
The function returns a new shuffled array and leaves your original data unchanged.
For reproducible results, pass a seeded generator: `Shuffle(data, Rng("experiment-1"))` will always produce the same permutation.
