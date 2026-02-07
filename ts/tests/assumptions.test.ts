/**
 * Assumption violation conformance tests.
 *
 * These tests verify that assumption violations are reported correctly and
 * consistently across all languages. The test data is loaded from shared
 * JSON files in tests/assumptions/.
 */

import * as fs from 'fs';
import * as path from 'path';
import {
  center,
  ratio,
  relSpread,
  spread,
  shift,
  avgSpread,
  disparity,
  centerBounds,
} from '../src/estimators';
import { signedRankMargin } from '../src/signedRankMargin';
import { AssumptionError } from '../src/assumptions';

interface ExpectedViolation {
  id: string;
}

interface TestInputs {
  x?: (number | string)[];
  y?: (number | string)[];
  misrate?: number | string;
  n?: number;
  seed?: string;
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

/**
 * Dispatches to the appropriate estimator function.
 */
function callFunction(funcName: string, inputs: TestInputs): void {
  const x = parseArray(inputs.x);
  const y = parseArray(inputs.y);

  switch (funcName) {
    case 'Center':
      center(x);
      break;
    case 'Ratio':
      ratio(x, y);
      break;
    case 'RelSpread':
      relSpread(x);
      break;
    case 'Spread':
      spread(x);
      break;
    case 'Shift':
      shift(x, y);
      break;
    case 'AvgSpread':
      avgSpread(x, y);
      break;
    case 'Disparity':
      disparity(x, y);
      break;
    case 'CenterBounds':
      centerBounds(x, parseValue(inputs.misrate!));
      break;
    case 'SignedRankMargin':
      signedRankMargin(inputs.n!, parseValue(inputs.misrate!));
      break;
    default:
      throw new Error(`Unknown function: ${funcName}`);
  }
}

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
    const expectedId = testCase.expected_violation.id;

    let thrownError: AssumptionError | null = null;
    try {
      callFunction(testCase.function, testCase.inputs);
    } catch (e) {
      if (e instanceof AssumptionError) {
        thrownError = e;
      } else {
        throw e;
      }
    }
    expect(thrownError).not.toBeNull();
    expect(thrownError!.violation.id).toBe(expectedId);
  });
});
