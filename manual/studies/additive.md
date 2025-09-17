## Additive ('Normal') Distribution

The $\Additive$ ('Normal') distribution has two parameters: the mean and the standard deviation,
  written as $\Additive(\mathrm{mean}, \mathrm{stdDev})$.

### Asymptotic Spread Value

Consider two independent draws $X$ and $Y$ from the $\Additive(\mathrm{mean}, \mathrm{stdDev})$ distribution.
The goal is to find the median of their absolute difference $|X-Y|$.
Define the difference $D=X-Y$.
By linearity of expectation, $E[D] = 0$. By independence, $\mathrm{Var}[D] = 2 \cdot \mathrm{stdDev}^2$.
Thus $D$ has distribution $\Additive(0, \sqrt{2} \cdot \mathrm{stdDev})$,
  and the problem reduces to finding the median of $|D|$.
The location parameter $\mathrm{mean}$ disappears, as expected,
  because absolute differences are invariant under shifts.

Let $\tau=\sqrt{2} \cdot \mathrm{stdDev}$, so that $D\sim \Additive(0,\tau)$.
The random variable $|D|$ then follows the Half-$\Additive$ ('Folded Normal') distribution with scale $\tau$.
Its cumulative distribution function for $z\ge 0$ becomes

$$
F_{|D|}(z) = \Pr(|D|\le z) = 2\Phi\!\left(\frac{z}{\tau}\right) - 1,
$$

where $\Phi$ denotes the standard $\Additive$ ('Normal') CDF.

The median $m$ is the point at which this cdf equals $1/2$.
Setting $F_{|D|}(m)=1/2$ gives

$$
2\Phi\!\left(\frac{m}{\tau}\right)-1 = \tfrac{1}{2}
\quad\Longrightarrow\quad
\Phi\!\left(\frac{m}{\tau}\right)=\tfrac{3}{4}.
$$

Applying the inverse cdf yields $m/\tau=z_{0.75}$.
Substituting back $\tau=\sqrt{2} \cdot \mathrm{stdDev}$ produces

$$
\Median(|X-Y|) = \sqrt{2} \cdot z_{0.75} \cdot \mathrm{stdDev}.
$$

Hence $\Median(|X-Y|) = \sqrt{2} \cdot z_{0.75} \cdot \mathrm{stdDev}$.

Define $z_{0.75} := \Phi^{-1}(0.75) \approx 0.6744897502$. Numerically,
  the median absolute difference is approximately $\sqrt{2} \cdot z_{0.75} \cdot \mathrm{stdDev} \approx 0.9538725524 \cdot \mathrm{stdDev}$.
This expression depends only on the scale parameter $\mathrm{stdDev}$, not on the mean,
  reflecting the translation invariance of the problem.

### Lemma: Average Estimator Drift Formula

For average estimators $T_n$ with asymptotic standard deviation $a \cdot \mathrm{stdDev} / \sqrt{n}$ around the mean $\mu$,
  define $\RelSpread[T_n] := \Spread[T_n] / \Spread[X]$.
In the $\Additive$ ('Normal') case, $\Spread[X] = \sqrt{2} \cdot z_{0.75} \cdot \mathrm{stdDev}$.

For any average estimator $T_n$ with asymptotic distribution $T_n \sim \text{approx } N(\mu, (a \cdot \mathrm{stdDev})^2 / n)$, the drift calculation follows:

- The spread of two independent estimates: $\Spread[T_n] = \sqrt{2} \cdot z_{0.75} \cdot a \cdot \mathrm{stdDev} / \sqrt{n}$
- The relative spread: $\RelSpread[T_n] = a / \sqrt{n}$
- The asymptotic drift: $\Drift(T,X) = a$

### Asymptotic Mean Drift

For the sample mean $\Mean(\x) = \frac{1}{n}\sum_{i=1}^n x_i$ applied to samples
  from $\Additive(\mathrm{mean}, \mathrm{stdDev})$,
  the sampling distribution of $\Mean$ is also additive with mean $\mathrm{mean}$
  and standard deviation $\mathrm{stdDev}/\sqrt{n}$.

Using the lemma with $a = 1$ (since the standard deviation is $\mathrm{stdDev}/\sqrt{n}$):

$$
\Drift(\Mean, X) = 1
$$

