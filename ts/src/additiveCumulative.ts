/**
 * Standard normal CDF.
 */

import { expFunction } from './expFunction';

/**
 * Computes the standard normal CDF.
 *
 * Two Chebyshev-fitted Horner chains and one exponential. The coefficients are produced by
 * tests/oracles/fit_additive_cumulative.py against a reference good to 36 digits, so they are
 * reproducible rather than transcribed.
 *
 * The exponential is expFunction rather than Math.exp: the platform's differs between
 * runtimes in the last bit, and this value reaches a comparison that selects an integer.
 *
 * No product needs pinning here: ECMAScript specifies every multiply and add as a separate
 * correctly-rounded binary64 operation, so an implementation may not fuse them.
 *
 * @param x -infinity..+infinity
 * @returns Area under the standard normal curve from -infinity to x
 */
export function additiveCumulative(z: number): number {
  const t = Math.abs(z) / Math.SQRT2;
  if (t < 0.5) {
    const s = t * t;
    const u = 8.0 * s - 1.0;
    let p = -1.2757552949301143e-19;
    p = p * u + 1.2307154179828511e-17;
    p = p * u - 1.0890239994332592e-15;
    p = p * u + 8.774530700097397e-14;
    p = p * u - 6.3744178527620835e-12;
    p = p * u + 4.1270254211564467e-10;
    p = p * u - 2.347229163519518e-8;
    p = p * u + 1.151603779513705e-6;
    p = p * u - 4.762336934468491e-5;
    p = p * u + 0.0016130716680617086;
    p = p * u - 0.04364205888669792;
    p = p * u + 1.0830752376761712;
    const erf = t * p;
    return z >= 0 ? 0.5 * (1.0 + erf) : 0.5 * (1.0 - erf);
  }

  let erfc = 0.0;
  if (t <= 4.3) {
    const u = (2.0 * t - 4.8) / 3.8;
    let p = 2.403093649825437e-9;
    p = p * u - 6.533436159455495e-9;
    p = p * u + 1.334437871983186e-9;
    p = p * u - 2.5055474016226743e-9;
    p = p * u + 5.2376178949357336e-8;
    p = p * u - 1.341394638617228e-7;
    p = p * u + 2.5376572107855777e-7;
    p = p * u - 6.147631059669139e-7;
    p = p * u + 1.561533370779237e-6;
    p = p * u - 3.688982809059467e-6;
    p = p * u + 8.492013869441648e-6;
    p = p * u - 1.9344330869926753e-5;
    p = p * u + 4.3285002216779125e-5;
    p = p * u - 9.489727696113043e-5;
    p = p * u + 0.0002037912849869451;
    p = p * u - 0.0004282777524202283;
    p = p * u + 0.00087969639542425;
    p = p * u - 0.001763698443638436;
    p = p * u + 0.0034462452415540026;
    p = p * u - 0.00655166763664565;
    p = p * u + 0.012094345026186722;
    p = p * u - 0.021629099761798037;
    p = p * u + 0.037371670355588804;
    p = p * u - 0.06218492139115531;
    p = p * u + 0.09925390090168178;
    p = p * u - 0.15121195850373031;
    p = p * u + 0.21849873453703333;
    erfc = expFunction(-(t * t)) * p;
  }

  return z >= 0 ? 1.0 - 0.5 * erfc : 0.5 * erfc;
}
