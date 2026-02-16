#import "/manual/definitions.typ": *

The $Sample$ function uses selection sampling (see @fan1962) to select $k$ elements from $n$ without replacement.

The algorithm makes a single pass through the data,
  deciding independently for each element whether to include it,
  using the #link(<sec-rng>)[Rng] generator for random decisions:

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
