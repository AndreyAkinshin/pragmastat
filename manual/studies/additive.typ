#import "/manual/definitions.typ": *

== Additive ('Normal') Distribution

The $Additive$ ('Normal') distribution has two parameters: the mean and the standard deviation,
written as $Additive(pmean, pstddev)$.

#v(0.5em)
*Asymptotic Spread Value*

Consider two independent draws $X$ and $Y$ from the $Additive(pmean, pstddev)$ distribution.
The goal is to find the median of their absolute difference $abs(X-Y)$.
Define the difference $D = X - Y$.
By linearity of expectation, $EE[D] = 0$. By independence, $Var[D] = 2 dot pstddev^2$.
Thus $D$ has distribution $Additive(0, sqrt(2) dot pstddev)$,
and the problem reduces to finding the median of $abs(D)$.
The location parameter $pmean$ disappears, as expected,
because absolute differences are invariant to shifts.

Let $tau=sqrt(2) dot pstddev$, so that $D tilde Additive(0, tau)$.
The random variable $abs(D)$ then follows the Half-$Additive$ ('Folded Normal') distribution with scale $tau$.
Its cumulative distribution function for $z >= 0$ becomes

$ F_(abs(D))(z) = Pr(abs(D) <= z) = 2 Phi lr((z\/tau)) - 1 $

where $Phi$ denotes the standard $Additive$ ('Normal') CDF.

The median $m$ is the point at which this cdf equals $1\/2$.
Setting $F_(abs(D))(m)=1\/2$ gives

$ 2 Phi lr((m\/tau)) - 1 = 1\/2 arrow.r.double Phi lr((m\/tau)) = 3\/4 $

Applying the inverse cdf yields $m\/tau = z_(0.75)$.
Substituting back $tau = sqrt(2) dot pstddev$ produces

$ Median(abs(X-Y)) = sqrt(2) dot z_(0.75) dot pstddev $

Define $z_(0.75) := Phi^(-1)(0.75) approx 0.6744897502$. Numerically,
the median absolute difference is approximately $sqrt(2) dot z_(0.75) dot pstddev approx 0.9538725524 dot pstddev$.
This expression depends only on the scale parameter $pstddev$, not on the mean,
reflecting the translation invariance of the problem.

#v(0.5em)
*Lemma: Average Estimator Drift Formula*

For average estimators $T_n$ with asymptotic standard deviation $a dot pstddev \/ sqrt(n)$ around the mean $mu$,
define $RelSpread[T_n] := Spread[T_n] \/ Spread[X]$.
In the $Additive$ ('Normal') case, $Spread[X] = sqrt(2) dot z_(0.75) dot pstddev$.

For any average estimator $T_n$ with asymptotic standard deviation $a dot pstddev \/ sqrt(n)$ around the mean $mu$, the drift calculation follows:

- The spread of two independent estimates: $Spread[T_n] = sqrt(2) dot z_(0.75) dot a dot pstddev \/ sqrt(n)$
- The relative spread: $RelSpread[T_n] = a \/ sqrt(n)$
- The asymptotic drift: $Drift(T, X) = a$

#v(0.5em)
*Asymptotic Mean Drift*

For the sample mean $Mean(vx) = 1\/n sum_(i=1)^n x_i$ applied to samples
from $Additive(pmean, pstddev)$,
the sampling distribution of $Mean$ is also additive with mean $pmean$
and standard deviation $pstddev\/sqrt(n)$.

Using the lemma with $a = 1$ (since the standard deviation is $pstddev\/sqrt(n)$):

$ Drift(Mean, X) = 1 $

$Mean$ achieves unit drift under the $Additive$ ('Normal') distribution, serving as the natural baseline for comparison.
$Mean$ is the optimal estimator under the $Additive$ ('Normal') distribution: no other estimator achieves lower $Drift$.

#v(0.5em)
*Asymptotic Median Drift*

For the sample median $Median(vx)$ applied to samples from $Additive(pmean, pstddev)$,
the asymptotic sampling distribution of $Median$ is approximately $Additive$ ('Normal')
with mean $pmean$ and standard deviation $sqrt(pi\/2) dot pstddev\/sqrt(n)$.

This result follows from the asymptotic theory of order statistics.
For the median of a sample from a continuous distribution with density $f$ and cumulative distribution $F$,
the asymptotic variance is $1\/(4n[f(F^(-1)(0.5))]^2)$.
For the $Additive$ ('Normal') distribution with standard deviation $pstddev$,
the density at the median (which equals the mean) is $1\/(pstddev sqrt(2 pi))$.
Thus the asymptotic variance becomes $pi dot pstddev^2 \/ (2n)$.

Using the lemma with $a = sqrt(pi\/2)$:

$ Drift(Median, X) = sqrt(pi\/2) $

