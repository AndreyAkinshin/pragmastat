namespace Pragmastat.Internal;

/// <summary>
/// Binary64 primitives shared by the distribution estimation models.
/// </summary>
/// <remarks>
/// Every operation here is written out explicitly so that all seven language implementations
/// evaluate the same graph and produce identical bits. None of these helpers may be replaced by
/// a fused, vectorized or library variant without a new algorithm identifier.
/// </remarks>
internal static class DistNumerics
{
  /// <summary>
  /// Next representable binary64 value toward positive infinity.
  /// </summary>
  /// <remarks>
  /// Implemented on the bit pattern rather than through <c>Math.BitIncrement</c>, which is
  /// unavailable on netstandard2.0.
  /// </remarks>
  public static double NextUp(double value)
  {
    if (double.IsNaN(value) || double.IsPositiveInfinity(value))
      return value;
    if (value == 0.0)
      return Constants.SmallestPositiveSubnormal;

    long bits = BitConverter.DoubleToInt64Bits(value);
    bits += value > 0.0 ? 1 : -1;
    return BitConverter.Int64BitsToDouble(bits);
  }

  /// <summary>
  /// Next representable binary64 value toward negative infinity.
  /// </summary>
  public static double NextDown(double value)
  {
    if (double.IsNaN(value) || double.IsNegativeInfinity(value))
      return value;
    if (value == 0.0)
      return -Constants.SmallestPositiveSubnormal;

    long bits = BitConverter.DoubleToInt64Bits(value);
    bits += value > 0.0 ? -1 : 1;
    return BitConverter.Int64BitsToDouble(bits);
  }

  /// <summary>
  /// Overflow-safe midpoint of two finite values.
  /// </summary>
  public static double Midpoint(double a, double b)
  {
    if (a < 0.0 && 0.0 < b)
      return a / 2.0 + b / 2.0;
    return a + (b - a) / 2.0;
  }

  /// <summary>
  /// Interpolates between <paramref name="a"/> and <paramref name="b"/>, reproducing both
  /// endpoints exactly.
  /// </summary>
  public static double Lerp(double a, double b, double t)
  {
    if (t == 0.0)
      return a;
    if (t == 1.0)
      return b;
    if (a < 0.0 && 0.0 < b)
      return (1.0 - t) * a + t * b;
    return a + t * (b - a);
  }

  /// <summary>
  /// Ratio clamped to the unit interval. Callers must have proved <c>0 &lt;= num &lt;= den</c>
  /// and <c>den &gt; 0</c>.
  /// </summary>
  public static double UnitRatio(double num, double den)
  {
    double t = num / den;
    if (double.IsNaN(t))
      throw new InvalidOperationException("Numerical conformance failure: UnitRatio produced NaN");
    return Min(1.0, Max(0.0, t));
  }

  /// <summary>
  /// Smallest integer k with <c>k &gt;= ceil(n / k)</c>, computed without forming <c>k*k</c>.
  /// </summary>
  public static int CeilSqrt(int n)
  {
    int k = 1;
    while (k < (n + k - 1) / k)
      k++;
    return k;
  }

  /// <summary>
  /// First index i in <c>[from, to)</c> with <c>values[i] &gt;= key</c>, over an ascending array.
  /// </summary>
  public static int LowerBound(IReadOnlyList<double> values, int from, int to, double key)
  {
    int lo = from, hi = to;
    while (lo < hi)
    {
      int mid = lo + (hi - lo) / 2;
      if (values[mid] < key)
        lo = mid + 1;
      else
        hi = mid;
    }

    return lo;
  }

  /// <summary>
  /// First index i with <c>values[i] &gt; key</c>, over an ascending array.
  /// </summary>
  public static int UpperBound(IReadOnlyList<double> values, double key)
  {
    int lo = 0, hi = values.Count;
    while (lo < hi)
    {
      int mid = lo + (hi - lo) / 2;
      if (values[mid] <= key)
        lo = mid + 1;
      else
        hi = mid;
    }

    return lo;
  }
}
