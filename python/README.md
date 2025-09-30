# Pragmastat Python Implementation

[![DOI](https://zenodo.org/badge/doi/10.5281/zenodo.17236778.svg)](https://doi.org/10.5281/zenodo.17236778)

A Python implementation of the Pragmastat statistical toolkit, providing robust statistical estimators for reliable analysis of real-world data.

## Installation

```bash
pip install pragmastat
```

## Requirements

- Python >= 3.8
- NumPy >= 1.20

## Usage

```python
from pragmastat import center, spread, rel_spread, shift, ratio, avg_spread, disparity

# Basic estimators
x = [1, 2, 3, 4, 5]
print(f"Center: {center(x)}")
print(f"Spread: {spread(x)}")
print(f"RelSpread: {rel_spread(x)}")

# Comparison estimators
y = [3, 4, 5, 6, 7]
print(f"Shift: {shift(x, y)}")
print(f"Ratio: {ratio(x, y)}")
print(f"AvgSpread: {avg_spread(x, y)}")
print(f"Disparity: {disparity(x, y)}")
```

## Estimators

### Single-sample estimators

- `center(x)`: Hodges-Lehmann estimator - median of all pairwise averages
- `spread(x)`: Shamos estimator - median of all pairwise absolute differences
- `rel_spread(x)`: Relative spread - spread divided by absolute center

### Two-sample estimators

- `shift(x, y)`: Hodges-Lehmann shift estimator - median of all pairwise differences
- `ratio(x, y)`: Median of all pairwise ratios
- `avg_spread(x, y)`: Weighted average of spreads
- `disparity(x, y)`: Normalized shift - shift divided by average spread

## Features

- Robust to outliers
- Supports both Python lists and NumPy arrays
- Type hints with numpy.typing
- Efficient vectorized NumPy operations

## License

MIT