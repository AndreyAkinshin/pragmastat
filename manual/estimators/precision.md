## Precision

$$
\Precision(\x) = \frac{2 \cdot \Spread(\x)}{\sqrt{n}}
$$

**Practical Recommendations**

The interval $\Center(\x) \pm k \cdot \Precision(\x)$ contains the true center value with probability depending on $k$.
Select $k \in \{1, 2, 3\}$ based on required confidence.

Low error costs: $\Center(\x) \pm \Precision(\x)$ provides a reasonable interval.  
High error costs: repeat the experiment 3–5 times and use $\Center(\x) \pm 2 \cdot \Precision(\x)$.  
Critical applications: repeat the experiment 7–10 times and use $\Center(\x) \pm 3 \cdot \Precision(\x)$.  

**Key Facts**

- Measures how much $\Center(\x)$ would fluctuate from sample to sample if samples of the same size
  were repeatedly drawn under the same conditions
- Domain: any real numbers
- Can be perceived as half of a confidence interval with high confidence level

**Properties**

$$
\Precision(\x + k) = \Precision(\x)
$$

$$
\Precision(k \cdot \x) = |k| \cdot \Precision(\x)
$$

$$
\Precision(\x) \geq 0
$$

**Comments on default factor of 2**

Practitioners tend to perceive estimations $a \pm b$ as "the value is inside $[a-b; a+b]$" ignoring uncertainty.
The unscaled interval $\Center(x) \pm \Spread(\x)/\sqrt{n}$ covers the true distribution center value only in $\approx 65\%$ of cases.
To reduce risks of misinterpretation of $\Center(\x) \pm \Precision(\x)$, it is reasonable to use the default factor for $\Precision(\x)$
  to ensure high coverage of such intervals for the true distribution center value.
For simpler calculations, it's convenient to use natural numbers as scale factors for $\Precision(\x)$.
The factor of $2$ is chosen since it's the smallest natural number that ensures decent coverage.
Natural coefficients $k$ produce a standardized discrete precision scale with values of
  $k \cdot \Precision(\x)$ or $2k \cdot \Spread(\x) / \sqrt{n}$. 

**Relation between Precision and Confidence Intervals**

In the strict normal model, it's convenient to express precision via the *standard error* (the standard deviation divided by the square root of sample size).
The standard error can be scaled to the margin of error (half of a confidence interval) for the given confidence level.
Choosing between confidence levels — $95\%$, $99\%$, $99.9\%$, or even $89\%$[^ci89] — remains arbitrary.
Practitioners struggle to extract insights from confidence intervals and levels without calculation tools.
This difficulty leads to frequent misinterpretation because no standard exists for choosing levels.
Knowing one confidence interval determines all others through constant relationships,
  yet practitioners report two numbers (interval size and confidence level) that are hard to comprehend together.
A single standardized value would serve practitioners better.

[^ci89]: See https://github.com/easystats/bayestestR/discussions/250

A standardized value simplifies reporting and improves consistency.
A memorable definition enables mental calculation without advanced tools.
Practitioners develop intuition through repeated use.

$\Precision$ has no direct mapping into traditional confidence intervals.
Confidence intervals ensure the declared coverage rate only under perfect normality.
Real data provides no guarantees that a $99\%$ confidence interval actually covers the true value in
  $99\%$ of experiments.
The table below shows the translation from $\Center(\x) \pm k \cdot \Precision(\x)$ to confidence levels under
  normality:

|  n|    k=1 |    k=2 |    k=3 |
|--:|-------:|-------:|-------:|
|  2| 0.78364| 0.88862| 0.92534|
|  3| 0.88660| 0.96741| 0.98507|
|  4| 0.89523| 0.98145| 0.99413|
|  5| 0.88560| 0.97826| 0.99282|
|  6| 0.90069| 0.98674| 0.99675|
|  7| 0.90220| 0.98965| 0.99803|
|  8| 0.90802| 0.99175| 0.99861|
|  9| 0.91230| 0.99368| 0.99917|
| 10| 0.91461| 0.99483| 0.99946|
| 11| 0.91687| 0.99556| 0.99960|
| 12| 0.91886| 0.99626| 0.99973|
| 13| 0.92022| 0.99678| 0.99980|
| 14| 0.92156| 0.99714| 0.99984|
| 15| 0.92291| 0.99748| 0.99988|
| 16| 0.92384| 0.99770| 0.99991|
| 17| 0.92447| 0.99796| 0.99993|
| 18| 0.92534| 0.99813| 0.99994|
| 19| 0.92629| 0.99828| 0.99995|
| 20| 0.92691| 0.99841| 0.99996|
| 21| 0.92720| 0.99853| 0.99997|
| 22| 0.92780| 0.99864| 0.99998|
| 23| 0.92817| 0.99871| 0.99998|
| 24| 0.92889| 0.99879| 0.99998|
| 25| 0.92916| 0.99883| 0.99998|
| 26| 0.92945| 0.99892| 0.99999|
| 27| 0.92979| 0.99897| 0.99999|
| 28| 0.93006| 0.99902| 0.99999|
| 29| 0.93029| 0.99907| 0.99999|
| 30| 0.93051| 0.99909| 0.99999|
