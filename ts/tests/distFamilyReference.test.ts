/**
 * The five parametric families against their shared dist-* fixtures.
 *
 * Each family answers three questions about itself, and each answer is assembled out
 * of the toolkit's own exponential, logarithm, normal cumulative and normal quantile.
 * Nothing on these paths reaches a platform transcendental, so the seven ports owe
 * each other identical bits and the suites are declared exact in tests/manifest.json:
 * the comparison below is by binary64 payload. A tolerance would pass on exactly the
 * divergence these suites exist to catch.
 *
 * The sibling distributions suite stays approximate, because drawing a sample still
 * goes through the platform's log, exp, cos and pow. That is a separate change with
 * its own effect on every seeded stream.
 */

import * as fs from 'fs';
import * as path from 'path';
import { Additive, Distribution, Exp, Multiplic, Power, Uniform } from '../src/distributions';
import { Rng } from '../src/rng';
import { expectBitwise } from './bitwise';

interface FamilyData {
  input: {
    parameters: Record<string, number>;
    x: number[];
    p: number[];
  };
  output: { density: number[]; cumulative: number[]; quantile: number[] };
}

// How many values the five suites hold between them: for each family, a density and a
// cumulative at every x and a quantile at every p. Pinned rather than derived, because
// a loader that finds four suites out of five satisfies every other assertion here by
// having less to compare.
const DIST_FAMILY_VALUE_COUNT = 7800;

const testDataPath = path.join(__dirname, '../../tests');

/**
 * Builds the family the suite names, rather than letting the parameter names decide:
 * two families could come to share a spelling, and a fixture read into the wrong family
 * would still produce numbers and still be compared.
 */
function build(suite: string, parameters: Record<string, number>): Distribution {
  const need = (name: string): number => {
    const value = parameters[name];
    if (value === undefined) {
      throw new Error(`${suite}: fixture has no parameter named ${name}`);
    }
    return value;
  };
  switch (suite) {
    case 'dist-uniform':
      return new Uniform(need('min'), need('max'));
    case 'dist-exp':
      return new Exp(need('rate'));
    case 'dist-additive':
      return new Additive(need('mean'), need('stdDev'));
    case 'dist-multiplic':
      return new Multiplic(need('logMean'), need('logStdDev'));
    case 'dist-power':
      return new Power(need('min'), need('shape'));
    default:
      throw new Error(`Unknown suite ${suite}`);
  }
}

describe('dist family reference', () => {
  const suites = ['dist-uniform', 'dist-exp', 'dist-additive', 'dist-multiplic', 'dist-power'];
  let compared = 0;

  suites.forEach((suite) => {
    describe(suite, () => {
      const dirPath = path.join(testDataPath, suite);
      const testFiles = fs
        .readdirSync(dirPath)
        .filter((f) => f.endsWith('.json'))
        .sort();

      it('should have test files', () => {
        expect(testFiles.length).toBeGreaterThan(0);
      });

      testFiles.forEach((fileName) => {
        it(`should pass ${fileName.replace('.json', '')}`, () => {
          const data: FamilyData = JSON.parse(
            fs.readFileSync(path.join(dirPath, fileName), 'utf8'),
          );
          const { parameters, x, p } = data.input;
          // A file that parses into empty arrays passes every comparison below, which
          // is the one way this test could report success while checking nothing.
          expect(x.length).toBeGreaterThan(0);
          expect(p.length).toBeGreaterThan(0);
          expect(data.output.density.length).toBe(x.length);
          expect(data.output.cumulative.length).toBe(x.length);
          expect(data.output.quantile.length).toBe(p.length);

          const d = build(suite, parameters);
          for (let i = 0; i < x.length; i++) {
            expectBitwise(
              `${suite}/${fileName} densityAt[${i}]`,
              d.densityAt(x[i]),
              data.output.density[i],
            );
            expectBitwise(
              `${suite}/${fileName} cumulativeAt[${i}]`,
              d.cumulativeAt(x[i]),
              data.output.cumulative[i],
            );
          }
          for (let i = 0; i < p.length; i++) {
            expectBitwise(
              `${suite}/${fileName} quantileAt[${i}]`,
              d.quantileAt(p[i]),
              data.output.quantile[i],
            );
          }
          compared += 2 * x.length + p.length;
        });
      });
    });
  });

  it(`compares all ${DIST_FAMILY_VALUE_COUNT} values`, () => {
    expect(compared).toBe(DIST_FAMILY_VALUE_COUNT);
  });
});

