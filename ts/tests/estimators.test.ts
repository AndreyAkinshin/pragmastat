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

describe('Estimators', () => {
  describe('center', () => {
    it('should throw error for empty array', () => {
      expect(() => center([])).toThrow('Input array cannot be empty');
    });

    it('should return the value for single element', () => {
      expect(center([5])).toBe(5);
    });

    it('should calculate center for simple array', () => {
      const result = center([1, 2, 3, 4, 5]);
      expect(result).toBe(3);
    });
  });

  describe('spread', () => {
    it('should throw error for empty array', () => {
      expect(() => spread([])).toThrow('Input array cannot be empty');
    });

    it('should return 0 for single element', () => {
      expect(spread([5])).toBe(0);
    });

    it('should calculate spread for simple array', () => {
      const result = spread([1, 2, 3, 4, 5]);
      expect(result).toBeCloseTo(2, 5);
    });
  });

  describe('volatility', () => {
    it('should throw error for empty array', () => {
      expect(() => volatility([])).toThrow('Input array cannot be empty');
    });

    it('should calculate volatility correctly', () => {
      const result = volatility([1, 2, 3, 4, 5]);
      expect(result).toBeCloseTo(2 / 3, 5);
    });
  });

  describe('precision', () => {
    it('should throw error for empty array', () => {
      expect(() => precision([])).toThrow('Input array cannot be empty');
    });

    it('should calculate precision correctly', () => {
      const result = precision([1, 2, 3, 4, 5]);
      // precision = 2 * spread / sqrt(n)
      // spread([1,2,3,4,5]) = 2
      // precision = 2 * 2 / sqrt(5) ≈ 1.789
      expect(result).toBeCloseTo(1.7888543819998317, 10);
    });
  });

  describe('medShift', () => {
    it('should throw error for empty arrays', () => {
      expect(() => medShift([], [])).toThrow('Input arrays cannot be empty');
      expect(() => medShift([1], [])).toThrow('Input arrays cannot be empty');
      expect(() => medShift([], [1])).toThrow('Input arrays cannot be empty');
    });

    it('should calculate shift correctly', () => {
      const result = medShift([1, 2, 3], [4, 5, 6]);
      expect(result).toBe(-3);
    });
  });

  describe('medRatio', () => {
    it('should throw error for empty arrays', () => {
      expect(() => medRatio([], [])).toThrow('Input arrays cannot be empty');
      expect(() => medRatio([1], [])).toThrow('Input arrays cannot be empty');
      expect(() => medRatio([], [1])).toThrow('Input arrays cannot be empty');
    });

    it('should calculate ratio correctly', () => {
      const result = medRatio([1, 2, 3], [2, 4, 6]);
      expect(result).toBe(0.5);
    });
  });

  describe('medSpread', () => {
    it('should throw error for empty arrays', () => {
      expect(() => medSpread([], [])).toThrow('Input arrays cannot be empty');
    });

    it('should calculate combined spread correctly', () => {
      const result = medSpread([1, 2], [3, 4]);
      expect(result).toBeCloseTo(1, 5);
    });

    it('should equal spread when both arrays are identical', () => {
      const x = [1, 2, 3, 4, 5];
      expect(medSpread(x, x)).toBeCloseTo(spread(x), 10);
    });

    it('should calculate weighted average of spreads', () => {
      // Different sized arrays
      const x = [1, 2, 3]; // spread = 1
      const y = [10, 14]; // spread = 4
      // Expected: (3 * 1 + 2 * 4) / (3 + 2) = (3 + 8) / 5 = 11 / 5 = 2.2
      expect(medSpread(x, y)).toBeCloseTo(2.2, 5);
    });
  });

  describe('medDisparity', () => {
    it('should throw error for empty arrays', () => {
      expect(() => medDisparity([], [])).toThrow('Input arrays cannot be empty');
    });

    it('should calculate disparity correctly', () => {
      const result = medDisparity([1, 2, 3], [4, 5, 6]);
      // medShift = -3, medSpread = 1, disparity = -3 / 1 = -3
      expect(result).toBeCloseTo(-3, 5);
    });
  });
});
