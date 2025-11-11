## Primer

Given two numeric samples $\x = (x_1, \ldots, x_n)$ and $\y = (y_1, \ldots, y_m)$, the toolkit provides the following primary procedures:

$\Center(\x) = \underset{1 \leq i \leq j \leq n}{\Median} \left((x_i + x_j)/2 \right)$ — robust average of $\x$

For $\x = (0, 2, 4, 6, 8)$:

$$
\begin{aligned}
\Center(\x) &= 4 \\
\Center(\x + 10) &= 14 \\
\Center(3\x) &= 12
\end{aligned}
$$

$\Spread(\x) = \underset{1 \leq i < j \leq n}{\Median} |x_i - x_j|$ — robust dispersion of $\x$

For $\x = (0, 2, 4, 6, 8)$:

$$
\begin{aligned}
\Spread(\x) &= 4 \\
\Spread(\x + 10) &= 4 \\
\Spread(2\x) &= 8
\end{aligned}
$$

$\RelSpread(\x) = \Spread(\x) / \left| \Center(\x) \right|$ — robust relative dispersion of $\x$

For $\x = (0, 2, 4, 6, 8)$:

$$
\begin{aligned}
\RelSpread(\x) &= 1 \\
\RelSpread(5\x) &= 1
\end{aligned}
$$

$\Shift(\x, \y) = \underset{1 \leq i \leq n, 1 \leq j \leq m}{\Median} \left(x_i - y_j \right)$ — robust signed difference ($\x-\y$)

For $\x = (0, 2, 4, 6, 8)$ and $\y = (10, 12, 14, 16, 18)$:

$$
\begin{aligned}
\Shift(\x, \y) &= -10 \\
\Shift(\x, \x) &= 0 \\
\Shift(\x + 7, \y + 3) &= -6 \\
\Shift(2\x, 2\y) &= -20 \\
\Shift(\y, \x) &= 10
\end{aligned}
$$

$\Ratio(\x, \y) = \underset{1 \leq i \leq n, 1 \leq j \leq m}{\Median} \left( x_i / y_j \right)$ — robust ratio ($\x/\y$)

For $\x = (1, 2, 4, 8, 16)$ and $\y = (2, 4, 8, 16, 32)$:

$$
\begin{aligned}
\Ratio(\x, \y) &= 0.5 \\
\Ratio(\x, \x) &= 1 \\
\Ratio(2\x, 5\y) &= 0.2
\end{aligned}
$$

$\AvgSpread(\x, \y) = (n\Spread(\x) + m\Spread(\y)) / (n + m)$ — robust average spread of $\x$ and $\y$

For $\x = (0, 3, 6, 9, 12)$ and $\y = (0, 2, 4, 6, 8)$:

$$
\begin{aligned}
\Spread(\x) &= 6 \\
\Spread(\y) &= 4 \\
\AvgSpread(\x, \y) &= 5 \\
\AvgSpread(\x, \x) &= 6 \\
\AvgSpread(2\x, 3\x) &= 15 \\
\AvgSpread(\y, \x) &= 5 \\
\AvgSpread(2\x, 2\y) &= 10
\end{aligned}
$$

$\Disparity(\x, \y) = \Shift(\x, \y) / \AvgSpread(\x, \y)$ — robust effect size between $\x$ and $\y$

For $\x = (0, 3, 6, 9, 12)$ and $\y = (0, 2, 4, 6, 8)$:

$$
\begin{aligned}
\Shift(\x, \y) &= 2 \\
\AvgSpread(\x, \y) &= 5 \\
\Disparity(\x, \y) &= 0.4 \\
\Disparity(\x + 5, \y + 5) &= 0.4 \\
\Disparity(2\x, 2\y) &= 0.4 \\
\Disparity(\y, \x) &= -0.4
\end{aligned}
$$

These procedures are designed to serve as default choices for routine analysis and comparison tasks in engineering contexts.
The toolkit has ready-to-use implementations for Python, TypeScript/JavaScript, R, C#, Kotlin, Rust, and Go.
