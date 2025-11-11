using Pragmastat.Distributions;
using Pragmastat.TestGenerator.Framework;
using Pragmastat.TestGenerator.Framework.Distributions;

namespace Pragmastat.Tests.Distributions;

public class UniformDistributionTests
{
  private const string SuiteName = "distribution-uniform";
  private readonly DistributionController<UniformDistribution> controller = new(SuiteName);

  public static readonly TheoryData<string> TestCaseNames = ReferenceTestSuiteHelper.GetTheoryData(SuiteName);

  [Theory]
  [MemberData(nameof(TestCaseNames))]
  public void UniformDistributionTest(string testName)
  {
    var testCase = controller.LoadTestCase(testName);
    var actual = controller.Run(testCase.Input);
    Assert.True(controller.Assert(testCase.Output, actual));
  }
}