Numerically, $sqrt(pi\/2) approx 1.2533$, so the median has approximately 25% higher drift than the mean
under the $Additive$ ('Normal') distribution.

#v(0.5em)
*Asymptotic Center Drift*

For the sample center $Center(vx) = attach(Median, b: 1 <= i <= j <= n) lr((x_i + x_j)\/2)$ applied to samples from $Additive(pmean, pstddev)$,
its asymptotic sampling distribution must be determined.

The center estimator computes all pairwise averages (including $i=j$) and takes their median.
For the $Additive$ ('Normal') distribution, asymptotic theory shows that the center estimator
is asymptotically $Additive$ ('Normal') with mean $pmean$.

The exact asymptotic variance of the center estimator for the $Additive$ ('Normal') distribution is:

$ Var[Center(X_(1:n))] = (pi dot pstddev^2)\/(3n) $

This gives an asymptotic standard deviation of:

$ StdDev[Center(X_(1:n))] = sqrt(pi\/3) dot pstddev\/sqrt(n) $

Using the lemma with $a = sqrt(pi\/3)$:

$ Drift(Center, X) = sqrt(pi\/3) $

Numerically, $sqrt(pi\/3) approx 1.0233$,
so the center estimator achieves a drift very close to 1 under the $Additive$ ('Normal') distribution,
performing nearly as well as the mean while offering greater robustness to outliers.

#v(0.5em)
*Lemma: Dispersion Estimator Drift Formula*

For dispersion estimators $T_n$ with asymptotic center $b dot pstddev$
and standard deviation $a dot pstddev \/ sqrt(n)$,
define $RelSpread[T_n] := Spread[T_n] \/ (b dot pstddev)$.

For any dispersion estimator $T_n$ with asymptotic distribution $T_n approxdist Additive(b dot pstddev, (a dot pstddev)^2 \/ n)$, the drift calculation follows:

- The spread of two independent estimates: $Spread[T_n] = sqrt(2) dot z_(0.75) dot a dot pstddev \/ sqrt(n)$
- The relative spread: $RelSpread[T_n] = sqrt(2) dot z_(0.75) dot a \/ (b sqrt(n))$
- The asymptotic drift: $Drift(T, X) = sqrt(2) dot z_(0.75) dot a \/ b$

Note: The $sqrt(2)$ factor comes from the standard deviation of the difference $D = T_1 - T_2$
of two independent estimates,
and the $z_(0.75)$ factor converts this standard deviation to the median absolute difference.

#v(0.5em)
*Asymptotic StdDev Drift*

For the sample standard deviation $StdDev(vx) = sqrt(1\/(n-1) sum_(i=1)^n (x_i - Mean(vx))^2)$
applied to samples from $Additive(pmean, pstddev)$,
the sampling distribution of $StdDev$ is approximately $Additive$ ('Normal') for large $n$
with mean $pstddev$ and standard deviation $pstddev\/sqrt(2n)$.

Applying the lemma with $a = 1\/sqrt(2)$ and $b = 1$:

$ Spread[StdDev(X_(1:n))] = sqrt(2) dot z_(0.75) dot 1\/sqrt(2) dot pstddev\/sqrt(n) = z_(0.75) dot pstddev\/sqrt(n) $

For the dispersion drift, we use the relative spread formula:

$ RelSpread[StdDev(X_(1:n))] = Spread[StdDev(X_(1:n))]\/Center[StdDev(X_(1:n))] $

Since $Center[StdDev(X_(1:n))] approx pstddev$ asymptotically:

$ RelSpread[StdDev(X_(1:n))] = (z_(0.75) dot pstddev\/sqrt(n))\/pstddev = z_(0.75)\/sqrt(n) $

Therefore:

$ Drift(StdDev, X) = lim_(n -> infinity) sqrt(n) dot RelSpread[StdDev(X_(1:n))] = z_(0.75) $

Numerically, $z_(0.75) approx 0.67449$.

#v(0.5em)
*Asymptotic MAD Drift*

For the median absolute deviation $MAD(vx) = Median(abs(x_i - Median(vx)))$
applied to samples from $Additive(pmean, pstddev)$,
the asymptotic distribution is approximately $Additive$ ('Normal').

For the $Additive$ ('Normal') distribution, the population MAD equals $z_(0.75) dot pstddev$.
The asymptotic standard deviation of the sample MAD is:

$ StdDev[MAD(X_(1:n))] = cmad dot pstddev\/sqrt(n) $

where $cmad approx 0.78$.

Applying the lemma with $a = cmad$ and $b = z_(0.75)$:

$ Spread[MAD(X_(1:n))] = sqrt(2) dot z_(0.75) dot cmad dot pstddev\/sqrt(n) $

Since $Center[MAD(X_(1:n))] approx z_(0.75) dot pstddev$ asymptotically:

