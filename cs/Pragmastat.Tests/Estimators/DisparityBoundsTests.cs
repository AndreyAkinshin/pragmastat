using JetBrains.Annotations;
using Pragmastat.Exceptions;
using Pragmastat.TestGenerator.Framework;
using Pragmastat.TestGenerator.Framework.DisparityBounds;

namespace Pragmastat.Tests.Estimators;

public class DisparityBoundsTests
{
  private const string SuiteName = "disparity-bounds";

  private readonly DisparityBoundsController sampleController = new(SuiteName);
  private readonly DisparityBoundsController rawController = new(SuiteName, compute: input => input.Seed != null
    ? Toolkit.DisparityBounds(input.X, input.Y, input.Misrate, input.Seed, assumeSorted: false)
    : Toolkit.DisparityBounds(input.X, input.Y, input.Misrate, assumeSorted: false));

  private DisparityBoundsController Controller(string entryPoint) =>
    entryPoint == ReferenceTestSuiteHelper.EntryPointRaw ? rawController : sampleController;

  [UsedImplicitly]
  public static readonly TheoryData<string, string> TestDataNames =
    ReferenceTestSuiteHelper.GetDualPathTheoryData(SuiteName, true);

  [Theory]
  [MemberData(nameof(TestDataNames))]
  public void DisparityBoundsTest(string testName, string entryPoint)
  {
    var controller = Controller(entryPoint);
    if (ReferenceTestSuiteHelper.IsErrorTestCase(SuiteName, testName, shared: true))
    {
      var errorTestCase = controller.LoadErrorTestCase(testName);
      var ex = Assert.Throws<AssumptionException>(() => controller.Run(errorTestCase.Input));
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
