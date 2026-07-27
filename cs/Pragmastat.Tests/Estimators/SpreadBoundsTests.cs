using JetBrains.Annotations;
using Pragmastat.Exceptions;
using Pragmastat.TestGenerator.Framework;
using Pragmastat.TestGenerator.Framework.SpreadBounds;

namespace Pragmastat.Tests.Estimators;

public class SpreadBoundsTests
{
  private const string SuiteName = "spread-bounds";

  private readonly SpreadBoundsController sampleController = new(SuiteName);
  private readonly SpreadBoundsController rawController = new(SuiteName, compute: input => input.Seed != null
    ? Toolkit.SpreadBounds(input.X, input.Misrate, input.Seed, assumeSorted: false)
    : Toolkit.SpreadBounds(input.X, input.Misrate, assumeSorted: false));

  private SpreadBoundsController Controller(string entryPoint) =>
    entryPoint == ReferenceTestSuiteHelper.EntryPointRaw ? rawController : sampleController;

  [UsedImplicitly]
  public static readonly TheoryData<string, string> TestDataNames =
    ReferenceTestSuiteHelper.GetDualPathTheoryData(SuiteName, true);

  [Theory]
  [MemberData(nameof(TestDataNames))]
  public void SpreadBoundsTest(string testName, string entryPoint)
  {
    var controller = Controller(entryPoint);
    // Detect error test cases by checking JSON structure
    if (ReferenceTestSuiteHelper.IsErrorTestCase(SuiteName, testName, shared: true))
    {
      var errorTestCase = controller.LoadErrorTestCase(testName);
      var ex = Assert.Throws<AssumptionException>(() =>
        controller.Run(errorTestCase.Input));
      ReferenceTestSuiteHelper.AssertErrorMatches(errorTestCase.ExpectedError, ex, entryPoint);
      return;
    }

    var testCase = controller.LoadTestCase(testName);
    var actual = controller.Run(testCase.Input);
    string context = $"Suite: {SuiteName}, test {testName} ({entryPoint})";
    BitwiseAssert.Equal(testCase.Output.Lower, actual.Lower, context + ", lower");
    BitwiseAssert.Equal(testCase.Output.Upper, actual.Upper, context + ", upper");
  }
}
