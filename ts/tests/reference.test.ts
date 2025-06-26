import * as fs from 'fs';
import * as path from 'path';
import {
  center,
  spread,
  volatility,
  precision,
  medShift,
  medRatio,
  medSpread,
  medDisparity,
} from '../src/estimators';

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
    volatility,
    precision,
    'med-shift': (x: number[], y?: number[]) => medShift(x, y!),
    'med-ratio': (x: number[], y?: number[]) => medRatio(x, y!),
    'med-spread': (x: number[], y?: number[]) => medSpread(x, y!),
    'med-disparity': (x: number[], y?: number[]) => medDisparity(x, y!),
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
        });
      });
    });
  });
});
