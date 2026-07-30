#import "/manual/definitions.typ": *

Two regimes, split at a threshold every implementation shares.

*Below the threshold: exact integers*

For $n < 62$ the value is accumulated in a 64-bit integer by the same multiplicative recurrence
  used above, with the division performed at each step rather than at the end:

$ binom(n, k) = product_(i=1)^k (n - k + i) \/ i $

Taking one factor and one divisor per step keeps the running product below $2^63$ wherever the
  answer itself is, because the partial product after $i$ steps is $binom(n-k+i, i)$, an integer
  never larger than the final value.
Every step divides exactly, since $i$ divides the product of $i$ consecutive integers.
The result is the exactly rounded double, in every implementation.

*At or above the threshold: the same recurrence in binary64*

The same sequence runs in floating point:

$ a_0 = 1, quad a_i = a_(i-1) dot (n - k + i) \/ i $

One multiply and one divide per step, in that order, and nothing else.
No separate numerator and denominator, no reassociation, no fused multiply-add: the sequence of
  roundings is what the seven implementations share, so any rearrangement that improves accuracy in
  one of them is a divergence.

$k$ is first replaced by $min(k, n - k)$.
That halves the work, and it also makes the function symmetric by construction rather than by
  arithmetic accident, which matters because the two call sites ask for $binom(n+m, n)$ and
  $binom(n+m, m)$.
Those are the same number, and normalizing makes them the same bits, so a comparison at the misrate
  floor cannot be decided by which of the two rounded higher.

The loop stops once the accumulator reaches infinity: the remaining steps multiply by a positive
  integer and divide by a positive integer, which cannot bring it back.
At $n = m = 100000$ that is 89 steps instead of 100000.

*Why the threshold is 62 and not higher*

Three of the seven ports could stay exact well past it.
Python's integers are unbounded, TypeScript has $"BigInt"$, and Rust could use a 128-bit integer;
  each would remain exact to roughly $n = 1030$, where the true value leaves binary64.
All three switch at 62 anyway.

Being more accurate than the other six is a divergence.
The admissible misrate is $2 \/ binom(n+m, n)$, and a port that computed a more accurate
  denominator would accept a misrate its siblings reject, on the same input.
So the threshold is a contract rather than a limit, and the cost of honoring it is visible:
  $binom(62, 31)$ is $465428353255261088$, and every implementation returns a value three units in
  the last place above it.
