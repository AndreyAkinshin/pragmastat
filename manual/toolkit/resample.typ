#import "/manual/definitions.typ": *

=== Resample

$ r.Resample(vx, k) $

Select $k$ elements from sample $vx$ with replacement using generator $r$.

#v(0.3em)
#list(marker: none, tight: true,
  [*Algorithm* — uniform sampling with replacement],
  [*Complexity* — $O(k)$ time],
  [*Output* — new array with $k$ elements (may contain duplicates)],
  [*Domain* — $k >= 0$, sample size $n >= 1$],
)

#v(0.5em)
*Notation*

#list(marker: none, tight: true,
  [$vx = (x_1, ..., x_n)$ #h(2em) sample ($n >= 1$)],
  [$x_i$ #h(2em) individual measurements],
)

#v(0.5em)
*Properties*

#list(marker: none, tight: true,
  [*Independence* #h(2em) each selection is independent with equal probability $1\/n$],
  [*Duplicates* #h(2em) same element may appear multiple times in output],
  [*Determinism* #h(2em) same generator state produces same selection],
)

#v(0.3em)
*Example*

- `Resample([1, 2, 3, 4, 5], 3, Rng("demo-resample"))` — select 3 with replacement
- `Resample(x, n, r)` — bootstrap sample of same size as original

#v(0.5em)
$Resample$ picks elements with replacement, allowing the same element to be selected multiple times.
This is essential for bootstrap methods where we simulate new samples from the observed data.
Unlike $Sample$ (without replacement), $Resample$ can produce outputs larger than the input
and will typically contain duplicate values.
For reproducible bootstrap, combine with a seeded generator: `Resample(data, n, Rng("bootstrap-1"))`.
