namespace Pragmastat.Tests;

/// <summary>
/// Exact (bitwise) comparison of a produced double against a reference fixture value.
/// </summary>
/// <remarks>
/// <para>
/// The randomization contract is bitwise: <c>Rng(seed)</c> must produce an identical sequence in
/// every language implementation, and the manual states it. "Close enough" is therefore not the
/// property under test, and a tolerance reports a broken contract as a pass. A one-ULP drift is
/// exactly what a fused multiply-add on an arm64 runner produces, and a 1e-12 comparison never
/// sees it.
/// </para>
/// <para>
/// The estimator suites and the additive/multiplic/exp/power distributions stay tolerant on
/// purpose: those draws go through log, exp, cos and pow, which every platform takes from a
/// different libm, so two correctly rounded implementations legitimately disagree in the last
/// bit. The RNG and the uniform distribution use only integer arithmetic and exactly reproducible
/// scaling, so they have no such excuse.
/// </para>
/// </remarks>
public static class BitwiseAssert
{
  public static void Equal(double expected, double actual, string filePath, int index)
  {
    long expectedBits = BitConverter.DoubleToInt64Bits(expected);
    long actualBits = BitConverter.DoubleToInt64Bits(actual);
    if (expectedBits == actualBits)
      return;

    Assert.Fail(
      $"File: {Path.GetFileName(filePath)}, index {index}: " +
      $"expected {expected:R} (0x{expectedBits:X16}), got {actual:R} (0x{actualBits:X16})");
  }
}
