import * as fs from 'fs';
import * as path from 'path';
import {
  center,
  spread,
  shift,
  ratio,
  _avgSpread as avgSpread,
  disparity,
  shiftBounds,
  ratioBounds,
  centerBounds,
  spreadBounds,
  disparityBounds,
  _avgSpreadBounds as avgSpreadBounds,
} from '../src/estimators';
import { signedRankMargin } from '../src/signedRankMargin';
import { AssumptionError } from '../src/assumptions';
import { pairwiseMargin } from '../src/pairwiseMargin';
import { Rng } from '../src/rng';
import { Additive, Exp, Multiplic, Power, Uniform } from '../src/distributions';
import { Sample } from '../src/sample';
import { MeasurementUnit } from '../src/measurement-unit';
import { Measurement } from '../src/measurement';
import { UnitRegistry } from '../src/unit-registry';
import { Metric, Threshold, compare1, compare2 } from '../src/compare';
import { expectBitwise, expectBitwiseBounds, expectBitwiseSequence } from './bitwise';

/**
 * Reference tests comparing against expected values from JSON files
 */

/**
 * Creates a Sample from raw values.
 *
 * Sample construction validates (empty / NaN / Inf) and always reports those
 * construction errors with the hardcoded subject 'x' — it cannot know whether
 * the values came from arg1 ('x') or arg2 ('y'). The Sample path therefore
 * skips the subject assertion for construction-time 'y' validity errors (see
 * `expectError` / `runDualPath`); the raw path still asserts subject fully.
 */
function sampleFromTestData(values: number[]): Sample {
  return Sample.of(values);
}

/**
 * Asserts a thrown AssumptionError against a fixture's `expected_error`.
 *
 * `isSampleCreation` marks Sample-path entries whose validity errors are raised
 * during Sample construction. Construction always reports subject 'x' for a
 * two-sample 'y' argument, so for those entries the subject assertion is skipped
 * on a 'y' expected-subject validity error (id is still asserted). The raw path
 * (`isSampleCreation === false`) always asserts subject fully.
 */
function expectError(
  thrownError: AssumptionError | null,
  expectedError: { id: string; subject: string },
  isSampleCreation: boolean,
): void {
  expect(thrownError).not.toBeNull();
  expect(thrownError!.violation!.id).toBe(expectedError.id);
  const skipSubject =
    isSampleCreation && expectedError.subject === 'y' && expectedError.id === 'validity';
  if (!skipSubject) {
    expect(thrownError!.violation!.subject).toBe(expectedError.subject);
  }
}

/**
 * Dual-path entry points: every reference fixture runs through BOTH the raw
 * native-array API and the Sample API so that Sample-adapter bugs are caught
 * (a past critical bug shipped because fixtures only ran through one path).
 *
 * `isSampleCreation` marks entries whose validity errors are raised during
 * Sample construction; see `expectError` for how the subject assertion is
 * handled for those entries.
 */
interface EntryPoint<R> {
  name: string;
  isSampleCreation: boolean;
  run: (data: TestData) => R;
}

interface TestData {
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  input: any;
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  output?: any;
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  expected_error?: any;
}

/** Extracts x (and optional y) raw arrays from a fixture's input field. */
function getInputArrays(input: TestData['input']): { x: number[]; y?: number[] } {
  if (Array.isArray(input)) {
    return { x: input };
  }
  if (input && typeof input === 'object' && 'x' in input) {
    return 'y' in input ? { x: input.x, y: input.y } : { x: input.x };
  }
  throw new Error('Invalid test data input format');
}

/**
 * Runs a single fixture through every provided entry point, asserting that each
 * matches the fixture's expected value/bounds or expected error.
 */
function runDualPath<R>(
  data: TestData,
  entries: EntryPoint<R>[],
  assertValue: (result: R, expected: TestData['output'], entryName: string) => void,
): void {
  for (const entry of entries) {
    if (data.expected_error) {
      let thrownError: AssumptionError | null = null;
      try {
        entry.run(data);
      } catch (e) {
        if (e instanceof AssumptionError) {
          thrownError = e;
        } else {
          throw e;
        }
      }
      expectError(thrownError, data.expected_error, entry.isSampleCreation);
    } else {
      const result = entry.run(data);
      assertValue(result, data.output, entry.name);
    }
  }
}

