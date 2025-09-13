import { center, spread, relSpread, shift, ratio, avgSpread, disparity } from '../src/estimators';

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

  describe('relSpread', () => {
    it('should throw error for empty array', () => {
      expect(() => relSpread([])).toThrow('Input array cannot be empty');
    });

    it('should calculate relSpread correctly', () => {
      const result = relSpread([1, 2, 3, 4, 5]);
      expect(result).toBeCloseTo(2 / 3, 5);
    });
  });

  describe('shift', () => {
    it('should throw error for empty arrays', () => {
      expect(() => shift([], [])).toThrow('Input arrays cannot be empty');
      expect(() => shift([1], [])).toThrow('Input arrays cannot be empty');
      expect(() => shift([], [1])).toThrow('Input arrays cannot be empty');
    });

    it('should calculate shift correctly', () => {
      const result = shift([1, 2, 3], [4, 5, 6]);
      expect(result).toBe(-3);
    });
  });

  describe('ratio', () => {
    it('should throw error for empty arrays', () => {
      expect(() => ratio([], [])).toThrow('Input arrays cannot be empty');
      expect(() => ratio([1], [])).toThrow('Input arrays cannot be empty');
      expect(() => ratio([], [1])).toThrow('Input arrays cannot be empty');
    });

    it('should calculate ratio correctly', () => {
      const result = ratio([1, 2, 3], [2, 4, 6]);
      expect(result).toBe(0.5);
    });
  });

  describe('avgSpread', () => {
    it('should throw error for empty arrays', () => {
      expect(() => avgSpread([], [])).toThrow('Input arrays cannot be empty');
    });

    it('should calculate combined spread correctly', () => {
      const result = avgSpread([1, 2], [3, 4]);
      expect(result).toBeCloseTo(1, 5);
    });

    it('should equal spread when both arrays are identical', () => {
      const x = [1, 2, 3, 4, 5];
      expect(avgSpread(x, x)).toBeCloseTo(spread(x), 10);
    });

    it('should calculate weighted average of spreads', () => {
      // Different sized arrays
      const x = [1, 2, 3]; // spread = 1
      const y = [10, 14]; // spread = 4
      // Expected: (3 * 1 + 2 * 4) / (3 + 2) = (3 + 8) / 5 = 11 / 5 = 2.2
      expect(avgSpread(x, y)).toBeCloseTo(2.2, 5);
    });
  });

  describe('disparity', () => {
    it('should throw error for empty arrays', () => {
      expect(() => disparity([], [])).toThrow('Input arrays cannot be empty');
    });

    it('should calculate disparity correctly', () => {
      const result = disparity([1, 2, 3], [4, 5, 6]);
      // shift = -3, avgSpread = 1, disparity = -3 / 1 = -3
      expect(result).toBeCloseTo(-3, 5);
    });
  });
});
