namespace Pragmastat.Tests;

/// <summary>
/// Exact (bitwise) comparison of a produced value against a reference fixture value.
/// </summary>
/// <remarks>
/// <para>
/// "Exact" here means the raw IEEE 754 payload, never <c>==</c>. The two are different predicates
/// and neither is uniformly stronger: <c>-0.0 == +0.0</c> is true, so <c>==</c> passes a
/// sign-of-zero divergence that a bitwise claim forbids (a difference or an average of symmetric
/// values reaches either zero), while <c>NaN == NaN</c> is false, so <c>==</c> fails on two
/// identical NaN payloads that a bitwise claim allows. Comparing the payload is the one that
/// matches what these assertions say they check. Every exact comparison in the suite routes
/// through this type, so there is a single spelling of "these two values are identical".
/// </para>
/// <para>
/// The randomization contract is bitwise: <c>Rng(seed)</c> must produce an identical sequence in
/// every language implementation, and the manual states it. "Close enough" is therefore not the
/// property under test, and a tolerance reports a broken contract as a pass. A one-ULP drift is
/// exactly what a fused multiply-add on an arm64 runner produces, and a 1e-12 comparison never
/// sees it.
/// </para>
/// <para>
/// The estimators are bitwise for a different reason: each one selects its answer out of a finite
/// set built from the input (a pairwise average, a pairwise absolute difference, an order
/// statistic), so a divergence is never a small error. Either the same element was selected and
/// the answer is bit-identical, or a different one was, and then the gap is data-dependent and
/// no epsilon bounds it. This is measured, not assumed: recomputing every estimator with log, exp,
/// pow and cos returning the neighboring representable value (the largest difference two
/// conforming libm implementations can legitimately have) leaves center, spread, shift, disparity,
/// avg-spread, all of their bounds, compare1 and the margins unmoved on every input.
/// </para>
/// <para>
/// The remaining suites keep a tolerance because they are genuinely approximate: ratio and
/// ratio-bounds compute exp(median(log x - log y)), which the same perturbation moves on 94% of
/// inputs by up to 16 ULP; compare2 composes ratio projections alongside exact ones, and a
/// per-suite tolerance cannot say "exact for shift, approximate for ratio"; and the additive,
/// multiplic, exp and power distributions transform their draws with the platform libm. The RNG
/// and the uniform distribution use only integer arithmetic and exactly reproducible scaling, so
/// they have no such excuse.
/// </para>
/// </remarks>
public static class BitwiseAssert
{
  /// <summary>Compares against a fixture value addressed by file and position.</summary>
  public static void Equal(double expected, double actual, string filePath, int index) =>
    Equal(expected, actual, $"File: {Path.GetFileName(filePath)}, index {index}");

  /// <summary>Compares against a fixture value addressed by a caller-supplied label.</summary>
  public static void Equal(double expected, double actual, string context)
  {
    long expectedBits = BitConverter.DoubleToInt64Bits(expected);
    long actualBits = BitConverter.DoubleToInt64Bits(actual);
    if (expectedBits == actualBits)
      return;

    Assert.Fail(
      $"{context}: " +
      $"expected {expected:R} (0x{expectedBits:X16}), got {actual:R} (0x{actualBits:X16})");
  }

  /// <summary>Compares a binary32 draw against a fixture value addressed by file and position.</summary>
  public static void Equal(float expected, float actual, string filePath, int index) =>
    Equal(expected, actual, $"File: {Path.GetFileName(filePath)}, index {index}");

  /// <summary>Compares a binary32 draw against a fixture value addressed by a caller-supplied label.</summary>
  public static void Equal(float expected, float actual, string context)
  {
    int expectedBits = BitConverter.SingleToInt32Bits(expected);
    int actualBits = BitConverter.SingleToInt32Bits(actual);
    if (expectedBits == actualBits)
      return;

    Assert.Fail(
      $"{context}: " +
      $"expected {expected:R} (0x{expectedBits:X8}), got {actual:R} (0x{actualBits:X8})");
  }

  /// <summary>
  /// Compares two sequences element by element on their payloads. A sequence claim is the scalar
  /// claim repeated, so it gets the scalar predicate rather than a second one built from
  /// <c>==</c>; the reported context carries the index so a single divergent element is
  /// identifiable.
  /// </summary>
  public static void Equal(IReadOnlyList<double> expected, IReadOnlyList<double> actual, string context)
  {
    if (expected.Count != actual.Count)
      Assert.Fail($"{context}: expected {expected.Count} values, got {actual.Count}");

    for (int i = 0; i < expected.Count; i++)
      Equal(expected[i], actual[i], $"{context}, index {i}");
  }
}