$ RelSpread[MAD(X_(1:n))] = (sqrt(2) dot z_(0.75) dot cmad dot pstddev\/sqrt(n))\/(z_(0.75) dot pstddev) = (sqrt(2) dot cmad)\/sqrt(n) $

Therefore:

$ Drift(MAD, X) = lim_(n -> infinity) sqrt(n) dot RelSpread[MAD(X_(1:n))] = sqrt(2) dot cmad $

Numerically, $sqrt(2) dot cmad approx sqrt(2) dot 0.78 approx 1.10$.

#v(0.5em)
*Asymptotic Spread Drift*

For the sample spread $Spread(vx) = attach(Median, b: 1 <= i < j <= n) abs(x_i - x_j)$
applied to samples from $Additive(pmean, pstddev)$,
the asymptotic distribution is approximately $Additive$ ('Normal').

The spread estimator computes all pairwise absolute differences and takes their median.
For the $Additive$ ('Normal') distribution, the population spread equals $sqrt(2) dot z_(0.75) dot pstddev$
as derived in the Asymptotic Spread Value section.

The asymptotic standard deviation of the sample spread for the $Additive$ ('Normal') distribution is:

$ StdDev[Spread(X_(1:n))] = cspr dot pstddev\/sqrt(n) $

where $cspr approx 0.72$.

Applying the lemma with $a = cspr$ and $b = sqrt(2) dot z_(0.75)$:

$ Spread[Spread(X_(1:n))] = sqrt(2) dot z_(0.75) dot cspr dot pstddev\/sqrt(n) $

Since $Center[Spread(X_(1:n))] approx sqrt(2) dot z_(0.75) dot pstddev$ asymptotically:

$ RelSpread[Spread(X_(1:n))] = (sqrt(2) dot z_(0.75) dot cspr dot pstddev\/sqrt(n))\/(sqrt(2) dot z_(0.75) dot pstddev) = cspr\/sqrt(n) $

Therefore:

$ Drift(Spread, X) = lim_(n -> infinity) sqrt(n) dot RelSpread[Spread(X_(1:n))] = cspr $

Numerically, $cspr approx 0.72$.

#v(0.5em)
*Summary*

*Summary for average estimators:*

#table(
  columns: 4,
  [Estimator], [$Drift(E, X)$], [$Drift^2(E, X)$], [$1\/Drift^2(E, X)$],
  [$Mean$], [$1$], [$1$], [$1$],
  [$Median$], [$approx 1.253$], [$pi\/2 approx 1.571$], [$2\/pi approx 0.637$],
  [$Center$], [$approx 1.023$], [$pi\/3 approx 1.047$], [$3\/pi approx 0.955$],
)

The squared drift values indicate the sample size adjustment needed when switching estimators.
For instance, switching from $Mean$ to $Median$ while maintaining the same precision
requires increasing the sample size by a factor of $pi\/2 approx 1.571$ (about 57% more observations).
Similarly, switching from $Mean$ to $Center$ requires only about 5% more observations.

The inverse squared drift (rightmost column) equals the classical statistical efficiency relative to the $Mean$.
The $Mean$ achieves optimal performance (unit efficiency) for the $Additive$ ('Normal') distribution,
as expected from classical theory.
The $Center$ maintains 95.5% efficiency while offering greater robustness to outliers,
making it an attractive alternative when some contamination is possible.
The $Median$, while most robust, operates at only 63.7% efficiency
under purely $Additive$ ('Normal') conditions.

*Summary for dispersion estimators:*

For the $Additive$ ('Normal') distribution, the asymptotic drift values reveal the relative precision of different dispersion estimators:

#table(
  columns: 4,
  [Estimator], [$Drift(E, X)$], [$Drift^2(E, X)$], [$1\/Drift^2(E, X)$],
  [$StdDev$], [$approx 0.67$], [$approx 0.45$], [$approx 2.22$],
  [$MAD$], [$approx 1.10$], [$approx 1.22$], [$approx 0.82$],
  [$Spread$], [$approx 0.72$], [$approx 0.52$], [$approx 1.92$],
)

The squared drift values indicate the sample size adjustment needed when switching estimators.
For instance, switching from $StdDev$ to $MAD$ while maintaining the same precision
requires increasing the sample size by a factor of $1.22\/0.45 approx 2.71$ (more than doubling the observations).
Similarly, switching from $StdDev$ to $Spread$ requires a factor of $0.52\/0.45 approx 1.16$.

The $StdDev$ achieves optimal performance for the $Additive$ ('Normal') distribution.
The $MAD$ requires about 2.7 times more data to match the $StdDev$ precision
while offering greater robustness to outliers.
The $Spread$ requires about 1.16 times more data to match the $StdDev$ precision under purely $Additive$ ('Normal') conditions while maintaining robustness.
