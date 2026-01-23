#import "/manual/definitions.typ": *

== ShiftBounds

$ ShiftBounds(vx, vy, misrate) = [z_((k_"left")), z_((k_"right"))] $

where

$ vz = { x_i - y_j }_(1 <= i <= n\, 1 <= j <= m) quad ("sorted") $

$ k_"left" = floor(PairwiseMargin(n, m, misrate) / 2) + 1 $

$ k_"right" = n m - floor(PairwiseMargin(n, m, misrate) / 2) $

- Provides bounds on $Shift(vx, vy)$ with specified $misrate$
- The $misrate$ represents the probability that the true shift falls outside the computed bounds
- Pragmatic alternative to traditional confidence intervals for the Hodges-Lehmann estimator
- Domain: any real numbers
- Unit: the same as measurements

$ ShiftBounds(vx + k, vy + k, misrate) = ShiftBounds(vx, vy, misrate) $

$ ShiftBounds(k dot vx, k dot vy, misrate) = k dot ShiftBounds(vx, vy, misrate) $
