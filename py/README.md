# Python

Install from PyPI:

```bash
pip install pragmastat==5.2.1
```

Source code: https://github.com/AndreyAkinshin/pragmastat/tree/v5.2.1/py

Pragmastat on PyPI: https://pypi.org/project/pragmastat/

## Demo

```python
from pragmastat import (
    Rng,
    median,
    center,
    spread,
    rel_spread,
    shift,
    ratio,
    avg_spread,
    disparity,
    pairwise_margin,
    shift_bounds,
    ratio_bounds,
)
from pragmastat.distributions import Additive, Exp, Multiplic, Power, Uniform


def main():
    # --- Randomization ---

    rng = Rng(1729)
    print(rng.uniform())  # 0.3943034703296536
    print(rng.uniform())  # 0.5730893757071377

    rng = Rng("experiment-1")
    print(rng.uniform())  # 0.9535207726895857

    rng = Rng(1729)
    print(rng.sample([0, 1, 2, 3, 4, 5, 6, 7, 8, 9], 3))  # [6, 8, 9]

    rng = Rng(1729)
    print(rng.shuffle([1, 2, 3, 4, 5]))  # [4, 2, 3, 5, 1]

    # --- Distribution Sampling ---

    rng = Rng(1729)
    dist = Uniform(0, 10)
    print(dist.sample(rng))  # 3.9430347032965365

    rng = Rng(1729)
    dist = Additive(0, 1)
    print(dist.sample(rng))  # -1.222932972163442

    rng = Rng(1729)
    dist = Exp(1)
    print(dist.sample(rng))  # 0.5013761944646019

    rng = Rng(1729)
    dist = Power(1, 2)
    print(dist.sample(rng))  # 1.284909255071668

    rng = Rng(1729)
    dist = Multiplic(0, 1)
    print(dist.sample(rng))  # 0.2943655336550937

    # --- Single-Sample Statistics ---

    x = [1, 3, 5, 7, 9]

    print(median(x))  # 5
    print(center(x))  # 5
    print(spread(x))  # 4
    print(spread([v + 10 for v in x]))  # 4
    print(spread([v * 2 for v in x]))  # 8
    print(rel_spread(x))  # 0.8

    # --- Two-Sample Comparison ---

    x = [0, 3, 6, 9, 12]
    y = [0, 2, 4, 6, 8]

    print(shift(x, y))  # 2
    print(shift(y, x))  # -2
    print(avg_spread(x, y))  # 5
    print(disparity(x, y))  # 0.4
    print(disparity(y, x))  # -0.4

    x = [1, 2, 4, 8, 16]
    y = [2, 4, 8, 16, 32]
    print(ratio(x, y))  # 0.5
    print(ratio(y, x))  # 2

    # --- Confidence Bounds ---

    x = list(range(1, 31))
    y = list(range(21, 51))

    print(pairwise_margin(30, 30, 1e-4))  # 390
    print(shift(x, y))  # -20
    bounds = shift_bounds(x, y, 1e-4)  # Bounds(lower=-30, upper=-10)
    print(f"Bounds(lower={bounds.lower}, upper={bounds.upper})")

    x = [1, 2, 3, 4, 5]
    y = [2, 3, 4, 5, 6]
    bounds = ratio_bounds(x, y, 0.05)  # Bounds(lower=0.333..., upper=1.5)
    print(f"Bounds(lower={bounds.lower}, upper={bounds.upper})")


if __name__ == "__main__":
    main()
```
