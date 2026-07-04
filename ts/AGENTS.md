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
│   ├── centerImpl.ts          # O(n log n) Hodges-Lehmann algorithm
│   ├── centerQuantilesImpl.ts # Center quantile binary search
│   ├── spreadImpl.ts          # O(n log n) Shamos algorithm
│   ├── shiftImpl.ts           # O((m+n) log L) shift quantiles
│   ├── constants.ts           # Internal constants
│   └── distributions/         # Uniform, Additive, Exp, Power, Multiplic
├── tests/
│   ├── reference.test.ts          # JSON fixture validation
│   ├── invariance.test.ts         # Mathematical property tests
│   ├── assumeSorted.test.ts       # assumeSorted=true vs default-path equivalence
│   ├── centerImplGuard.test.ts    # centerImpl convergence guard + Bounds.contains
│   ├── spreadImplGuard.test.ts    # spreadImpl convergence guard
│   ├── mutation.test.ts           # Caller arrays and Samples are never mutated
│   ├── ratioBoundsErrors.test.ts  # ratioBounds assumption-error priority
│   ├── properties.test.ts         # Unit propagation, misrate domain, n==2 symmetry
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

Each estimator is overloaded with TWO public entry points (one shared impl, no
duplicated logic):

**(a) Typed Sample API** — accepts a `Sample`, returns a `Measurement` (or
`Bounds`) carrying the propagated unit:

```typescript
function center(x: Sample): Measurement
function spread(x: Sample): Measurement
function shift(x: Sample, y: Sample): Measurement
function ratio(x: Sample, y: Sample): Measurement
function disparity(x: Sample, y: Sample): Measurement
function centerBounds(x: Sample, misrate?: number): Bounds
function spreadBounds(x: Sample, misrate?: number, seed?: string): Bounds
function shiftBounds(x: Sample, y: Sample, misrate?: number): Bounds
function ratioBounds(x: Sample, y: Sample, misrate?: number): Bounds
function disparityBounds(x: Sample, y: Sample, misrate?: number, seed?: string): Bounds
```

**(b) Raw native-array API** — accepts `number[]`, returns a plain `number` (or
unitless `Bounds`), and takes an `assumeSorted` flag:

```typescript
function center(x: number[], assumeSorted?: boolean): number
function spread(x: number[], assumeSorted?: boolean): number
function shift(x: number[], y: number[], assumeSorted?: boolean): number
function ratio(x: number[], y: number[], assumeSorted?: boolean): number
function disparity(x: number[], y: number[], assumeSorted?: boolean): number
function centerBounds(x: number[], misrate?: number, assumeSorted?: boolean): Bounds
function spreadBounds(x: number[], misrate?: number, seed?: string, assumeSorted?: boolean): Bounds
function shiftBounds(x: number[], y: number[], misrate?: number, assumeSorted?: boolean): Bounds
function ratioBounds(x: number[], y: number[], misrate?: number, assumeSorted?: boolean): Bounds
function disparityBounds(x: number[], y: number[], misrate?: number, seed?: string, assumeSorted?: boolean): Bounds
```

`assumeSorted` (default `false`) skips the internal ascending sort. Passing
`true` on UNSORTED input is undefined behavior — it feeds unsorted data to a
sorted-only kernel and may produce a wrong result or ERROR. This holds for
`spreadBounds`/`disparityBounds` too: their disjoint-pair shuffle always runs on
the passed order (so the flag never affects the shuffle), but the sparity check
runs the sorted-only spread kernel under `assumeSorted`, so the flag is inert
only on genuinely SORTED input.

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

## Determinism

The `centerImpl` and `spreadImpl` functions use deterministic pivot selection via FNV-1a hash. Uses the library's own `Rng` class instead of `Math.random()`.

## Linting

- ESLint for linting
- Prettier for formatting
- Strict TypeScript configuration
