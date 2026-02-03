#import "/manual/definitions.typ": *

== Sample

$ r.Sample(vx, k) $

Select $k$ elements from sample $vx$ without replacement using generator $r$.

#v(0.3em)
#list(marker: none, tight: true,
  [*Algorithm* — selection sampling (Fan, Muller, Rezucha 1962), see #link(<sec-prng>)[Pseudorandom Number Generation]],
  [*Complexity* — $O(n)$ time, single pass],
  [*Output* — preserves original order of selected elements],
  [*Domain* — $1 <= k <= n$],
)

#v(0.5em)
*Notation*

#list(marker: none, tight: true,
  [$vx = (x_1, ..., x_n)$, $vy = (y_1, ..., y_m)$ #h(2em) samples ($n, m >= 1$)],
  [$x_i$, $y_j$ #h(2em) individual measurements],
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

- `Sample([1, 2, 3, 4, 5], 3, Rng("demo-sample"))` — select 3 elements
- `Sample(x, n, r) = x` — selecting all elements returns original order

#v(0.5em)
Use $Sample$ when you need to pick a random subset of your data without replacement.
Common uses include bootstrap resampling, creating cross-validation splits, or reducing a large dataset to a manageable size.
Every possible subset of size $k$ has equal probability of being selected, and the selected elements keep their original order.
To make your subsampling reproducible, combine it with a seeded generator: `Sample(data, 100, Rng("training-set"))` will always select the same 100 elements.
