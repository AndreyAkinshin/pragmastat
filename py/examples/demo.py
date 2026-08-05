from pragmastat import (
    Metric,
    Rng,
    Sample,
    Threshold,
    center,
    center_bounds,
    compare1,
    compare2,
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

    x = Sample(list(range(1, 201)))

    result = center(x)
    print(result)  # 100.5
    bounds = center_bounds(x, 1e-3)
    print(bounds)  # [86;115]
    print(spread(x))  # 59
    bounds = spread_bounds(x, 1e-3, seed="demo")
    print(bounds)  # [44;87]

    # --- Two-Sample ---

    x = Sample(list(range(1, 201)))
    y = Sample(list(range(101, 301)))

    print(shift(x, y))  # -100
    bounds = shift_bounds(x, y, 1e-3)
    print(bounds)  # [-120;-80]
    print(ratio(x, y))  # 0.5008354224706334
    bounds = ratio_bounds(x, y, 1e-3)
    print(bounds)  # [0.4066666666666668;0.5958333333333332]
    print(disparity(x, y))  # -1.694915254237288
    bounds = disparity_bounds(x, y, 1e-3, seed="demo")
    print(bounds)  # [-3.1025641025641026;-0.8494623655913979]

    # --- Confirmatory Comparison ---

    x = Sample(list(range(1, 201)))
    projection = compare1(x, [Threshold(Metric.CENTER, 50.0, 1e-3)])[0]
    print(projection.verdict)  # greater

    x = Sample(list(range(1, 201)))
    y = Sample(list(range(101, 301)))
    projection = compare2(x, y, [Threshold(Metric.SHIFT, 0.0, 1e-3)])[0]
    print(projection.verdict)  # less

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
