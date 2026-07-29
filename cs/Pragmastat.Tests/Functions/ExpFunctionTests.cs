using JetBrains.Annotations;
using Pragmastat.Functions;
using Pragmastat.TestGenerator.Framework;
using Pragmastat.TestGenerator.Framework.SingleDoubleValue;

namespace Pragmastat.Tests.Functions;

/// <summary>
/// The shared fixture for the reproducible exponential, checked by payload.
/// </summary>
/// <remarks>
/// <para>
/// Every other suite reaches this function through a margin, so it is covered wherever an
/// Edgeworth expansion happens to look and nowhere else. This one calls it directly on 1032
/// arguments: the band exp(-t*t) reaches, the wider band the expansions reach, the whole finite
/// range, and the points where the construction changes behavior rather than merely continues.
/// </para>
/// <para>
/// Compared bitwise, not within a tolerance, because the suite exists to catch a last-bit
/// difference: the manifest declares it exact, and a margin built on this function selects an
/// order statistic, so one bit here is a different confidence interval rather than a small error.
/// </para>
/// <para>
/// The boundaries file carries results down to the smallest denormal, which makes the comparison
/// double as a check that System.Text.Json round-trips them: 5e-324 and 1e-323 are read back and
/// their payloads matched against what the implementation returns, so a parser that mangled them
/// would fail here rather than pass quietly.
/// </para>
/// </remarks>
public class ExpFunctionTests
{
  private const string SuiteName = "exp-function";

  // The size of the generated fixture: four files, 1032 arguments. Pinned rather than counted from
  // whatever the loader happened to find, because a count derived from the loader's own result
  // cannot report the failure it is here for: a loader that reads no files makes no comparisons
  // and passes every assertion it never makes.
  private const int FixtureFileCount = 4;
  private const int FixtureArgumentCount = 1032;

  // eps: 0 so that the controller cannot be used loosely by a later edit; the comparisons below
  // route through BitwiseAssert, which reports the divergent index and both payloads.
  private readonly SingleDoubleValueController controller = new(
    SuiteName,
    new Dictionary<string, Func<double, double>> { ["exp_function"] = ExpFunction.Value },
    eps: 0,
    shared: true);

  [UsedImplicitly]
  public static readonly TheoryData<string> TestDataNames =
    ReferenceTestSuiteHelper.GetTheoryData(SuiteName, shared: true);

  [Theory]
  [MemberData(nameof(TestDataNames))]
  public void ExpFunctionTest(string testName)
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
  /// Both of AdditiveCumulative's range comparisons are false for a NaN, so without an explicit guard it
  /// leaves the tail branch as a finite 0 or 1: an undefined input answered rather than reported.
  /// The approximation it replaced propagated NaN, so losing that would be a regression.
  /// </summary>
  [Fact]
  public void AdditiveCumulativeCarriesNaNThroughAndAnswersBothInfinities()
  {
    Assert.True(double.IsNaN(AdditiveCumulative.Value(double.NaN)));
    BitwiseAssert.Equal(1.0, AdditiveCumulative.Value(double.PositiveInfinity), "AdditiveCumulative(+Inf)");
    BitwiseAssert.Equal(0.0, AdditiveCumulative.Value(double.NegativeInfinity), "AdditiveCumulative(-Inf)");
    BitwiseAssert.Equal(0.5, AdditiveCumulative.Value(0.0), "AdditiveCumulative(+0)");
    BitwiseAssert.Equal(0.5, AdditiveCumulative.Value(-0.0), "AdditiveCumulative(-0)");
  }

  /// <summary>
  /// The values outside the reduction band, which the shared fixture cannot carry: JSON has no way
  /// to express an infinity, so the generated suite stops at 709.78 and these arguments are
  /// covered here or nowhere.
  /// </summary>
  [Fact]
  public void ReturnsTheDeclaredValuesOutsideTheReductionBand()
  {
    Assert.True(double.IsNaN(ExpFunction.Value(double.NaN)));
    BitwiseAssert.Equal(double.PositiveInfinity, ExpFunction.Value(709.8), "ExpFunction(709.8)");
    BitwiseAssert.Equal(double.PositiveInfinity, ExpFunction.Value(double.PositiveInfinity), "ExpFunction(+Inf)");
    BitwiseAssert.Equal(0.0, ExpFunction.Value(-745.3), "ExpFunction(-745.3)");
    BitwiseAssert.Equal(0.0, ExpFunction.Value(double.NegativeInfinity), "ExpFunction(-Inf)");
    BitwiseAssert.Equal(1.0, ExpFunction.Value(0.0), "ExpFunction(0)");
  }
}
