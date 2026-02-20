import {
  center,
  spread,
  relSpread,
  shift,
  ratio,
  _avgSpread as avgSpread,
  disparity,
} from '../src/estimators';
import { Rng } from '../src';

/**
 * Invariance tests for mathematical properties of estimators
 */

describe('Invariance Tests', () => {
  const seed = 1729;
  const sampleSizes = [2, 3, 4, 5, 6, 7, 8, 9, 10];

  function performTestOne(expr1: (x: number[]) => number, expr2: (x: number[]) => number): void {
    const rng = new Rng(seed);
    for (const n of sampleSizes) {
      const x = Array.from({ length: n }, () => rng.uniformFloat());
      const result1 = expr1(x);
      const result2 = expr2(x);
      expect(result1).toBeCloseTo(result2, 9);
    }
  }

  function performTestTwo(
    expr1: (x: number[], y: number[]) => number,
    expr2: (x: number[], y: number[]) => number,
  ): void {
    const rng = new Rng(seed);
    for (const n of sampleSizes) {
      const x = Array.from({ length: n }, () => rng.uniformFloat());
      const y = Array.from({ length: n }, () => rng.uniformFloat());
      const result1 = expr1(x, y);
      const result2 = expr2(x, y);
      expect(result1).toBeCloseTo(result2, 9);
    }
  }

  // Helper functions
  const addScalar = (arr: number[], scalar: number): number[] => arr.map((x) => x + scalar);
  const mulScalar = (arr: number[], scalar: number): number[] => arr.map((x) => x * scalar);

  // Center invariance tests
  describe('center', () => {
    it('should be shift equivariant', () => {
      performTestOne(
        (x) => center(addScalar(x, 2)),
        (x) => center(x) + 2,
      );
    });

    it('should be scale equivariant', () => {
      performTestOne(
        (x) => center(mulScalar(x, 2)),
        (x) => 2 * center(x),
      );
    });

    it('should be negate equivariant', () => {
      performTestOne(
        (x) => center(mulScalar(x, -1)),
        (x) => -1 * center(x),
      );
    });
  });

  // Spread invariance tests
  describe('spread', () => {
    it('should be shift invariant', () => {
      performTestOne(
        (x) => spread(addScalar(x, 2)),
        (x) => spread(x),
      );
    });

    it('should be scale equivariant', () => {
      performTestOne(
        (x) => spread(mulScalar(x, 2)),
        (x) => 2 * spread(x),
      );
    });

    it('should be negate invariant', () => {
      performTestOne(
        (x) => spread(mulScalar(x, -1)),
        (x) => spread(x),
      );
    });
  });

  // RelSpread invariance tests
  describe('relSpread', () => {
    it('should be scale invariant', () => {
      performTestOne(
        (x) => relSpread(mulScalar(x, 2)),
        (x) => relSpread(x),
      );
    });
  });

  // Shift invariance tests
  describe('shift', () => {
    it('should be shift equivariant', () => {
      performTestTwo(
        (x, y) => shift(addScalar(x, 3), addScalar(y, 2)),
        (x, y) => shift(x, y) + 1,
      );
    });

    it('should be scale equivariant', () => {
      performTestTwo(
        (x, y) => shift(mulScalar(x, 2), mulScalar(y, 2)),
        (x, y) => 2 * shift(x, y),
      );
    });

    it('should be antisymmetric', () => {
      performTestTwo(
        (x, y) => shift(x, y),
        (x, y) => -1 * shift(y, x),
      );
    });
  });

  // Ratio invariance tests
  describe('ratio', () => {
    it('should be scale equivariant', () => {
      performTestTwo(
        (x, y) => ratio(mulScalar(x, 2), mulScalar(y, 3)),
        (x, y) => (2.0 / 3) * ratio(x, y),
      );
    });
  });

  // AvgSpread invariance tests
  describe('avgSpread', () => {
    it('should equal spread for identical samples', () => {
      performTestOne(
        (x) => avgSpread(x, x),
        (x) => spread(x),
      );
    });

    it('should be symmetric', () => {
      performTestTwo(
        (x, y) => avgSpread(x, y),
        (x, y) => avgSpread(y, x),
      );
    });

    it('should calculate average correctly', () => {
      performTestOne(
        (x) => avgSpread(x, mulScalar(x, 5)),
        (x) => 3 * spread(x),
      );
    });

    it('should be scale equivariant', () => {
      performTestTwo(
        (x, y) => avgSpread(mulScalar(x, -2), mulScalar(y, -2)),
        (x, y) => 2 * avgSpread(x, y),
      );
    });
  });

  // Disparity invariance tests
  describe('disparity', () => {
    it('should be shift invariant', () => {
      performTestTwo(
        (x, y) => disparity(addScalar(x, 2), addScalar(y, 2)),
        (x, y) => disparity(x, y),
      );
    });

    it('should be scale invariant', () => {
      performTestTwo(
        (x, y) => disparity(mulScalar(x, 2), mulScalar(y, 2)),
        (x, y) => disparity(x, y),
      );
    });

    it('should be scale antisymmetric with negative scale', () => {
      performTestTwo(
        (x, y) => disparity(mulScalar(x, -2), mulScalar(y, -2)),
        (x, y) => -1 * disparity(x, y),
      );
    });

    it('should be antisymmetric', () => {
      performTestTwo(
        (x, y) => disparity(x, y),
        (x, y) => -1 * disparity(y, x),
      );
    });
  });
});

