#import "/manual/definitions.typ": *

== Invariance

Invariance properties determine how estimators respond to data transformations.
These properties are crucial for analysis design and interpretation:

- *Location-invariant* estimators are invariant to additive shifts: $T(vx+k)=T(vx)$
- *Scale-invariant* estimators are invariant to positive rescaling: $T(k dot vx)=T(vx)$ for $k>0$
- *Equivariant* estimators change predictably with transformations, maintaining relative relationships

Choosing estimators with appropriate invariance properties ensures that results remain
  meaningful across different measurement scales, units, and data transformations.
For example, when comparing datasets collected with different instruments or protocols,
  location-invariant estimators eliminate the need for data centering,
  while scale-invariant estimators eliminate the need for normalization.

*Location-invariance*: An estimator $T$ is location-invariant if adding a constant to the measurements leaves the result unchanged:

$ T(vx + k) = T(vx) $

$ T(vx + k, vy + k) = T(vx, vy) $

*Location-equivariance*: An estimator $T$ is location-equivariant if it shifts with the data:

$ T(vx + k) = T(vx) + k $

$ T(vx + k_1, vy + k_2) = T(vx, vy) + f(k_1, k_2) $

*Scale-invariance*: An estimator $T$ is scale-invariant if multiplying by a positive constant leaves the result unchanged:

$ T(k dot vx) = T(vx) quad "for" k > 0 $

$ T(k dot vx, k dot vy) = T(vx, vy) quad "for" k > 0 $

*Scale-equivariance*: An estimator $T$ is scale-equivariant if it scales proportionally with the data:

$ T(k dot vx) = k dot T(vx) "or" abs(k) dot T(vx) quad "for" k != 0 $

$ T(k dot vx, k dot vy) = k dot T(vx, vy) "or" abs(k) dot T(vx, vy) quad "for" k != 0 $

#table(
  columns: 3,
  [], [*Location*], [*Scale*],
  [Center], [Equivariant], [Equivariant],
  [Spread], [Invariant], [Equivariant],
  [RelSpread], [–], [Invariant],
  [Shift], [Invariant], [Equivariant],
  [Ratio], [–], [Invariant],
  [AvgSpread], [Invariant], [Equivariant],
  [Disparity], [Invariant], [Invariant],
)
