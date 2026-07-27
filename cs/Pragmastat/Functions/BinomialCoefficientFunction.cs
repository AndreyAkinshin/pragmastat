using Pragmastat.Internal;

namespace Pragmastat.Functions;

public static class BinomialCoefficientFunction
{
  public const int MaxAcceptableN = 62;

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

  public static long BinomialCoefficient(int n, int k)
  {
    if (n < 0 || n > MaxAcceptableN)
      throw new ArgumentOutOfRangeException(nameof(n));
    if (k < 0 || k > n)
      return 0;

    return PascalTriangle.Value[n, k];
  }

  /// <summary>
  /// C(n, k) in binary64 by the multiplicative recurrence C(n, k) = prod_{i=1..k} (n-k+i)/i.
  /// Both arguments are integral by contract: the callers pass sample sizes.
  /// </summary>
  /// <remarks>
  /// It replaced an exp(LogFactorial) formulation, for three reasons. The specification defines
  /// the admissible misrate as `misrate &gt;= 2 / C(n+m, n)`, an exact integer quantity; measured
  /// against exact BigInteger binomials over all 79797 pairs with 4 &lt;= n+m &lt;= 400, the
  /// Stirling path missed the nearest double on 99.9% of them with a worst relative error of
  /// 9.5e-13, while this recurrence misses on 75.6% with a worst relative error of 2.3e-15.
  ///
  /// It is also portable. Stirling reached the answer through Log and Exp, which every language
  /// takes from a different libm, so the last bit of the misrate floor was a property of the host
  /// runtime rather than of the specification. This form calls nothing.
  ///
  /// Normalizing k to the smaller half makes the function symmetric by construction, which matters
  /// because the two call sites ask for C(n+m, n) and C(n+m, m). Those are the same number, yet
  /// the Stirling path returned different bits for them on 59052 of those 79797 pairs, leaving the
  /// comparison at the misrate floor to be decided by which of the two rounded higher. This form
  /// returns the same bits for both.
  ///
  /// Every step is one binary64 multiply followed by one binary64 divide. Do not reassociate it,
  /// do not accumulate numerator and denominator separately: all seven implementations perform
  /// the identical sequence, and that is the property being preserved.
  /// </remarks>
  public static double BinomialCoefficient(double n, double k)
  {
    Assertion.Positive(nameof(n), n);
    Assertion.InRangeInclusive(nameof(k), k, 0, n);

    int nn = (int)n;
    int kk = (int)k;
    if (kk > nn - kk)
      kk = nn - kk;

    double acc = 1.0;
    for (int i = 1; i <= kk; i++)
      acc = acc * (double)(nn - kk + i) / (double)i;
    return acc;
  }
}
