import { Sample } from '../src/sample';
import {
  center,
  spread,
  shift,
  ratio,
  disparity,
  centerBounds,
  spreadBounds,
  shiftBounds,
  ratioBounds,
  disparityBounds,
} from '../src/estimators';

describe('raw native-array API does not mutate the caller array', () => {
  it('leaves the input array byte-for-byte unchanged', () => {
    const a = [3, 1, 2, 5, 4, 8, 6, 7];
    const o = [...a];
    const b = [13, 11, 12, 15, 14, 18, 16, 17];

    // Point estimators (number[] overload).
    center(a);
    spread(a);
    shift(a, b);
    ratio(a, b);
    disparity(a, b);

    // Bounds estimators with a generous misrate valid for 8 elements.
    centerBounds(a, 0.3);
    spreadBounds(a, 0.3);
    shiftBounds(a, b, 0.3);
    ratioBounds(a, b, 0.3);
    disparityBounds(a, b, 0.3);

    expect(a).toEqual(o);
  });

  it('leaves SORTED input arrays unchanged under assumeSorted=true (aliased, no copy)', () => {
    // With assumeSorted=true the raw API skips the copying sort and hands the
    // caller's array straight to the kernels, so any in-place mutation there
    // would corrupt the caller's data.
    const a = [1, 2, 3, 5, 8, 13, 21, 34];
    const o = [...a];
    const b = [11, 12, 14, 15, 17, 18, 19, 20];
    const p = [...b];

    // Point estimators (number[] overload, assumeSorted=true).
    center(a, true);
    spread(a, true);
    shift(a, b, true);
    ratio(a, b, true);
    disparity(a, b, true);

    // Bounds estimators with a generous misrate valid for 8 elements.
    centerBounds(a, 0.3, true);
    spreadBounds(a, 0.3, undefined, true);
    shiftBounds(a, b, 0.3, true);
    ratioBounds(a, b, 0.3, true);
    disparityBounds(a, b, 0.3, undefined, true);

    expect(a).toEqual(o);
    expect(b).toEqual(p);
  });
});

describe('Sample immutability', () => {
  it('prevents mutating values through the public accessor', () => {
    const sample = Sample.of([1, 2, 100]);
    expect(() => {
      (sample.values as number[])[2] = 3;
    }).toThrow(TypeError);
  });

  it('prevents mutating weights through the public accessor', () => {
    const sample = Sample.weighted([1, 2], [1, 2]);
    expect(() => {
      (sample.weights as number[])[0] = 5;
    }).toThrow(TypeError);
  });
});
