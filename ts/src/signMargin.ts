/**
 * SignMargin function for one-sample bounds based on Binomial(n, 0.5).
 */

import { minAchievableMisrateOneSample } from './minMisrate';
import { AssumptionError } from './assumptions';
import { Rng } from './rng';

export function signMarginRandomized(n: number, misrate: number, rng: Rng): number {
  if (n <= 0) throw AssumptionError.domain('x');
  if (isNaN(misrate) || misrate < 0 || misrate > 1) throw AssumptionError.domain('misrate');
  const minMisrate = minAchievableMisrateOneSample(n);
  if (misrate < minMisrate) throw AssumptionError.domain('misrate');

  const target = misrate / 2;
  if (target <= 0) return 0;
  if (target >= 1) return n * 2;

  const [rLow, logCdfLow, logPmfHigh] = binomCdfSplit(n, target);
  const logTarget = Math.log(target);
  const logNum = logTarget > logCdfLow ? logSubExp(logTarget, logCdfLow) : -Infinity;

  let p = isFinite(logPmfHigh) && isFinite(logNum) ? Math.exp(logNum - logPmfHigh) : 0;
  p = Math.max(0, Math.min(1, p));

  const u = rng.uniformFloat();
  const r = u < p ? rLow + 1 : rLow;
  return r * 2;
}

function binomCdfSplit(n: number, target: number): [number, number, number] {
  const logTarget = Math.log(target);
  let logPmf = -n * Math.LN2;
  let logCdf = logPmf;
  let rLow = 0;
  if (logCdf > logTarget) return [0, logCdf, logPmf];
  for (let k = 1; k <= n; k++) {
    const logPmfNext = logPmf + Math.log(n - k + 1) - Math.log(k);
    const logCdfNext = logAddExp(logCdf, logPmfNext);
    if (logCdfNext > logTarget) return [rLow, logCdf, logPmfNext];
    rLow = k;
    logPmf = logPmfNext;
    logCdf = logCdfNext;
  }
  return [rLow, logCdf, -Infinity];
}

function logAddExp(a: number, b: number): number {
  if (a === -Infinity) return b;
  if (b === -Infinity) return a;
  const m = Math.max(a, b);
  return m + Math.log(Math.exp(a - m) + Math.exp(b - m));
}

function logSubExp(a: number, b: number): number {
  if (b === -Infinity) return a;
  const diff = Math.exp(b - a);
  return diff >= 1 ? -Infinity : a + Math.log(1 - diff);
}
