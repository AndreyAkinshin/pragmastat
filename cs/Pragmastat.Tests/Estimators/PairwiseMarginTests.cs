using JetBrains.Annotations;
using Pragmastat.TestGenerator.Framework;
using Pragmastat.TestGenerator.Framework.PairwiseMargin;

namespace Pragmastat.Tests.Estimators;

public class PairwiseMarginTests
{
  private const string SuiteName = "pairwise-margin";
  private readonly PairwiseMarginController controller = new(SuiteName);

  [UsedImplicitly]
  public static readonly TheoryData<string> TestDataNames = ReferenceTestSuiteHelper.GetTheoryData(SuiteName, true);

  [Theory]
  [MemberData(nameof(TestDataNames))]
  public void PairwiseMarginTest(string testName)
  {
    var testCase = controller.LoadTestCase(testName);
    var actual = controller.Run(testCase.Input);
    Assert.True(controller.Assert(testCase.Output, actual));
  }
}
