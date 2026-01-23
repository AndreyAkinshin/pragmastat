#import "/manual/definitions.typ": *

== Center

$ Center(vx) = attach(Median, b: 1 <= i <= j <= n) lr((x_i + x_j) / 2) $

- Measures average (central tendency, measure of location)
- Equals the _Hodges-Lehmann estimator_ (@hodges1963, @sen1963), renamed to $Center$ for clarity
- Also known as 'pseudomedian' because it is consistent with $Median$ for symmetric distributions
- Pragmatic alternative to $Mean$ and $Median$
- Asymptotically, $Center[X]$ is the $Median$ of the arithmetic average of two random measurements from $X$
- Straightforward implementations have $O(n^2 log n)$ complexity; a fast $O(n log n)$ version is provided in the Algorithms section.
- Domain: any real numbers
- Unit: the same as measurements

$ Center(vx + k) = Center(vx) + k $

$ Center(k dot vx) = k dot Center(vx) $
