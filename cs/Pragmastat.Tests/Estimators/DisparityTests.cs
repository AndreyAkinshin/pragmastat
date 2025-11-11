using JetBrains.Annotations;
using Pragmastat.TestGenerator.Framework;
using Pragmastat.TestGenerator.Framework.TwoSample;

namespace Pragmastat.Tests.Estimators;

public class DisparityTests
{
  private const string SuiteName = "disparity";
  private readonly TwoSampleEstimatorController controller = new(SuiteName, input => input.GetSampleX().Disparity(input.GetSampleY()));

  [UsedImplicitly]
  public static readonly TheoryData<string> TestDataNames = ReferenceTestSuiteHelper.GetTheoryData(SuiteName, true);

  [Theory]
  [MemberData(nameof(TestDataNames))]
  public void DisparityTest(string testName)
  {
    var testCase = controller.LoadTestCase(testName);
    var actual = controller.Run(testCase.Input);
    Assert.True(controller.Assert(testCase.Output, actual));
  }
}
