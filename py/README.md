# Pragmastat

This is a Python implementation of 'Pragmastat: Pragmatic Statistical Toolkit', which presents a toolkit of statistical procedures that provide reliable results across diverse real-world distributions, with ready-to-use implementations and detailed explanations.

- PDF manual for this version: [pragmastat-v4.0.0.pdf](https://github.com/AndreyAkinshin/pragmastat/releases/download/v4.0.0/pragmastat-v4.0.0.pdf)
- Markdown manual for this version: [pragmastat-v4.0.0.md](https://github.com/AndreyAkinshin/pragmastat/releases/download/v4.0.0/pragmastat-v4.0.0.md)
- Source code for this version: [pragmastat/py/v4.0.0](https://github.com/AndreyAkinshin/pragmastat/tree/v4.0.0/py)
- Latest online manual: https://pragmastat.dev
- Manual DOI: [10.5281/zenodo.17236778](https://doi.org/10.5281/zenodo.17236778)

## Installation

Install from PyPI:

```bash
pip install pragmastat==4.0.0
```

## Demo

```python
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
```

## The MIT License

Copyright (c) 2025 Andrey Akinshin

Permission is hereby granted, free of charge, to any person obtaining
a copy of this software and associated documentation files (the
"Software"), to deal in the Software without restriction, including
without limitation the rights to use, copy, modify, merge, publish,
distribute, sublicense, and/or sell copies of the Software, and to
permit persons to whom the Software is furnished to do so, subject to
the following conditions:

The above copyright notice and this permission notice shall be
included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE
LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION
WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
