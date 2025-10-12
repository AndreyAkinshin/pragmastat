import { fastShift } from '../src/fastShift';

const TOLERANCE = 1e-9;

function generateRandomValues(n: number, seed: number): number[] {
  const values: number[] = [];
  let state = seed;
  for (let i = 0; i < n; i++) {
    state = (state * 1664525 + 1013904223) % 4294967296;
    values.push((state / 4294967296 - 0.5) * 10);
  }
  return values;
}

function naiveQuantiles(x: number[], y: number[], p: number[]): number[] {
  const diffs: number[] = [];
  for (const xi of x) {
    for (const yj of y) {
      diffs.push(xi - yj);
    }
  }
  diffs.sort((a, b) => a - b);

  const result: number[] = [];
  for (const pk of p) {
    const n = diffs.length;
    const h = 1.0 + (n - 1) * pk;
    const lo = Math.max(1, Math.min(n, Math.floor(h)));
    const hi = Math.max(1, Math.min(n, Math.ceil(h)));
    const gamma = h - lo;

    const a = diffs[lo - 1];
    const b = diffs[hi - 1];

    result.push(gamma === 0.0 ? a : (1.0 - gamma) * a + gamma * b);
  }
  return result;
}

describe('fastShift', () => {
  describe('Small arrays - matches naive', () => {
    it('should match naive for various small array sizes', () => {
      const seed = 1729;
      let currentSeed = seed;

      for (let m = 1; m <= 20; m++) {
        for (let n = 1; n <= 20; n++) {
          for (let iteration = 0; iteration < 2; iteration++) {
            const x = generateRandomValues(m, currentSeed++);
            const y = generateRandomValues(n, currentSeed++);
            const p = [0.0, 0.25, 0.5, 0.75, 1.0];

            const actual = fastShift(x, y, p);
            const expected = naiveQuantiles(x, y, p);

            expect(actual.length).toBe(expected.length);
            for (let i = 0; i < expected.length; i++) {
              expect(actual[i]).toBeCloseTo(expected[i], 9);
            }
          }
        }
      }
    });
  });

  describe('Medium arrays - matches naive', () => {
    it('should match naive for medium-sized arrays', () => {
      const seed = 42;
      let currentSeed = seed;

      for (let size = 20; size <= 100; size += 20) {
        for (let iteration = 0; iteration < 2; iteration++) {
          const x = generateRandomValues(size, currentSeed++);
          const y = generateRandomValues(Math.floor(size / 2), currentSeed++);
          const p = [0.1, 0.5, 0.9];

          const actual = fastShift(x, y, p);
          const expected = naiveQuantiles(x, y, p);

          expect(actual.length).toBe(expected.length);
          for (let i = 0; i < expected.length; i++) {
            expect(actual[i]).toBeCloseTo(expected[i], 9);
          }
        }
      }
    });
  });

  describe('Different distributions - all quantiles', () => {
    it('should work correctly with different distributions', () => {
      const distributions = [
        { mean: 0, scale: 1 },
        { mean: 5, scale: 2 },
        { mean: -10, scale: 1 },
        { mean: 0, scale: 0.1 },
      ];

      const probabilities = [0.0, 0.05, 0.1, 0.25, 0.5, 0.75, 0.9, 0.95, 1.0];

      let seed = 2024;
      for (const dist of distributions) {
        const x = generateRandomValues(15, seed++).map((v) => v * dist.scale + dist.mean);
        const y = generateRandomValues(10, seed++).map((v) => v * dist.scale + dist.mean);

        const actual = fastShift(x, y, probabilities);
        const expected = naiveQuantiles(x, y, probabilities);

        expect(actual.length).toBe(expected.length);
        for (let i = 0; i < expected.length; i++) {
          expect(actual[i]).toBeCloseTo(expected[i], 9);
        }
      }
    });
  });

  describe('Unsorted input - matches sorted', () => {
    it('should handle unsorted input correctly', () => {
      const seed = 999;
      let currentSeed = seed;

      for (let trial = 0; trial < 20; trial++) {
        const xRaw = generateRandomValues(20, currentSeed++);
        const yRaw = generateRandomValues(15, currentSeed++);
        const p = [0.25, 0.5, 0.75];

        const xSorted = [...xRaw].sort((a, b) => a - b);
        const ySorted = [...yRaw].sort((a, b) => a - b);

        const xShuffled = [...xRaw].sort(() => Math.random() - 0.5);
        const yShuffled = [...yRaw].sort(() => Math.random() - 0.5);

        const resultUnsorted = fastShift(xShuffled, yShuffled, p, false);
        const resultSorted = fastShift(xSorted, ySorted, p, true);

        expect(resultUnsorted.length).toBe(resultSorted.length);
        for (let i = 0; i < resultSorted.length; i++) {
          expect(resultUnsorted[i]).toBeCloseTo(resultSorted[i], 9);
        }
      }
    });
  });

  describe('Single element - returns constant', () => {
    it('should return constant difference for single elements', () => {
      const seed = 123;
      let currentSeed = seed;

      for (let trial = 0; trial < 10; trial++) {
        const x = generateRandomValues(1, currentSeed++);
        const y = generateRandomValues(1, currentSeed++);
        const p = [0.0, 0.25, 0.5, 0.75, 1.0];

        const result = fastShift(x, y, p);
        const expected = x[0] - y[0];

        for (const q of result) {
          expect(q).toBeCloseTo(expected, 9);
        }
      }
    });
  });

  describe('Identical arrays - median is zero', () => {
    it('should return zero median for identical arrays', () => {
      const seed = 456;

      for (let size = 1; size <= 30; size += 5) {
        const x = generateRandomValues(size, seed + size);
        const p = [0.5];

        const result = fastShift(x, x, p);

        expect(result[0]).toBeCloseTo(0.0, 9);
      }
    });
  });

  describe('Asymmetric sizes - correct results', () => {
    it('should handle asymmetric array sizes correctly', () => {
      const configs = [
        { m: 1, n: 100 },
        { m: 100, n: 1 },
        { m: 10, n: 50 },
        { m: 50, n: 10 },
      ];

      const seed = 789;
      let currentSeed = seed;

      for (const { m, n } of configs) {
        const x = generateRandomValues(m, currentSeed++);
        const y = generateRandomValues(n, currentSeed++);
        const p = [0.0, 0.5, 1.0];

        const actual = fastShift(x, y, p);
        const expected = naiveQuantiles(x, y, p);

        expect(actual.length).toBe(expected.length);
        for (let i = 0; i < expected.length; i++) {
          expect(actual[i]).toBeCloseTo(expected[i], 9);
        }
      }
    });
  });

  describe('Extreme quantiles - match min/max', () => {
    it('should match actual min and max for extreme quantiles', () => {
      const seed = 321;
      let currentSeed = seed;

      for (let trial = 0; trial < 10; trial++) {
        const x = generateRandomValues(10 + trial, currentSeed++);
        const y = generateRandomValues(8 + Math.floor(trial / 2), currentSeed++);
        const p = [0.0, 1.0];

        const result = fastShift(x, y, p);

        let min = Infinity;
        let max = -Infinity;
        for (const xi of x) {
          for (const yj of y) {
            const diff = xi - yj;
            if (diff < min) min = diff;
            if (diff > max) max = diff;
          }
        }

        expect(result[0]).toBeCloseTo(min, 9);
        expect(result[1]).toBeCloseTo(max, 9);
      }
    });
  });

  describe('Many probabilities - monotonic increasing', () => {
    it('should produce monotonically increasing quantiles', () => {
      const seed = 654;
      let currentSeed = seed;

      for (let trial = 0; trial < 10; trial++) {
        const x = generateRandomValues(25, currentSeed++);
        const y = generateRandomValues(20, currentSeed++);

        const p = Array.from({ length: 21 }, (_, i) => i / 20.0);

        const result = fastShift(x, y, p);

        for (let i = 1; i < result.length; i++) {
          expect(result[i]).toBeGreaterThanOrEqual(result[i - 1] - TOLERANCE);
        }
      }
    });
  });

  describe('Negative values - handled correctly', () => {
    it('should handle negative values correctly', () => {
      const seed = 111;

      for (let trial = 0; trial < 10; trial++) {
        const x = generateRandomValues(15, seed + trial).map((v) => v - 50);
        const y = generateRandomValues(12, seed + trial + 1000).map((v) => v - 50);
        const p = [0.25, 0.5, 0.75];

        const actual = fastShift(x, y, p);
        const expected = naiveQuantiles(x, y, p);

        for (let i = 0; i < expected.length; i++) {
          expect(actual[i]).toBeCloseTo(expected[i], 9);
        }
      }
    });
  });

  describe('Duplicate values - handled correctly', () => {
    it('should handle arrays with duplicate values', () => {
      const seed = 222;

      for (let trial = 0; trial < 5; trial++) {
        const x = generateRandomValues(12, seed + trial).map((v) => Math.round(v * 5) / 5.0);
        const y = generateRandomValues(10, seed + trial + 500).map((v) => Math.round(v * 5) / 5.0);
        const p = [0.0, 0.5, 1.0];

        const actual = fastShift(x, y, p);
        const expected = naiveQuantiles(x, y, p);

        for (let i = 0; i < expected.length; i++) {
          expect(actual[i]).toBeCloseTo(expected[i], 9);
        }
      }
    });
  });

  describe('Very small values - numerical stability', () => {
    it('should maintain numerical stability with very small values', () => {
      const seed = 333;

      for (let trial = 0; trial < 5; trial++) {
        const x = generateRandomValues(10, seed + trial).map((v) => v * 1e-8);
        const y = generateRandomValues(10, seed + trial + 200).map((v) => v * 1e-8);
        const p = [0.5];

        const result = fastShift(x, y, p);

        expect(result[0]).not.toBeNaN();
        expect(isFinite(result[0])).toBe(true);
      }
    });
  });

  describe('Large values - numerical stability', () => {
    it('should maintain numerical stability with large values', () => {
      const seed = 444;

      for (let trial = 0; trial < 5; trial++) {
        const x = generateRandomValues(10, seed + trial).map((v) => v * 1e6 + 1e6);
        const y = generateRandomValues(10, seed + trial + 300).map((v) => v * 1e5 + 1e6);
        const p = [0.5];

        const result = fastShift(x, y, p);

        expect(result[0]).not.toBeNaN();
        expect(isFinite(result[0])).toBe(true);
      }
    });
  });

  describe('Zero spread - all same', () => {
    it('should return constant when all elements are same', () => {
      const x = Array(10).fill(5.0);
      const y = Array(8).fill(2.0);
      const p = [0.0, 0.25, 0.5, 0.75, 1.0];

      const result = fastShift(x, y, p);

      for (const q of result) {
        expect(q).toBeCloseTo(3.0, 9);
      }
    });
  });

  describe('Large arrays - performance test', () => {
    it('should complete in reasonable time for 500x500 arrays', () => {
      const x = generateRandomValues(500, 1729);
      const y = generateRandomValues(500, 1730);
      const p = [0.5];

      const start = Date.now();
      const result = fastShift(x, y, p);
      const elapsed = Date.now() - start;

      expect(result.length).toBe(1);
      expect(elapsed).toBeLessThan(5000);
    });
  });

  describe('Very large arrays - performance test', () => {
    it('should complete in reasonable time for 1000x1000 arrays', () => {
      const x = generateRandomValues(1000, 9999);
      const y = generateRandomValues(1000, 10000);
      const p = [0.5];

      const start = Date.now();
      const result = fastShift(x, y, p);
      const elapsed = Date.now() - start;

      expect(result.length).toBe(1);
      expect(elapsed).toBeLessThan(10000);
    });
  });

  describe('Many quantiles - performance test', () => {
    it('should handle many quantiles efficiently', () => {
      const x = generateRandomValues(200, 7777);
      const y = generateRandomValues(200, 7778);
      const p = Array.from({ length: 21 }, (_, i) => i / 20.0);

      const start = Date.now();
      const result = fastShift(x, y, p);
      const elapsed = Date.now() - start;

      expect(result.length).toBe(21);
      expect(elapsed).toBeLessThan(5000);
    });
  });

  describe('Null inputs - throws exception', () => {
    it('should throw error for null inputs', () => {
      const x = [1, 2];
      const y = [3, 4];
      const p = [0.5];

      expect(() => fastShift(null as unknown as number[], y, p)).toThrow();
      expect(() => fastShift(x, null as unknown as number[], p)).toThrow();
      expect(() => fastShift(x, y, null as unknown as number[])).toThrow();
    });
  });

  describe('Empty arrays - throws exception', () => {
    it('should throw error for empty arrays', () => {
      const empty: number[] = [];
      const valid = [1, 2];
      const p = [0.5];

      expect(() => fastShift(empty, valid, p)).toThrow('x and y must be non-empty');
      expect(() => fastShift(valid, empty, p)).toThrow('x and y must be non-empty');
    });
  });

  describe('Invalid probabilities - throws exception', () => {
    it('should throw error for invalid probabilities', () => {
      const x = [1, 2];
      const y = [3, 4];

      expect(() => fastShift(x, y, [-0.1])).toThrow('Probabilities must be within [0, 1]');
      expect(() => fastShift(x, y, [1.1])).toThrow('Probabilities must be within [0, 1]');
      expect(() => fastShift(x, y, [NaN])).toThrow('Probabilities must be within [0, 1]');
    });
  });

  describe('NaN in data - throws exception', () => {
    it('should throw error for NaN in data', () => {
      const xWithNaN = [1, NaN];
      const yWithNaN = [3, NaN];
      const valid = [1, 2];
      const p = [0.5];

      expect(() => fastShift(xWithNaN, valid, p)).toThrow('NaN values found in x');
      expect(() => fastShift(valid, yWithNaN, p)).toThrow('NaN values found in y');
    });
  });

  describe('Empty probabilities - returns empty', () => {
    it('should return empty array for empty probabilities', () => {
      const x = [1, 2];
      const y = [3, 4];
      const p: number[] = [];

      const result = fastShift(x, y, p);

      expect(result.length).toBe(0);
    });
  });

  describe('Shift invariance - X shift', () => {
    it('should be invariant to shifts in X', () => {
      const seed = 555;

      for (let trial = 0; trial < 5; trial++) {
        const x = generateRandomValues(15, seed + trial);
        const y = generateRandomValues(12, seed + trial + 100);
        const p = [0.25, 0.5, 0.75];
        const shift = trial * 10;

        const result1 = fastShift(x, y, p);
        const xShifted = x.map((v) => v + shift);
        const result2 = fastShift(xShifted, y, p);

        for (let i = 0; i < result1.length; i++) {
          expect(result2[i]).toBeCloseTo(result1[i] + shift, 9);
        }
      }
    });
  });

  describe('Shift invariance - Y shift', () => {
    it('should be invariant to shifts in Y', () => {
      const seed = 666;

      for (let trial = 0; trial < 5; trial++) {
        const x = generateRandomValues(15, seed + trial);
        const y = generateRandomValues(12, seed + trial + 100);
        const p = [0.25, 0.5, 0.75];
        const shift = trial * 10;

        const result1 = fastShift(x, y, p);
        const yShifted = y.map((v) => v + shift);
        const result2 = fastShift(x, yShifted, p);

        for (let i = 0; i < result1.length; i++) {
          expect(result2[i]).toBeCloseTo(result1[i] - shift, 9);
        }
      }
    });
  });

  describe('Scale invariance', () => {
    it('should scale appropriately with data', () => {
      const seed = 777;

      for (let trial = 0; trial < 5; trial++) {
        const x = generateRandomValues(15, seed + trial);
        const y = generateRandomValues(12, seed + trial + 100);
        const p = [0.5];
        const scale = 2.0;

        const result1 = fastShift(x, y, p);
        const xScaled = x.map((v) => v * scale);
        const yScaled = y.map((v) => v * scale);
        const result2 = fastShift(xScaled, yScaled, p);

        for (let i = 0; i < result1.length; i++) {
          expect(result2[i]).toBeCloseTo(result1[i] * scale, 6);
        }
      }
    });
  });
});
