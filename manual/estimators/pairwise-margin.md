## PairwiseMargin

$$
\PairwiseMargin(n, m, \misrate)
$$

- Determines how many extreme pairwise differences to exclude when constructing bounds
- Based on the distribution of $\Dominance(\x, \y) = \sum_{i=1}^n \sum_{j=1}^m \mathbb{1}(x_i > y_j)$ under random sampling
- Returns the total margin split evenly between lower and upper tails
- Used by $\ShiftBounds$ to select appropriate order statistics
- Can be computed exactly for small samples or approximated for large samples (see Algorithms section)
- Domain: $n, m \geq 1$, $\misrate \in (0; 1)$
- Unit: count (number of pairwise differences)

$$
\PairwiseMargin(n, m, \misrate) = \PairwiseMargin(m, n, \misrate)
$$

$$
\PairwiseMargin(n, m, \misrate) \geq 0
$$

$$
\PairwiseMargin(n, m, \misrate) \leq nm
$$
