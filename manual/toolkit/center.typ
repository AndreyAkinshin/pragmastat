#import "/manual/definitions.typ": *

== Center

$ Center(vx) = attach(Median, b: 1 <= i <= j <= n) (x_i + x_j) / 2 $

Robust measure of location (central tendency).

#v(0.3em)
#list(marker: none, tight: true,
  [*Also known as* — Hodges-Lehmann estimator, pseudomedian],
  [*Asymptotic* — median of the average of two random measurements from $X$],
  [*Complexity* — $O(n^2 log n)$ naive, $O(n log n)$ fast (see #link(<sec-fast-center>)[Fast Center])],
  [*Domain* — any real numbers],
  [*Unit* — same as measurements],
)

#v(0.5em)
*Properties*

#list(marker: none, tight: true,
  [*Shift equivariance* #h(2em) $Center(vx + k) = Center(vx) + k$],
  [*Scale equivariance* #h(2em) $Center(k dot vx) = k dot Center(vx)$],
)

#v(0.3em)
*Example*

- `Center([0, 2, 4, 6, 8]) = 4`
- `Center(x + 10) = 14` #h(1em) `Center(3x) = 12`

#v(0.5em)
*References*

- @hodges1963
- @sen1963

#v(0.5em)
$Center$ is the recommended default for representing "where the data is."
It works like the familiar mean but does not break when the data contains a few bad measurements or outliers.
Up to 29% of data can be corrupted before $Center$ becomes unreliable.
When data is clean, $Center$ is nearly as precise as the mean (95% efficiency), so the added protection comes at almost no cost.
When uncertain whether to use mean, median, or something else, start with $Center$.
