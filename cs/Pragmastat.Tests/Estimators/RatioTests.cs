using JetBrains.Annotations;
using Pragmastat.TestGenerator.Framework;
using Pragmastat.TestGenerator.Framework.TwoSample;

namespace Pragmastat.Tests.Estimators;

public class RatioTests
{
  private const string SuiteName = "ratio";
  private readonly TwoSampleEstimatorController controller = new(SuiteName, input => input.GetSampleX().Ratio(input.GetSampleY()));

  [UsedImplicitly]
  public static readonly TheoryData<string> TestDataNames = ReferenceTestSuiteHelper.GetTheoryData(SuiteName, true);

  [Theory]
  [MemberData(nameof(TestDataNames))]
  public void RatioTest(string testName)
  {
    var testCase = controller.LoadTestCase(testName);
    var actual = controller.Run(testCase.Input);
    Assert.True(controller.Assert(testCase.Output, actual));
  }
}
