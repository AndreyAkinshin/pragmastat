import { logFunction } from './logFunction';

/** Half-width of the central band, in p - 1/2. */
const CENTRAL_HALF_WIDTH = 0.425;

/**
 * CENTRAL_HALF_WIDTH^2, the constant the central branch reduces by. r = CENTRAL_OFFSET - q*q is
 * then non-negative across the band and vanishes exactly at its edge.
 */
const CENTRAL_OFFSET = 0.180625;

/**
 * Where the tail splits. One approximant cannot cover w from 1.6 to 27 at a usable length:
 * measured, a single rational fit of degree (8, 8) over the whole tail reaches only 3.0e-10.
 */
const TAIL_SPLIT = 5.0;

/**
 * Computes Phi^-1(p), the value a standard normal falls below with probability p.
 *
 * IEEE 754 fixes nothing about this function, and a quantile is a value a caller compares and
 * reports, so it has to be the toolkit's own for the reason the exponential and the logarithm
 * already are.
 *
 * Three branches, each a rational approximant from tests/oracles/fit_additive_quantile.py:
 *
 *   central   |p - 1/2| <= 0.425     x = q * R1(r),  q = p - 1/2,  r = 0.180625 - q^2
 *   near tail w <= 5                 x = +-R2(w),    w = sqrt(-log(min(p, 1-p)))
 *   far tail  w > 5                  x = +-R3(w)
 *
 * The central form is odd in q by construction, so p = 1/2 returns exactly zero rather than a
 * cancellation of two nearly equal numbers. The tails run through the toolkit's logarithm, which
 * is why this primitive could not exist before that one did.
 *
 * Rational rather than polynomial, and three branches rather than one, because both were measured:
 * a degree-12 polynomial over the central band reaches 1.6e-07, and one fit across the whole tail
 * reaches 3.0e-10. The assembled function measures 7.7 ulp worst over a sweep of 320000 arguments
 * and 1.03 on average; the stored fixture is a set of regular grids and reaches only 4.4, so a
 * grid is not the instrument that reports this number.
 *
 * Nothing calls it yet; the distributions that need it move together.
 */
export function additiveQuantile(p: number): number {
  if (Number.isNaN(p)) {
    return p;
  }
  if (p <= 0.0) {
    return Number.NEGATIVE_INFINITY;
  }
  if (p >= 1.0) {
    return Number.POSITIVE_INFINITY;
  }

  const q = p - 0.5;
  if (Math.abs(q) <= CENTRAL_HALF_WIDTH) {
    const r = CENTRAL_OFFSET - q * q;
    let num = 3907.007639023816;
    num = num * r + 74060.81462563902;
    num = num * r + 214247.3801984238;
    num = num * r + 213550.16248438213;
    num = num * r + 95692.73551269759;
    num = num * r + 21627.48116049056;
    num = num * r + 2548.2248573022625;
    num = num * r + 148.75654077063152;
    num = num * r + 3.3871328727963665;
    let den = 9398.46451720659;
    den = den * r + 72342.69749284587;
    den = den * r + 140376.82723612545;
    den = den * r + 109244.70555458817;
    den = den * r + 41169.98166790856;
    den = den * r + 8176.878340636071;
    den = den * r + 871.2838940641986;
    den = den * r + 46.92338689522055;
    den = den * r + 1.0;
    return q * (num / den);
  }

  // Outside the central band the quantile is derived from the smaller tail, and the sign put back
  // afterwards. Working from min(p, 1-p) keeps the argument away from 1, where the subtraction
  // would throw away the significant digits the answer depends on.
  const lower = q < 0.0 ? p : 1.0 - p;
  const w = Math.sqrt(-logFunction(lower));

  let value: number;
  if (w <= TAIL_SPLIT) {
    let num = 0.054350362850595586;
    num = num * w + 1.5133041619767231;
    num = num * w + 12.302571927534103;
    num = num * w + 43.456109632558785;
    num = num * w + 75.71921394020897;
    num = num * w + 29.56505902784448;
    num = num * w - 57.880330387639276;
    num = num * w - 38.02928654598471;
    num = num * w - 3.5717440640448346;
    let den = 2.7706964359226424e-8;
    den = den * w + 0.03842783348289235;
    den = den * w + 1.0704443396791978;
    den = den * w + 8.80416037381848;
    den = den * w + 32.81079285831595;
    den = den * w + 66.49202694561696;
    den = den * w + 59.37032702530251;
    den = den * w + 17.47732083935553;
    den = den * w + 1.0;
    value = num / den;
  } else {
    let num = 8.392823453560843e-8;
    num = num * w + 2.1314331124416687e-5;
    num = num * w + 0.0016436514333316493;
    num = num * w + 0.051392875575643406;
    num = num * w + 0.715697208514527;
    num = num * w + 4.497647129161168;
    num = num * w + 12.072411664076757;
    num = num * w + 10.942951231396602;
    num = num * w - 3.792525101710398;
    num = num * w - 10.24280793582052;
    num = num * w - 3.009101693252925;
    let den = 8.578649609090119e-17;
    den = den * w + 5.934613897538525e-8;
    den = den * w + 1.5071576669864384e-5;
    den = den * w + 0.0011624636891466385;
    den = den * w + 0.03638643084310481;
    den = den * w + 0.5090592389031116;
    den = den * w + 3.2585635875673855;
    den = den * w + 9.433144981171784;
    den = den * w + 12.233693860928529;
    den = den * w + 6.82391943088949;
    den = den * w + 1.0;
    value = num / den;
  }

  return q < 0.0 ? -value : value;
}
