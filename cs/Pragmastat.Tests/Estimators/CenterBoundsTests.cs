using JetBrains.Annotations;
using Pragmastat.Exceptions;
using Pragmastat.TestGenerator.Framework;
using Pragmastat.TestGenerator.Framework.CenterBounds;

namespace Pragmastat.Tests.Estimators;

public class CenterBoundsTests
{
  private const string SuiteName = "center-bounds";

  private readonly CenterBoundsController sampleController = new(SuiteName);
  private readonly CenterBoundsController rawController = new(SuiteName, compute: input =>
    Toolkit.CenterBounds(input.X, input.Misrate, assumeSorted: false));

  private CenterBoundsController Controller(string entryPoint) =>
    entryPoint == ReferenceTestSuiteHelper.EntryPointRaw ? rawController : sampleController;

  [UsedImplicitly]
  public static readonly TheoryData<string, string> TestDataNames =
    ReferenceTestSuiteHelper.GetDualPathTheoryData(SuiteName, true);

  [Theory]
  [MemberData(nameof(TestDataNames))]
  public void CenterBoundsTest(string testName, string entryPoint)
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
    Assert.True(
      controller.Assert(testCase.Output, actual),
      $"Test: {testName} ({entryPoint}), Expected: [{testCase.Output.Lower}, {testCase.Output.Upper}], Actual: [{actual.Lower}, {actual.Upper}]");
  }
}
