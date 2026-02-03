import * as fs from 'fs';
import * as path from 'path';
import {
  center,
  spread,
  relSpread,
  shift,
  ratio,
  avgSpread,
  disparity,
  shiftBounds,
  ratioBounds,
} from '../src/estimators';
import { AssumptionError } from '../src/assumptions';
import { pairwiseMargin } from '../src/pairwiseMargin';
import { Rng } from '../src/rng';
import { Additive, Exp, Multiplic, Power, Uniform } from '../src/distributions';

/**
 * Reference tests comparing against expected values from JSON files
 */

describe('Reference Tests', () => {
  const testDataPath = path.join(__dirname, '../../tests');

  // Map estimator names to functions
  type EstimatorFunction = (x: number[], y?: number[]) => number;

  const estimators: Record<string, EstimatorFunction> = {
    center,
    spread,
    'rel-spread': relSpread,
    shift: (x: number[], y?: number[]) => shift(x, y!),
    ratio: (x: number[], y?: number[]) => ratio(x, y!),
    'avg-spread': (x: number[], y?: number[]) => avgSpread(x, y!),
    disparity: (x: number[], y?: number[]) => disparity(x, y!),
  };

  // Get all test directories
  const testDirs = fs
    .readdirSync(testDataPath)
    .filter((dir) => fs.statSync(path.join(testDataPath, dir)).isDirectory())
    .filter((dir) => estimators[dir] !== undefined);

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
          const estimatorFunc = estimators[dirName];

          if (!estimatorFunc) {
            throw new Error(`Unknown estimator: ${dirName}`);
          }

          // Determine if this is a one-sample or two-sample test
          try {
            if (data.input && typeof data.input === 'object' && 'x' in data.input) {
              if ('y' in data.input) {
                // Two-sample test
                const result = estimatorFunc(data.input.x, data.input.y);
                expect(result).toBeCloseTo(data.output, 9);
              } else {
                // One-sample test with x property
                const result = estimatorFunc(data.input.x);
                expect(result).toBeCloseTo(data.output, 9);
              }
            } else if (Array.isArray(data.input)) {
              // One-sample test with direct array
              const result = estimatorFunc(data.input);
              expect(result).toBeCloseTo(data.output, 9);
            } else {
              throw new Error(`Invalid test data format in ${filePath}`);
            }
          } catch (e) {
            // Skip cases that violate assumptions - tested separately
            if (e instanceof AssumptionError) {
              return;
            }
            throw e;
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
          const result = shiftBounds(data.input.x, data.input.y, data.input.misrate);
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
          const result = ratioBounds(data.input.x, data.input.y, data.input.misrate);
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
          const actual = Array.from({ length: data.input.count }, () => rng.uniform());

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
          const actual = Array.from({ length: data.input.count }, () => rng.uniform());

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
            rng.uniformRange(data.input.min, data.input.max),
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
      expect(() => rng.sample([1, 2, 3], -1)).toThrow('k must be non-negative');
    });
  });
});
