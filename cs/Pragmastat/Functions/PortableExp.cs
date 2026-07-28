namespace Pragmastat.Functions;

/// <summary>
/// The exponential every implementation evaluates, in place of the platform's.
/// </summary>
/// <remarks>
/// <para>
/// IEEE 754 fixes the result of each arithmetic operation and of the square root, and fixes
/// nothing about the exponential. Conforming libraries disagree in the last bit and do:
/// measured on one Edgeworth crossover, Go's software exp and glibc's return neighbouring
/// values, which moved the reported margin by two. Since a margin selects an order statistic,
/// that is a different confidence interval from the same inputs.
/// </para>
/// <para>
/// So the exponential cannot be the platform's. This one is built from operations the standard
/// does fix:
/// </para>
/// <para>
/// k = floor(y/ln2 + 1/2), r = (y - k*ln2Hi) - k*ln2Lo, exp(y) = 2^k * exp(r)
/// </para>
/// <para>
/// with exp(r) on |r| &lt;= ln2/2 from the polynomial in tests/oracles/fit_exp.py. Fitting
/// (exp(r) - 1 - r)/r^2 rather than exp(r) keeps the two leading terms exact and leaves the
/// polynomial supplying a correction below 0.07, so the assembled result stays within a
/// couple of ulp of the true exponential while being reproducible everywhere.
/// </para>
/// <para>
/// floor(x + 1/2) rather than a rounding function: Go rounds halves away from zero and R
/// rounds them to even, so naming a rounding is naming a disagreement.
/// </para>
/// </remarks>
internal static class PortableExp
{
  // Constants of the range reduction, emitted by tests/oracles/fit_exp.py.
  //
  // ln 2 is split so that k*Ln2Hi is exact: Ln2Hi carries 33 significant bits and |k| needs at
  // most 11, which leaves the product inside the 53 available. Without the split the reduction
  // would lose the low bits of r, and r is where the accuracy lives.
  private const double InvLn2 = 1.4426950408889634e+00;
  private const double Ln2Hi = 6.9314718036912382e-01;
  private const double Ln2Lo = 1.9082149292705877e-10;

  /// <summary>
  /// e raised to <paramref name="y"/>, identically on every platform.
  /// </summary>
  /// <param name="y">-infinity..+infinity</param>
  public static double Value(double y)
  {
    if (double.IsNaN(y))
      return y;
    // Past these the answer is not in doubt, and stating the cutoffs keeps the reduction
    // from having to produce a k it cannot scale by.
    if (y > 709.79)
      return double.PositiveInfinity;
    if (y < -745.2)
      return 0.0;

    double k = Floor(y * InvLn2 + 0.5);
    double r = (y - k * Ln2Hi) - k * Ln2Lo;

    double q = 1.6086622436215554e-10;
    q = q * r + 2.0918129454967065e-09;
    q = q * r + 2.5052071109214447e-08;
    q = q * r + 2.7557263301474753e-07;
    q = q * r + 2.7557319247204789e-06;
    q = q * r + 2.4801587336421322e-05;
    q = q * r + 1.9841269841263304e-04;
    q = q * r + 1.3888888888879082e-03;
    q = q * r + 8.3333333333333332e-03;
    q = q * r + 4.1666666666666678e-02;
    q = q * r + 1.6666666666666666e-01;
    q = q * r + 5.0000000000000000e-01;
    double p = 1.0 + r + r * r * q;

    // Two scalings rather than one. Splitting k in half keeps the first factor inside the
    // normal range whatever k is, so only the second can denormalise or overflow, and it
    // does so in a single rounding.
    int half = (int)k / 2;
    return p * Pow2(half) * Pow2((int)k - half);
  }

  /// <summary>
  /// Two raised to <paramref name="n"/>, exactly.
  /// </summary>
  /// <remarks>
  /// Math.ScaleB would say this, but it arrived in .NET Core 2.0 and this library still targets
  /// netstandard2.0, so it is not available on every target framework. Assembling the exponent
  /// field is available on all of them, is one code path rather than one per target, and is
  /// exact by construction: the cutoffs in <see cref="Value"/> hold k inside [-1075, 1024], so
  /// both halves land in [-538, 512] and the result is always a normal power of two.
  /// </remarks>
  private static double Pow2(int n) => BitConverter.Int64BitsToDouble((long)(n + 1023) << 52);
}
