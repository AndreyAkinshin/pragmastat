#import "/manual/definitions.typ": *

The $ExpFunction$ function evaluates $e^y$ using only operations whose results IEEE 754 fixes, so
  that seven independent implementations produce identical bits.

The construction is the classical range reduction.
Writing $y$ as a multiple of $ln 2$ plus a small remainder turns the exponential into a power of
  two, which is exact, multiplied by an exponential of a small argument, which a polynomial
  reproduces well:

$ k = floor(y \/ (ln 2) + 1 \/ 2), quad r = y - k ln 2, quad e^y = 2^k dot e^r $

with $abs(r) <= (ln 2) \/ 2 approx 0.347$.
Four decisions within that outline determine whether the result is reproducible, and each is
  described below.

*The rounding is written out rather than named*

$k$ is computed as $floor(y \/ (ln 2) + 1 \/ 2)$ rather than by calling a rounding function.
Languages disagree about what rounding means: some round halves away from zero, others round them
  to even.
Naming a rounding would therefore name a disagreement, while the floor behaves identically
  everywhere.

*$ln 2$ is split so that the subtraction stays exact*

$ln 2$ is stored as two doubles: a high part $L_"hi"$ carrying 32 significant bits, and a low
  remainder $L_"lo"$ holding what the first one drops.
The technique is due to Cody and Waite (@codywaite1980) and is standard in libraries that evaluate
  elementary functions (@muller2016).
The reduction then proceeds in two named halves:

$ "hi" = y - k L_"hi", quad "lo" = k L_"lo", quad r = "hi" - "lo" $

Since $abs(k)$ never exceeds 11 bits, the product $k L_"hi"$ needs at most $32 + 11 = 43$ of the 53
  bits available and is therefore exact, which makes $"hi"$ exact as well.
Computing $y - k ln 2$ in a single step would instead subtract two nearby numbers and lose the low
  bits of $r$, which is where the accuracy of the result resides.

*The polynomial fits a correction rather than the function*

The quantity approximated is not $e^r$ but

$ Q(r) = (e^r - 1 - r) \/ r^2 $

evaluated as a chain of 12 coefficients by Horner's method.
The two leading terms then carry no fitting error at all, and the polynomial supplies only a
  correction whose magnitude never exceeds $0.07$.
The coefficients come out as the Taylor series $1\/2, 1\/6, 1\/24, dots$, which a reader can verify
  by inspection.

*The two halves of the reduction are kept apart until the end*

The result is assembled from $"hi"$ and $"lo"$ rather than from $r$:

$ p = 1 - (("lo" - r^2 Q(r)) - "hi") $

In exact arithmetic this equals $1 + r + r^2 Q(r)$, since $r = "hi" - "lo"$.
The two differ in binary64 because $r$ reaches $0.347$, so adding it to $1$ shifts the mantissa two
  places and the low bits fall off the end.
Reconstructing the sum from the two halves preserves them: $"hi"$ is exact, and only the correction
  term $r^2 Q(r)$ is exposed to the rounded $r$.

This single line accounts for most of the accuracy the function achieves.
Measured against a sixty-digit reference, it reduces the worst relative error from
  $2.19 dot 10^(-16)$ to $1.30 dot 10^(-16)$ and raises the share of correctly rounded results from
  $72.6%$ to $90.0%$, at no cost: the same operations in a different order.

*The scaling proceeds in two steps*

$ e^y = (p dot 2^j) dot 2^(k - j), quad j = "trunc"(k \/ 2) $

Applying $2^k$ in one step would require constructing $2^(-1075)$ at the bottom of the range, which
  is not a representable number.
Splitting the exponent keeps the first factor within the normal range for every $k$, so only the
  second factor can denormalize or overflow, and it does so in a single rounding.

Obtaining $2^n$ exactly is the one place where the seven implementations differ in wording rather
  than in meaning.
Some languages provide a scaling primitive, some construct the exponent field directly, and two
  provide neither and use exponentiation guarded by a test that it is exact across the whole range
  the reduction can reach.
