# Pragmastat Python Implementation

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
from pragmastat import center, spread, volatility, precision, med_shift, med_ratio, med_spread, med_disparity

# Basic estimators
x = [1, 2, 3, 4, 5]
print(f"Center: {center(x)}")
print(f"Spread: {spread(x)}")
print(f"Volatility: {volatility(x)}")
print(f"Precision: {precision(x)}")

# Comparison estimators
y = [3, 4, 5, 6, 7]
print(f"Shift: {med_shift(x, y)}")
print(f"Ratio: {med_ratio(x, y)}")
print(f"Spread: {med_spread(x, y)}")
print(f"Disparity: {med_disparity(x, y)}")
```

## Estimators

### Single-sample estimators

- `center(x)`: Hodges-Lehmann estimator - median of all pairwise averages
- `spread(x)`: Shamos estimator - median of all pairwise absolute differences
- `volatility(x)`: Relative spread - spread divided by absolute center
- `precision(x)`: Standard error estimate - 2 * spread / sqrt(n)

### Two-sample estimators

- `med_shift(x, y)`: Hodges-Lehmann shift estimator - median of all pairwise differences
- `med_ratio(x, y)`: Median of all pairwise ratios
- `med_spread(x, y)`: Weighted average of spreads
- `med_disparity(x, y)`: Normalized shift - shift divided by average spread

## Features

- Robust to outliers
- Supports both Python lists and NumPy arrays
- Type hints with numpy.typing
- Efficient vectorized NumPy operations

## License

MIT