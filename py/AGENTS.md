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
│   ├── estimators.py       # Public API: center, spread, shift, etc.
│   ├── pairwise_margin.py  # Margin calculation for shift bounds (internal)
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
| `Rng` | Deterministic PRNG with `uniform_float()`, `sample()`, `shuffle()` |
| `Bounds` | NamedTuple with `lower` and `upper` fields |
| `Distribution` | Protocol for sampling distributions |

## Public Functions

```python
def center(x: Sequence[float] | NDArray) -> float
def spread(x: Sequence[float] | NDArray) -> float
def rel_spread(x: Sequence[float] | NDArray) -> float  # Deprecated
def shift(x: Sequence[float] | NDArray, y: Sequence[float] | NDArray) -> float
def ratio(x: Sequence[float] | NDArray, y: Sequence[float] | NDArray) -> float
def disparity(x: Sequence[float] | NDArray, y: Sequence[float] | NDArray) -> float
def shift_bounds(x, y, misrate: float = 1e-3) -> Bounds
def ratio_bounds(x, y, misrate: float = 1e-3) -> Bounds
def disparity_bounds(x, y, misrate: float = 1e-3, seed: str | None = None) -> Bounds
def center_bounds(x: Sequence[float] | NDArray, misrate: float = 1e-3) -> Bounds
def spread_bounds(x: Sequence[float] | NDArray, misrate: float = 1e-3, seed: str | None = None) -> Bounds
```

## Testing

- **Reference tests**: Load JSON fixtures from `../tests/` directory
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
from pragmastat import center, AssumptionError

try:
    result = center(x)
except AssumptionError as e:
    # e.violation.id: "validity", "domain", "positivity", "sparity"
    # e.violation.subject: "x", "y", "misrate"
    pass
```

Error conditions:
- Empty or non-finite input arrays (`validity`)
- `misrate` outside valid range (`domain`)
- Non-positive values for `ratio` (`positivity`)
- Tie-dominant sample (`sparity`)
- `rel_spread` is deprecated; use `spread(x) / abs(center(x))` instead

## Determinism

The `_fast_center` and `_fast_spread` functions use deterministic pivot selection via FNV-1a hash. Uses the library's own `Rng` class instead of Python's `random` module.

## Linting

Uses `ruff` for linting and formatting:
- All rules enabled by default
- Format verification in CI
