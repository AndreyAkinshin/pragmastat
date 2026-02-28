# Python

Install from PyPI:

```bash
pip install pragmastat==10.0.6
```

Source code: https://github.com/AndreyAkinshin/pragmastat/tree/v10.0.6/py

Pragmastat on PyPI: https://pypi.org/project/pragmastat/

## Demo

```python
from pragmastat import (
    Rng,
    Sample,
    center,
    center_bounds,
    disparity,
    disparity_bounds,
    ratio,
    ratio_bounds,
    shift,
    shift_bounds,
    spread,
    spread_bounds,
)
from pragmastat.distributions import Additive, Exp, Multiplic, Power, Uniform


def main():
    # --- One-Sample ---

    x = Sample(list(range(1, 23)))

    result = center(x)
    print(result.value)  # 11.5
    bounds = center_bounds(x, 1e-3)
    print(f"Bounds(lower={bounds.lower}, upper={bounds.upper})")  # Bounds(lower=6.0, upper=17.0)
    print(spread(x).value)  # 7.0
    bounds = spread_bounds(x, 1e-3, seed="demo")
    print(f"Bounds(lower={bounds.lower}, upper={bounds.upper})")  # Bounds(lower=1.0, upper=18.0)

    # --- Two-Sample ---

    x = Sample(list(range(1, 31)))
    y = Sample(list(range(21, 51)))

    print(shift(x, y).value)  # -20.0
    bounds = shift_bounds(x, y, 1e-3)
    print(f"Bounds(lower={bounds.lower}, upper={bounds.upper})")  # Bounds(lower=-28.0, upper=-12.0)
    print(ratio(x, y).value)  # 0.43669798282695127
    bounds = ratio_bounds(x, y, 1e-3)
    print(
        f"Bounds(lower={bounds.lower}, upper={bounds.upper})"
    )  # Bounds(lower=0.23255813953488377, upper=0.6428571428571428)
    print(disparity(x, y).value)  # -2.2222222222222223
    bounds = disparity_bounds(x, y, 1e-3, seed="demo")
    print(f"Bounds(lower={bounds.lower}, upper={bounds.upper})")  # Bounds(lower=-29.0, upper=-0.4782608695652174)

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