$\Mean$ achieves unit drift under $\Additive$ ('Normal') distribution, serving as the natural baseline for comparison.
$\Mean$ is the optimal estimator under $\Additive$ ('Normal') distribution: no other estimators achieve lower $\Drift$.

### Asymptotic Median Drift

For the sample median $\Median(\x)$ applied to samples from $\Additive(\mathrm{mean}, \mathrm{stdDev})$,
  the asymptotic sampling distribution of $\Median$ is approximately $\Additive$ ('Normal')
  with mean $\mathrm{mean}$ and standard deviation $\sqrt{\pi/2} \cdot \mathrm{stdDev}/\sqrt{n}$.

This result follows from the asymptotic theory of order statistics.
For the median of a sample from a continuous distribution with density $f$ and cumulative distribution $F$,
  the asymptotic variance is $1/(4n[f(F^{-1}(0.5))]^2)$.
For the $\Additive$ ('Normal') distribution with standard deviation $\mathrm{stdDev}$,
  the density at the median (which equals the mean) is $1/(\mathrm{stdDev}\sqrt{2\pi})$.
Thus the asymptotic variance becomes $\pi \cdot \mathrm{stdDev}^2/(2n)$.

Using the lemma with $a = \sqrt{\pi/2}$:

$$
\Drift(\Median, X) = \sqrt{\frac{\pi}{2}}
$$

Numerically, $\sqrt{\pi/2} \approx 1.2533$, so the median has approximately 25% higher drift than the mean
  under the $\Additive$ ('Normal') distribution.

### Asymptotic Center Drift

For the sample center $\Center(\x) = \underset{1 \leq i \leq j \leq n}{\Median} \left(\frac{x_i + x_j}{2}\right)$ applied to samples from $\Additive(\mathrm{mean}, \mathrm{stdDev})$,
  we need to determine the asymptotic sampling distribution.

The center estimator computes all pairwise averages (including $i=j$) and takes their median.
For the $\Additive$ ('Normal') distribution, the asymptotic theory shows that the center estimator
  is asymptotically $\Additive$ ('Normal') with mean $\mathrm{mean}$.

The exact asymptotic variance of the center estimator for the $\Additive$ ('Normal') distribution is:

$$
\mathrm{Var}[\Center(X_{1:n})] = \frac{\pi \cdot \mathrm{stdDev}^2}{3n}
$$

This gives an asymptotic standard deviation of:

$$
\mathrm{StdDev}[\Center(X_{1:n})] = \sqrt{\frac{\pi}{3}} \cdot \frac{\mathrm{stdDev}}{\sqrt{n}}
$$

Using the lemma with $a = \sqrt{\pi/3}$:

$$
\Drift(\Center, X) = \sqrt{\frac{\pi}{3}}
$$

Numerically, $\sqrt{\pi/3} \approx 1.0233$,
  so the center estimator achieves drift very close to 1 under the $\Additive$ ('Normal') distribution,
  performing nearly as well as the mean while offering greater robustness to outliers.

### Lemma: Dispersion Estimator Drift Formula

For dispersion estimators $T_n$ with asymptotic center $b \cdot \mathrm{stdDev}$
  and standard deviation $a \cdot \mathrm{stdDev} / \sqrt{n}$,
  define $\RelSpread[T_n] := \Spread[T_n] / (b \cdot \mathrm{stdDev})$.

For any dispersion estimator $T_n$ with asymptotic distribution $T_n \sim \text{approx } N(b \cdot \mathrm{stdDev}, (a \cdot \mathrm{stdDev})^2 / n)$, the drift calculation follows:

- The spread of two independent estimates: $\Spread[T_n] = \sqrt{2} \cdot z_{0.75} \cdot a \cdot \mathrm{stdDev} / \sqrt{n}$
- The relative spread: $\RelSpread[T_n] = \sqrt{2} \cdot z_{0.75} \cdot a / (b\sqrt{n})$
- The asymptotic drift: $\Drift(T,X) = \sqrt{2} \cdot z_{0.75} \cdot a / b$

Note: The $\sqrt{2}$ factor comes from the standard deviation of the difference $D = T_1 - T_2$
  of two independent estimates,
  and the $z_{0.75}$ factor converts this standard deviation to the median absolute difference.

### Asymptotic StdDev Drift

