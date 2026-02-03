# Python Implementation

## Build Commands

```bash
mise run py:ci       # Full CI: clean → restore → check → test → pack
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
│   ├── __init__.py         # Public exports
│   ├── estimators.py       # Public API: median, center, spread, shift, etc.
│   ├── pairwise_margin.py  # Margin calculation for shift bounds
│   ├── rng.py              # Deterministic xoshiro256++ PRNG
│   ├── xoshiro256.py       # PRNG core implementation
│   ├── fast_center.py      # O(n log n) Hodges-Lehmann algorithm
│   ├── fast_spread.py      # O(n log n) Shamos algorithm
│   ├── fast_shift.py       # O((m+n) log L) shift quantiles
│   ├── _constants.py       # Internal constants
│   └── distributions/      # Uniform, Additive, Exp, Power, Multiplic
├── tests/
│   ├── test_reference.py   # JSON fixture validation
│   ├── test_invariance.py  # Mathematical property tests
│   └── test_performance.py
├── examples/
│   └── demo.py
└── pyproject.toml
```

## Key Types

| Type | Purpose |
|------|---------|
| `Rng` | Deterministic PRNG with `uniform()`, `sample()`, `shuffle()` |
| `Bounds` | NamedTuple with `lower` and `upper` fields |
| `Distribution` | Protocol for sampling distributions |

## Public Functions

```python
def median(x: Sequence[float] | NDArray) -> float
def center(x: Sequence[float] | NDArray) -> float
def spread(x: Sequence[float] | NDArray) -> float
def rel_spread(x: Sequence[float] | NDArray) -> float
def shift(x: Sequence[float] | NDArray, y: Sequence[float] | NDArray) -> float
def ratio(x: Sequence[float] | NDArray, y: Sequence[float] | NDArray) -> float
def avg_spread(x: Sequence[float] | NDArray, y: Sequence[float] | NDArray) -> float
def disparity(x: Sequence[float] | NDArray, y: Sequence[float] | NDArray) -> float
def shift_bounds(x, y, misrate: float) -> Bounds
def ratio_bounds(x, y, misrate: float) -> Bounds
def pairwise_margin(n: int, m: int, misrate: float) -> int
```

## Testing

- **Reference tests**: Load JSON fixtures from `../tests/` directory
- **Invariance tests**: Verify mathematical properties
- **Tolerance**: `1e-10` for floating-point comparisons

```bash
pytest tests/                    # All tests
pytest tests/test_reference.py   # Reference tests only
pytest tests/ -v                 # Verbose output
```

## Error Handling

Functions raise `ValueError` for invalid inputs:

```python
try:
    result = center(x)
except ValueError as e:
    # Handle: empty input, invalid parameters
```

Error conditions:
- Empty input arrays
- `misrate` outside `[0, 1]`
- Division by zero (e.g., `rel_spread` when center is zero)
- Non-positive values in `y` for `ratio`

## Determinism

The `_fast_center` and `_fast_spread` functions use deterministic pivot selection via FNV-1a hash. Uses the library's own `Rng` class instead of Python's `random` module.

## Linting

Uses `ruff` for linting and formatting:
- All rules enabled by default
- Format verification in CI
