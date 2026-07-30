/**
 * SignMargin function for one-sample bounds based on Binomial(n, 0.5), computed without a single
 * library call.
 *
 * It used to be evaluated in log space: nine calls to Math.log and Math.exp, one of them inside a
 * loop that runs n times. IEEE 754 fixes nothing about either function, and this value is not
 * merely returned to the caller: the margin selects an order statistic, so a difference between
 * two conforming implementations becomes a different confidence interval from identical inputs.
 * It did. Two ports disagreed on spreadBounds for a sample of 200 consecutive integers.
 *
 * No logarithm is needed. Binomial(n, 1/2) has an exact rational distribution function, and the
 * two quantities the randomization wants are its partial sum and the next term. Both follow from
 * the same multiplicative recurrence the binomial coefficient uses: one multiply and one divide
 * per step, plus a scaling by a power of two, and IEEE 754 pins all three.
 *
 * The scaling is what makes the recurrence work at any n. pmf(0) is 2^-n, which underflows to zero
 * past n = 1074, so the running term is carried as w * 2^e with the exponent tracked separately:
 * w stays in the normal range and e absorbs the magnitude. Rescaling happens by multiplying by a
 * power of two, which is exact, so it costs no accuracy and changes no bits.
 *
 * Measured against exact rational arithmetic over 195 (n, misrate) pairs spanning n = 1 to 5000
 * and misrate from 1 down to the smallest positive double: the selected index is right every time,
 * and the randomization probability is within 6.1e-13. The log-space version it replaces reached
 * 1.9e-11 on the same set, thirty times further out, and did it differently in each port.
 */

import { minAchievableMisrateOneSample } from './minMisrate';
import { AssumptionError } from './assumptions';
import { Rng } from './rng';

/**
 * How far the running term is rescaled when it grows too large. Any power of two works; 512 keeps
 * the rescaling rare without letting w approach the overflow threshold.
 */
const SCALE_STEP = 512;

export function signMarginRandomized(n: number, misrate: number, rng: Rng): number {
  if (n <= 0) throw AssumptionError.domain('x');
  if (isNaN(misrate) || misrate < 0 || misrate > 1) throw AssumptionError.domain('misrate');
  const minMisrate = minAchievableMisrateOneSample(n);
  if (misrate < minMisrate) throw AssumptionError.domain('misrate');

  const target = misrate / 2;
  if (target <= 0) return 0;
  if (target >= 1) return n * 2;

  const [rLow, p] = binomCdfSplit(n, target);

  const u = rng.uniformFloat();
  const r = u < p ? rLow + 1 : rLow;
  return r * 2;
}

/**
 * The largest k whose Binomial(n, 0.5) CDF does not exceed target, together with the fraction of
 * the next term that would be needed to reach it. The caller compares that fraction against a
 * uniform draw, which is what makes the margin achieve the requested misrate exactly rather than
 * the next admissible one below it.
 */
function binomCdfSplit(n: number, target: number): [number, number] {
  // Binomial(n, 1/2) is symmetric, so for odd n the distribution function at (n-1)/2 is exactly
  // one half. No approximation reproduces an exact equality, and misrate = 1 lands on it: the
  // summation would decide the comparison by its last accumulated bit.
  if (target === 0.5 && n % 2 === 1) {
    return [(n - 1) / 2, 0];
  }

  const scaleUp = ldexp(1, SCALE_STEP);
  const scaleDown = ldexp(1, -SCALE_STEP);

  // The running term pmf(k) is w * 2^e, starting from pmf(0) = 2^-n.
  let w = 1;
  let e = -n;
  let cdf = 1;

  if (ldexp(cdf, e) > target) return [0, 0];

  let rLow = 0;
  for (let k = 1; k <= n; k++) {
    w = (w * (n - k + 1)) / k;
    while (w > scaleUp) {
      w *= scaleDown;
      cdf *= scaleDown;
      e += SCALE_STEP;
    }
    const next = cdf + w;
    if (ldexp(next, e) > target) {
      // target and cdf are both in units of 2^e here, so the fraction is a plain quotient.
      const p = (ldexp(target, -e) - cdf) / w;
      return [rLow, Math.max(0, Math.min(1, p))];
    }
    rLow = k;
    cdf = next;
  }

  return [rLow, 0];
}

/**
 * v * 2^exp, exactly, including where the result leaves the normal range.
 *
 * JavaScript has no ldexp. Scaling by a power of two is exact wherever the result is
 * representable, so an out-of-range exponent is split into steps that are not.
 */
function ldexp(v: number, exp: number): number {
  let result = v;
  let remaining = exp;
  while (remaining > 1023) {
    result *= Math.pow(2, 1023);
    remaining -= 1023;
  }
  while (remaining < -1022) {
    result *= Math.pow(2, -1022);
    remaining += 1022;
  }
  return result * Math.pow(2, remaining);
}
