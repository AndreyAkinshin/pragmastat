/**
 * PairwiseMargin function for computing confidence bound margins
 *
 * Determines how many extreme pairwise differences to exclude when constructing bounds
 * based on the distribution of dominance statistics.
 */

const MAX_EXACT_SIZE = 400;
const MAX_ACCEPTABLE_BINOM_N = 65;

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
 * @throws Error if n <= 0, m <= 0, or misrate is outside [0, 1]
 */
export function pairwiseMargin(n: number, m: number, misrate: number): number {
  if (n <= 0) {
    throw new Error('n must be positive');
  }
  if (m <= 0) {
    throw new Error('m must be positive');
  }
  if (misrate < 0 || misrate > 1) {
    throw new Error('misrate must be in range [0, 1]');
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
  const total =
    n + m < MAX_ACCEPTABLE_BINOM_N
      ? binomialCoefficient(n + m, m)
      : binomialCoefficientFloat(n + m, m);

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
  const z = (u - mu - 0.5) / su;

  // Standard normal PDF and CDF
  const phi = Math.exp((-z * z) / 2) / Math.sqrt(2 * Math.PI);
  const bigPhi = gauss(z);

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

/**
 * Computes the standard normal CDF using ACM Algorithm 209
 *
 * Calculates (1/sqrt(2*pi)) * integral from -infinity to x of e^(-u^2/2) du
 * by means of polynomial approximations due to A. M. Murray of Aberdeen University.
 *
 * See: http://dl.acm.org/citation.cfm?id=367664
 *
 * @param x -infinity..+infinity
 * @returns Area under the Standard Normal Curve from -infinity to x
 */
function gauss(x: number): number {
  let z: number;
  if (Math.abs(x) < 1e-9) {
    z = 0.0;
  } else {
    let y = Math.abs(x) / 2;
    if (y >= 3.0) {
      z = 1.0;
    } else if (y < 1.0) {
      const w = y * y;
      z =
        ((((((((0.000124818987 * w - 0.001075204047) * w + 0.005198775019) * w - 0.019198292004) *
          w +
          0.059054035642) *
          w -
          0.151968751364) *
          w +
          0.319152932694) *
          w -
          0.5319230073) *
          w +
          0.797884560593) *
        y *
        2.0;
    } else {
      y = y - 2.0;
      z =
        (((((((((((((-0.000045255659 * y + 0.00015252929) * y - 0.000019538132) * y -
          0.000676904986) *
          y +
          0.001390604284) *
          y -
          0.00079462082) *
          y -
          0.002034254874) *
          y +
          0.006549791214) *
          y -
          0.010557625006) *
          y +
          0.011630447319) *
          y -
          0.009279453341) *
          y +
          0.005353579108) *
          y -
          0.002141268741) *
          y +
          0.000535310849) *
          y +
        0.999936657524;
    }
  }

  return x > 0.0 ? (z + 1.0) / 2 : (1.0 - z) / 2;
}

/**
 * Computes binomial coefficient C(n, k) using integer arithmetic
 */
function binomialCoefficient(n: number, k: number): number {
  if (k > n) {
    return 0;
  }
  if (k === 0 || k === n) {
    return 1;
  }

  k = Math.min(k, n - k); // Take advantage of symmetry
  let result = 1;

  for (let i = 0; i < k; i++) {
    result = (result * (n - i)) / (i + 1);
  }

  return result;
}

/**
 * Computes binomial coefficient using floating-point logarithms for large values
 */
function binomialCoefficientFloat(n: number, k: number): number {
  if (k > n) {
    return 0;
  }
  if (k === 0 || k === n) {
    return 1;
  }

  k = Math.min(k, n - k); // Take advantage of symmetry

  // Use log-factorial function: C(n, k) = exp(log(n!) - log(k!) - log((n-k)!))
  const logResult = logFactorial(n) - logFactorial(k) - logFactorial(n - k);
  return Math.exp(logResult);
}

/**
 * Computes the natural logarithm of n!
 */
function logFactorial(n: number): number {
  if (n === 0 || n === 1) {
    return 0;
  }

  const x = n + 1; // n! = Gamma(n+1)

  if (x < 1e-5) {
    return 0;
  }

  // DONT TOUCH: Stirling's approximation is inaccurate for small x.
  // Use Gamma recurrence: Gamma(x) = Gamma(x+k) / (x*(x+1)*...*(x+k-1))
  // These branches appear unreachable in current usage (n+m >= 65), but
  // are retained for correctness if the function is used in other contexts.
  if (x < 1) {
    return stirlingApproxLog(x + 3) - Math.log(x * (x + 1) * (x + 2));
  }
  if (x < 2) {
    return stirlingApproxLog(x + 2) - Math.log(x * (x + 1));
  }
  if (x < 3) {
    return stirlingApproxLog(x + 1) - Math.log(x);
  }

  return stirlingApproxLog(x);
}

/**
 * Stirling's approximation with Bernoulli correction
 */
function stirlingApproxLog(x: number): number {
  let result = x * Math.log(x) - x + Math.log((2 * Math.PI) / x) / 2;

  // Bernoulli correction series
  const B2 = 1.0 / 6.0;
  const B4 = -1.0 / 30.0;
  const B6 = 1.0 / 42.0;
  const B8 = -1.0 / 30.0;
  const B10 = 5.0 / 66.0;

  const x2 = x * x;
  const x3 = x2 * x;
  const x5 = x3 * x2;
  const x7 = x5 * x2;
  const x9 = x7 * x2;

  result += B2 / (2 * x) + B4 / (12 * x3) + B6 / (30 * x5) + B8 / (56 * x7) + B10 / (90 * x9);

  return result;
}
