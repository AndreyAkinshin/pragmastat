/**
 * Assumption violation conformance tests.
 *
 * These tests verify that assumption violations are reported correctly and
 * consistently across all languages. The test data is loaded from shared
 * JSON files in tests/assumptions/.
 */

import * as fs from 'fs';
import * as path from 'path';
import { center, ratio, relSpread, spread, shift, avgSpread, disparity } from '../src/estimators';
import { AssumptionError } from '../src/assumptions';

interface ExpectedViolation {
  id: string;
}

interface TestInputs {
  x?: (number | string)[];
  y?: (number | string)[];
}

interface AssumptionTestCase {
  name: string;
  function: string;
  inputs: TestInputs;
  expected_violation: ExpectedViolation;
}

interface AssumptionTestSuite {
  suite: string;
  description: string;
  cases: AssumptionTestCase[];
}

interface SuiteEntry {
  name: string;
  file: string;
  description: string;
}

interface Manifest {
  name: string;
  description: string;
  suites: SuiteEntry[];
}

/**
 * Parses a JSON value into a number, handling special values.
 */
function parseValue(v: number | string): number {
  if (typeof v === 'number') {
    return v;
  }
  if (v === 'NaN') {
    return NaN;
  }
  if (v === 'Infinity') {
    return Infinity;
  }
  if (v === '-Infinity') {
    return -Infinity;
  }
  throw new Error(`Unknown string value: ${v}`);
}

/**
 * Parses an array of JSON values into a number array.
 */
function parseArray(arr: (number | string)[] | undefined): number[] {
  if (arr === undefined) {
    return [];
  }
  return arr.map(parseValue);
}

type EstimatorFn = (x: number[], y: number[]) => number;

/**
 * Function dispatch: maps function names to actual implementations.
 */
const FUNCTION_MAP: Record<string, EstimatorFn> = {
  Center: (x, _y) => center(x),
  Ratio: (x, y) => ratio(x, y),
  RelSpread: (x, _y) => relSpread(x),
  Spread: (x, _y) => spread(x),
  Shift: (x, y) => shift(x, y),
  AvgSpread: (x, y) => avgSpread(x, y),
  Disparity: (x, y) => disparity(x, y),
};

/**
 * Loads all assumption test cases from the shared test data.
 */
function loadAssumptionTestCases(): { suiteName: string; testCase: AssumptionTestCase }[] {
  const repoRoot = path.join(__dirname, '../..');
  const assumptionsDir = path.join(repoRoot, 'tests', 'assumptions');

  const manifestPath = path.join(assumptionsDir, 'manifest.json');
  const manifest: Manifest = JSON.parse(fs.readFileSync(manifestPath, 'utf-8'));

  const testCases: { suiteName: string; testCase: AssumptionTestCase }[] = [];

  for (const suiteEntry of manifest.suites) {
    const suitePath = path.join(assumptionsDir, suiteEntry.file);
    const suite: AssumptionTestSuite = JSON.parse(fs.readFileSync(suitePath, 'utf-8'));

    for (const testCase of suite.cases) {
      testCases.push({ suiteName: suite.suite, testCase });
    }
  }

  return testCases;
}

describe('Assumption Violation Tests', () => {
  const testCases = loadAssumptionTestCases();

  test.each(testCases)('$suiteName/$testCase.name', ({ testCase }) => {
    const x = parseArray(testCase.inputs.x);
    const y = parseArray(testCase.inputs.y);

    const func = FUNCTION_MAP[testCase.function];
    if (!func) {
      throw new Error(`Unknown function: ${testCase.function}`);
    }

    const expectedId = testCase.expected_violation.id;

    expect(() => func(x, y)).toThrow(AssumptionError);

    try {
      func(x, y);
    } catch (e) {
      if (e instanceof AssumptionError) {
        expect(e.violation.id).toBe(expectedId);
      } else {
        throw e;
      }
    }
  });
});
