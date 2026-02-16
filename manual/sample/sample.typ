#import "/manual/definitions.typ": *

== Sample <sec-sample>

$ r.Sample(vx, k) $

Select $k$ elements from sample $vx$ without replacement using generator $r$.

#v(0.3em)
#list(marker: none, tight: true,
  [*Algorithm* — selection sampling (Fan, Muller, Rezucha 1962), see #link(<sec-alg-sample>)[Sample]],
  [*Complexity* — $O(n)$ time, single pass],
  [*Output* — preserves original order of selected elements],
  [*Domain* — $k >= 0$ (clamped to $n$ if $k > n$)],
)

#v(0.5em)
*Notation*

#list(marker: none, tight: true,
  [$vx = (x_1, ..., x_n)$ #h(2em) sample ($n >= 0$)],
  [$x_i$ #h(2em) individual measurements],
)

#v(0.5em)
*Properties*

#list(marker: none, tight: true,
  [*Simple random sample* #h(2em) each $k$-subset has equal probability],
  [*Order preservation* #h(2em) selected elements appear in order of first occurrence],
  [*Determinism* #h(2em) same generator state produces same selection],
)

#v(0.3em)
*Example*

- `Rng("demo-sample").Sample([1, 2, 3, 4, 5], 3)` — select 3 elements
- `r.Sample(x, n) = x` — selecting all elements returns original order

#v(0.3em)
*Implementation names*

#table(
  columns: 2,
  align: (left, left),
  [*Language*], [*Method*],
  [*C\#*], [`Rng.Sample()`],
  [*Go*], [`Sample()`],
  [*Kotlin*], [`Rng.sample()`],
  [*Rust*], [`Rng::sample()`],
  [*Python*], [`Rng.sample()`],
  [*R*], [`rng$sample()`],
  [*TypeScript*], [`Rng.sample()`],
)

#v(0.5em)
$Sample$ picks a random subset of data without replacement.
Common uses include random subsetting, creating cross-validation splits, or reducing a large dataset to a manageable size.
Every possible subset of size $k$ has equal probability of being selected, and the selected elements keep their original order.
To make your subsampling reproducible, combine it with a seeded generator: `Sample(data, 100, Rng("training-set"))` will always select the same 100 elements.

#pagebreak()
=== Algorithm <sec-alg-sample>

#include "sample-algorithm.typ"

#pagebreak()
=== Tests

#include "sample-tests.typ"

=== References

#include "sample-references.typ"
