#import "/manual/definitions.typ": *

This section compares the toolkit's robust estimators against traditional statistical methods
  to demonstrate their advantages across diverse conditions.
While traditional estimators often work well under ideal conditions,
  the toolkit's estimators maintain reliable performance across diverse real-world scenarios.

Average Estimators:

*Mean* (arithmetic average):
$ Mean(vx) = 1/n sum_(i=1)^n x_i $

*Median*:
$ Median(vx) = cases(
  x_(((n+1)\/2)) & "if" n "is odd",
  (x_((n\/2)) + x_((n\/2+1))) / 2 & "if" n "is even"
) $

*Center* (Hodges-Lehmann estimator):
$ Center(vx) = attach(Median, b: 1 <= i <= j <= n) ((x_i + x_j) / 2) $

Dispersion Estimators:

*Standard Deviation*:
$ StdDev(vx) = sqrt(1/(n-1) sum_(i=1)^n (x_i - Mean(vx))^2) $

*Median Absolute Deviation* (around the median):
$ MAD(vx) = Median(abs(x_i - Median(vx))) $

*Spread* (Shamos scale estimator):
$ Spread(vx) = attach(Median, b: 1 <= i < j <= n) abs(x_i - x_j) $
