/**
 * PairwiseMargin function for computing confidence bound margins
 *
 * Determines how many extreme pairwise differences to exclude when constructing bounds
 * based on the distribution of dominance statistics.
 */

import { AssumptionError } from './assumptions';
import { binomialCoefficient } from './binomial';
import { additiveCumulative } from './additiveCumulative';
import { minAchievableMisrateTwoSample } from './minMisrate';
import { expFunction } from './expFunction';

const MAX_EXACT_SIZE = 400;

/**
 * PairwiseMargin determines how many extreme pairwise differences to exclude
 * when constructing bounds based on the distribution of dominance statistics.
 * Uses exact calculation for small samples (n+m <= 400) and Edgeworth
 * approximation for larger samples.
 *
 * @param n Sample size of first sample (must be positive)
 * @param m Sample size of second sample (must be positive)
 * @param misrate Misclassification rate (must be in [0, 1])
 * @returns Integer representing the total margin split between lower and upper tails
 * @throws AssumptionError if n <= 0, m <= 0, or misrate is outside [0, 1]
 */
export function pairwiseMargin(n: number, m: number, misrate: number): number {
  if (n <= 0) {
    throw AssumptionError.domain('x');
  }
  if (m <= 0) {
    throw AssumptionError.domain('y');
  }
  if (misrate < 0 || misrate > 1 || Number.isNaN(misrate)) {
    throw AssumptionError.domain('misrate');
  }

  const minMisrate = minAchievableMisrateTwoSample(n, m);
  if (misrate < minMisrate) {
    throw AssumptionError.domain('misrate');
  }

  if (n + m <= MAX_EXACT_SIZE) {
    return pairwiseMarginExact(n, m, misrate);
  } else {
    return pairwiseMarginApprox(n, m, misrate);
  }
}

/**
 * Uses the exact distribution based on Loeffler's recurrence
 */
function pairwiseMarginExact(n: number, m: number, misrate: number): number {
  return pairwiseMarginExactRaw(n, m, misrate / 2) * 2;
}

/**
 * Uses Edgeworth approximation for large samples
 */
function pairwiseMarginApprox(n: number, m: number, misrate: number): number {
  return pairwiseMarginApproxRaw(n, m, misrate / 2) * 2;
}

/**
 * Inversed implementation of Andreas Löffler's (1982)
 * "Über eine Partition der nat. Zahlen und ihre Anwendung beim U-Test"
 */
function pairwiseMarginExactRaw(n: number, m: number, p: number): number {
  // Same entry point the misrate floor uses for C(n+m, n): at the floor the check
  // `1/total >= misrate/2` compares the two against each other, so they must agree bitwise.
  const total = binomialCoefficient(n + m, m);

  const pmf: number[] = [1]; // pmf[0] = 1
  const sigma: number[] = [0]; // sigma[0] is unused

  let u = 0;
  let cdf = 1.0 / total;

  if (cdf >= p) {
    return 0;
  }

  while (true) {
    u++;

    // Ensure sigma has entry for u
    if (sigma.length <= u) {
      let value = 0;
      for (let d = 1; d <= n; d++) {
        if (u % d === 0 && u >= d) {
          value += d;
        }
      }
      for (let d = m + 1; d <= m + n; d++) {
        if (u % d === 0 && u >= d) {
          value -= d;
        }
      }
      sigma.push(value);
    }

    // Compute pmf[u] using Loeffler recurrence
    let sum = 0.0;
    for (let i = 0; i < u; i++) {
      sum += pmf[i] * sigma[u - i];
    }
    sum /= u;
    pmf.push(sum);

    cdf += sum / total;
    if (cdf >= p) {
      return u;
    }
    if (sum === 0) {
      break;
    }
  }

  return pmf.length - 1;
}

/**
 * Inverse Edgeworth Approximation
 */
function pairwiseMarginApproxRaw(n: number, m: number, misrate: number): number {
  let a = 0;
  let b = n * m;
  while (a < b - 1) {
    const c = Math.floor((a + b) / 2);
    const p = edgeworthCdf(n, m, c);
    if (p < misrate) {
      a = c;
    } else {
      b = c;
    }
  }

  return edgeworthCdf(n, m, b) < misrate ? b : a;
}

/**
 * Computes the CDF using Edgeworth expansion
 */
function edgeworthCdf(n: number, m: number, u: number): number {
  const mu = (n * m) / 2.0;
  const su = Math.sqrt((n * m * (n + m + 1)) / 12.0);
  // -0.5 continuity correction: computing P(U ≥ u) for a right-tail discrete CDF
  const z = (u - mu - 0.5) / su;

  // Standard normal PDF and CDF. expFunction rather than Math.exp: the platform's exponential
  // differs between runtimes in the last bit, and this density feeds a search that selects an
  // integer margin.
  const phi = expFunction((-z * z) / 2) / Math.sqrt(2 * Math.PI);
  const bigPhi = additiveCumulative(z);

  // Pre-compute powers of n and m for efficiency
  const n2 = n * n;
  const n3 = n2 * n;
  const n4 = n2 * n2;
  const m2 = m * m;
  const m3 = m2 * m;
  const m4 = m2 * m2;

  // Compute moments
  const mu2 = (n * m * (n + m + 1)) / 12.0;
  const mu4 =
    (n * m * (n + m + 1) * (5 * m * n * (m + n) - 2 * (m2 + n2) + 3 * m * n - 2 * (n + m))) / 240.0;

  const mu6 =
    (n *
      m *
      (n + m + 1) *
      (35 * m2 * n2 * (m2 + n2) +
        70 * m3 * n3 -
        42 * m * n * (m3 + n3) -
        14 * m2 * n2 * (n + m) +
        16 * (n4 + m4) -
        52 * n * m * (n2 + m2) -
        43 * n2 * m2 +
        32 * (m3 + n3) +
        14 * m * n * (n + m) +
        8 * (n2 + m2) +
        16 * n * m -
        8 * (n + m))) /
    4032.0;

  // Pre-compute powers of mu2 and related terms
  const mu2_2 = mu2 * mu2;
  const mu2_3 = mu2_2 * mu2;
  const mu4_mu2_2 = mu4 / mu2_2;

  // Factorial constants: 4! = 24, 6! = 720, 8! = 40320
  const e3 = (mu4_mu2_2 - 3) / 24.0;
  const e5 = (mu6 / mu2_3 - 15 * mu4_mu2_2 + 30) / 720.0;
  const e7 = (35 * (mu4_mu2_2 - 3) * (mu4_mu2_2 - 3)) / 40320.0;

  // Pre-compute powers of z for Hermite polynomials
  const z2 = z * z;
  const z3 = z2 * z;
  const z5 = z3 * z2;
  const z7 = z5 * z2;

  // Hermite polynomial derivatives: f_n = -phi * H_n(z)
  const f3 = -phi * (z3 - 3 * z);
  const f5 = -phi * (z5 - 10 * z3 + 15 * z);
  const f7 = -phi * (z7 - 21 * z5 + 105 * z3 - 105 * z);

  // Edgeworth expansion
  const edgeworth = bigPhi + e3 * f3 + e5 * f5 + e7 * f7;

  // Clamp to [0, 1]
  return Math.max(0, Math.min(1, edgeworth));
}
