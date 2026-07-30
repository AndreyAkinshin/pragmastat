#import "/manual/definitions.typ": *

The $SignMargin$ function determines the exclusion count for disjoint-pair sign-test bounds
  by inverting the $"Binomial"(n, 1\/2)$ CDF.

Given $n$ pairs and a desired $misrate$, the algorithm finds
  the number of extreme order statistics to exclude so that the resulting bounds
  contain the true parameter with probability $1 - misrate$.

*Binomial CDF computation*

Each disjoint-pair value exceeds the true parameter with probability $1\/2$ by construction,
  since that parameter is the median of the distribution those values are drawn from
  (for #link(<sec-spread-bounds>)[$SpreadBounds$], the absolute differences $abs(X_1 - X_2)$).
The count of values above the parameter therefore follows $"Binomial"(n, 1\/2)$;
  no symmetry assumption is needed.
The distribution function is the partial sum

$ Pr(W <= k) = sum_(i=0)^k binom(n, i) 2^(-n) $

which the algorithm accumulates term by term until it reaches $misrate \/ 2$, taking each term
  from the previous one by the multiplicative recurrence
  $binom(n, k+1) = binom(n, k) dot (n - k) \/ (k + 1)$.

No logarithm appears, and that is deliberate rather than incidental.
The sum used to be evaluated in log space, which put nine library calls on a path that returns an
  index: IEEE 754 fixes neither the logarithm nor the exponential, and two ports disagreed on
  #link(<sec-spread-bounds>)[$SpreadBounds$] for a sample of 200 consecutive integers.
The recurrence calls nothing.
Its terms are one multiply and one divide apart, and the running term is carried as $w dot 2^e$
  with the exponent tracked separately, since $Pr(W = 0) = 2^(-n)$ underflows past $n = 1074$;
  rescaling multiplies by a power of two, which is exact.

The sum is accurate rather than exact.
Measured against exact rational arithmetic over 195 $(n, misrate)$ pairs spanning $n = 1$ to
  $5000$, it selects the right index every time and the randomization fraction below is within
  $6.1 dot 10^(-13)$; the log-space form reached $1.9 dot 10^(-11)$ on the same set.
One case is handled directly because no summation can reproduce it: $"Binomial"(n, 1\/2)$ is
  symmetric, so for odd $n$ the distribution function at $(n-1) \/ 2$ is exactly one half, and at
  $misrate = 1$ the comparison would otherwise be settled by the last accumulated bit.

*Grid point identification*

Because the Binomial CDF is a step function, the exact $misrate$ typically falls between
  two adjacent grid points $k$ and $k + 1$.
The algorithm identifies these adjacent values:
$k_"lo"$ is the largest integer where $Pr(W <= k_"lo") <= misrate / 2$
and $k_"hi" = k_"lo" + 1$.

*Randomized cutoff*

To match the requested $misrate$ exactly rather than conservatively,
  the algorithm interpolates between the two grid points.
It computes a probability $p$ such that
  using margin $2 k_"hi"$ with probability $p$ and margin $2 k_"lo"$ with probability $1 - p$
  yields an expected coverage of exactly $1 - misrate$.
A uniform random draw determines which margin to return.

This randomization ensures that the bounds are calibrated exactly at the requested error rate
  under weak continuity, rather than being conservative due to the discreteness of the Binomial distribution.

#source-include("cs/Pragmastat/Functions/SignMargin.cs", "cs")
