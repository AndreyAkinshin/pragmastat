# Python

Install from PyPI:

```bash
pip install pragmastat==10.0.1
```

Source code: https://github.com/AndreyAkinshin/pragmastat/tree/v10.0.1/py

Pragmastat on PyPI: https://pypi.org/project/pragmastat/

## Demo

```python
from pragmastat import (
    Rng,
    center,
    spread,
    shift,
    ratio,
    disparity,
    center_bounds,
    shift_bounds,
    ratio_bounds,
    spread_bounds,
    disparity_bounds,
)
from pragmastat.distributions import Additive, Exp, Multiplic, Power, Uniform


def main():
    # --- One-Sample ---

    x = list(range(1, 21))

    print(center(x))  # 10.5
    bounds = center_bounds(x, 0.05)
    print(
        f"Bounds(lower={bounds.lower}, upper={bounds.upper})"
    )  # Bounds(lower=7.5, upper=13.5)
    print(spread(x))  # 6.0
    bounds = spread_bounds(x, 0.05, seed="demo")
    print(
        f"Bounds(lower={bounds.lower}, upper={bounds.upper})"
    )  # Bounds(lower=2.0, upper=10.0)

    # --- Two-Sample ---

    x = list(range(1, 31))
    y = list(range(21, 51))

    print(shift(x, y))  # -20.0
    bounds = shift_bounds(x, y, 0.05)
    print(
        f"Bounds(lower={bounds.lower}, upper={bounds.upper})"
    )  # Bounds(lower=-25.0, upper=-15.0)
    print(ratio(x, y))  # 0.43669798282695127
    bounds = ratio_bounds(x, y, 0.05)
    print(
        f"Bounds(lower={bounds.lower}, upper={bounds.upper})"
    )  # Bounds(lower=0.31250000000000006, upper=0.5599999999999999)
    print(disparity(x, y))  # -2.2222222222222223
    bounds = disparity_bounds(x, y, 0.05, seed="demo")
    print(
        f"Bounds(lower={bounds.lower}, upper={bounds.upper})"
    )  # Bounds(lower=-13.0, upper=-0.8235294117647058)

    # --- Randomization ---

    rng = Rng("demo-uniform")
    print(rng.uniform_float())  # 0.2640554428629759
    print(rng.uniform_float())  # 0.9348534835582796

    rng = Rng("demo-uniform-int")
    print(rng.uniform_int(0, 100))  # 41

    rng = Rng("demo-sample")
    print(rng.sample([0, 1, 2, 3, 4, 5, 6, 7, 8, 9], 3))  # [3, 8, 9]

    rng = Rng("demo-resample")
    print(rng.resample([1, 2, 3, 4, 5], 7))  # [3, 1, 3, 2, 4, 1, 2]

    rng = Rng("demo-shuffle")
    print(rng.shuffle([1, 2, 3, 4, 5]))  # [4, 2, 3, 5, 1]

    # --- Distributions ---

    rng = Rng("demo-dist-additive")
    print(Additive(0, 1).sample(rng))  # 0.17410448679568188

    rng = Rng("demo-dist-multiplic")
    print(Multiplic(0, 1).sample(rng))  # 1.1273244602673853

    rng = Rng("demo-dist-exp")
    print(Exp(1).sample(rng))  # 0.6589065267276553

    rng = Rng("demo-dist-power")
    print(Power(1, 2).sample(rng))  # 1.023677535537084

    rng = Rng("demo-dist-uniform")
    print(Uniform(0, 10).sample(rng))  # 6.54043657816832


if __name__ == "__main__":
    main()
```
