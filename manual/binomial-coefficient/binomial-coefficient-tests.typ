#import "/manual/definitions.typ": *

$ BinomialCoefficient(n, k) $

This function has no fixture suite of its own, and the reason is worth stating rather than leaving
  to inference.

Every value it produces reaches a shared suite already.
$binom(n+m, n)$ is the admissible misrate that #link(<sec-pairwise-margin>)[$PairwiseMargin$]
  rejects below, and $binom(n+m, m)$ normalizes the exact distribution that same function inverts,
  so the `pairwise-margin` fixtures compare this function's output at every sample size they cover,
  by the integer it decides rather than by the value itself.
An integer that changes is a fixture that fails; a last-bit difference that changes no integer is
  not something a fixture of this function would be able to interpret either.

What the shared fixtures cannot check is the two properties that hold by construction, so each port
  checks those directly.

*Symmetry* — $binom(n, k)$ and $binom(n, n-k)$ are the same number, so they must be the same bits.
The two call sites ask for $binom(n+m, n)$ and $binom(n+m, m)$, and the misrate floor compares one
  against the other; a one-unit gap between them would decide that comparison.
Normalizing $k$ to the smaller half makes the property hold by construction rather than by
  arithmetic, and the test holds it there.

*The threshold* — the switch from integer to floating-point arithmetic at $n + k = 62$ is a
  cross-language contract, and the values on either side of it are pinned.
$binom(61, 30)$ is exact; $binom(62, 31)$ is three units in the last place above the true value, in
  every port, deliberately.
A port that became more accurate here would reject a misrate its siblings accept.

*The overflow boundary* — the recurrence returns infinity from $n = 1021$ while the exact value
  stays inside binary64 through $n = 1029$.
That window is the price of the shared operation order, and it moves if the order changes: deferring
  the divide would push the boundary out and change every rounding along the way.
One port pins both ends of the window, which is enough because all seven perform the identical
  sequence and the shared margin fixtures are what establish that.
