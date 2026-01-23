#import "/manual/definitions.typ": *

== PairwiseMargin

$ PairwiseMargin(n, m, misrate) $

- Determines how many extreme pairwise differences to exclude when constructing bounds
- Based on the distribution of $"Dominance"(vx, vy) = sum_(i=1)^n sum_(j=1)^m bb(1)(x_i > y_j)$ under random sampling
- Returns the total margin split evenly between lower and upper tails
- Used by $ShiftBounds$ to select appropriate order statistics
- Can be computed exactly for small samples or approximated for large samples (see Algorithms section)
- Domain: $n, m >= 1$, $misrate in (0, 1)$
- Unit: count (number of pairwise differences)

$ PairwiseMargin(n, m, misrate) = PairwiseMargin(m, n, misrate) $

$ PairwiseMargin(n, m, misrate) >= 0 $

$ PairwiseMargin(n, m, misrate) <= n m $