describe('Reference Tests', () => {
  const testDataPath = path.join(__dirname, '../../tests');

  // Value estimators: each dir runs through BOTH the raw native-array API
  // (assumeSorted=false) and the Sample API. avg-spread has no public raw API
  // (internal helper), so it runs through the Sample path only.
  //
  // Each entry returns a Measurement-like `{ value }` so a single comparator
  // can assert against the fixture's numeric `output`.
  interface ValueResult {
    value: number;
  }

  type RawOne = (x: number[]) => number;
  type SampleOne = (x: Sample) => Measurement;
  type RawTwo = (x: number[], y: number[]) => number;
  type SampleTwo = (x: Sample, y: Sample) => Measurement;

  function oneSampleEntries(raw: RawOne | null, sample: SampleOne): EntryPoint<ValueResult>[] {
    const entries: EntryPoint<ValueResult>[] = [];
    if (raw) {
      entries.push({
        name: 'raw',
        isSampleCreation: false,
        run: (data) => ({ value: raw(getInputArrays(data.input).x) }),
      });
    }
    entries.push({
      name: 'sample',
      isSampleCreation: true,
      run: (data) => sample(sampleFromTestData(getInputArrays(data.input).x)),
    });
    return entries;
  }

  function twoSampleEntries(raw: RawTwo | null, sample: SampleTwo): EntryPoint<ValueResult>[] {
    const entries: EntryPoint<ValueResult>[] = [];
    if (raw) {
      entries.push({
        name: 'raw',
        isSampleCreation: false,
        run: (data) => {
          const { x, y } = getInputArrays(data.input);
          return { value: raw(x, y!) };
        },
      });
    }
    entries.push({
      name: 'sample',
      isSampleCreation: true,
      run: (data) => {
        const { x, y } = getInputArrays(data.input);
        const sx = sampleFromTestData(x);
        const sy = sampleFromTestData(y!);
        return sample(sx, sy);
      },
    });
    return entries;
  }

  const valueEstimatorEntries: Record<string, EntryPoint<ValueResult>[]> = {
    center: oneSampleEntries(center, center),
    spread: oneSampleEntries(spread, spread),
    shift: twoSampleEntries(shift, shift),
    ratio: twoSampleEntries(ratio, ratio),
    disparity: twoSampleEntries(disparity, disparity),
    // avg-spread: Sample-only internal helper, no raw public API.
    'avg-spread': twoSampleEntries(null, avgSpread),
  };

  /**
   * Every estimator suite is compared bit for bit (`expectBitwise`) unless it is
   * listed below.
   *
   * The default is exactness because these estimators return an element selected
   * out of a pairwise set (or an average of two such elements). A divergence is
   * therefore never a small error: either the same element was selected and the
   * answer is bit-identical, or a different one was, and the gap is
   * data-dependent and unbounded by any epsilon. A tolerance hides exactly the
   * failure it appears to guard against. That is measured, not assumed:
   * recomputing every estimator with each call to `log`, `exp`, `pow` and `cos`
   * returning the neighbouring representable value (the largest legitimate
   * difference between two conforming libm implementations) moved none of them,
   * on any input.
   *
   * Suites that stay on a tolerance, and why:
   *
   * `ratio` is `exp(median(log x - log y))`: unlike the selection estimators it
   * really is approximate. Perturbing libm by one ULP moves it on 94% of inputs,
   * by up to 16 ULP, so exact equality across implementations is not a property
   * it has. `ratio-bounds` inherits this from the same projection. `compare2`
   * keeps a tolerance for the reason given above its own `describe`.
   */
  const TOLERANT_VALUE_DIRS = new Set(['ratio']);
  const TOLERANT_BOUNDS_DIRS = new Set(['ratio-bounds']);

  const makeAssertValue =
    (subject: string, exact: boolean) =>
    (result: ValueResult, expected: number, entryName: string): void => {
      if (exact) {
        expectBitwise(`${subject} [${entryName}]`, result.value, expected);
      } else {
        expect(result.value).toBeCloseTo(expected, 9);
      }
    };

  // ---------------------------------------------------------------------------
  // Bounds-estimator dual-path infrastructure
  // ---------------------------------------------------------------------------
  interface BoundsResult {
    lower: number;
    upper: number;
  }

  const makeAssertBounds =
    (subject: string, exact: boolean) =>
    (result: BoundsResult, expected: { lower: number; upper: number }, entryName: string): void => {
      if (exact) {
        expectBitwiseBounds(`${subject} [${entryName}]`, result, expected);
      } else {
        expect(result.lower).toBeCloseTo(expected.lower, 9);
        expect(result.upper).toBeCloseTo(expected.upper, 9);
      }
    };

  // One-sample bounds (centerBounds, spreadBounds): raw + sample entry points.
  function oneSampleBoundsEntries(
    raw: (x: number[], misrate: number, seed?: string) => BoundsResult,
    sample: (x: Sample, misrate: number, seed?: string) => BoundsResult,
  ): EntryPoint<BoundsResult>[] {
    return [
      {
        name: 'raw',
        isSampleCreation: false,
        run: (data) => raw(getInputArrays(data.input).x, data.input.misrate, data.input.seed),
      },
      {
        name: 'sample',
        isSampleCreation: true,
        run: (data) =>
          sample(
            sampleFromTestData(getInputArrays(data.input).x),
            data.input.misrate,
            data.input.seed,
          ),
      },
    ];
  }

  // Two-sample bounds (shiftBounds, ratioBounds, disparityBounds).
  function twoSampleBoundsEntries(
    raw: (x: number[], y: number[], misrate: number, seed?: string) => BoundsResult,
    sample: (x: Sample, y: Sample, misrate: number, seed?: string) => BoundsResult,
  ): EntryPoint<BoundsResult>[] {
    return [
      {
        name: 'raw',
        isSampleCreation: false,
        run: (data): BoundsResult => {
          const { x, y } = getInputArrays(data.input);
          return raw(x, y!, data.input.misrate, data.input.seed);
        },
      },
      {
        name: 'sample',
        isSampleCreation: true,
        run: (data): BoundsResult => {
          const { x, y } = getInputArrays(data.input);
          const sx = sampleFromTestData(x);
          const sy = sampleFromTestData(y!);
          return sample(sx, sy, data.input.misrate, data.input.seed);
        },
      },
    ];
  }

  // Runs every *.json fixture under `dirName` through both bounds entry points.
  function runBoundsDir(dirName: string, entries: EntryPoint<BoundsResult>[]): void {
    describe(dirName, () => {
      const dirPath = path.join(testDataPath, dirName);
      if (!fs.existsSync(dirPath)) return;
      const testFiles = fs
        .readdirSync(dirPath)
        .filter((f) => f.endsWith('.json'))
        .sort();

      it('should have test files', () => {
        expect(testFiles.length).toBeGreaterThan(0);
      });

      const exact = !TOLERANT_BOUNDS_DIRS.has(dirName);

      testFiles.forEach((fileName) => {
        const filePath = path.join(dirPath, fileName);
        const testName = fileName.replace('.json', '');
        it(`should pass ${testName}`, () => {
          const data: TestData = JSON.parse(fs.readFileSync(filePath, 'utf8'));
          runDualPath(data, entries, makeAssertBounds(`${dirName}/${fileName}`, exact));
        });
      });
    });
  }

  runBoundsDir(
    'shift-bounds',
    twoSampleBoundsEntries(
      (x, y, misrate) => shiftBounds(x, y, misrate),
      (x, y, misrate) => shiftBounds(x, y, misrate),
    ),
  );
  runBoundsDir(
    'ratio-bounds',
    twoSampleBoundsEntries(
      (x, y, misrate) => ratioBounds(x, y, misrate),
      (x, y, misrate) => ratioBounds(x, y, misrate),
    ),
  );
  runBoundsDir(
    'center-bounds',
    oneSampleBoundsEntries(
      (x, misrate) => centerBounds(x, misrate),
      (x, misrate) => centerBounds(x, misrate),
    ),
  );
  runBoundsDir(
    'spread-bounds',
    oneSampleBoundsEntries(
      (x, misrate, seed) => spreadBounds(x, misrate, seed),
      (x, misrate, seed) => spreadBounds(x, misrate, seed),
    ),
  );
  runBoundsDir(
    'disparity-bounds',
    twoSampleBoundsEntries(
      (x, y, misrate, seed) => disparityBounds(x, y, misrate, seed),
      (x, y, misrate, seed) => disparityBounds(x, y, misrate, seed),
    ),
  );

  // Get all test directories that match a value estimator
  const testDirs = fs
    .readdirSync(testDataPath)
    .filter((dir) => fs.statSync(path.join(testDataPath, dir)).isDirectory())
    .filter((dir) => dir in valueEstimatorEntries);

  testDirs.forEach((dirName) => {
    describe(dirName, () => {
      const dirPath = path.join(testDataPath, dirName);
      const testFiles = fs
        .readdirSync(dirPath)
        .filter((file) => file.endsWith('.json'))
        .sort();

      const entries = valueEstimatorEntries[dirName];
      const exact = !TOLERANT_VALUE_DIRS.has(dirName);

      testFiles.forEach((fileName) => {
        const filePath = path.join(dirPath, fileName);
        const testName = fileName.replace('.json', '');

        it(`should pass ${testName}`, () => {
          const data: TestData = JSON.parse(fs.readFileSync(filePath, 'utf8'));
          runDualPath(data, entries, makeAssertValue(`${dirName}/${fileName}`, exact));
        });
      });
    });
  });

  // PairwiseMargin tests
  describe('pairwise-margin', () => {
    const dirPath = path.join(testDataPath, 'pairwise-margin');
    if (fs.existsSync(dirPath)) {
      const testFiles = fs
        .readdirSync(dirPath)
        .filter((file) => file.endsWith('.json'))
        .sort();

      testFiles.forEach((fileName) => {
        const filePath = path.join(dirPath, fileName);
        const testName = fileName.replace('.json', '');

        it(`should pass ${testName}`, () => {
          const data = JSON.parse(fs.readFileSync(filePath, 'utf8'));

          // Handle error test cases
          if (data.expected_error) {
            let thrownError: AssumptionError | null = null;
            try {
              pairwiseMargin(data.input.n, data.input.m, data.input.misrate);
            } catch (e) {
              if (e instanceof AssumptionError) {
                thrownError = e;
              } else {
                throw e;
              }
            }
            expect(thrownError).not.toBeNull();
            expect(thrownError!.violation!.id).toBe(data.expected_error.id);
            expect(thrownError!.violation!.subject).toBe(data.expected_error.subject);

            return;
          }

          const result = pairwiseMargin(data.input.n, data.input.m, data.input.misrate);
          expect(result).toBe(data.output);
        });
      });
    }
  });

  // The randomization contract is bitwise: `new Rng(seed)` must produce an
  // identical sequence in every language implementation, and the manual states
  // so. "Close enough" is therefore not the property under test — a tolerance
  // reports a broken contract as a pass. That is not theoretical: a compiler is
  // free to fuse a multiply into an add and change the last bit of a draw (Go's
  // arm64 backend does exactly this in `UniformFloat64Range`), and the tolerant
  // comparison that used to live here would have shipped it. So every suite
  // below (rng, shuffle, sample, resample, uniform distribution) compares
  // payloads.
  //
  // The additive/multiplic/exp/power distributions stay tolerant on purpose:
  // their draws go through `log`, `exp`, `cos` and `pow`, which every language
  // takes from a different libm, so bitwise equality there is not achievable.

  // Rng uniform tests
  describe('rng-uniform', () => {
    const dirPath = path.join(testDataPath, 'rng');
    if (fs.existsSync(dirPath)) {
      const testFiles = fs
        .readdirSync(dirPath)
        .filter((file) => file.startsWith('uniform-seed-') && file.endsWith('.json'))
        .sort();

      testFiles.forEach((fileName) => {
        const filePath = path.join(dirPath, fileName);
        const testName = fileName.replace('.json', '');

        it(`should pass ${testName}`, () => {
          const data = JSON.parse(fs.readFileSync(filePath, 'utf8'));
          const rng = new Rng(data.input.seed);
          const actual = Array.from({ length: data.input.count }, () => rng.uniformFloat());

          expectBitwiseSequence('uniformFloat()', actual, data.output);
        });
      });
    }
  });

  // Rng uniform int tests
  describe('rng-uniform-int', () => {
    const dirPath = path.join(testDataPath, 'rng');
    if (fs.existsSync(dirPath)) {
      const testFiles = fs
        .readdirSync(dirPath)
        .filter((file) => file.startsWith('uniform-int-') && file.endsWith('.json'))
        .sort();

      testFiles.forEach((fileName) => {
        const filePath = path.join(dirPath, fileName);
        const testName = fileName.replace('.json', '');

        it(`should pass ${testName}`, () => {
          const data = JSON.parse(fs.readFileSync(filePath, 'utf8'));
          const rng = new Rng(data.input.seed);
          const actual = Array.from({ length: data.input.count }, () =>
            rng.uniformInt(data.input.min, data.input.max),
          );

          expectBitwiseSequence(
            `uniformInt(${data.input.min}, ${data.input.max})`,
            actual,
            data.output,
          );
        });
      });
    }
  });

  // Rng string seed tests
  describe('rng-string-seed', () => {
    const dirPath = path.join(testDataPath, 'rng');
    if (fs.existsSync(dirPath)) {
      const testFiles = fs
        .readdirSync(dirPath)
        .filter((file) => file.startsWith('uniform-string-') && file.endsWith('.json'))
        .sort();

      testFiles.forEach((fileName) => {
        const filePath = path.join(dirPath, fileName);
        const testName = fileName.replace('.json', '');

        it(`should pass ${testName}`, () => {
          const data = JSON.parse(fs.readFileSync(filePath, 'utf8'));
          const rng = new Rng(data.input.seed);
          const actual = Array.from({ length: data.input.count }, () => rng.uniformFloat());

          expectBitwiseSequence(
            `uniformFloat() [seed ${JSON.stringify(data.input.seed)}]`,
            actual,
            data.output,
          );
        });
      });
    }
  });

  // Rng uniform range tests
  describe('rng-uniform-range', () => {
    const dirPath = path.join(testDataPath, 'rng');
    if (fs.existsSync(dirPath)) {
      const testFiles = fs
        .readdirSync(dirPath)
        .filter((file) => file.startsWith('uniform-range-') && file.endsWith('.json'))
        .sort();

      testFiles.forEach((fileName) => {
        const filePath = path.join(dirPath, fileName);
        const testName = fileName.replace('.json', '');

        it(`should pass ${testName}`, () => {
          const data = JSON.parse(fs.readFileSync(filePath, 'utf8'));
          const rng = new Rng(data.input.seed);
          const actual = Array.from({ length: data.input.count }, () =>
            rng.uniformFloatRange(data.input.min, data.input.max),
          );

          expectBitwiseSequence(
            `uniformFloatRange(${data.input.min}, ${data.input.max})`,
            actual,
            data.output,
          );
        });
      });
    }
  });

  // Rng uniform bool tests
  describe('rng-uniform-bool', () => {
    const dirPath = path.join(testDataPath, 'rng');
    if (fs.existsSync(dirPath)) {
      const testFiles = fs
        .readdirSync(dirPath)
        .filter((file) => file.startsWith('uniform-bool-seed-') && file.endsWith('.json'))
        .sort();

      testFiles.forEach((fileName) => {
        const filePath = path.join(dirPath, fileName);
        const testName = fileName.replace('.json', '');

        it(`should pass ${testName}`, () => {
          const data = JSON.parse(fs.readFileSync(filePath, 'utf8'));
          const rng = new Rng(data.input.seed);
          const actual = Array.from({ length: data.input.count }, () => rng.uniformBool());

          expect(actual).toEqual(data.output);
        });
      });
    }
  });

  // Shuffle tests
  describe('shuffle', () => {
    const dirPath = path.join(testDataPath, 'shuffle');
    if (fs.existsSync(dirPath)) {
      const testFiles = fs
        .readdirSync(dirPath)
        .filter((file) => file.endsWith('.json'))
        .sort();

      testFiles.forEach((fileName) => {
        const filePath = path.join(dirPath, fileName);
        const testName = fileName.replace('.json', '');

        it(`should pass ${testName}`, () => {
          const data = JSON.parse(fs.readFileSync(filePath, 'utf8'));
          const rng = new Rng(data.input.seed);
          const actual = rng.shuffle<number>(data.input.x);

          expectBitwiseSequence('shuffle()', actual, data.output);
        });
      });
    }
  });

  // Sample tests
  describe('sample', () => {
    const dirPath = path.join(testDataPath, 'sample');
    if (fs.existsSync(dirPath)) {
      const testFiles = fs
        .readdirSync(dirPath)
        .filter((file) => file.endsWith('.json'))
        .sort();

      testFiles.forEach((fileName) => {
        const filePath = path.join(dirPath, fileName);
        const testName = fileName.replace('.json', '');

        it(`should pass ${testName}`, () => {
          const data = JSON.parse(fs.readFileSync(filePath, 'utf8'));
          const rng = new Rng(data.input.seed);
          const actual = rng.sample<number>(data.input.x, data.input.k);

          expectBitwiseSequence(`sample(k=${data.input.k})`, actual, data.output);
        });
      });
    }
  });

  // Resample tests
  describe('resample', () => {
    const dirPath = path.join(testDataPath, 'resample');
    if (fs.existsSync(dirPath)) {
      const testFiles = fs
        .readdirSync(dirPath)
        .filter((file) => file.endsWith('.json'))
        .sort();

      testFiles.forEach((fileName) => {
        const filePath = path.join(dirPath, fileName);
        const testName = fileName.replace('.json', '');

        it(`should pass ${testName}`, () => {
          const data = JSON.parse(fs.readFileSync(filePath, 'utf8'));
          const rng = new Rng(data.input.seed);
          const actual = rng.resample<number>(data.input.x, data.input.k);

          expectBitwiseSequence(`resample(k=${data.input.k})`, actual, data.output);
        });
      });
    }
  });

  // Distribution tests
  describe('distributions/uniform', () => {
    const dirPath = path.join(testDataPath, 'distributions', 'uniform');
    if (fs.existsSync(dirPath)) {
      const testFiles = fs
        .readdirSync(dirPath)
        .filter((file) => file.endsWith('.json'))
        .sort();

      testFiles.forEach((fileName) => {
        const filePath = path.join(dirPath, fileName);
        const testName = fileName.replace('.json', '');

        it(`should pass ${testName}`, () => {
          const data = JSON.parse(fs.readFileSync(filePath, 'utf8'));
          const rng = new Rng(data.input.seed);
          const dist = new Uniform(data.input.min, data.input.max);
          const actual = Array.from({ length: data.input.count }, () => dist.sample(rng));

          expectBitwiseSequence(
            `Uniform(${data.input.min}, ${data.input.max}).sample()`,
            actual,
            data.output,
          );
        });
      });
    }
  });

  describe('distributions/additive', () => {
    const dirPath = path.join(testDataPath, 'distributions', 'additive');
    if (fs.existsSync(dirPath)) {
      const testFiles = fs
        .readdirSync(dirPath)
        .filter((file) => file.endsWith('.json'))
        .sort();

      testFiles.forEach((fileName) => {
        const filePath = path.join(dirPath, fileName);
        const testName = fileName.replace('.json', '');

        it(`should pass ${testName}`, () => {
          const data = JSON.parse(fs.readFileSync(filePath, 'utf8'));
          const rng = new Rng(data.input.seed);
          const dist = new Additive(data.input.mean, data.input.stdDev);
          const actual = Array.from({ length: data.input.count }, () => dist.sample(rng));

          for (let i = 0; i < actual.length; i++) {
            expect(actual[i]).toBeCloseTo(data.output[i], 12);
          }
        });
      });
    }
  });

  describe('distributions/multiplic', () => {
    const dirPath = path.join(testDataPath, 'distributions', 'multiplic');
    if (fs.existsSync(dirPath)) {
      const testFiles = fs
        .readdirSync(dirPath)
        .filter((file) => file.endsWith('.json'))
        .sort();

      testFiles.forEach((fileName) => {
        const filePath = path.join(dirPath, fileName);
        const testName = fileName.replace('.json', '');

        it(`should pass ${testName}`, () => {
          const data = JSON.parse(fs.readFileSync(filePath, 'utf8'));
          const rng = new Rng(data.input.seed);
          const dist = new Multiplic(data.input.logMean, data.input.logStdDev);
          const actual = Array.from({ length: data.input.count }, () => dist.sample(rng));

          for (let i = 0; i < actual.length; i++) {
            expect(actual[i]).toBeCloseTo(data.output[i], 12);
          }
        });
      });
    }
  });

  describe('distributions/exp', () => {
    const dirPath = path.join(testDataPath, 'distributions', 'exp');
    if (fs.existsSync(dirPath)) {
      const testFiles = fs
        .readdirSync(dirPath)
        .filter((file) => file.endsWith('.json'))
        .sort();

      testFiles.forEach((fileName) => {
        const filePath = path.join(dirPath, fileName);
        const testName = fileName.replace('.json', '');

        it(`should pass ${testName}`, () => {
          const data = JSON.parse(fs.readFileSync(filePath, 'utf8'));
          const rng = new Rng(data.input.seed);
          const dist = new Exp(data.input.rate);
          const actual = Array.from({ length: data.input.count }, () => dist.sample(rng));

          for (let i = 0; i < actual.length; i++) {
            expect(actual[i]).toBeCloseTo(data.output[i], 12);
          }
        });
      });
    }
  });

  describe('distributions/power', () => {
    const dirPath = path.join(testDataPath, 'distributions', 'power');
    if (fs.existsSync(dirPath)) {
      const testFiles = fs
        .readdirSync(dirPath)
        .filter((file) => file.endsWith('.json'))
        .sort();

      testFiles.forEach((fileName) => {
        const filePath = path.join(dirPath, fileName);
        const testName = fileName.replace('.json', '');

        it(`should pass ${testName}`, () => {
          const data = JSON.parse(fs.readFileSync(filePath, 'utf8'));
          const rng = new Rng(data.input.seed);
          const dist = new Power(data.input.min, data.input.shape);
          const actual = Array.from({ length: data.input.count }, () => dist.sample(rng));

          for (let i = 0; i < actual.length; i++) {
            expect(actual[i]).toBeCloseTo(data.output[i], 12);
          }
        });
      });
    }
  });

  describe('sample validation', () => {
    it('should throw error for negative k', () => {
      const rng = new Rng('test-sample-validation');
      expect(() => rng.sample([1, 2, 3], -1)).toThrow('k must be positive');
    });
  });

  // SignedRankMargin tests
  describe('signed-rank-margin', () => {
    const dirPath = path.join(testDataPath, 'signed-rank-margin');
    if (fs.existsSync(dirPath)) {
      const testFiles = fs
        .readdirSync(dirPath)
        .filter((file) => file.endsWith('.json'))
        .sort();

      testFiles.forEach((fileName) => {
        const filePath = path.join(dirPath, fileName);
        const testName = fileName.replace('.json', '');

        it(`should pass ${testName}`, () => {
          const data = JSON.parse(fs.readFileSync(filePath, 'utf8'));

          // Handle error test cases
          if (data.expected_error) {
            let thrownError: AssumptionError | null = null;
            try {
              signedRankMargin(data.input.n, data.input.misrate);
            } catch (e) {
              if (e instanceof AssumptionError) {
                thrownError = e;
              } else {
                throw e;
              }
            }
            expect(thrownError).not.toBeNull();
            expect(thrownError!.violation!.id).toBe(data.expected_error.id);
            expect(thrownError!.violation!.subject).toBe(data.expected_error.subject);

            return;
          }

          const result = signedRankMargin(data.input.n, data.input.misrate);
          expect(result).toBe(data.output);
        });
      });
    }
  });

  // AvgSpreadBounds tests
  describe('avg-spread-bounds', () => {
    const dirPath = path.join(testDataPath, 'avg-spread-bounds');
    if (fs.existsSync(dirPath)) {
      const testFiles = fs
        .readdirSync(dirPath)
        .filter((f) => f.endsWith('.json'))
        .sort();

      it('should have test files', () => {
        expect(testFiles.length).toBeGreaterThan(0);
      });

      testFiles.forEach((fileName) => {
        const filePath = path.join(dirPath, fileName);
        const testName = fileName.replace('.json', '');

        it(`should pass ${testName}`, () => {
          const data = JSON.parse(fs.readFileSync(filePath, 'utf8'));

          if (data.expected_error) {
            let thrownError: AssumptionError | null = null;
            try {
              const sx = sampleFromTestData(data.input.x);
              const sy = sampleFromTestData(data.input.y);
              avgSpreadBounds(sx, sy, data.input.misrate, data.input.seed);
            } catch (e) {
              if (e instanceof AssumptionError) {
                thrownError = e;
              } else {
                throw e;
              }
            }
            // Sample-only path: construction reports 'y' validity as 'x'.
            expectError(thrownError, data.expected_error, true);
            return;
          }

          const sx = Sample.of(data.input.x);
          const sy = Sample.of(data.input.y);
          const result = avgSpreadBounds(sx, sy, data.input.misrate, data.input.seed);
          expectBitwiseBounds(`avg-spread-bounds/${fileName}`, result, data.output);
        });
      });
    }
  });

  // Sample construction tests
  describe('sample-construction', () => {
    const dirPath = path.join(testDataPath, 'sample-construction');
    if (fs.existsSync(dirPath)) {
      const testFiles = fs
        .readdirSync(dirPath)
        .filter((f) => f.endsWith('.json'))
        .sort();

      testFiles.forEach((fileName) => {
        const filePath = path.join(dirPath, fileName);
        const testName = fileName.replace('.json', '');

        it(`should pass ${testName}`, () => {
          const data = JSON.parse(fs.readFileSync(filePath, 'utf8'));

          // Parse special float values (NaN, Infinity, -Infinity)
          const rawValues: number[] = data.input.values.map((v: string | number) => {
            if (v === 'NaN') return NaN;
            if (v === 'Infinity') return Infinity;
            if (v === '-Infinity') return -Infinity;
            return v as number;
          });

          if (data.expected_error) {
            expect(() => {
              if (data.input.weights) {
                Sample.weighted(rawValues, data.input.weights);
              } else {
                Sample.of(rawValues);
              }
            }).toThrow();
            return;
          }

          let sample: Sample;
          if (data.input.weights) {
            sample = Sample.weighted(rawValues, data.input.weights);
          } else {
            sample = Sample.of(rawValues);
          }

          expect(sample.size).toBe(data.output.size);
          expect(sample.isWeighted).toBe(data.output.is_weighted);
          // Bitwise, not toBeCloseTo. Both are public values derived by summing
          // the weights, and a sum depends on the order it is taken in: floating-point
          // addition is not associative. A tolerance here would accept an implementation
          // that reduces pairwise or accumulates in extended precision, which is exactly
          // the divergence these fields exist to pin. Absent on unweighted fixtures.
          if (data.output.total_weight !== undefined) {
            expectBitwise(
              `sample-construction/${fileName}.totalWeight`,
              sample.totalWeight,
              data.output.total_weight,
            );
          }
          if (data.output.weighted_size !== undefined) {
            expectBitwise(
              `sample-construction/${fileName}.weightedSize`,
              sample.weightedSize,
              data.output.weighted_size,
            );
          }
        });
      });
    }
  });

  // Unit propagation tests
  describe('unit-propagation', () => {
    const dirPath = path.join(testDataPath, 'unit-propagation');
    if (fs.existsSync(dirPath)) {
      const registry = UnitRegistry.standard();
      const testFiles = fs
        .readdirSync(dirPath)
        .filter((f) => f.endsWith('.json'))
        .sort();

      testFiles.forEach((fileName) => {
        const filePath = path.join(dirPath, fileName);
        const testName = fileName.replace('.json', '');

        it(`should pass ${testName}`, () => {
          const data = JSON.parse(fs.readFileSync(filePath, 'utf8'));

          // weighted-rejected test
          if (data.expected_error === 'weighted_not_supported') {
            const xUnit = data.input.x_unit
              ? registry.resolve(data.input.x_unit)
              : MeasurementUnit.NUMBER;
            const sx = Sample.weighted(data.input.x, data.input.x_weights, xUnit);
            expect(() => {
              const estimatorName: string = data.input.estimator;
              if (estimatorName === 'center') {
                center(sx);
              } else if (estimatorName === 'spread') {
                spread(sx);
              } else if (estimatorName === 'shift') {
                const sy = Sample.of(data.input.y);
                shift(sx, sy);
              } else if (estimatorName === 'ratio') {
                const sy = Sample.of(data.input.y);
                ratio(sx, sy);
              } else if (estimatorName === 'disparity') {
                const sy = Sample.of(data.input.y);
                disparity(sx, sy);
              }
            }).toThrow();
            return;
          }

          const estimatorName: string = data.input.estimator;
          const xUnit = data.input.x_unit
            ? registry.resolve(data.input.x_unit)
            : MeasurementUnit.NUMBER;
          const sx = Sample.withUnit(data.input.x, xUnit);

          if (data.input.y !== undefined) {
            // Two-sample
            const yUnit = data.input.y_unit
              ? registry.resolve(data.input.y_unit)
              : MeasurementUnit.NUMBER;
            const sy = Sample.withUnit(data.input.y, yUnit);

            let result: Measurement;
            if (estimatorName === 'shift') {
              result = shift(sx, sy);
            } else if (estimatorName === 'ratio') {
              result = ratio(sx, sy);
            } else if (estimatorName === 'disparity') {
              result = disparity(sx, sy);
            } else {
              throw new Error(`Unknown two-sample estimator: ${estimatorName}`);
            }

            expect(result.unit.id).toBe(data.output.unit);
            if (data.output.value !== undefined) {
              expectBitwise(`unit-propagation/${fileName}`, result.value, data.output.value);
            }
          } else {
            // One-sample
            let result: Measurement;
            if (estimatorName === 'center') {
              result = center(sx);
            } else if (estimatorName === 'spread') {
              result = spread(sx);
            } else {
              throw new Error(`Unknown one-sample estimator: ${estimatorName}`);
            }

            expect(result.unit.id).toBe(data.output.unit);
            if (data.output.value !== undefined) {
              expectBitwise(`unit-propagation/${fileName}`, result.value, data.output.value);
            }
          }
        });
      });
    }
  });

  // Compare1 tests
  describe('compare1', () => {
    const dirPath = path.join(testDataPath, 'compare1');
    if (fs.existsSync(dirPath)) {
      const testFiles = fs
        .readdirSync(dirPath)
        .filter((f) => f.endsWith('.json'))
        .sort();

      testFiles.forEach((fileName) => {
        const filePath = path.join(dirPath, fileName);
        const testName = fileName.replace('.json', '');

        it(`should pass ${testName}`, () => {
          const data = JSON.parse(fs.readFileSync(filePath, 'utf8'));

          // Handle error test cases
          if (data.expected_error) {
            let thrownError: AssumptionError | null = null;
            try {
              const sx = sampleFromTestData(data.input.x);
              const thresholds = data.input.thresholds.map(
                (t: { metric: string; value: number; misrate: number }) =>
                  new Threshold(t.metric as Metric, new Measurement(t.value), t.misrate),
              );
              compare1(sx, thresholds, data.input.seed);
            } catch (e) {
              if (e instanceof AssumptionError) {
                thrownError = e;
              } else {
                throw e;
              }
            }
            // Sample-only path: construction reports 'y' validity as 'x'.
            expectError(thrownError, data.expected_error, true);
            return;
          }

          const sx = Sample.of(data.input.x);
          const thresholds = data.input.thresholds.map(
            (t: { metric: string; value: number; misrate: number }) =>
              new Threshold(t.metric as Metric, new Measurement(t.value), t.misrate),
          );
          const results = compare1(sx, thresholds, data.input.seed);

          expect(results.length).toBe(data.output.projections.length);
          for (let i = 0; i < results.length; i++) {
            const subject = `compare1/${fileName} projection ${i}`;
            const projection = data.output.projections[i];
            expectBitwise(`${subject}.estimate`, results[i].estimate.value, projection.estimate);
            expectBitwise(`${subject}.lower`, results[i].bounds.lower, projection.lower);
            expectBitwise(`${subject}.upper`, results[i].bounds.upper, projection.upper);
            expect(results[i].verdict).toBe(projection.verdict);
          }
        });
      });
    }
  });

  // Compare2 tests
  //
  // Compared per projection rather than per suite, because the class differs from
  // one projection to the next. Each is produced by the bounds estimator for its
  // threshold's metric: `shift` and `disparity` select an element out of a pairwise
  // set and are exact, `ratio` runs through `log` and `exp` and is not. A suite-wide
  // predicate has to be the weakest one present, so a handful of ratio thresholds
  // used to put every shift and disparity projection standing beside them on a
  // tolerance too.
  //
  // The metric of projection i is `input.thresholds[i].metric`: the fixture emits one
  // projection per threshold, in order, and the `order-*` fixtures exist to pin that
  // alignment. Reading it off the fixture rather than off the returned projection
  // keeps the expectation independent of the code under test. Every field of a
  // projection follows the same class, so estimate and bounds are compared together;
  // the verdict is a string and is exact regardless.
  describe('compare2', () => {
    const dirPath = path.join(testDataPath, 'compare2');
    if (fs.existsSync(dirPath)) {
      const testFiles = fs
        .readdirSync(dirPath)
        .filter((f) => f.endsWith('.json'))
        .sort();

      testFiles.forEach((fileName) => {
        const filePath = path.join(dirPath, fileName);
        const testName = fileName.replace('.json', '');

        it(`should pass ${testName}`, () => {
          const data = JSON.parse(fs.readFileSync(filePath, 'utf8'));

          // Handle error test cases
          if (data.expected_error) {
            let thrownError: AssumptionError | null = null;
            try {
              const sx = sampleFromTestData(data.input.x);
              const sy = sampleFromTestData(data.input.y);
              const thresholds = data.input.thresholds.map(
                (t: { metric: string; value: number; misrate: number }) =>
                  new Threshold(t.metric as Metric, new Measurement(t.value), t.misrate),
              );
              compare2(sx, sy, thresholds, data.input.seed);
            } catch (e) {
              if (e instanceof AssumptionError) {
                thrownError = e;
              } else {
                throw e;
              }
            }
            // Sample-only path: construction reports 'y' validity as 'x'.
            expectError(thrownError, data.expected_error, true);
            return;
          }

          const sx = Sample.of(data.input.x);
          const sy = Sample.of(data.input.y);
          const thresholds = data.input.thresholds.map(
            (t: { metric: string; value: number; misrate: number }) =>
              new Threshold(t.metric as Metric, new Measurement(t.value), t.misrate),
          );
          const results = compare2(sx, sy, thresholds, data.input.seed);

          expect(results.length).toBe(data.output.projections.length);
          for (let i = 0; i < results.length; i++) {
            const subject = `compare2/${fileName} projection ${i}`;
            const projection = data.output.projections[i];
            if (data.input.thresholds[i].metric === Metric.Ratio) {
              expect(results[i].estimate.value).toBeCloseTo(projection.estimate, 9);
              expect(results[i].bounds.lower).toBeCloseTo(projection.lower, 9);
              expect(results[i].bounds.upper).toBeCloseTo(projection.upper, 9);
            } else {
              expectBitwise(`${subject}.estimate`, results[i].estimate.value, projection.estimate);
              expectBitwiseBounds(subject, results[i].bounds, projection);
            }
            expect(results[i].verdict).toBe(projection.verdict);
          }
        });
      });
    }
  });
});
