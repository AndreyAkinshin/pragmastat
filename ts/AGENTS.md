# TypeScript Implementation

## Build Commands

```bash
mise run ts:ci       # Full CI: clean → restore → check → build → test
mise run ts:test     # Run tests only
mise run ts:check    # Lint (ESLint) + format check (Prettier)
mise run ts:check:fix # Auto-format code
mise run ts:demo     # Run demo with ts-node
mise run ts:build    # Compile TypeScript
mise run ts:coverage # Run tests with coverage
mise run ts:pack     # Create npm tarball
```

## Architecture

```
ts/
├── src/
│   ├── index.ts            # Public exports
│   ├── estimators.ts       # Public API: median, center, spread, shift, etc.
│   ├── pairwiseMargin.ts   # Margin calculation for shift bounds
│   ├── rng.ts              # Deterministic xoshiro256++ PRNG
│   ├── xoshiro256.ts       # PRNG core implementation
│   ├── fastCenter.ts       # O(n log n) Hodges-Lehmann algorithm
│   ├── fastSpread.ts       # O(n log n) Shamos algorithm
│   ├── fastShift.ts        # O((m+n) log L) shift quantiles
│   ├── constants.ts        # Internal constants
│   ├── utils.ts            # Internal utilities
│   └── distributions/      # Uniform, Additive, Exp, Power, Multiplic
├── tests/
│   ├── reference.test.ts   # JSON fixture validation
│   ├── invariance.test.ts  # Mathematical property tests
│   └── performance.test.ts
├── examples/
│   └── demo.ts
├── package.json
└── tsconfig.json
```

## Key Types

| Type | Purpose |
|------|---------|
| `Rng` | Deterministic PRNG with `uniform()`, `sample()`, `shuffle()` |
| `Bounds` | Object with `lower` and `upper` properties |
| `Distribution` | Interface for sampling distributions |

## Public Functions

```typescript
function median(x: number[]): number
function center(x: number[]): number
function spread(x: number[]): number
function relSpread(x: number[]): number
function shift(x: number[], y: number[]): number
function ratio(x: number[], y: number[]): number
function avgSpread(x: number[], y: number[]): number
function disparity(x: number[], y: number[]): number
function shiftBounds(x: number[], y: number[], misrate: number): Bounds
function ratioBounds(x: number[], y: number[], misrate: number): Bounds
function pairwiseMargin(n: number, m: number, misrate: number): number
```

## Distributions

```typescript
class Uniform implements Distribution { constructor(min: number, max: number) }
class Additive implements Distribution { constructor(location: number, scale: number) }
class Exp implements Distribution { constructor(rate: number) }
class Power implements Distribution { constructor(scale: number, exponent: number) }
class Multiplic implements Distribution { constructor(location: number, scale: number) }
```

## Testing

- **Reference tests**: Load JSON fixtures from `../tests/` directory
- **Invariance tests**: Verify mathematical properties
- **Tolerance**: `1e-10` for floating-point comparisons

```bash
npm test                  # All tests
npm run test:coverage     # With coverage report
```

## Error Handling

Functions throw `Error` for invalid inputs:

```typescript
try {
    const result = center(x);
} catch (e) {
    // Handle: empty input, invalid parameters
}
```

Error conditions:
- Empty input arrays
- `misrate` outside `[0, 1]`
- Division by zero (e.g., `relSpread` when center is zero)
- Non-positive values in `y` for `ratio`

## Determinism

The `fastCenter` and `fastSpread` functions use deterministic pivot selection via FNV-1a hash. Uses the library's own `Rng` class instead of `Math.random()`.

## Linting

- ESLint for linting
- Prettier for formatting
- Strict TypeScript configuration
