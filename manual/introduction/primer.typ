#import "/manual/definitions.typ": *

== Primer

Given two numeric samples $vx = (x_1, ..., x_n)$ and $vy = (y_1, ..., y_m)$, the toolkit provides the following primary procedures:

$Center(vx) = attach(Median, b: 1 <= i <= j <= n) ((x_i + x_j)\/2)$ — robust average of $vx$

For $vx = (0, 2, 4, 6, 8)$:

$
Center(vx) &= 4 \
Center(vx + 10) &= 14 \
Center(3 vx) &= 12
$

$Spread(vx) = attach(Median, b: 1 <= i < j <= n) abs(x_i - x_j)$ — robust dispersion of $vx$

For $vx = (0, 2, 4, 6, 8)$:

$
Spread(vx) &= 4 \
Spread(vx + 10) &= 4 \
Spread(2 vx) &= 8
$

$RelSpread(vx) = Spread(vx) \/ abs(Center(vx))$ — robust relative dispersion of $vx$

For $vx = (0, 2, 4, 6, 8)$:

$
RelSpread(vx) &= 1 \
RelSpread(5 vx) &= 1
$

$Shift(vx, vy) = attach(Median, b: 1 <= i <= n\, 1 <= j <= m) (x_i - y_j)$ — robust signed difference ($vx - vy$)

For $vx = (0, 2, 4, 6, 8)$ and $vy = (10, 12, 14, 16, 18)$:

$
Shift(vx, vy) &= -10 \
Shift(vx, vx) &= 0 \
Shift(vx + 7, vy + 3) &= -6 \
Shift(2 vx, 2 vy) &= -20 \
Shift(vy, vx) &= 10
$

$Ratio(vx, vy) = attach(Median, b: 1 <= i <= n\, 1 <= j <= m) (x_i \/ y_j)$ — robust ratio ($vx \/ vy$)

For $vx = (1, 2, 4, 8, 16)$ and $vy = (2, 4, 8, 16, 32)$:

$
Ratio(vx, vy) &= 0.5 \
Ratio(vx, vx) &= 1 \
Ratio(2 vx, 5 vy) &= 0.2
$

$AvgSpread(vx, vy) = (n dot Spread(vx) + m dot Spread(vy)) \/ (n + m)$ — robust average spread of $vx$ and $vy$

For $vx = (0, 3, 6, 9, 12)$ and $vy = (0, 2, 4, 6, 8)$:

$
Spread(vx) &= 6 \
Spread(vy) &= 4 \
AvgSpread(vx, vy) &= 5 \
AvgSpread(vx, vx) &= 6 \
AvgSpread(2 vx, 3 vx) &= 15 \
AvgSpread(vy, vx) &= 5 \
AvgSpread(2 vx, 2 vy) &= 10
$

$Disparity(vx, vy) = Shift(vx, vy) \/ AvgSpread(vx, vy)$ — robust effect size between $vx$ and $vy$

For $vx = (0, 3, 6, 9, 12)$ and $vy = (0, 2, 4, 6, 8)$:

$
Shift(vx, vy) &= 2 \
AvgSpread(vx, vy) &= 5 \
Disparity(vx, vy) &= 0.4 \
Disparity(vx + 5, vy + 5) &= 0.4 \
Disparity(2 vx, 2 vy) &= 0.4 \
Disparity(vy, vx) &= -0.4
$

$PairwiseMargin(n, m, misrate)$ — determines how many extreme pairwise differences to exclude when constructing bounds based on the distribution of dominance statistics

For $n = 30, m = 30$:

$
PairwiseMargin(30, 30, 10^(-6)) &= 276 \
PairwiseMargin(30, 30, 10^(-5)) &= 328 \
PairwiseMargin(30, 30, 10^(-4)) &= 390 \
PairwiseMargin(30, 30, 10^(-3)) &= 464
$

$ShiftBounds(vx, vy, misrate)$ — bounds on $Shift(vx, vy)$ with specified misrate;
  these bounds fail to cover the true value of shift in $misrate$ probability in the long run

For $vx = (1, 2, ..., 30)$ and $vy = (21, 22, ..., 50)$:

$
Shift(vx, vy) &= -20 \
ShiftBounds(vx, vy, 10^(-6)) &= [-33, -7] \
ShiftBounds(vx, vy, 10^(-5)) &= [-32, -8] \
ShiftBounds(vx, vy, 10^(-4)) &= [-30, -10] \
ShiftBounds(vx, vy, 10^(-3)) &= [-28, -12]
$

These procedures are designed to serve as default choices for routine analysis and comparison tasks in engineering contexts.
The toolkit has ready-to-use implementations for Python, TypeScript/JavaScript, R, C\#, Kotlin, Rust, and Go.
