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
 * Invariance tests for mathematical properties of estimators
 */

describe('Invariance Tests', () => {
  const seed = 1729;
  const sampleSizes = [2, 3, 4, 5, 6, 7, 8, 9, 10];

  // Simple linear congruential generator for reproducible random numbers
  class SimpleRng {
    private state: number;

    constructor(seed: number) {
      this.state = seed;
    }

    next(): number {
      this.state = ((this.state * 1103515245 + 12345) & 0x7fffffff) >>> 0;
      return this.state / 0x7fffffff;
    }

    nextArray(n: number): number[] {
      return Array.from({ length: n }, () => this.next());
    }
  }

  function performTestOne(expr1: (x: number[]) => number, expr2: (x: number[]) => number): void {
    const rng = new SimpleRng(seed);
    for (const n of sampleSizes) {
      const x = rng.nextArray(n);
      const result1 = expr1(x);
      const result2 = expr2(x);
      expect(result1).toBeCloseTo(result2, 9);
    }
  }

  function performTestTwo(
    expr1: (x: number[], y: number[]) => number,
    expr2: (x: number[], y: number[]) => number,
  ): void {
    const rng = new SimpleRng(seed);
    for (const n of sampleSizes) {
      const x = rng.nextArray(n);
      const y = rng.nextArray(n);
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

  // Volatility invariance tests
  describe('volatility', () => {
    it('should be scale invariant', () => {
      performTestOne(
        (x) => volatility(mulScalar(x, 2)),
        (x) => volatility(x),
      );
    });
  });

  // Precision invariance tests
  describe('precision', () => {
    it('should be shift invariant', () => {
      performTestOne(
        (x) => precision(addScalar(x, 2)),
        (x) => precision(x),
      );
    });

    it('should be scale equivariant', () => {
      performTestOne(
        (x) => precision(mulScalar(x, 2)),
        (x) => 2 * precision(x),
      );
    });

    it('should be scale equivariant with negative scale', () => {
      performTestOne(
        (x) => precision(mulScalar(x, -2)),
        (x) => 2 * precision(x),
      );
    });
  });

  // MedShift invariance tests
  describe('medShift', () => {
    it('should be shift equivariant', () => {
      performTestTwo(
        (x, y) => medShift(addScalar(x, 3), addScalar(y, 2)),
        (x, y) => medShift(x, y) + 1,
      );
    });

    it('should be scale equivariant', () => {
      performTestTwo(
        (x, y) => medShift(mulScalar(x, 2), mulScalar(y, 2)),
        (x, y) => 2 * medShift(x, y),
      );
    });

    it('should be antisymmetric', () => {
      performTestTwo(
        (x, y) => medShift(x, y),
        (x, y) => -1 * medShift(y, x),
      );
    });
  });

  // MedRatio invariance tests
  describe('medRatio', () => {
    it('should be scale equivariant', () => {
      performTestTwo(
        (x, y) => medRatio(mulScalar(x, 2), mulScalar(y, 3)),
        (x, y) => (2.0 / 3) * medRatio(x, y),
      );
    });
  });

  // MedSpread invariance tests
  describe('medSpread', () => {
    it('should equal spread for identical samples', () => {
      performTestOne(
        (x) => medSpread(x, x),
        (x) => spread(x),
      );
    });

    it('should be symmetric', () => {
      performTestTwo(
        (x, y) => medSpread(x, y),
        (x, y) => medSpread(y, x),
      );
    });

    it('should calculate average correctly', () => {
      performTestOne(
        (x) => medSpread(x, mulScalar(x, 5)),
        (x) => 3 * spread(x),
      );
    });

    it('should be scale equivariant', () => {
      performTestTwo(
        (x, y) => medSpread(mulScalar(x, -2), mulScalar(y, -2)),
        (x, y) => 2 * medSpread(x, y),
      );
    });
  });

  // MedDisparity invariance tests
  describe('medDisparity', () => {
    it('should be shift invariant', () => {
      performTestTwo(
        (x, y) => medDisparity(addScalar(x, 2), addScalar(y, 2)),
        (x, y) => medDisparity(x, y),
      );
    });

    it('should be scale invariant', () => {
      performTestTwo(
        (x, y) => medDisparity(mulScalar(x, 2), mulScalar(y, 2)),
        (x, y) => medDisparity(x, y),
      );
    });

    it('should be scale antisymmetric with negative scale', () => {
      performTestTwo(
        (x, y) => medDisparity(mulScalar(x, -2), mulScalar(y, -2)),
        (x, y) => -1 * medDisparity(x, y),
      );
    });

    it('should be antisymmetric', () => {
      performTestTwo(
        (x, y) => medDisparity(x, y),
        (x, y) => -1 * medDisparity(y, x),
      );
    });
  });
});
