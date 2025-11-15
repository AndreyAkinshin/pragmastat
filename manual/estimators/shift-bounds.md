## ShiftBounds

$$
\ShiftBounds(\x, \y, \misrate) = [z_{(k_{\mathrm{left}})}; z_{(k_{\mathrm{right}})}]
$$

where

$$
\z = \left\{ x_i - y_j \right\}_{1 \leq i \leq n,\, 1 \leq j \leq m} \text{ (sorted)}
$$

$$
k_{\mathrm{left}} = \lfloor \PairwiseMargin(n, m, \misrate) / 2 \rfloor + 1
$$

$$
k_{\mathrm{right}} = nm - \lfloor \PairwiseMargin(n, m, \misrate) / 2 \rfloor
$$

- Provides bounds on $\Shift(\x, \y)$ with specified $\misrate$
- The $\misrate$ represents the probability that the true shift falls outside the computed bounds
- Pragmatic alternative to traditional confidence intervals for the Hodges-Lehmann estimator
- Domain: any real numbers
- Unit: the same as measurements

$$
\ShiftBounds(\x + k, \y + k, \misrate) = \ShiftBounds(\x, \y, \misrate)
$$

$$
\ShiftBounds(k \cdot \x, k \cdot \y, \misrate) = k \cdot \ShiftBounds(\x, \y, \misrate)
$$