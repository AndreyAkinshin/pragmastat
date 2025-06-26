# Pragmastat TypeScript Implementation

This is the TypeScript implementation of Pragmastat, a pragmatic statistical toolkit designed for analyzing real-world data.

## Installation

```bash
npm install pragmastat
```

## Usage

```typescript
import { 
  center, 
  spread, 
  volatility, 
  precision,
  medShift,
  medRatio,
  medSpread,
  medDisparity
} from 'pragmastat';

// One-sample estimators
const data = [1, 2, 3, 4, 5];
console.log('Center:', center(data));
console.log('Spread:', spread(data));
console.log('Volatility:', volatility(data));
console.log('Precision:', precision(data));

// Two-sample estimators
const x = [1, 2, 3];
const y = [4, 5, 6];
console.log('Median Shift:', medShift(x, y));
console.log('Median Ratio:', medRatio(x, y));
console.log('Median Spread:', medSpread(x, y));
console.log('Median Disparity:', medDisparity(x, y));
```

## Estimators

### One-Sample Estimators

- **center**: Hodges-Lehmann location estimator - robust measure of central tendency
- **spread**: Shamos scale estimator - robust measure of dispersion
- **volatility**: Relative dispersion measure - spread normalized by center
- **precision**: Precision measure - estimated standard error of the center

### Two-Sample Estimators

- **medShift**: Hodges-Lehmann shift estimator - robust measure of location difference
- **medRatio**: Robust ratio estimator - median of all pairwise ratios
- **medSpread**: Pooled spread estimator - combined measure of dispersion
- **medDisparity**: Effect size measure - normalized difference between samples

## Development

### Building

```bash
# Build TypeScript to JavaScript
npm run build

# Or use the build script
./build.sh build
```

### Testing

```bash
# Run all tests
npm test

# Run tests with coverage
npm run test:coverage

# Run tests in watch mode
npm run test:watch
```

### Code Quality

```bash
# Run ESLint
npm run lint

# Format code with Prettier
npm run format

# Check formatting
npm run format:check
```

### Build Script

The `build.sh` script provides convenient commands:

```bash
./build.sh test      # Run all tests
./build.sh build     # Build TypeScript to JavaScript
./build.sh check     # Run linting and format checking
./build.sh clean     # Clean build artifacts
./build.sh format    # Format code with Prettier
./build.sh install   # Install npm dependencies
./build.sh coverage  # Run tests with coverage report
./build.sh watch     # Run tests in watch mode
./build.sh all       # Run all tasks
```

## Project Structure

```
ts/
├── src/                 # Source code
│   ├── index.ts        # Main entry point
│   ├── estimators.ts   # Estimator implementations
│   └── utils.ts        # Utility functions
├── tests/              # Test files
│   ├── estimators.test.ts    # Unit tests
│   ├── invariance.test.ts    # Mathematical invariance tests
│   └── reference.test.ts     # Reference tests against JSON data
├── dist/               # Compiled JavaScript (generated)
├── package.json        # NPM package configuration
├── tsconfig.json       # TypeScript configuration
├── jest.config.js      # Jest test configuration
├── .eslintrc.js        # ESLint configuration
├── .prettierrc         # Prettier configuration
├── build.sh           # Build script
└── README.md          # This file
```

## License

This project is licensed under the MIT License - see the LICENSE file for details.