/**
 * The arguments the fixtures cannot carry, because JSON writes neither a NaN nor an
 * infinity.
 *
 * Without this table the fixtures are the only check the trio has, and they sample the
 * interior of each support through the family's own quantile. That left two defects
 * alive: Uniform reported a perfectly ordinary density for NaN in six ports, since
 * neither half of `x < min || x >= max` holds for a NaN and control fell straight
 * through, and R raised an error from thirteen of the fifteen functions. Neither is
 * reachable from a stored fixture.
 */
describe('dist family special values', () => {
  const families: Array<[string, Distribution, number, number]> = [
    ['dist-uniform', new Uniform(0, 1), 0, 1],
    ['dist-exp', new Exp(1), 0, Infinity],
    ['dist-additive', new Additive(0, 1), -Infinity, Infinity],
    ['dist-multiplic', new Multiplic(0, 1), 0, Infinity],
    ['dist-power', new Power(1, 2), 1, Infinity],
  ];

  families.forEach(([name, d, lower, upper]) => {
    it(`${name} propagates NaN through all three`, () => {
      expect(Number.isNaN(d.densityAt(NaN))).toBe(true);
      expect(Number.isNaN(d.cumulativeAt(NaN))).toBe(true);
      expect(Number.isNaN(d.quantileAt(NaN))).toBe(true);
    });

    it(`${name} places the infinities outside its support`, () => {
      expect(d.densityAt(-Infinity)).toBe(0);
      expect(d.densityAt(Infinity)).toBe(0);
      expect(d.cumulativeAt(-Infinity)).toBe(0);
      expect(d.cumulativeAt(Infinity)).toBe(1);
    });

    it(`${name} answers the endpoints outside the unit interval`, () => {
      [0, -0.1, -Infinity].forEach((p) => expect(d.quantileAt(p)).toBe(lower));
      [1, 1.1, Infinity].forEach((p) => expect(d.quantileAt(p)).toBe(upper));
    });
  });
});

// Every family draws by inverting its own quantile.
//
// A fixture says two numbers agreed on the day it was written. This says they cannot disagree:
// the same seed drives a draw and a direct quantile call, and the two must land on the same bits
// for every family. It is what makes "a distribution is three functions" a statement about the
// code rather than about the documentation, and it would fail the moment a family grew a second
// sampling route.
describe('sampling is the quantile of a drawn cumulance', () => {
  // lifts is true for the two families whose quantileAt(0) is not a sample.
  const families: Array<[string, Distribution, boolean]> = [
    ['Uniform', new Uniform(-2.5, 7.5), false],
    ['Exp', new Exp(0.5), false],
    ['Additive', new Additive(1.0, 2.0), true],
    ['Multiplic', new Multiplic(-1.0, 0.5), true],
    ['Power', new Power(2.0, 3.0), false],
  ];

  families.forEach(([name, d, lifts]) => {
    it(`${name}`, () => {
      const drawing = new Rng('identity-seed');
      const inverting = new Rng('identity-seed');
      for (let i = 0; i < 1000; i++) {
        const drawn = d.sample(drawing);
        // The same cumulance the draw consumed, taken from a generator on the same seed.
        let u = inverting.uniformFloat();
        if (lifts && u === 0) {
          u = 5e-324;
        }
        const inverted = d.quantileAt(u);
        expectBitwise(`${name} draw ${i}`, drawn, inverted);
      }
    });
  });
});
