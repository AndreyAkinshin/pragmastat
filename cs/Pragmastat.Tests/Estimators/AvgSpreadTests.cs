using JetBrains.Annotations;
using Pragmastat.Exceptions;
using Pragmastat.TestGenerator.Framework;
using Pragmastat.TestGenerator.Framework.TwoSample;

namespace Pragmastat.Tests.Estimators;

public class AvgSpreadTests
{
  private const string SuiteName = "avg-spread";
  private readonly TwoSampleEstimatorController controller = new(SuiteName, input => Pragmastat.Estimators.AvgSpreadEstimator.Instance.Estimate(input.GetSampleX(), input.GetSampleY()));

  [UsedImplicitly]
  public static readonly TheoryData<string> TestDataNames = ReferenceTestSuiteHelper.GetTheoryData(SuiteName, true);

  [Theory]
  [MemberData(nameof(TestDataNames))]
  public void AvgSpreadTest(string testName)
  {
    var testCase = controller.LoadTestCase(testName);
    try
    {
      var actual = controller.Run(testCase.Input);
      Assert.True(controller.Assert(testCase.Output, actual));
    }
    catch (AssumptionException)
    {
      // Skip cases that violate assumptions - tested separately
    }
  }
}
