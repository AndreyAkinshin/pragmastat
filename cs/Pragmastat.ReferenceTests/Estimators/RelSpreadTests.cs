using JetBrains.Annotations;
using Pragmastat.ReferenceTests.Generator.Framework;
using Pragmastat.ReferenceTests.Generator.Framework.OneSample;

namespace Pragmastat.ReferenceTests.Estimators;

public class RelSpreadTests
{
  private const string SuiteName = "rel-spread";
  private readonly OneSampleEstimatorController controller = new(SuiteName, input => input.ToSample().RelSpread());

  [UsedImplicitly]
  public static readonly TheoryData<string> TestDataNames = ReferenceTestSuiteHelper.GetTheoryData(SuiteName, true);

  [Theory]
  [MemberData(nameof(TestDataNames))]
  public void RelSpreadTest(string testName)
  {
    var testCase = controller.LoadTestCase(testName);
    var actual = controller.Run(testCase.Input);
    Assert.True(controller.Assert(testCase.Output, actual));
  }
}
