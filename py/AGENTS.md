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
│   ├── center_impl.py             # O(n log n) Hodges-Lehmann algorithm
│   ├── _center_quantiles_impl.py  # Center quantile binary search (internal)
│   ├── spread_impl.py             # O(n log n) Shamos algorithm
│   ├── shift_impl.py              # O((m+n) log L) shift quantiles
│   ├── _constants.py              # Internal constants
│   └── distributions/             # Uniform, Additive, Exp, Power, Multiplic
├── tests/
│   ├── test_assume_sorted.py      # assume-sorted equivalence + convergence-guard misuse
│   ├── test_invariance.py         # Mathematical property tests
│   ├── test_mutation.py           # Raw-API input-mutation safety
│   ├── test_performance.py        # Performance smoke test
│   └── test_reference.py          # JSON fixture validation (includes sample-construction, unit-propagation)
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

Every estimator exposes **two entry points through one function**: each accepts
either a typed `Sample` or a raw `ArrayLike` (`Sequence[float]` or numpy
`NDArray`), distinguished at runtime.

- **Typed Sample input**: point estimators return `Measurement`, bounds return
  `Bounds` (units propagated from the input). The cached sorted view is used, so
  `assume_sorted` is ignored for `Sample` input.
- **Raw native-array input**: point estimators return a plain `float`, bounds
  return `Bounds` (with `NUMBER_UNIT`). Pass `assume_sorted=True` to skip the
  internal sort when the input is already sorted ascending. `assume_sorted` is a
  keyword-only argument (default `False`).

```python
ArrayLike = Union[Sequence[float], NDArray]

# Point estimators: Sample -> Measurement, ArrayLike -> float
def center(x: Sample | ArrayLike, *, assume_sorted: bool = False) -> Measurement | float
def spread(x: Sample | ArrayLike, *, assume_sorted: bool = False) -> Measurement | float
def shift(x: Sample | ArrayLike, y: Sample | ArrayLike, *, assume_sorted: bool = False) -> Measurement | float
def ratio(x: Sample | ArrayLike, y: Sample | ArrayLike, *, assume_sorted: bool = False) -> Measurement | float
def disparity(x: Sample | ArrayLike, y: Sample | ArrayLike, *, assume_sorted: bool = False) -> Measurement | float

# Bounds estimators: always return Bounds
def center_bounds(x: Sample | ArrayLike, misrate: float = 1e-3, *, assume_sorted: bool = False) -> Bounds
def spread_bounds(x: Sample | ArrayLike, misrate: float = 1e-3, seed: str | None = None, *, assume_sorted: bool = False) -> Bounds
def shift_bounds(x: Sample | ArrayLike, y: Sample | ArrayLike, misrate: float = 1e-3, *, assume_sorted: bool = False) -> Bounds
def ratio_bounds(x: Sample | ArrayLike, y: Sample | ArrayLike, misrate: float = 1e-3, *, assume_sorted: bool = False) -> Bounds
def disparity_bounds(x: Sample | ArrayLike, y: Sample | ArrayLike, misrate: float = 1e-3, seed: str | None = None, *, assume_sorted: bool = False) -> Bounds
```

Each public function wraps a private `_*_raw` helper that operates on native
arrays. For the order-independent estimators the helper takes the
`assume_sorted` flag, and the `Sample` path calls it with the cached
`sorted_values` and `assume_sorted=True`. The shuffle-based bounds
(`_spread_bounds_raw`, `_avg_spread_bounds_raw`, `_disparity_bounds_raw`)
differ: the disjoint-pair shuffle must see the ORIGINAL order, so these helpers
take original-order values plus optional pre-sorted views (`sorted_view` /
`sorted_x` / `sorted_y`) used only for the order-independent sub-computations;
the `Sample` path passes `values` together with the cached `sorted_values`.

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

## Determinism

The pure-Python `_center_impl` and `_spread_impl` functions are deterministic: their randomized pivot-row selection uses the library's own `Rng` class (not Python's `random` module), seeded from the input values via an FNV-1a hash.

The optional C extensions are deterministic too, though they reach the result by different internal routes. The C `center` uses a middle-element pivot strategy (no PRNG), so its pivot sequence differs from the pure-Python center's FNV-seeded random pivots; both still converge to the same value, because the selection result is independent of the pivot path. The C `spread` seeds a xoshiro256++ generator from an FNV-1a hash of the input values, mirroring the pure-Python `Rng` bit-for-bit, so the C and pure-Python spread kernels do follow identical narrowing paths. Every kernel is deterministic for a given input and returns identical results across the C and pure-Python implementations.

## Linting

Uses `ruff` for linting and formatting:
- All rules enabled by default
- Format verification in CI
