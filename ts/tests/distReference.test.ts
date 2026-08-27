/**
 * LinearDistModel and NeighborDistModel against their shared dist-linear/dist-neighbor
 * fixtures.
 *
 * Unlike the five parametric families, this suite's oracle is not one of the seven ports: it is
 * an independent binary64 + exact-Fraction reimplementation (tests/oracles/dist_reference.py),
 * written directly from the normative algorithm so a bug shared by every language cannot pass
 * itself. Every case is compared by binary64 payload, matching the suite's declared exact class.
 *
 * This suite covers the raw-vector path only (a plain number array plus an optional
 * assumeSorted flag) -- the measured Sample-wrapped path has its own coverage elsewhere.
 */

import * as fs from 'fs';
import * as path from 'path';
import { LinearDistModel, NeighborDistModel } from '../src/distributions';
import { Rng } from '../src/rng';
import { AssumptionError } from '../src/assumptions';
import { expectBitwise } from './bitwise';

const testDataPath = path.join(__dirname, '../../tests');

type DistModelName = 'linear' | 'neighbor';
type DistOp = 'quantile' | 'cumulative' | 'density';

// The surface every dist-* fixture query reaches, shared by both reconstructions so the loader
// below does not need to know which one it is holding.
interface DistQueryModel {
  quantile(cumulances: readonly number[]): number[];
  cumulative(values: readonly number[]): number[];
  density(values: readonly number[]): number[];
  samples(rng: Rng, count: number): number[];
}

/**
 * The raw JSON literal a fixture number can carry: JSON has no NaN or Infinity literal, so the
 * generator spells them as the strings "NaN"/"Infinity"/"-Infinity" and everything else arrives
 * as an ordinary number.
 */
type RawFloat = number | 'NaN' | 'Infinity' | '-Infinity';

function parseSpecialFloat(value: RawFloat): number {
  if (typeof value === 'number') return value;
  switch (value) {
    case 'NaN':
      return NaN;
    case 'Infinity':
      return Infinity;
    case '-Infinity':
      return -Infinity;
    default:
      throw new Error(`Unrecognized special float literal ${String(value)}`);
  }
}

function parseSpecialFloats(values: readonly RawFloat[]): number[] {
  return values.map(parseSpecialFloat);
}

interface DistQueryInput {
  x: RawFloat[];
  y: RawFloat[];
  assume_sorted?: boolean;
}

interface DistQueryCase {
  input: DistQueryInput;
  output?: RawFloat[];
  expected_error?: { id: string; subject: string };
}

interface DistSampleInput {
  x: RawFloat[];
  seed: string;
  count: number;
}

interface DistSampleCase {
  input: DistSampleInput;
  output: RawFloat[];
}

interface DistLandscapeInput {
  x: RawFloat[];
}

interface DistLandscapeOutput {
  k0: number;
  radius: number;
  boundaries: RawFloat[];
  cumulative: RawFloat[];
  masses: RawFloat[];
  densities: RawFloat[];
  sources: number[];
}

interface DistLandscapeCase {
  input: DistLandscapeInput;
  output: DistLandscapeOutput;
}

// Builds the reconstruction the suite names, rather than letting the fixture decide: the model
// under test is always the raw-vector constructor, never the Sample-wrapped one.
function buildDistModel(model: DistModelName, x: number[], assumeSorted: boolean): DistQueryModel {
  switch (model) {
    case 'linear':
      return new LinearDistModel(x, assumeSorted);
    case 'neighbor':
      return new NeighborDistModel(x, assumeSorted);
    default:
      throw new Error(`Unknown dist model ${model as string}`);
  }
}

function runDistOp(d: DistQueryModel, op: DistOp, y: number[]): number[] {
  switch (op) {
    case 'quantile':
      return d.quantile(y);
    case 'cumulative':
      return d.cumulative(y);
    case 'density':
      return d.density(y);
    default:
      throw new Error(`Unknown dist operation ${op as string}`);
  }
}

function listFixtures(suite: string): string[] {
  const dirPath = path.join(testDataPath, suite);
  return fs
    .readdirSync(dirPath)
    .filter((f) => f.endsWith('.json'))
    .sort();
}

function readFixture<T>(suite: string, fileName: string): T {
  return JSON.parse(fs.readFileSync(path.join(testDataPath, suite, fileName), 'utf8')) as T;
}

/**
 * Runs the operation and asserts against `expected_error` when present, or against `output`
 * otherwise. The presence check comes first and unconditionally: trying the operation, catching
 * whatever it throws, and only then checking for `expected_error` would let a real bug -- an
 * exception on a case that expects success -- pass silently, since nothing would be there to
 * compare it against.
 */
