#import "/manual/definitions.typ": *

== Spread

$ Spread(vx) = attach(Median, b: 1 <= i < j <= n) abs(x_i - x_j) $

- Measures dispersion (variability, scatter)
- Corner case: for $n=1$, $Spread(vx) = 0$
- Equals the _Shamos scale estimator_ (@shamos1976), renamed to $Spread$ for clarity
- Pragmatic alternative to the standard deviation and the median absolute deviation
- Asymptotically, $Spread[X]$ is the median of the absolute difference between two random measurements from distribution $X$
- Straightforward implementations have $O(n^2 log n)$ complexity; a fast $O(n log n)$ version is provided in the Algorithms section.
- Domain: any real numbers
- Unit: the same as measurements

$ Spread(vx + k) = Spread(vx) $

$ Spread(k dot vx) = abs(k) dot Spread(vx) $

$ Spread(vx) >= 0 $
