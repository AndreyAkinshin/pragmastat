using Pragmastat.Distributions;
using Pragmastat.Randomization;
using Pragmastat.TestGenerator.Framework;
using Pragmastat.TestGenerator.Framework.Distributions;

namespace Pragmastat.Tests.Distributions;

/// <summary>
/// The five parametric families against their shared dist-* fixtures.
/// </summary>
/// <remarks>
/// <para>
/// Each family answers three questions about itself, and each answer is assembled out of the
/// toolkit's own exponential, logarithm, normal cumulative and normal quantile. Nothing on these
/// paths reaches a platform transcendental, so the seven ports owe each other identical bits and
/// the suites are declared exact in tests/manifest.json: the controller compares by binary64
/// payload. A tolerance would pass on exactly the divergence these suites exist to catch.
/// </para>
/// <para>
/// This replaces two classes that read cs/tests/distribution-normal and
/// cs/tests/distribution-uniform: two families out of five, compared at 1e-9, and visible to no
/// other port.
/// </para>
/// </remarks>
public class DistFamilyReferenceTests
{
  public static readonly TheoryData<string> UniformCases = ReferenceTestSuiteHelper.GetTheoryData("dist-uniform", true);
  public static readonly TheoryData<string> ExpCases = ReferenceTestSuiteHelper.GetTheoryData("dist-exp", true);
  public static readonly TheoryData<string> AdditiveCases =
    ReferenceTestSuiteHelper.GetTheoryData("dist-additive", true);
  public static readonly TheoryData<string> MultiplicCases =
    ReferenceTestSuiteHelper.GetTheoryData("dist-multiplic", true);
  public static readonly TheoryData<string> PowerCases = ReferenceTestSuiteHelper.GetTheoryData("dist-power", true);

  private static void Check<TDistribution>(string suiteName, string testName)
    where TDistribution : IDistribution
  {
    var controller = new DistributionController<TDistribution>(suiteName);
    var testCase = controller.LoadTestCase(testName);
    var actual = controller.Run(testCase.Input);
    Assert.True(controller.Assert(testCase.Output, actual), $"{suiteName}/{testName}");
  }

  [Theory]
  [MemberData(nameof(UniformCases))]
  public void UniformReference(string testName) => Check<Uniform>("dist-uniform", testName);

  [Theory]
  [MemberData(nameof(ExpCases))]
  public void ExpReference(string testName) => Check<Exp>("dist-exp", testName);

  [Theory]
  [MemberData(nameof(AdditiveCases))]
  public void AdditiveReference(string testName) => Check<Additive>("dist-additive", testName);

  [Theory]
  [MemberData(nameof(MultiplicCases))]
  public void MultiplicReference(string testName) => Check<Multiplic>("dist-multiplic", testName);

  [Theory]
  [MemberData(nameof(PowerCases))]
  public void PowerReference(string testName) => Check<Power>("dist-power", testName);

  /// <summary>
  /// The arguments the fixtures cannot carry, because JSON writes neither a NaN nor an infinity.
  /// </summary>
  /// <remarks>
  /// Without this table the fixtures are the only check the trio has, and they sample the interior
  /// of each support through the family's own quantile. That left two defects alive: Uniform
  /// reported a perfectly ordinary density for NaN in six ports, since neither half of
  /// <c>x &lt; Min || x &gt;= Max</c> holds for a NaN and control fell straight through, and R
  /// raised an error from thirteen of the fifteen functions. Neither is reachable from a fixture.
  /// </remarks>
  public static TheoryData<string, IDistribution, double, double> Families =>
    new()
    {
      { "dist-uniform", new Uniform(0, 1), 0, 1 },
      { "dist-exp", new Exp(1), 0, double.PositiveInfinity },
      { "dist-additive", new Additive(0, 1), double.NegativeInfinity, double.PositiveInfinity },
      { "dist-multiplic", new Multiplic(0, 1), 0, double.PositiveInfinity },
      { "dist-power", new Power(1, 2), 1, double.PositiveInfinity },
    };

