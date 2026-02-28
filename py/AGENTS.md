# Python Implementation

## Build Commands

```bash
mise run py:ci       # Full CI: clean -> restore -> check -> test -> pack
mise run py:test     # Run tests only
mise run py:check    # Lint (ruff) + format check
mise run py:check:fix # Auto-fix lint issues and format
mise run py:demo     # Run demo
mise run py:build    # Build distribution package
mise run py:pack     # Build and verify with twine
```

## Architecture

```
py/
├── pragmastat/
│   ├── __init__.py                # Public exports
│   ├── estimators.py              # Public API: center, spread, shift, etc.
│   ├── sample.py                  # Sample class with values, weights, unit
│   ├── measurement.py             # Measurement frozen dataclass (value + unit)
│   ├── measurement_unit.py        # MeasurementUnit frozen dataclass
│   ├── bounds.py                  # Bounds frozen dataclass (lower, upper, unit)
│   ├── unit_registry.py           # UnitRegistry for unit lookup by ID
│   ├── assumptions.py             # Input validation and error types
│   ├── pairwise_margin.py         # Margin calculation for shift bounds (internal)
│   ├── sign_margin.py             # Sign margin for binomial CDF inversion
│   ├── signed_rank_margin.py      # Signed-rank margin computation
│   ├── min_misrate.py             # Minimum achievable misrate calculation
│   ├── gauss_cdf.py               # Standard normal CDF (ACM Algorithm 209)
│   ├── rng.py                     # Deterministic xoshiro256++ PRNG
│   ├── xoshiro256.py              # PRNG core implementation
│   ├── fast_center.py             # O(n log n) Hodges-Lehmann algorithm
│   ├── _fast_center_quantiles.py  # Center quantile binary search (internal)
│   ├── fast_spread.py             # O(n log n) Shamos algorithm
│   ├── fast_shift.py              # O((m+n) log L) shift quantiles
│   ├── _constants.py              # Internal constants
│   └── distributions/             # Uniform, Additive, Exp, Power, Multiplic
├── tests/
│   ├── test_reference.py          # JSON fixture validation (includes sample-construction, unit-propagation)
│   ├── test_invariance.py         # Mathematical property tests
│   └── test_performance.py
├── examples/
│   └── demo.py
└── pyproject.toml
```

## Key Types

| Type | Purpose |
|------|---------|
| `Sample` | Wraps values with optional weights and a measurement unit |
| `MeasurementUnit` | Frozen dataclass with id, family, abbreviation, full_name, base_units |
| `Measurement` | Frozen dataclass pairing a value with its unit |
| `Bounds` | Frozen dataclass with lower, upper, and unit |
| `UnitRegistry` | Registry for unit lookup by ID |
| `Rng` | Deterministic PRNG with `uniform_float()`, `sample()`, `shuffle()` |
| `Distribution` | Protocol for sampling distributions |

## Standard Units

| Constant | Family | Usage |
|----------|--------|-------|
| `NUMBER_UNIT` | Number | Default unit for plain numeric samples |
| `RATIO_UNIT` | Ratio | Output unit for `ratio()` and `ratio_bounds()` |
| `DISPARITY_UNIT` | Disparity | Output unit for `disparity()` and `disparity_bounds()` |

## Public Functions

All estimator functions accept `Sample` objects and return `Measurement` or `Bounds`:

```python
def center(x: Sample) -> Measurement
def spread(x: Sample) -> Measurement
def rel_spread(x: Sample) -> Measurement  # Deprecated
def shift(x: Sample, y: Sample) -> Measurement
def ratio(x: Sample, y: Sample) -> Measurement
def disparity(x: Sample, y: Sample) -> Measurement
def center_bounds(x: Sample, misrate: float = 1e-3) -> Bounds
def spread_bounds(x: Sample, misrate: float = 1e-3, seed: str | None = None) -> Bounds
def shift_bounds(x: Sample, y: Sample, misrate: float = 1e-3) -> Bounds
def ratio_bounds(x: Sample, y: Sample, misrate: float = 1e-3) -> Bounds
def disparity_bounds(x: Sample, y: Sample, misrate: float = 1e-3, seed: str | None = None) -> Bounds
```

## Unit Propagation Rules

| Estimator | Output Unit |
|-----------|-------------|
| center, center_bounds | x.unit |
| spread, spread_bounds | x.unit |
| shift, shift_bounds | finer(x.unit, y.unit) |
| ratio, ratio_bounds | RATIO_UNIT |
| disparity, disparity_bounds | DISPARITY_UNIT |

Two-sample estimators check unit compatibility and convert to the finer unit.

## Sample Construction

```python
from pragmastat import Sample, NUMBER_UNIT

x = Sample([1, 2, 3, 4, 5])                          # unweighted, NUMBER_UNIT
x = Sample([1, 2, 3], unit=NUMBER_UNIT)               # explicit unit
x = Sample([1, 2, 3], weights=[0.5, 0.3, 0.2])       # weighted
```

Validation at construction time:
- Non-empty values
- All values finite (no NaN/Inf)
- Weights length matches values, non-negative, positive sum

Weighted samples are rejected by all estimators.

## Testing

- **Reference tests**: Load JSON fixtures from `../tests/` directory
- **Sample construction tests**: From `../tests/sample-construction/`
- **Unit propagation tests**: From `../tests/unit-propagation/`
- **Invariance tests**: Verify mathematical properties
- **Tolerance**: `1e-9` for floating-point comparisons

```bash
mise run py:test                 # All tests (preferred)
pytest tests/                    # All tests (raw)
pytest tests/test_reference.py   # Reference tests only
```

## Error Handling

Functions raise `AssumptionError` (with `violation` attribute containing `id` and `subject`) for invalid inputs:

```python
from pragmastat import Sample, center, AssumptionError

try:
    result = center(Sample([1, 2, 3]))
except AssumptionError as e:
    # e.violation.id: "validity", "domain", "positivity", "sparity"
    # e.violation.subject: "x", "y", "misrate"
    pass
```

Weighted samples raise `ValueError` (not `AssumptionError`).

Error conditions:
- Empty or non-finite input arrays (`validity`)
- `misrate` outside valid range (`domain`)
- Non-positive values for `ratio` (`positivity`)
- Tie-dominant sample (`sparity`)
- `rel_spread` is deprecated; use `spread(x).value / abs(center(x).value)` instead

## Determinism

The `_fast_center` and `_fast_spread` functions use deterministic pivot selection via FNV-1a hash. Uses the library's own `Rng` class instead of Python's `random` module.

## Linting

Uses `ruff` for linting and formatting:
- All rules enabled by default
- Format verification in CI
