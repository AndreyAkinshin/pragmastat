import * as fs from 'fs';
import * as path from 'path';
import {
  center,
  spread,
  relSpread,
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

/**
 * Reference tests comparing against expected values from JSON files
 */

/**
 * Creates a Sample from raw values, remapping the AssumptionError subject
 * from the default 'x' to the given subject (e.g. 'y').
 */
function sampleFromTestData(values: number[], subject: 'x' | 'y'): Sample {
  try {
    return Sample.of(values);
  } catch (e) {
    if (e instanceof AssumptionError && subject !== 'x') {
      throw new AssumptionError({ id: e.violation!.id, subject });
    }
    throw e;
  }
}

describe('Reference Tests', () => {
  const testDataPath = path.join(__dirname, '../../tests');

  // Map estimator names to functions that work with raw test data
  type OneSampleEstimator = (x: Sample) => Measurement;
  type TwoSampleEstimator = (x: Sample, y: Sample) => Measurement;

  const oneSampleEstimators: Record<string, OneSampleEstimator> = {
    center,
    spread,
    'rel-spread': relSpread,
  };

  const twoSampleEstimators: Record<string, TwoSampleEstimator> = {
    shift,
    ratio,
    'avg-spread': avgSpread,
    disparity,
  };

  const allEstimatorNames = new Set([
    ...Object.keys(oneSampleEstimators),
    ...Object.keys(twoSampleEstimators),
  ]);

  // Get all test directories
  const testDirs = fs
    .readdirSync(testDataPath)
    .filter((dir) => fs.statSync(path.join(testDataPath, dir)).isDirectory())
    .filter((dir) => allEstimatorNames.has(dir));

  testDirs.forEach((dirName) => {
    describe(dirName, () => {
      const dirPath = path.join(testDataPath, dirName);
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
              if (data.input && typeof data.input === 'object' && 'x' in data.input) {
                const sx = sampleFromTestData(data.input.x, 'x');
                if ('y' in data.input) {
                  const sy = sampleFromTestData(data.input.y, 'y');
                  const fn = twoSampleEstimators[dirName];
                  fn(sx, sy);
                } else {
                  const fn = oneSampleEstimators[dirName];
                  fn(sx);
                }
              } else if (Array.isArray(data.input)) {
                const sx = sampleFromTestData(data.input, 'x');
                const fn = oneSampleEstimators[dirName];
                fn(sx);
              }
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

          // Determine if this is a one-sample or two-sample test
          if (data.input && typeof data.input === 'object' && 'x' in data.input) {
            if ('y' in data.input) {
              // Two-sample test
              const sx = Sample.of(data.input.x);
              const sy = Sample.of(data.input.y);
              const fn = twoSampleEstimators[dirName];
              const result = fn(sx, sy);
              expect(result.value).toBeCloseTo(data.output, 9);
            } else {
              // One-sample test with x property
              const sx = Sample.of(data.input.x);
              const fn = oneSampleEstimators[dirName];
              const result = fn(sx);
              expect(result.value).toBeCloseTo(data.output, 9);
            }
          } else if (Array.isArray(data.input)) {
            // One-sample test with direct array
            const sx = Sample.of(data.input);
            const fn = oneSampleEstimators[dirName];
            const result = fn(sx);
            expect(result.value).toBeCloseTo(data.output, 9);
          } else {
            throw new Error(`Invalid test data format in ${filePath}`);
          }
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

  // ShiftBounds tests
  describe('shift-bounds', () => {
    const dirPath = path.join(testDataPath, 'shift-bounds');
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
              const sx = sampleFromTestData(data.input.x, 'x');
              const sy = sampleFromTestData(data.input.y, 'y');
              shiftBounds(sx, sy, data.input.misrate);
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

          const sx = Sample.of(data.input.x);
          const sy = Sample.of(data.input.y);
          const result = shiftBounds(sx, sy, data.input.misrate);
          expect(result.lower).toBeCloseTo(data.output.lower, 9);
          expect(result.upper).toBeCloseTo(data.output.upper, 9);
        });
      });
    }
  });

  // RatioBounds tests
  describe('ratio-bounds', () => {
    const dirPath = path.join(testDataPath, 'ratio-bounds');
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
              const sx = sampleFromTestData(data.input.x, 'x');
              const sy = sampleFromTestData(data.input.y, 'y');
              ratioBounds(sx, sy, data.input.misrate);
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

          const sx = Sample.of(data.input.x);
          const sy = Sample.of(data.input.y);
          const result = ratioBounds(sx, sy, data.input.misrate);
          expect(result.lower).toBeCloseTo(data.output.lower, 9);
          expect(result.upper).toBeCloseTo(data.output.upper, 9);
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

  // CenterBounds tests
  describe('center-bounds', () => {
    const dirPath = path.join(testDataPath, 'center-bounds');
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
              const sx = sampleFromTestData(data.input.x, 'x');
              centerBounds(sx, data.input.misrate);
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

          const sx = Sample.of(data.input.x);
          const result = centerBounds(sx, data.input.misrate);
          expect(result.lower).toBeCloseTo(data.output.lower, 9);
          expect(result.upper).toBeCloseTo(data.output.upper, 9);
        });
      });
    }
  });

  // SpreadBounds tests
  describe('spread-bounds', () => {
    const dirPath = path.join(testDataPath, 'spread-bounds');
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

          if (data.expected_error) {
            let thrownError: AssumptionError | null = null;
            try {
              const sx = sampleFromTestData(data.input.x, 'x');
              spreadBounds(sx, data.input.misrate, data.input.seed);
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

          const sx = Sample.of(data.input.x);
          const result = spreadBounds(sx, data.input.misrate, data.input.seed);
          expect(result.lower).toBeCloseTo(data.output.lower, 9);
          expect(result.upper).toBeCloseTo(data.output.upper, 9);
        });
      });
    }
  });

  // DisparityBounds tests
  describe('disparity-bounds', () => {
    const dirPath = path.join(testDataPath, 'disparity-bounds');
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
              const sx = sampleFromTestData(data.input.x, 'x');
              const sy = sampleFromTestData(data.input.y, 'y');
              disparityBounds(sx, sy, data.input.misrate, data.input.seed);
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

          const sx = Sample.of(data.input.x);
          const sy = Sample.of(data.input.y);
          const result = disparityBounds(sx, sy, data.input.misrate, data.input.seed);
          expect(result.lower).toBeCloseTo(data.output.lower, 9);
          expect(result.upper).toBeCloseTo(data.output.upper, 9);
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
              const sx = sampleFromTestData(data.input.x, 'x');
              const sy = sampleFromTestData(data.input.y, 'y');
              avgSpreadBounds(sx, sy, data.input.misrate, data.input.seed);
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
});