  [Theory]
  [MemberData(nameof(Families))]
  public void NaNPropagatesThroughAllThree(string name, IDistribution d, double lower, double upper)
  {
    _ = lower;
    _ = upper;
    Assert.True(double.IsNaN(d.DensityAt(double.NaN)), $"{name}: DensityAt(NaN)");
    Assert.True(double.IsNaN(d.CumulativeAt(double.NaN)), $"{name}: CumulativeAt(NaN)");
    Assert.True(double.IsNaN(d.QuantileAt(double.NaN)), $"{name}: QuantileAt(NaN)");
  }

  [Theory]
  [MemberData(nameof(Families))]
  public void TheInfinitiesLieOutsideEverySupport(string name, IDistribution d, double lower, double upper)
  {
    _ = lower;
    _ = upper;
    Assert.True(d.DensityAt(double.NegativeInfinity) == 0.0, $"{name}: DensityAt(-Infinity)");
    Assert.True(d.DensityAt(double.PositiveInfinity) == 0.0, $"{name}: DensityAt(+Infinity)");
    Assert.True(d.CumulativeAt(double.NegativeInfinity) == 0.0, $"{name}: CumulativeAt(-Infinity)");
    Assert.True(d.CumulativeAt(double.PositiveInfinity) == 1.0, $"{name}: CumulativeAt(+Infinity)");
  }

  [Theory]
  [MemberData(nameof(Families))]
  public void AProbabilityOutsideTheUnitIntervalReturnsTheEndpoints(
    string name,
    IDistribution d,
    double lower,
    double upper)
  {
    foreach (double p in new[] { 0.0, -0.1, double.NegativeInfinity })
      Assert.True(d.QuantileAt(p) == lower, $"{name}: QuantileAt({p})");
    foreach (double p in new[] { 1.0, 1.1, double.PositiveInfinity })
      Assert.True(d.QuantileAt(p) == upper, $"{name}: QuantileAt({p})");
  }

  // lifts is true for the two families whose QuantileAt(0) is not a sample.
  public static TheoryData<string, IDistribution, bool> SamplingIdentityFamilies =>
    new()
    {
      { "Uniform", new Uniform(-2.5, 7.5), false },
      { "Exp", new Exp(0.5), false },
      { "Additive", new Additive(1.0, 2.0), true },
      { "Multiplic", new Multiplic(-1.0, 0.5), true },
      { "Power", new Power(2.0, 3.0), false },
    };

  /// <summary>
  /// Every family draws by inverting its own quantile.
  /// </summary>
  /// <remarks>
  /// A fixture says two numbers agreed on the day it was written. This says they cannot disagree:
  /// the same seed drives a draw and a direct quantile call, and the two must land on the same
  /// bits for every family. It is what makes "a distribution is three functions" a statement
  /// about the code rather than about the documentation, and it would fail the moment a family
  /// grew a second sampling route.
  /// </remarks>
  [Theory]
  [MemberData(nameof(SamplingIdentityFamilies))]
  public void SamplingIsTheQuantileOfADrawnCumulance(string name, IDistribution d, bool lifts)
  {
    var drawing = new Rng("identity-seed");
    var inverting = new Rng("identity-seed");
    for (int i = 0; i < 1000; i++)
    {
      double drawn = d.Sample(drawing);
      // The same cumulance the draw consumed, taken from a generator on the same seed.
      double u = inverting.UniformDouble();
      if (lifts && u == 0.0)
        u = 5e-324;
      double inverted = d.QuantileAt(u);
      Assert.True(
        BitConverter.DoubleToInt64Bits(drawn) == BitConverter.DoubleToInt64Bits(inverted),
        $"{name} draw {i}: Sample gave {drawn}, QuantileAt({u}) gave {inverted}");
    }
  }
}