For the sample standard deviation $\StdDev(\x) = \sqrt{\frac{1}{n-1}\sum_{i=1}^n (x_i - \Mean(\x))^2}$
  applied to samples from $\Additive(\mathrm{mean}, \mathrm{stdDev})$,
  the sampling distribution of $\StdDev$ is approximately $\Additive$ ('Normal') for large $n$
  with mean $\mathrm{stdDev}$ and standard deviation $\mathrm{stdDev}/\sqrt{2n}$.

Applying the lemma with $a = 1/\sqrt{2}$ and $b = 1$:

$$
\Spread[\StdDev(X_{1:n})] = \sqrt{2} \cdot z_{0.75} \cdot \frac{1}{\sqrt{2}} \cdot \frac{\mathrm{stdDev}}{\sqrt{n}} = z_{0.75} \cdot \frac{\mathrm{stdDev}}{\sqrt{n}}
$$

For the dispersion drift, we use the relative spread formula:

$$
\RelSpread[\StdDev(X_{1:n})] = \frac{\Spread[\StdDev(X_{1:n})]}{\Center[\StdDev(X_{1:n})]}
$$

Since $\Center[\StdDev(X_{1:n})] \approx \mathrm{stdDev}$ asymptotically:

$$
\RelSpread[\StdDev(X_{1:n})] = \frac{z_{0.75} \cdot \mathrm{stdDev}/\sqrt{n}}{\mathrm{stdDev}} = \frac{z_{0.75}}{\sqrt{n}}
$$

Therefore:

$$
\Drift(\StdDev, X) = \lim_{n \to \infty} \sqrt{n} \cdot \RelSpread[\StdDev(X_{1:n})] = z_{0.75}
$$

Numerically, $z_{0.75} \approx 0.67449$.

### Asymptotic MAD Drift

For the median absolute deviation $\MAD(\x) = \Median(|x_i - \Median(\x)|)$
  applied to samples from $\Additive(\mathrm{mean}, \mathrm{stdDev})$,
  the asymptotic distribution is approximately $\Additive$ ('Normal').

For the $\Additive$ ('Normal') distribution, the population MAD equals $z_{0.75} \cdot \mathrm{stdDev}$.
The asymptotic standard deviation of the sample MAD is:

$$
\mathrm{StdDev}[\MAD(X_{1:n})] = c_{\mathrm{mad}} \cdot \frac{\mathrm{stdDev}}{\sqrt{n}}
$$

where $c_{\mathrm{mad}} \approx 0.78$.

Applying the lemma with $a = c_{\mathrm{mad}}$ and $b = z_{0.75}$:

$$
\Spread[\MAD(X_{1:n})] = \sqrt{2} \cdot z_{0.75} \cdot c_{\mathrm{mad}} \cdot \frac{\mathrm{stdDev}}{\sqrt{n}}
$$

Since $\Center[\MAD(X_{1:n})] \approx z_{0.75} \cdot \mathrm{stdDev}$ asymptotically:

$$
\RelSpread[\MAD(X_{1:n})] = \frac{\sqrt{2} \cdot z_{0.75} \cdot c_{\mathrm{mad}} \cdot \mathrm{stdDev}/\sqrt{n}}{z_{0.75} \cdot \mathrm{stdDev}} = \frac{\sqrt{2} \cdot c_{\mathrm{mad}}}{\sqrt{n}}
$$

Therefore:

$$
\Drift(\MAD, X) = \lim_{n \to \infty} \sqrt{n} \cdot \RelSpread[\MAD(X_{1:n})] = \sqrt{2} \cdot c_{\mathrm{mad}}
$$

Numerically, $\sqrt{2} \cdot c_{\mathrm{mad}} \approx \sqrt{2} \cdot 0.78 \approx 1.10$.

### Asymptotic Spread Drift

For the sample spread $\Spread(\x) = \underset{1 \leq i < j \leq n}{\Median} |x_i - x_j|$
  applied to samples from $\Additive(\mathrm{mean}, \mathrm{stdDev})$,
  the asymptotic distribution is approximately $\Additive$ ('Normal').

The spread estimator computes all pairwise absolute differences and takes their median.
For the $\Additive$ ('Normal') distribution, the population spread equals $\sqrt{2} \cdot z_{0.75} \cdot \mathrm{stdDev}$
  as derived in the Asymptotic Spread Value section.

