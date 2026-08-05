using JetBrains.Annotations;
using Pragmastat.Functions;
using Pragmastat.TestGenerator.Framework;
using Pragmastat.TestGenerator.Framework.SingleDoubleValue;

namespace Pragmastat.Tests.Functions;

/// <summary>
/// The reproducible logarithm against the shared fixture.
/// </summary>
/// <remarks>
/// <para>
/// Every ratio fixture reaches this function, and every one of them compares with a tolerance,
/// because until this branch the platform logarithm decided the last bits. That is not cover: a
/// tolerance forgives precisely the divergence this suite exists to catch.
/// </para>
/// <para>
/// What makes it matter is the shape of ratio: exp(median(log x - log y)). The logarithm feeds a
/// median, a median selects an element out of the pairwise set, and a last-bit difference in log
/// therefore changes which element is selected rather than the last bit of the answer. The gap
/// between two neighboring pairwise elements is whatever the data says it is.
/// </para>
/// <para>
/// The arguments run down to the smallest denormal, which makes the comparison double as a check
/// that System.Text.Json round-trips them: a parser that mangled 5e-324 would hand the
/// implementation a different number and surface here as a wrong logarithm.
/// </para>
/// </remarks>
public class LogFunctionTests
{
  private const string SuiteName = "log-function";

  // The size of the generated fixture: five files, 1745 arguments. Pinned rather than counted from
  // whatever the loader happened to find, because a count derived from the loader's own result
  // cannot report the failure it is here for: a loader that reads no files makes no comparisons
  // and passes every assertion it never makes.
  private const int FixtureFileCount = 5;
  private const int FixtureArgumentCount = 1745;

  // eps: 0 so that the controller cannot be used loosely by a later edit; the comparisons below
  // route through BitwiseAssert, which reports the divergent index and both payloads.
  private readonly SingleDoubleValueController controller = new(
    SuiteName,
    new Dictionary<string, Func<double, double>> { ["log_function"] = LogFunction.Value },
    eps: 0,
    shared: true);

  [UsedImplicitly]
  public static readonly TheoryData<string> TestDataNames =
    ReferenceTestSuiteHelper.GetTheoryData(SuiteName, shared: true);

  [Theory]
  [MemberData(nameof(TestDataNames))]
  public void LogFunctionTest(string testName)
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
  /// The arguments outside the reduction's domain, which the shared fixture cannot carry: JSON has
  /// no way to write a negative infinity or a NaN, so the generated suite holds only the positive
  /// finite range and these are covered here or nowhere.
  /// </summary>
  /// <remarks>
  /// These are the cases IEEE 754 leaves a platform no freedom about, so agreeing on them is not a
  /// choice this toolkit makes; it is what makes the replacement a logarithm at all.
  /// </remarks>
  [Fact]
  public void ReturnsTheDeclaredValuesOutsideTheReductionDomain()
  {
    BitwiseAssert.Equal(double.NegativeInfinity, LogFunction.Value(0.0), "LogFunction(+0)");
    BitwiseAssert.Equal(double.NegativeInfinity, LogFunction.Value(-0.0), "LogFunction(-0)");
    Assert.True(double.IsNaN(LogFunction.Value(-1.0)));
    Assert.True(double.IsNaN(LogFunction.Value(double.NaN)));
    BitwiseAssert.Equal(double.PositiveInfinity, LogFunction.Value(double.PositiveInfinity), "LogFunction(+Inf)");
    // Exactly +0, not -0: ratio exponentiates this result, and the two zeros are
    // distinct doubles that the payload comparison in every other suite would tell apart.
    BitwiseAssert.Equal(0.0, LogFunction.Value(1.0), "LogFunction(1)");
  }
}
