from pragmastat import (
    center,
    spread,
    rel_spread,
    shift,
    ratio,
    avg_spread,
    disparity,
    pairwise_margin,
    shift_bounds,
)


def main():
    x = [0, 2, 4, 6, 8]
    print(center(x))  # 4
    print(center([v + 10 for v in x]))  # 14
    print(center([v * 3 for v in x]))  # 12

    print(spread(x))  # 4
    print(spread([v + 10 for v in x]))  # 4
    print(spread([v * 2 for v in x]))  # 8

    print(rel_spread(x))  # 1
    print(rel_spread([v * 5 for v in x]))  # 1

    y = [10, 12, 14, 16, 18]
    print(shift(x, y))  # -10
    print(shift(x, x))  # 0
    print(shift([v + 7 for v in x], [v + 3 for v in y]))  # -6
    print(shift([v * 2 for v in x], [v * 2 for v in y]))  # -20
    print(shift(y, x))  # 10

    x = [1, 2, 4, 8, 16]
    y = [2, 4, 8, 16, 32]
    print(ratio(x, y))  # 0.5
    print(ratio(x, x))  # 1
    print(ratio([v * 2 for v in x], [v * 5 for v in y]))  # 0.2

    x = [0, 3, 6, 9, 12]
    y = [0, 2, 4, 6, 8]
    print(spread(x))  # 6
    print(spread(y))  # 4

    print(avg_spread(x, y))  # 5
    print(avg_spread(x, x))  # 6
    print(avg_spread([v * 2 for v in x], [v * 3 for v in x]))  # 15
    print(avg_spread(y, x))  # 5
    print(avg_spread([v * 2 for v in x], [v * 2 for v in y]))  # 10

    print(shift(x, y))  # 2
    print(avg_spread(x, y))  # 5

    print(disparity(x, y))  # 0.4
    print(disparity([v + 5 for v in x], [v + 5 for v in y]))  # 0.4
    print(disparity([v * 2 for v in x], [v * 2 for v in y]))  # 0.4
    print(disparity(y, x))  # -0.4

    x = list(range(1, 31))
    y = list(range(21, 51))

    print(pairwise_margin(30, 30, 1e-6))  # 276
    print(pairwise_margin(30, 30, 1e-5))  # 328
    print(pairwise_margin(30, 30, 1e-4))  # 390
    print(pairwise_margin(30, 30, 1e-3))  # 464

    print(shift(x, y))  # -20

    bounds = shift_bounds(x, y, 1e-6)  # [-33, -7]
    print(f"Bounds(lower={bounds.lower}, upper={bounds.upper})")
    bounds = shift_bounds(x, y, 1e-5)  # [-32, -8]
    print(f"Bounds(lower={bounds.lower}, upper={bounds.upper})")
    bounds = shift_bounds(x, y, 1e-4)  # [-30, -10]
    print(f"Bounds(lower={bounds.lower}, upper={bounds.upper})")
    bounds = shift_bounds(x, y, 1e-3)  # [-28, -12]
    print(f"Bounds(lower={bounds.lower}, upper={bounds.upper})")


if __name__ == "__main__":
    main()
