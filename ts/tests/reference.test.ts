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
  assertValue: (result: R, expected: TestData['output']) => void,
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
      assertValue(result, data.output);
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

  const assertValue = (result: ValueResult, expected: number): void => {
    expect(result.value).toBeCloseTo(expected, 9);
  };

  // ---------------------------------------------------------------------------
  // Bounds-estimator dual-path infrastructure
  // ---------------------------------------------------------------------------
  interface BoundsResult {
    lower: number;
    upper: number;
  }

  const assertBounds = (result: BoundsResult, expected: { lower: number; upper: number }): void => {
    expect(result.lower).toBeCloseTo(expected.lower, 9);
    expect(result.upper).toBeCloseTo(expected.upper, 9);
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

      testFiles.forEach((fileName) => {
        const filePath = path.join(dirPath, fileName);
        const testName = fileName.replace('.json', '');
        it(`should pass ${testName}`, () => {
          const data: TestData = JSON.parse(fs.readFileSync(filePath, 'utf8'));
          runDualPath(data, entries, assertBounds);
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

      testFiles.forEach((fileName) => {
        const filePath = path.join(dirPath, fileName);
        const testName = fileName.replace('.json', '');

        it(`should pass ${testName}`, () => {
          const data: TestData = JSON.parse(fs.readFileSync(filePath, 'utf8'));
          runDualPath(data, entries, assertValue);
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

          for (let i = 0; i < actual.length; i++) {
            expect(actual[i]).toBeCloseTo(data.output[i], 15);
          }
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

          expect(actual).toEqual(data.output);
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

          for (let i = 0; i < actual.length; i++) {
            expect(actual[i]).toBeCloseTo(data.output[i], 15);
          }
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

          for (let i = 0; i < actual.length; i++) {
            expect(actual[i]).toBeCloseTo(data.output[i], 12);
          }
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
          const actual = rng.shuffle(data.input.x);

          for (let i = 0; i < actual.length; i++) {
            expect(actual[i]).toBeCloseTo(data.output[i], 15);
          }
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
          const actual = rng.sample(data.input.x, data.input.k);

          for (let i = 0; i < actual.length; i++) {
            expect(actual[i]).toBeCloseTo(data.output[i], 15);
          }
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
          const actual = rng.resample(data.input.x, data.input.k);

          for (let i = 0; i < actual.length; i++) {
            expect(actual[i]).toBeCloseTo(data.output[i], 15);
          }
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

          for (let i = 0; i < actual.length; i++) {
            expect(actual[i]).toBeCloseTo(data.output[i], 12);
          }
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
          expect(result.lower).toBeCloseTo(data.output.lower, 9);
          expect(result.upper).toBeCloseTo(data.output.upper, 9);
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
              expect(result.value).toBeCloseTo(data.output.value, 9);
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
              expect(result.value).toBeCloseTo(data.output.value, 9);
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
            expect(results[i].estimate.value).toBeCloseTo(data.output.projections[i].estimate, 9);
            expect(results[i].bounds.lower).toBeCloseTo(data.output.projections[i].lower, 9);
            expect(results[i].bounds.upper).toBeCloseTo(data.output.projections[i].upper, 9);
            expect(results[i].verdict).toBe(data.output.projections[i].verdict);
          }
        });
      });
    }
  });

  // Compare2 tests
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
            expect(results[i].estimate.value).toBeCloseTo(data.output.projections[i].estimate, 9);
            expect(results[i].bounds.lower).toBeCloseTo(data.output.projections[i].lower, 9);
            expect(results[i].bounds.upper).toBeCloseTo(data.output.projections[i].upper, 9);
            expect(results[i].verdict).toBe(data.output.projections[i].verdict);
          }
        });
      });
    }
  });
});
