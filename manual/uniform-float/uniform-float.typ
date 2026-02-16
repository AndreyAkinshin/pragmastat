#import "/manual/definitions.typ": *

== UniformFloat <sec-uniform-float>

$ r.UniformFloat() $

Draw a uniform random value in $[0, 1)$ using generator $r$.
$UniformFloat$ is the primitive draw; all other randomization functions and
distribution samplers build on top of it.

#v(0.3em)
#list(marker: none, tight: true,
  [*Distribution* — $Uniform(0, 1)$],
  [*Range* — $[0, 1)$ (includes 0, excludes 1)],
  [*Precision* — 53-bit mantissa ($2^(53)$ distinct values)],
  [*Complexity* — $O(1)$ per draw],
)

#v(0.5em)
*Properties*

#list(marker: none, tight: true,
  [*Determinism* #h(2em) same generator state produces same value],
  [*Independence* #h(2em) successive draws are uncorrelated],
  [*Uniformity* #h(2em) all representable values in $[0, 1)$ equally likely],
)

#v(0.3em)
*Example (conceptual)*

#list(marker: none, tight: true,
  [*Call* — `Rng("demo").UniformFloat()` (conceptual name; see mapping below)],
  [*Repeat* — 10 successive calls produce 10 independent values],
)

#v(0.3em)
*Implementation names*

#table(
  columns: 2,
  align: (left, left),
  [*Language*], [*Method*],
  [*C\#*], [`UniformDouble()`],
  [*Go*], [`UniformFloat64()`],
  [*Kotlin*], [`uniformDouble()`],
  [*Rust*], [`uniform_f64()`],
  [*Python*], [`uniform_float()`],
  [*R*], [`uniform_float()`],
  [*TypeScript*], [`uniformFloat()`],
)

#v(0.5em)
$UniformFloat$ is the fundamental operation of the random number generator.
All other randomization functions — #link(<sec-sample>)[$Sample$], #link(<sec-shuffle>)[$Shuffle$], #link(<sec-resample>)[$Resample$],
  and the distribution samplers — are built on top of uniform draws.
See #link(<sec-reframing-uniform>)[Naming] for why the toolkit
  uses the name $UniformFloat$ instead of the traditional `.Next()`.

#pagebreak()
=== Algorithm <sec-alg-uniform>

#include "uniform-float-algorithm.typ"

#pagebreak()
=== Notes <sec-reframing-uniform>

#include "uniform-float-notes.typ"

=== References

#include "uniform-float-references.typ"
