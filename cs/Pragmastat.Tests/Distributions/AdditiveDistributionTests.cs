using Pragmastat.Distributions;
using Pragmastat.TestGenerator.Framework;
using Pragmastat.TestGenerator.Framework.Distributions;

namespace Pragmastat.Tests.Distributions;

public class AdditiveDistributionTests
{
  private const string SuiteName = "distribution-normal";
  private readonly DistributionController<Additive> controller = new(SuiteName);

  public static readonly TheoryData<string> TestCaseNames = ReferenceTestSuiteHelper.GetTheoryData(SuiteName);

  [Theory]
  [MemberData(nameof(TestCaseNames))]
  public void NormalDistributionTest(string testName)
  {
    var testCase = controller.LoadTestCase(testName);
    var actual = controller.Run(testCase.Input);
    Assert.True(controller.Assert(testCase.Output, actual));
  }
}
