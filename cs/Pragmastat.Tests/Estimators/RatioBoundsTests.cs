using JetBrains.Annotations;
using Pragmastat.Exceptions;
using Pragmastat.TestGenerator.Framework;
using Pragmastat.TestGenerator.Framework.RatioBounds;

namespace Pragmastat.Tests.Estimators;

public class RatioBoundsTests
{
  private const string SuiteName = "ratio-bounds";

  private readonly RatioBoundsController sampleController = new(SuiteName);
  private readonly RatioBoundsController rawController = new(SuiteName, compute: input =>
    Toolkit.RatioBounds(input.X, input.Y, input.Misrate, assumeSorted: false));

  private RatioBoundsController Controller(string entryPoint) =>
    entryPoint == ReferenceTestSuiteHelper.EntryPointRaw ? rawController : sampleController;

  [UsedImplicitly]
  public static readonly TheoryData<string, string> TestDataNames =
    ReferenceTestSuiteHelper.GetDualPathTheoryData(SuiteName, true);

  [Theory]
  [MemberData(nameof(TestDataNames))]
  public void RatioBoundsTest(string testName, string entryPoint)
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
    Assert.True(controller.Assert(testCase.Output, actual));
  }
}
