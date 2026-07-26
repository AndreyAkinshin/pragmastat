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
The CDF is computed exactly:

$ Pr(W <= k) = sum_(i=0)^k binom(n, i) 2^(-n) $

The algorithm evaluates this sum incrementally,
  accumulating probabilities until the cumulative probability reaches $misrate / 2$.

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
