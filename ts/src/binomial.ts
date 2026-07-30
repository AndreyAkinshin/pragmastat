/**
 * Binomial coefficients for the misrate floor and the exact pairwise-margin distribution.
 *
 * Both call sites want C(n+m, k) for the same n+m: the admissible misrate is
 * `2 / C(n+m, n)` and the Loeffler recurrence normalizes its pmf by `C(n+m, m)`.
 * They share one entry point so they cannot drift onto different routes, which is
 * what makes the admissibility check agree with the distribution it guards.
 */

/**
 * Below this total, C(n, k) fits the 64-bit integers the other six ports use, so every
 * implementation returns the exactly rounded value; at or above it they all switch to the
 * binary64 multiplicative recurrence. BigInt would not overflow here, but the threshold is
 * a cross-language contract rather than an implementation limit: computing more of the
 * range exactly than the other ports do would put this one on a different misrate floor.
 */
const MAX_ACCEPTABLE_BINOM_N = 62;

/**
 * Computes C(n, k) as a binary64, by the route all seven implementations agree on.
 */
export function binomialCoefficient(n: number, k: number): number {
  if (k > n) {
    return 0;
  }
  return n < MAX_ACCEPTABLE_BINOM_N ? exact(n, k) : recurrence(n, k);
}

/**
 * Computes C(n, k) in exact integer arithmetic, then rounds once.
 *
 * BigInt because the range this path serves leaves 2^53 behind: C(56, 27) alone overflows
 * a Number accumulator and returned a pairwise margin of 784 where 782 is correct.
 */
function exact(n: number, k: number): number {
  if (k === 0 || k === n) {
    return 1;
  }

  k = Math.min(k, n - k); // Take advantage of symmetry
  let result = 1n; // exact integer arithmetic: each partial product is divisible

  for (let i = 0; i < k; i++) {
    result = (result * BigInt(n - i)) / BigInt(i + 1);
  }

  return Number(result);
}

/**
 * Computes C(n, k) in binary64 by the multiplicative recurrence
 * C(n, k) = prod_{i=1..k} (n-k+i)/i.
 *
 * It replaced an exp-of-Stirling formulation, for two reasons. The specification defines the
 * admissible misrate as `misrate >= 2 / C(n+m, n)`, an exact integer quantity; measured here
 * against exact BigInt binomials over 4 <= n <= 400, Stirling was inexact on 98.9% of cases
 * with a worst relative error of 1.6e-8, while this recurrence is inexact on 74.9% with a
 * worst relative error of 2.3e-15. And Stirling reached the answer through Math.log and
 * Math.exp, which every language takes from a different libm: perturbing those by a single
 * ulp moved the computed misrate floor on 75579 of 79797 sample-size pairs. This form calls
 * nothing, so the same perturbation moves none of them.
 *
 * Normalizing k to the smaller half also makes the function symmetric by construction, which
 * matters because the two call sites ask for C(n+m, n) and C(n+m, m). Those are the same
 * number, and now they are also the same bits, so the comparison at the misrate floor cannot
 * be decided by which of the two rounded higher.
 *
 * Every step is one multiply followed by one divide. Do not reassociate it, do not accumulate
 * a numerator and a denominator separately: all seven implementations perform this identical
 * sequence, and that is the property being preserved.
 *
 * The sequence overflows early, and deliberately so. The intermediate `acc * (n-k+i)` runs up
 * to n/2 times above the final value, so the central binomial returns +Inf from n = 1021 while
 * the exact value stays inside binary64 until n = 1030. Across that window the misrate floor
 * `2/C` collapses from around 7e-307 to 0, which only a misrate below 1e-306 could tell apart.
 * Deferring the divide to keep the intermediate small would be a different sequence of
 * roundings, and only this one is shared.
 */
function recurrence(n: number, k: number): number {
  if (k > n - k) {
    k = n - k;
  }
  let acc = 1.0;
  for (let i = 1; i <= k; i++) {
    acc = (acc * (n - k + i)) / i;
    // Once the accumulator reaches infinity the remaining steps cannot bring it back: each one
    // multiplies by a positive integer and divides by a positive integer. Stopping there is the
    // same sequence of roundings, arrived at sooner: at n = m = 100000 it is 89 steps instead
    // of 100000.
    if (!Number.isFinite(acc)) {
      break;
    }
  }
  return acc;
}
