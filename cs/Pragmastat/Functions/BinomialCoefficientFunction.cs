using Pragmastat.Internal;

namespace Pragmastat.Functions;

// Internal, matching every other port: Go keeps this unexported, Rust pub(crate), Python
// under a leading underscore. It was public here alone, which is how a double-typed
// signature survived on a function whose arguments are sample sizes.
internal static class BinomialCoefficientFunction
{
  internal const int MaxAcceptableN = 62;

  private static readonly Lazy<long[,]> PascalTriangle = new(BuildPascalTriangle);

  private static long[,] BuildPascalTriangle()
  {
    checked
    {
      long[,] triangle = new long[MaxAcceptableN + 1, MaxAcceptableN + 1];
      for (int i = 0; i <= MaxAcceptableN; i++)
      {
        triangle[i, 0] = 1;
        for (int j = 1; j <= i; j++)
          triangle[i, j] = triangle[i - 1, j - 1] + triangle[i - 1, j];
      }

      return triangle;
    }
  }

  internal static long BinomialCoefficientExact(int n, int k)
  {
    if (n < 0 || n > MaxAcceptableN)
      throw new ArgumentOutOfRangeException(nameof(n));
    if (k < 0 || k > n)
      return 0;

    return PascalTriangle.Value[n, k];
  }

  /// <summary>
  /// C(n, k) in binary64.
  /// </summary>
  /// <remarks>
  /// Below <see cref="MaxAcceptableN"/> the exact integer from Pascal's triangle, converted
  /// once; above it the multiplicative recurrence C(n, k) = prod_{i=1..k} (n-k+i)/i.
  ///
  /// The arguments are integers because the callers pass sample sizes. They used to be
  /// doubles, which was a trap rather than a generalization: the body truncates, so a
  /// non-integral argument was silently answered for its floor. A type is the right place
  /// to say what a function accepts.
  ///
  /// The recurrence replaced an exp-of-Stirling formulation for two reasons. Against exact
  /// big-integer binomials over 4 &lt;= n &lt;= 400, Stirling missed the nearest double on 99.9%
  /// of cases with a worst relative error of 8.1e-9; the recurrence is within 2.2e-15. And
  /// Stirling reached the answer through Log and Exp, which every language takes from a
  /// different library: perturbing those by a single ulp moved the computed misrate floor on
  /// 75579 of 79797 sample-size pairs, and moves none of them now.
  ///
  /// Normalizing k to the smaller half makes the function symmetric by construction, which
  /// matters because the two call sites ask for C(n+m, n) and C(n+m, m). Those are the same
  /// number, and now they are also the same bits, so the comparison at the misrate floor
  /// cannot be decided by which of the two rounded higher.
  ///
  /// Every step is one binary64 multiply followed by one binary64 divide. Do not reassociate
  /// it and do not accumulate numerator and denominator separately: all seven implementations
  /// perform the identical sequence, and that is the property being preserved.
  /// </remarks>
  internal static double BinomialCoefficient(int n, int k)
  {
    Assertion.Positive(nameof(n), n);
    if (k < 0 || k > n)
      throw new ArgumentOutOfRangeException(nameof(k));

    if (n < MaxAcceptableN)
      return BinomialCoefficientExact(n, k);

    int kk = k;
    if (kk > n - kk)
      kk = n - kk;

    double acc = 1.0;
    for (int i = 1; i <= kk; i++)
    {
      acc = acc * (double)(n - kk + i) / (double)i;
      // Once the accumulator reaches infinity the remaining steps cannot bring it back: each
      // one multiplies by a positive integer and divides by a positive integer. Stopping there
      // is the same sequence of roundings, arrived at sooner: at n = m = 100000 it is 89 steps
      // instead of 100000.
      if (double.IsPositiveInfinity(acc))
        break;
    }

    return acc;
  }
}
