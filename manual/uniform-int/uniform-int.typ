#import "/manual/definitions.typ": *

== UniformInt <sec-uniform-int>

$ r.UniformInt(a, b) $

Generate a uniform random integer in $[a, b)$ using generator $r$.
This draw is derived from the underlying uniform float stream.

#v(0.3em)
#list(marker: none, tight: true,
  [*Range* — $[a, b)$ (includes $a$, excludes $b$)],
  [*Complexity* — $O(1)$ per draw],
)

#v(0.3em)
*Example (conceptual)*

#list(marker: none, tight: true,
  [*Call* — `Rng(42).UniformInt(a, b)` (conceptual name; see mapping below)],
  [*Range* — integer in $[0, 100)$ or $[-50, 50)$],
)

#v(0.3em)
*Implementation names*

#table(
  columns: 2,
  align: (left, left),
  [*Language*], [*Method*],
  [*C\#*], [`UniformInt32()` / `UniformInt64()` / `UniformInt16()` / `UniformInt8()`],
  [*Go*], [`UniformIntN()` / `UniformInt64()` / `UniformInt32()` / `UniformInt16()` / `UniformInt8()`],
  [*Kotlin*], [`uniformInt()` / `uniformLong()` / `uniformShort()` / `uniformByte()`],
  [*Rust*], [`uniform_i32()` / `uniform_i64()` / `uniform_i16()` / `uniform_i8()`],
  [*Python*], [`uniform_int()`],
  [*R*], [`uniform_int()`],
  [*TypeScript*], [`uniformInt()`],
)

#v(0.3em)
$UniformInt$ draws are derived from the same generator core as $UniformFloat$
  and map a uniform 64-bit value into $[a, b)$.
The implementation uses modulo reduction; ranges that do not divide $2^(64)$
  introduce a slight bias (acceptable for simulation, not for cryptographic use).
See #link(<sec-alg-uniform>)[UniformFloat → Algorithm] for the core generator details.

Unsigned variants are available in languages that support them.
