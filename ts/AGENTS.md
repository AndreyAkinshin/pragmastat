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
│   ├── index.ts               # Public exports
│   ├── estimators.ts          # Public API: center, spread, shift, etc.
│   ├── assumptions.ts         # Input validation and error types
│   ├── pairwiseMargin.ts      # Margin calculation for shift bounds (internal)
│   ├── signMargin.ts          # Sign margin for binomial CDF inversion
│   ├── signedRankMargin.ts    # Signed-rank margin computation
│   ├── minMisrate.ts          # Minimum achievable misrate calculation
│   ├── gaussCdf.ts            # Standard normal CDF (ACM Algorithm 209)
│   ├── rng.ts                 # Deterministic xoshiro256++ PRNG
│   ├── xoshiro256.ts          # PRNG core implementation
│   ├── fastCenter.ts          # O(n log n) Hodges-Lehmann algorithm
│   ├── fastCenterQuantiles.ts # Center quantile binary search
│   ├── fastSpread.ts          # O(n log n) Shamos algorithm
│   ├── fastShift.ts           # O((m+n) log L) shift quantiles
│   ├── constants.ts           # Internal constants
│   └── distributions/         # Uniform, Additive, Exp, Power, Multiplic
├── tests/
│   ├── reference.test.ts      # JSON fixture validation
│   ├── invariance.test.ts     # Mathematical property tests
│   └── performance.test.ts
├── examples/
│   └── demo.ts
├── package.json
└── tsconfig.json
```

## Key Types

| Type | Purpose |
|------|---------|
| `Rng` | Deterministic PRNG with `uniformFloat()`, `sample()`, `shuffle()` |
| `Bounds` | Object with `lower` and `upper` properties |
| `Distribution` | Interface for sampling distributions |

## Public Functions

```typescript
function center(x: number[]): number
function spread(x: number[]): number
function relSpread(x: number[]): number  // Deprecated
function shift(x: number[], y: number[]): number
function ratio(x: number[], y: number[]): number
function disparity(x: number[], y: number[]): number
function shiftBounds(x: number[], y: number[], misrate?: number): Bounds
function ratioBounds(x: number[], y: number[], misrate?: number): Bounds
function disparityBounds(x: number[], y: number[], misrate?: number, seed?: string): Bounds
function centerBounds(x: number[], misrate?: number): Bounds
function spreadBounds(x: number[], misrate?: number, seed?: string): Bounds
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
- **Tolerance**: `1e-9` for floating-point comparisons

```bash
mise run ts:test          # All tests (preferred)
npm test                  # All tests (raw)
npm run test:coverage     # With coverage report
```

## Error Handling

Functions throw `AssumptionError` (extends `Error`) with `violation` property:

```typescript
import { center, AssumptionError } from 'pragmastat';

try {
    const result = center(x);
} catch (e) {
    if (e instanceof AssumptionError) {
        // e.violation.id: "validity", "domain", "positivity", "sparity"
        // e.violation.subject: "x", "y", "misrate"
    }
}
```

Error conditions:
- Empty or non-finite input arrays (`validity`)
- `misrate` outside valid range (`domain`)
- Non-positive values for `ratio` (`positivity`)
- Tie-dominant sample (`sparity`)
- `relSpread` is deprecated; use `spread(x) / Math.abs(center(x))` instead

## Determinism

The `fastCenter` and `fastSpread` functions use deterministic pivot selection via FNV-1a hash. Uses the library's own `Rng` class instead of `Math.random()`.

## Linting

- ESLint for linting
- Prettier for formatting
- Strict TypeScript configuration
