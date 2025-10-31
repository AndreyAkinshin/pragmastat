/**
 * Performance tests for fast Center, Spread, and Shift implementations
 */

import { fastCenter } from '../src/fastCenter';
import { fastSpread } from '../src/fastSpread';
import { fastShift } from '../src/fastShift';

describe('Fast Center Performance', () => {
  it('should complete in reasonable time for n=100000', () => {
    const n = 100000;
    const x = Array.from({ length: n }, (_, i) => i + 1);

    const start = Date.now();
    const result = fastCenter(x);
    const elapsed = Date.now() - start;

    const expected = 50000.5;
    expect(Math.abs(result - expected)).toBeLessThan(1e-9);
    expect(elapsed).toBeLessThan(5000); // Should complete in less than 5 seconds
  });
});

describe('Fast Spread Performance', () => {
  it('should complete in reasonable time for n=100000', () => {
    const n = 100000;
    const x = Array.from({ length: n }, (_, i) => i + 1);

    const start = Date.now();
    const result = fastSpread(x);
    const elapsed = Date.now() - start;

    const expected = 29290;
    expect(Math.abs(result - expected)).toBeLessThan(1e-9);
    expect(elapsed).toBeLessThan(5000); // Should complete in less than 5 seconds
  });
});

describe('Fast Shift Performance', () => {
  it('should complete in reasonable time for n=m=100000', () => {
    const n = 100000;
    const x = Array.from({ length: n }, (_, i) => i + 1);
    const y = Array.from({ length: n }, (_, i) => i + 1);

    const start = Date.now();
    const result = fastShift(x, y, [0.5])[0];
    const elapsed = Date.now() - start;

    const expected = 0;
    expect(Math.abs(result - expected)).toBeLessThan(1e-9);
    expect(elapsed).toBeLessThan(5000); // Should complete in less than 5 seconds
  });
});
