#import "/manual/definitions.typ": *

== Shift

$ Shift(vx, vy) = attach(Median, b: 1 <= i <= n\, 1 <= j <= m) (x_i - y_j) $

- Measures the median of pairwise differences between elements of two samples
- Equals the _Hodges-Lehmann estimator_ for two samples (@hodges1963)
- Asymptotically, $Shift[X, Y]$ is the median of the difference of random measurements from $X$ and $Y$
- Straightforward implementations have $O(m n log(m n))$ complexity; a fast $O((m+n) log L)$ version is provided in the Algorithms section.
- Domain: any real numbers
- Unit: the same as measurements

$ Shift(vx, vx) = 0 $

$ Shift(vx + k_x, vy + k_y) = Shift(vx, vy) + k_x - k_y $

$ Shift(k dot vx, k dot vy) = k dot Shift(vx, vy) $

$ Shift(vx, vy) = -Shift(vy, vx) $
