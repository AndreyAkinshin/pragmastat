## Center

$$
\Center(\x) = \underset{1 \leq i \leq j \leq n}{\Median} \left(\frac{x_i + x_j}{2} \right)
$$

- Measures average (central tendency, measure of location)
- Equals the *Hodges-Lehmann estimator* ([@hodges1963], [@sen1963]), renamed to $\Center$ for clarity
- Also known as 'pseudomedian' because it is consistent with $\Median$ for symmetric distributions
- Pragmatic alternative to $\Mean$ and $\Median$
- Asymptotically, $\Center[X]$ is the $\Median$ of the arithmetic average of two random measurements from $X$
- Straightforward implementations have $O(n^2 \log n)$ complexity; a fast $O(n \log n)$ version is provided in the Algorithms section.
- Domain: any real numbers
- Unit: the same as measurements

$$
\Center(\x + k) = \Center(\x) + k
$$

$$
\Center(k \cdot \x) = k \cdot \Center(\x)
$$
