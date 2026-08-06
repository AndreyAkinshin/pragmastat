using JetBrains.Annotations;
using Pragmastat.Functions;
using Pragmastat.Internal;
using Pragmastat.TestGenerator.Framework;
using Pragmastat.TestGenerator.Framework.SingleDoubleValue;

namespace Pragmastat.Tests.Functions;

/// <summary>
/// The reproducible inverse normal CDF, checked by payload against the shared fixture and against the
/// claims the reduction has to satisfy on its own.
/// </summary>
/// <remarks>
/// <para>
/// Nothing in the library reaches this function yet, so unlike the exponential it is not exercised
/// even indirectly through a margin. The shared suite is the only place the seven implementations
/// meet on it, and the agreement has to already hold on the day the callers move into log space,
/// or the estimator fixtures would disagree by construction from that day on.
/// </para>
/// <para>
/// Compared bitwise, not within a tolerance, because the suite exists to catch a last-bit
/// difference: the manifest declares it exact, and a quantile that inverts a cumulative function
/// built on this turns one bit here into a different answer rather than a small error.
/// </para>
/// <para>
/// What the fixture cannot carry is checked here instead: the values IEEE 754 fixes, which JSON
/// has no way to write, and the constants, which are transcribed and are therefore the one part of
/// a fitted function that a typo can break silently.
/// </para>
/// <para>
/// Agreement with the platform is a smell test rather than the contract, and is asserted as such:
/// a few ulp anywhere is a different fit, not a different rounding.
/// </para>
/// </remarks>
public class AdditiveQuantileTests
{
  private const string SuiteName = "additive-quantile";

  // The size of the generated fixture: five files, 547 arguments. Pinned rather than counted from
  // whatever the loader happened to find, because a count derived from the loader's own result
  // cannot report the failure it is here for: a loader that reads no files makes no comparisons
  // and passes every assertion it never makes.
  private const int FixtureFileCount = 5;
  private const int FixtureArgumentCount = 547;

  // eps: 0 so that the controller cannot be used loosely by a later edit; the comparisons below
  // route through BitwiseAssert, which reports the divergent index and both payloads.
  private readonly SingleDoubleValueController controller = new(
    SuiteName,
    new Dictionary<string, Func<double, double>> { ["additive_quantile"] = AdditiveQuantile.Value },
    eps: 0,
    shared: true);

  [UsedImplicitly]
  public static readonly TheoryData<string> TestDataNames =
    ReferenceTestSuiteHelper.GetTheoryData(SuiteName, shared: true);

  [Theory]
  [MemberData(nameof(TestDataNames))]
  public void AdditiveQuantileTest(string testName)
  {
    var testCase = controller.LoadTestCase(testName);
    var actual = controller.Run(testCase.Input);
    BitwiseAssert.Equal(testCase.Output, actual, $"Suite {SuiteName}, file {testName}.json");
  }

  /// <summary>
  /// Asserts that the suite is the size the generator produces, so that an empty or partial read
  /// is a failure rather than a silent pass.
  /// </summary>
  [Fact]
  public void SuiteCoversEveryGeneratedArgument()
  {
    string directory = ReferenceTestSuiteHelper.GetTestSuiteDirectory(SuiteName, shared: true);
    string[] testNames = Directory
      .GetFiles(directory, "*.json", SearchOption.TopDirectoryOnly)
      .Select(Path.GetFileNameWithoutExtension)
      .ToArray()!;

    Assert.Equal(FixtureFileCount, testNames.Length);
    Assert.Equal(FixtureArgumentCount, testNames.Sum(name => controller.LoadTestCase(name).Input.Arg.Length));
  }

  /// <summary>
  /// The split of ln 2 has to be the same one the exponential carries, or two functions that both
  /// claim to be reproducible disagree about the constant they reduce by.
  /// </summary>

  /// <summary>The values the definition fixes, none of which may reach an approximant.</summary>
  [Fact]
  public void SpecialValues()
  {
    Assert.Equal(0.0, AdditiveQuantile.Value(0.5));
    Assert.False(double.IsNegative(AdditiveQuantile.Value(0.5)));
    Assert.Equal(double.NegativeInfinity, AdditiveQuantile.Value(0.0));
    Assert.Equal(double.PositiveInfinity, AdditiveQuantile.Value(1.0));
    Assert.Equal(double.NegativeInfinity, AdditiveQuantile.Value(-0.1));
    Assert.Equal(double.PositiveInfinity, AdditiveQuantile.Value(1.1));
    Assert.True(double.IsNaN(AdditiveQuantile.Value(double.NaN)));
  }

  /// <summary>The quantiles a reader is most likely to check by hand.</summary>
  [Theory]
  [InlineData(0.75, 0.674489750196082)]
  [InlineData(0.9, 1.2815515655446004)]
  [InlineData(0.95, 1.6448536269514722)]
  [InlineData(0.975, 1.959963984540054)]
  [InlineData(0.99, 2.3263478740408408)]
  [InlineData(0.995, 2.5758293035489004)]
  [InlineData(0.999, 3.090232306167813)]
  public void TextbookValues(double p, double want)
  {
    double got = AdditiveQuantile.Value(p);
    Assert.True(Math.Abs(got - want) <= 8.0 * 2.220446049250313e-16 * Math.Abs(want),
      $"Phi^-1({p}) = {got}, expected about {want}");
  }

  /// <summary>
  /// The central branch is exactly odd about 1/2, because it consumes q = p - 1/2 directly.
  /// </summary>
  /// <remarks>
  /// The tails are not, and asserting it would assert something false: they work from
  /// min(p, 1-p), and 1.0 - 0.95 is 0.050000000000000044 rather than 0.05. The band edge is
  /// excluded for a related reason, since 0.5 + 0.425 - 0.5 returns 0.42500000000000004.
  /// </remarks>
  [Theory]
  [InlineData(0.25)]
  [InlineData(0.125)]
  [InlineData(0.0625)]
  [InlineData(0.4)]
  public void CentralBranchIsOddAboutTheCenter(double q)
  {
    Assert.Equal(q, 0.5 + q - 0.5);
    Assert.Equal(-AdditiveQuantile.Value(0.5 - q), AdditiveQuantile.Value(0.5 + q));
  }

  /// <summary>
  /// Monotone, and reaching the far tail rather than saturating. The smallest positive double is
  /// the last argument a caller can pass, and its quantile is finite.
  /// </summary>
  [Fact]
  public void MonotoneIntoTheFarTail()
  {
    double previous = double.NegativeInfinity;
    for (double p = double.Epsilon; p < 0.5; p *= 8.0)
    {
      double got = AdditiveQuantile.Value(p);
      Assert.True(got > previous, $"not monotone at p = {p}");
      Assert.True(double.IsFinite(got), $"not finite at p = {p}");
      previous = got;
    }
    Assert.True(Math.Abs(AdditiveQuantile.Value(double.Epsilon) - -38.467405617144) < 1e-9);
  }
}