function checkQueryCase(model: DistModelName, op: DistOp, data: DistQueryCase): void {
  const x = parseSpecialFloats(data.input.x);
  const y = parseSpecialFloats(data.input.y);
  const assumeSorted = data.input.assume_sorted ?? false;

  if (data.expected_error) {
    let thrown: AssumptionError | null = null;
    try {
      const d = buildDistModel(model, x, assumeSorted);
      runDistOp(d, op, y);
    } catch (e) {
      if (e instanceof AssumptionError) thrown = e;
      else throw e;
    }
    expect(thrown).not.toBeNull();
    expect(thrown!.violation!.id).toBe(data.expected_error.id);
    expect(thrown!.violation!.subject).toBe(data.expected_error.subject);
    return;
  }

  const d = buildDistModel(model, x, assumeSorted);
  const actual = runDistOp(d, op, y);
  const expected = parseSpecialFloats(data.output!);
  expect(actual.length).toBe(expected.length);
  for (let i = 0; i < actual.length; i++) {
    expectBitwise(`${model}/${op} output[${i}]`, actual[i], expected[i]);
  }
}

function checkDistQuerySuite(model: DistModelName, op: DistOp): void {
  const suite = `dist-${model}/${op}`;
  const files = listFixtures(suite);

  it('should have test files', () => {
    expect(files.length).toBeGreaterThan(0);
  });

  files.forEach((fileName) => {
    it(`should pass ${fileName.replace('.json', '')}`, () => {
      checkQueryCase(model, op, readFixture<DistQueryCase>(suite, fileName));
    });
  });
}

// No sample fixture declares expected_error: an invalid x is already covered by the query
// suites above, and drawing from a built model cannot fail on its own.
function checkDistSampleSuite(model: DistModelName): void {
  const suite = `dist-${model}/sample`;
  const files = listFixtures(suite);

  it('should have test files', () => {
    expect(files.length).toBeGreaterThan(0);
  });

  files.forEach((fileName) => {
    it(`should pass ${fileName.replace('.json', '')}`, () => {
      const data = readFixture<DistSampleCase>(suite, fileName);
      const x = parseSpecialFloats(data.input.x);
      const d = buildDistModel(model, x, false);
      const rng = new Rng(data.input.seed);
      const actual = d.samples(rng, data.input.count);
      const expected = parseSpecialFloats(data.output);
      expect(actual.length).toBe(expected.length);
      for (let i = 0; i < actual.length; i++) {
        expectBitwise(`${model}/sample output[${i}]`, actual[i], expected[i]);
      }
    });
  });
}

// Every case here builds successfully by construction: the generator only emits a landscape for
// a sample that already passed the query suites' validity checks.
function checkDistNeighborLandscapeSuite(): void {
  const suite = 'dist-neighbor/landscape';
  const files = listFixtures(suite);

  it('should have test files', () => {
    expect(files.length).toBeGreaterThan(0);
  });

  files.forEach((fileName) => {
    it(`should pass ${fileName.replace('.json', '')}`, () => {
      const data = readFixture<DistLandscapeCase>(suite, fileName);
      const x = parseSpecialFloats(data.input.x);
      const d = new NeighborDistModel(x, false);
      const landscape = d.getLandscape();
      const expected = data.output;

      expect(landscape.scaleIndex).toBe(expected.k0);
      expect(landscape.radius).toBe(expected.radius);

      const checkSeq = (name: string, actual: readonly number[], rawExpected: RawFloat[]): void => {
        const exp = parseSpecialFloats(rawExpected);
        expect(actual.length).toBe(exp.length);
        for (let i = 0; i < actual.length; i++) {
          expectBitwise(`${name}[${i}]`, actual[i], exp[i]);
        }
      };
      checkSeq('boundaries', landscape.boundaries, expected.boundaries);
      checkSeq('cumulative', landscape.cumulative, expected.cumulative);
      checkSeq('masses', landscape.masses, expected.masses);
      checkSeq('densities', landscape.densities, expected.densities);

      expect(landscape.sources.length).toBe(expected.sources.length);
      for (let i = 0; i < landscape.sources.length; i++) {
        expect(landscape.sources[i]).toBe(expected.sources[i]);
      }
    });
  });
}

describe('dist-linear reference', () => {
  (['quantile', 'cumulative', 'density'] as DistOp[]).forEach((op) => {
    describe(op, () => checkDistQuerySuite('linear', op));
  });
  describe('sample', () => checkDistSampleSuite('linear'));
});

describe('dist-neighbor reference', () => {
  (['quantile', 'cumulative', 'density'] as DistOp[]).forEach((op) => {
    describe(op, () => checkDistQuerySuite('neighbor', op));
  });
  describe('sample', () => checkDistSampleSuite('neighbor'));
  describe('landscape', checkDistNeighborLandscapeSuite);
});