describe('shuffle', () => {
  it('should preserve multiset', () => {
    for (const n of [1, 2, 5, 10, 100]) {
      const x = Array.from({ length: n }, (_, i) => i);
      const rng = new Rng(42);
      const shuffled = rng.shuffle(x);
      expect([...shuffled].sort((a, b) => a - b)).toEqual(x);
    }
  });
});

describe('sample', () => {
  it('should return correct size', () => {
    const x = Array.from({ length: 10 }, (_, i) => i);
    for (const k of [1, 3, 5, 10, 15]) {
      const rng = new Rng(42);
      const sampled = rng.sample(x, k);
      expect(sampled.length).toBe(Math.min(k, x.length));
    }
  });

  it('should only contain elements from source', () => {
    const x = Array.from({ length: 10 }, (_, i) => i);
    const rng = new Rng(42);
    const sampled = rng.sample(x, 5);
    for (const elem of sampled) {
      expect(x).toContain(elem);
    }
  });

  it('should preserve order', () => {
    const x = Array.from({ length: 10 }, (_, i) => i);
    const rng = new Rng(42);
    const sampled = rng.sample(x, 5);
    for (let i = 1; i < sampled.length; i++) {
      expect(sampled[i]).toBeGreaterThan(sampled[i - 1]);
    }
  });

  it('should have no duplicates', () => {
    for (const n of [2, 3, 5, 10, 20]) {
      const source = Array.from({ length: n }, (_, i) => i);
      for (const k of [1, Math.floor(n / 2), n]) {
        const rng = new Rng(42);
        const sampled = rng.sample(source, k);
        expect(new Set(sampled).size).toBe(sampled.length);
      }
    }
  });
});

describe('resample', () => {
  it('should only contain elements from source', () => {
    const x = Array.from({ length: 5 }, (_, i) => i);
    const rng = new Rng(42);
    const resampled = rng.resample(x, 10);
    for (const elem of resampled) {
      expect(x).toContain(elem);
    }
  });

  it('should throw for k=0', () => {
    const rng = new Rng(42);
    expect(() => rng.resample([1, 2, 3], 0)).toThrow();
  });

  it('should throw for negative k', () => {
    const rng = new Rng(42);
    expect(() => rng.resample([1, 2, 3], -1)).toThrow();
  });
});

describe('shuffle errors', () => {
  it('should throw for empty array', () => {
    const rng = new Rng(42);
    expect(() => rng.shuffle([])).toThrow();
  });
});

describe('sample errors', () => {
  it('should throw for k=0', () => {
    const rng = new Rng(42);
    expect(() => rng.sample([1, 2, 3], 0)).toThrow();
  });

  it('should throw for empty array', () => {
    const rng = new Rng(42);
    expect(() => rng.sample([], 1)).toThrow();
  });
});
