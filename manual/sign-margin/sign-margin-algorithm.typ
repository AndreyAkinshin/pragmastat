#import "/manual/definitions.typ": *

The $SignMargin$ function determines the exclusion count for disjoint-pair sign-test bounds
  by inverting the $"Binomial"(n, 1\/2)$ CDF.

Given $n$ pairs and a desired $misrate$, the algorithm finds
  the number of extreme order statistics to exclude so that the resulting bounds
  contain the true parameter with probability $1 - misrate$.

*Binomial CDF computation*

Under the symmetry assumption, the number of positive signs among $n$ disjoint-pair differences
  follows $"Binomial"(n, 1\/2)$.
The CDF is computed exactly:

$ Pr(W <= k) = sum_(i=0)^k binom(n, i) 2^(-n) $

The algorithm evaluates this sum incrementally,
  accumulating probabilities until the cumulative probability reaches $misrate / 2$.

*Grid point identification*

Because the Binomial CDF is a step function, the exact $misrate$ typically falls between
  two adjacent grid points $k$ and $k + 1$.
The algorithm identifies these adjacent values:
$k_"lo"$ is the largest integer where $Pr(W <= k_"lo") < misrate / 2$
and $k_"hi" = k_"lo" + 1$.

*Randomized cutoff*

To match the requested $misrate$ exactly rather than conservatively,
  the algorithm interpolates between the two grid points.
It computes a probability $p$ such that
  using margin $2 k_"lo"$ with probability $p$ and margin $2 k_"hi"$ with probability $1 - p$
  yields an expected coverage of exactly $1 - misrate$.
A uniform random draw determines which margin to return.

This randomization ensures that the bounds are calibrated exactly at the requested error rate
  under weak continuity, rather than being conservative due to the discreteness of the Binomial distribution.

#source-include("cs/Pragmastat/Functions/SignMargin.cs", "cs")
