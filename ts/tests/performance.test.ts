/**
 * Performance tests for fast Center and Spread implementations
 */

import { fastCenter } from '../src/fastCenter';
import { fastSpread } from '../src/fastSpread';
import { median } from '../src/utils';

// Simple O(n^2) implementations for comparison
function centerSimple(x: number[]): number {
  const n = x.length;
  const pairwiseAverages: number[] = [];
  for (let i = 0; i < n; i++) {
    for (let j = i; j < n; j++) {
      pairwiseAverages.push((x[i] + x[j]) / 2);
    }
  }
  return median(pairwiseAverages);
}

function spreadSimple(x: number[]): number {
  const n = x.length;
  if (n === 1) {
    return 0;
  }
  const pairwiseDiffs: number[] = [];
  for (let i = 0; i < n; i++) {
    for (let j = i + 1; j < n; j++) {
      pairwiseDiffs.push(Math.abs(x[i] - x[j]));
    }
  }
  return median(pairwiseDiffs);
}

// Seeded random number generator for reproducibility
function seededRandom(seed: number): () => number {
  let state = seed;
  return (): number => {
    state = (state * 1664525 + 1013904223) % 4294967296;
    return (state / 4294967296) * 2 - 1; // Range [-1, 1]
  };
}

describe('Fast Center Correctness', () => {
  it('should match simple implementation for various sizes', () => {
    const rng = seededRandom(1729);

    for (let n = 1; n <= 100; n++) {
      for (let iter = 0; iter < n; iter++) {
        const x = Array.from({ length: n }, () => rng());

        const expected = centerSimple(x);
        const actual = fastCenter(x);

        expect(Math.abs(expected - actual)).toBeLessThan(1e-9);
      }
    }
  });
});

describe('Fast Spread Correctness', () => {
  it('should match simple implementation for various sizes', () => {
    const rng = seededRandom(1729);

    for (let n = 1; n <= 100; n++) {
      for (let iter = 0; iter < n; iter++) {
        const x = Array.from({ length: n }, () => rng());

        const expected = spreadSimple(x);
        const actual = fastSpread(x);

        expect(Math.abs(expected - actual)).toBeLessThan(1e-9);
      }
    }
  });
});

describe('Fast Center Performance', () => {
  it('should complete in reasonable time for n=100000', () => {
    const rng = seededRandom(1729);
    const n = 100000;
    const x = Array.from({ length: n }, () => rng());

    const start = Date.now();
    const result = fastCenter(x);
    const elapsed = Date.now() - start;

    console.log(`\nCenter for n=${n}: ${result.toFixed(6)}`);
    console.log(`Elapsed time: ${elapsed}ms`);

    expect(elapsed).toBeLessThan(5000); // Should complete in less than 5 seconds
  });
});

describe('Fast Spread Performance', () => {
  it('should complete in reasonable time for n=100000', () => {
    const rng = seededRandom(1729);
    const n = 100000;
    const x = Array.from({ length: n }, () => rng());

    const start = Date.now();
    const result = fastSpread(x);
    const elapsed = Date.now() - start;

    console.log(`\nSpread for n=${n}: ${result.toFixed(6)}`);
    console.log(`Elapsed time: ${elapsed}ms`);

    expect(elapsed).toBeLessThan(5000); // Should complete in less than 5 seconds
  });
});
