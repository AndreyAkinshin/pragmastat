#import "/manual/definitions.typ": *

$SpreadBounds$ targets the population spread
$Spread = median abs(X_1 - X_2)$ with distribution-free coverage.
This note records the approaches we discussed and tried, why most of them fail,
and which theoretical limits force the final design.
Here "distribution-free" means the stated misrate bound holds for *all* distributions of $X$,
not just for a parametric family or under smoothness assumptions.

*Setup and notation*

Let $X_1, .., X_n$ be i.i.d. real-valued observations.
Let $m = floor(n / 2)$ and let $\pi$ be a random disjoint pairing of indices
independent of the values. Define
$ d_i = abs(X_{pi(2i-1)} - X_{pi(2i)}) $ for $i = 1..m$,
and let $d_((1)) <= .. <= d_((m))$ be their order statistics.

Let $G$ be the distribution of $D = abs(X_1 - X_2)$
and let $theta$ be any median of $G$.
The target parameter is $Spread = theta$.

*Valid pivot and exact coverage*

Assume weak continuity of $G$ (no ties), so $P(D <= theta) = 1/2$.
Then the sign count $S$ (the number of indices $i$ with $d_i <= theta$)
has distribution $B tilde "Binomial"(m, 1/2)$.
For any integer $r$,

$
P(theta < d_((r+1))) = P(S <= r), and
P(theta > d_((m-r))) = P(S <= r).
$

Therefore the interval
$[ d_((r+1)), d_((m-r)) ]$
has coverage $1 - 2 P(S <= r)$ for *all* distributions of $X$.
This is the core distribution-free pivot used by $SpreadBounds$.

If $G$ has atoms (ties), then $P(D <= theta) >= 1/2$ for any median $theta$.
The binomial pivot becomes conservative:
the same interval still covers with probability at least $1 - 2 P(S <= r)$,
but exact matching of a requested misrate is impossible.

*Approaches that look reasonable but fail*

*All pairwise differences as if independent*.
Construct $N = n(n-1)/2$ absolute differences and apply a sign-test or binomial
confidence interval as if those $N$ values were i.i.d.
This ignores strong dependence between pairs, so coverage can be arbitrarily wrong.
Correcting for dependence would require the joint law of all pairwise differences,
which depends on the unknown distribution and is not distribution-free.

*U-statistic inequalities for the median*.
Hoeffding-type bounds give distribution-free confidence bands for U-statistics,
but for a median of pairwise differences they are extremely conservative.
Intervals frequently collapse to almost $[min, max]$, which is unusable in practice.
Asymptotic U-quantile results require smoothness and density assumptions,
so they are not distribution-free.

*Deterministic pairing based on order or sorting*.
Pairing consecutive elements in the given order is value-independent,
but it is not permutation-invariant: changing the input order changes the result.
If the order is adversarial (or structured), coverage can deviate arbitrarily.
Pairing after sorting (or pairing extremes) is value-dependent,
so the distribution-free pivot no longer applies.

*Choose the "best" pairing based on the data*.
Searching many pairings and picking the tightest interval is data-dependent.
The selection event depends on the observed values, so coverage is not unconditional.
Coverage can become arbitrarily low.

*Bootstrap, asymptotic normality, or variance estimation*.
These are not distribution-free and can be anti-conservative for heavy tails or small samples.
Studentization helps asymptotically but still depends on distributional regularity
(density, smoothness, finite moments).

*Mid-p or continuity-corrected binomial intervals*.
These reduce discreteness but do not guarantee the nominal misrate
even for the binomial model itself, so the distribution-free guarantee is lost.

*Deterministic disjoint pairs: why it is conservative*

With disjoint pairs, the pivot is valid, but the binomial CDF is discrete.
The achievable misrates form a grid
$ 2 * P(B <= r) $.
If the requested misrate falls between grid points,
a deterministic method must round down, which is conservative by design.
The minimum achievable misrate is $2^(1-m)$.

Deterministic "interpolation" between neighboring order statistics would use
value distances instead of ranks, making coverage distribution-dependent
and invalidating the distribution-free guarantee.
As $m$ grows, the grid becomes finer (step size is $O(1 / sqrt(m))$),
so deterministic rounding converges asymptotically, but exact matching remains impossible
for any finite $n$.

*Working approach: randomized cutoff*

The final design keeps disjoint pairs but randomizes the cutoff $r$ between
adjacent grid points. Let $t = misrate / 2$.
Define $F(r) = P(B <= r)$ and $f(r) = P(B = r)$.
Let $r_l$ be the largest integer such that $F(r_l) <= t$ and let $r_h = r_l + 1$.
Choose $r = r_h$ with probability
$ p = (t - F(r_l)) / f(r_h) $
and $r = r_l$ otherwise. This makes the tail probability exactly $t$.

Randomization affects only the cutoff and is independent of data values,
so the distribution-free property is preserved.

*Theoretical limits*

- *Distribution-free + deterministic + exact misrate is impossible* for finite $n$.
  The pivot statistic is integer-valued, so the coverage function takes
  only finitely many values. Exact matching requires randomization.
- *Exact matching for all distributions* is impossible because ties break the binomial pivot.
  Under weak continuity (no ties), randomized cutoffs achieve exact misrate;
  with atoms, coverage is only guaranteed to be conservative.
- *Deterministic exact misrate* is possible only with extra assumptions
  (parametric family, smooth density at the target, known variance),
  which violates distribution-free guarantees.
- *Any method that treats all pairwise differences as independent* is invalid
  because dependence destroys the binomial pivot.
- *Any data-dependent choice of pairing or cutoff* breaks unconditional coverage
  and can make the method arbitrarily anti-conservative.
