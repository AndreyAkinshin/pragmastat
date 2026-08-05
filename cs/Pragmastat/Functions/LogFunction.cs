namespace Pragmastat.Functions;

/// <summary>
/// The natural logarithm this toolkit evaluates, in place of the platform's.
///
/// IEEE 754 fixes the result of each arithmetic operation and of the square root, and fixes
/// nothing about the logarithm. The reason this one has to be ours is stronger than for most
/// transcendentals: Ratio is exp(median(log x - log y)), so a last-bit difference in log moves
/// the median's input and changes which pairwise element is selected. That is a different answer,
/// not a small error in the same one.
///
/// The constants and the polynomial come from tests/oracles/fit_log.py, which documents the
/// reduction and what is fitted. Which of the two range endpoints a port names depends on how it
/// splits x: a library frexp returns a significand in [0.5, 1) and the reduction doubles the low
/// end, while reading the exponent field gives one in [1, 2) and it halves the high end. Both
/// arrive at the same [sqrt(2)/2, sqrt(2)) and the seven agree bit for bit; only the endpoint each
/// has to name differs.
/// </summary>
internal static class LogFunction
{
  // Constants of the range reduction. ln 2 is split so that k*Ln2Hi is exact: it carries 32
  // significant bits and |k| needs at most 11, which leaves the product inside the 53 available.
  private const double Ln2Hi = 0.6931471803691238;
  private const double Ln2Lo = 1.9082149292705877e-10;
  private const double Sqrt2 = 1.4142135623730951;

  // Chebyshev fit of (log((1+s)/(1-s)) - 2*s) / s^3 over the reduced range. The leading terms are
  // the recognizable 2/3, 2/5, 2/7 of the odd series doubled, which is where the assembly's
  // missing factor of two lives; the tail departs from the series because the fit minimizes the
  // worst error rather than matching Taylor.
  private static readonly double[] PolyP =
  [
    0.666666666666667,
    0.39999999999899505,
    0.28571428625975487,
    0.2222221113479508,
    0.18182889125261723,
    0.15331721600556042,
    0.14616449685043406
  ];

  /// <summary>
  /// Returns the natural logarithm of <paramref name="x"/>.
  ///
  /// The special cases follow IEEE 754 and match what every platform returns for them, since they
  /// are the cases a platform has no freedom about.
  /// </summary>
  public static double Value(double x)
  {
    if (double.IsNaN(x))
      return x;
    if (x < 0)
      return double.NaN;
    if (x == 0)
      return double.NegativeInfinity;
    if (double.IsPositiveInfinity(x))
      return double.PositiveInfinity;
    if (x == 1)
      return 0;

    // The exponent is read out of the bit pattern rather than through Math.ILogB and Math.ScaleB,
    // which netstandard2.0 does not carry. Building against it is the reason to spell this out:
    // cs:test runs on a later target where both exist, so a call to either compiles and passes the
    // tests while cs:build fails on the shipped framework.
    //
    // Subnormals are scaled into the normal range first, since their exponent field holds no
    // information. Scaling by a power of two is exact.
    double value = x;
    int k = 0;
    if (value < 2.2250738585072014e-308)
    {
      value *= 1.8014398509481984e16;
      k -= 54;
    }

    long bits = BitConverter.DoubleToInt64Bits(value);
    k += (int)((bits >> 52) & 0x7ff) - 1023;
    // Replace the exponent field with the one for [1, 2).
    double m = BitConverter.Int64BitsToDouble((bits & 0x000fffffffffffffL) | 0x3ff0000000000000L);

    // Recenter on [sqrt(2)/2, sqrt(2)) so |f| stays small at both ends. Halving is exact.
    if (m >= Sqrt2)
    {
      m *= 0.5;
      k += 1;
    }

    double f = m - 1;
    double s = f / (2 + f);
    double z = s * s;

    double acc = PolyP[PolyP.Length - 1];
    for (int i = PolyP.Length - 2; i >= 0; i--)
      acc = acc * z + PolyP[i];

    double r = z * acc;
    double hfsq = 0.5 * f * f;
    double kf = k;

    if (k == 0)
      return f - (hfsq - s * (hfsq + r));
    return kf * Ln2Hi - (hfsq - (s * (hfsq + r) + kf * Ln2Lo) - f);
  }
}
