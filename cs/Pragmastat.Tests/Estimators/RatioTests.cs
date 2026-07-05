using JetBrains.Annotations;
using Pragmastat.Estimators;
using Pragmastat.Exceptions;
using Pragmastat.TestGenerator.Framework;
using Pragmastat.TestGenerator.Framework.TwoSample;

namespace Pragmastat.Tests.Estimators;

public class RatioTests
{
  private const string SuiteName = "ratio";

  private readonly TwoSampleEstimatorController sampleController =
    new(SuiteName, input => RatioEstimator.Instance.Estimate(input.GetSampleX(), input.GetSampleY()));
  private readonly TwoSampleEstimatorController rawController =
    new(SuiteName, input => RatioEstimator.Instance.Estimate(input.X, input.Y, assumeSorted: false));

  private TwoSampleEstimatorController Controller(string entryPoint) =>
    entryPoint == ReferenceTestSuiteHelper.EntryPointRaw ? rawController : sampleController;

  [UsedImplicitly]
  public static readonly TheoryData<string, string> TestDataNames =
    ReferenceTestSuiteHelper.GetDualPathTheoryData(SuiteName, true);

  [Theory]
  [MemberData(nameof(TestDataNames))]
  public void RatioTest(string testName, string entryPoint)
  {
    var controller = Controller(entryPoint);
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
