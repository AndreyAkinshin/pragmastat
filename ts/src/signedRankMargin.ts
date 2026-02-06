/**
 * SignedRankMargin function for one-sample bounds.
 *
 * One-sample analog of PairwiseMargin using Wilcoxon signed-rank distribution.
 */

import { gaussCdf } from './gaussCdf';
import { minAchievableMisrateOneSample } from './minMisrate';
import { AssumptionError } from './assumptions';

// Maximum n for exact computation. Limited to 63 because 2^n must fit in a BigInt
// (which has arbitrary precision in JS). The CDF division uses scaled BigInt arithmetic
// to avoid precision loss when converting to Number.
const SIGNED_RANK_MAX_EXACT_SIZE = 63;

/**
 * SignedRankMargin computes the margin for one-sample signed-rank bounds.
 * Uses Wilcoxon signed-rank distribution to determine the margin that achieves
 * the specified misrate.
 *
 * @param n Sample size (must be positive)
 * @param misrate Misclassification rate (must be in [0, 1])
 * @returns Integer margin
 * @throws Error if inputs are invalid or misrate is below minimum achievable
 */
export function signedRankMargin(n: number, misrate: number): number {
  if (n <= 0) {
    throw AssumptionError.domain('x');
  }
  if (isNaN(misrate) || misrate < 0 || misrate > 1) {
    throw AssumptionError.domain('misrate');
  }

  const minMisrate = minAchievableMisrateOneSample(n);
  if (misrate < minMisrate) {
    throw AssumptionError.domain('misrate');
  }

  if (n <= SIGNED_RANK_MAX_EXACT_SIZE) {
    return signedRankMarginExact(n, misrate);
  } else {
    return signedRankMarginApprox(n, misrate);
  }
}

/**
 * Computes one-sided margin using exact Wilcoxon signed-rank distribution.
 */
function signedRankMarginExact(n: number, misrate: number): number {
  return signedRankMarginExactRaw(n, misrate / 2) * 2;
}

function signedRankMarginExactRaw(n: number, p: number): number {
  const total = BigInt(1) << BigInt(n);
  const maxW = Math.floor((n * (n + 1)) / 2);

  const count: bigint[] = new Array(maxW + 1).fill(BigInt(0));
  count[0] = BigInt(1);

  for (let i = 1; i <= n; i++) {
    const maxWi = Math.min(Math.floor((i * (i + 1)) / 2), maxW);
    for (let w = maxWi; w >= i; w--) {
      count[w] = count[w] + count[w - i];
    }
  }

  // Use scaled BigInt division to avoid precision loss for large n (n > 53)
  const PRECISION = BigInt(10) ** BigInt(18);
  let cumulative = BigInt(0);
  for (let w = 0; w <= maxW; w++) {
    cumulative = cumulative + count[w];
    const cdf = Number((cumulative * PRECISION) / total) / Number(PRECISION);
    if (cdf >= p) {
      return w;
    }
  }

  return maxW;
}

/**
 * Computes one-sided margin using Edgeworth approximation for large n.
 */
function signedRankMarginApprox(n: number, misrate: number): number {
  return signedRankMarginApproxRaw(n, misrate / 2) * 2;
}

function signedRankMarginApproxRaw(n: number, misrate: number): number {
  const maxW = Math.floor((n * (n + 1)) / 2);
  let a = 0;
  let b = maxW;

  while (a < b - 1) {
    const c = Math.floor((a + b) / 2);
    const cdf = signedRankEdgeworthCdf(n, c);
    if (cdf < misrate) {
      a = c;
    } else {
      b = c;
    }
  }

  return signedRankEdgeworthCdf(n, b) < misrate ? b : a;
}

/**
 * Edgeworth expansion for Wilcoxon signed-rank distribution CDF.
 */
function signedRankEdgeworthCdf(n: number, w: number): number {
  const mu = (n * (n + 1)) / 4.0;
  const sigma2 = (n * (n + 1) * (2 * n + 1)) / 24.0;
  const sigma = Math.sqrt(sigma2);

  // +0.5 continuity correction: computing P(W ≤ w) for a left-tail discrete CDF
  const z = (w - mu + 0.5) / sigma;
  const phi = Math.exp((-z * z) / 2) / Math.sqrt(2 * Math.PI);
  const bigPhi = gaussCdf(z);

  const mu4 = centralMoment4(n);
  const kappa4 = mu4 - 3 * sigma2 * sigma2;

  const e3 = kappa4 / (24 * sigma2 * sigma2);

  const z2 = z * z;
  const z3 = z2 * z;
  const f3 = -phi * (z3 - 3 * z);

  const edgeworth = bigPhi + e3 * f3;
  return Math.max(0, Math.min(1, edgeworth));
}

/**
 * Computes the 4th central moment of signed-rank distribution.
 * E[(W - mu)^4] where W is the Wilcoxon signed-rank statistic.
 */
function centralMoment4(n: number): number {
  const n2 = n * n;
  const n3 = n2 * n;
  const n4 = n2 * n2;
  const n5 = n4 * n;

  return (9 * n5 + 45 * n4 + 65 * n3 + 15 * n2 - 14 * n) / 480.0;
}
