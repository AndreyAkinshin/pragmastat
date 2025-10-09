using Pragmastat.Distributions;
using Pragmastat.ReferenceTests.Distributions.ReferenceTesting;
using Pragmastat.ReferenceTests.ReferenceTesting;

namespace Pragmastat.ReferenceTests.Distributions;

public class UniformDistributionTests : DistributionTestBase<UniformDistribution>
{
  private const string SuiteName = "distribution-uniform";
  protected override string GetSuiteName() => SuiteName;

  protected override ReferenceTestCaseInputBuilder<DistributionInput> GetInputBuilder()
  {
    return new DistributionInputBuilder()
      .Add(new UniformDistribution(0, 1))
      .Add(new UniformDistribution(2, 3))
      .Add(new UniformDistribution(-1, 1))
      .Add(new UniformDistribution(-5, -2))
      .Add(new UniformDistribution(0, 10))
      .Add(new UniformDistribution(-2.5, 7.5));
  }

  public static readonly TheoryData<string> TestCaseNames = ReferenceTestSuiteHelper.GetTheoryData(SuiteName);

  [Theory]
  [MemberData(nameof(TestCaseNames))]
  public void UniformDistributionTest(string testName) => PerformTest(testName);
}
