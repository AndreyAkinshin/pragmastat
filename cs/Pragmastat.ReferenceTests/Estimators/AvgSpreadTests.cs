using JetBrains.Annotations;
using Pragmastat.ReferenceTests.Generator.Framework;
using Pragmastat.ReferenceTests.Generator.Framework.TwoSample;

namespace Pragmastat.ReferenceTests.Estimators;

public class AvgSpreadTests
{
  private const string SuiteName = "avg-spread";
  private readonly TwoSampleEstimatorController controller = new(SuiteName, input => input.GetSampleX().AvgSpread(input.GetSampleY()));

  [UsedImplicitly]
  public static readonly TheoryData<string> TestDataNames = ReferenceTestSuiteHelper.GetTheoryData(SuiteName, true);

  [Theory]
  [MemberData(nameof(TestDataNames))]
  public void AvgSpreadTest(string testName)
  {
    var testCase = controller.LoadTestCase(testName);
    var actual = controller.Run(testCase.Input);
    Assert.True(controller.Assert(testCase.Output, actual));
  }
}