The asymptotic standard deviation of the sample spread for the $\Additive$ ('Normal') distribution is:

$$
\mathrm{StdDev}[\Spread(X_{1:n})] = c_{\mathrm{spr}} \cdot \frac{\mathrm{stdDev}}{\sqrt{n}}
$$

where $c_{\mathrm{spr}} \approx 0.72$.

Applying the lemma with $a = c_{\mathrm{spr}}$ and $b = \sqrt{2} \cdot z_{0.75}$:

$$
\Spread[\Spread(X_{1:n})] = \sqrt{2} \cdot z_{0.75} \cdot c_{\mathrm{spr}} \cdot \frac{\mathrm{stdDev}}{\sqrt{n}}
$$

Since $\Center[\Spread(X_{1:n})] \approx \sqrt{2} \cdot z_{0.75} \cdot \mathrm{stdDev}$ asymptotically:

$$
\RelSpread[\Spread(X_{1:n})] = \frac{\sqrt{2} \cdot z_{0.75} \cdot c_{\mathrm{spr}} \cdot \mathrm{stdDev}/\sqrt{n}}{\sqrt{2} \cdot z_{0.75} \cdot \mathrm{stdDev}} = \frac{c_{\mathrm{spr}}}{\sqrt{n}}
$$

Therefore:

$$
\Drift(\Spread, X) = \lim_{n \to \infty} \sqrt{n} \cdot \RelSpread[\Spread(X_{1:n})] = c_{\mathrm{spr}}
$$

Numerically, $c_{\mathrm{spr}} \approx 0.72$.

### Summary

**Summary for average estimators:**

| Estimator | $\Drift(E, X)$ | $\Drift^2(E, X)$ | $1/\Drift^2(E, X)$ |
|-----------|----------------|------------------|--------------------|
| $\Mean$   | $1$            | $1$              | $1$                |
| $\Median$ | $\approx 1.253$ | $\pi/2 \approx 1.571$ | $2/\pi \approx 0.637$ |
| $\Center$ | $\approx 1.023$ | $\pi/3 \approx 1.047$ | $3/\pi \approx 0.955$ |

The squared drift values indicate the sample size adjustment needed when switching estimators.
For instance, switching from $\Mean$ to $\Median$ while maintaining the same precision
  requires increasing the sample size by a factor of $\pi/2 \approx 1.571$ (about 57% more observations).
Similarly, switching from $\Mean$ to $\Center$ requires only about 5% more observations.

The efficiency column shows the classical statistical efficiency relative to the $\Mean$.
The $\Mean$ achieves optimal performance (unit efficiency) for the $\Additive$ ('Normal') distribution,
  as expected from classical theory.
The $\Center$ maintains 95.5% efficiency while offering greater robustness to outliers,
  making it an attractive alternative when some contamination is possible.
The $\Median$, while most robust, operates at only 63.7% efficiency
  under purely $\Additive$ ('Normal') conditions.

**Summary for dispersion estimators:**

For the $\Additive$ ('Normal') distribution, the asymptotic drift values reveal the relative precision of different dispersion estimators:

| Estimator | $\Drift(E, X)$ | $\Drift^2(E, X)$ | $1/\Drift^2(E, X)$ |
|-----------|----------------|------------------|--------------------|
| $\StdDev$ | $\approx 0.67$ | $\approx 0.45$   | $\approx 2.22$     |
| $\MAD$    | $\approx 1.10$ | $\approx 1.22$   | $\approx 0.82$     |
| $\Spread$ | $\approx 0.72$ | $\approx 0.52$   | $\approx 1.92$     |

The squared drift values indicate the sample size adjustment needed when switching estimators.
For instance, switching from $\StdDev$ to $\MAD$ while maintaining the same precision
  requires increasing the sample size by a factor of $1.22/0.45 \approx 2.71$ (more than doubling the observations).
Similarly, switching from $\StdDev$ to $\Spread$ requires a factor of $0.52/0.45 \approx 1.16$.

The $\StdDev$ achieves optimal performance for the $\Additive$ ('Normal') distribution.
The $\MAD$ requires about 2.7 times more data to match $\StdDev$ precision,
  while offering greater robustness to outliers.
The $\Spread$ requires about 1.16 times more data to match $\StdDev$ precision under purely $\Additive$ ('Normal') conditions while maintaining robustness.