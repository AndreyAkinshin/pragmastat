using Pragmastat.Distributions;
using Pragmastat.ReferenceTests.Distributions.ReferenceTesting;
using Pragmastat.ReferenceTests.ReferenceTesting;

namespace Pragmastat.ReferenceTests.Distributions;

public class AdditiveDistributionTests : DistributionTestBase<AdditiveDistribution>
{
  private const string SuiteName = "distribution-normal";
  protected override string GetSuiteName() => SuiteName;

  protected override ReferenceTestCaseInputBuilder<DistributionInput> GetInputBuilder()
  {
    return new DistributionInputBuilder()
      .Add(new AdditiveDistribution())
      .Add(new AdditiveDistribution(1, 2))
      .Add(new AdditiveDistribution(-1, 0.5))
      .Add(new AdditiveDistribution(5, 3))
      .Add(new AdditiveDistribution(0, 0.1))
      .Add(new AdditiveDistribution(-2, 10));
  }

  public static readonly TheoryData<string> TestCaseNames = ReferenceTestSuiteHelper.GetTheoryData(SuiteName);

  [Theory]
  [MemberData(nameof(TestCaseNames))]
  public void NormalDistributionTest(string testName) => PerformTest(testName);
}
