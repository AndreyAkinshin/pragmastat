using Pragmastat.Distributions;
using Pragmastat.ReferenceTests.Distributions.ReferenceTesting;
using Pragmastat.ReferenceTests.ReferenceTesting;

namespace Pragmastat.ReferenceTests.Distributions;

public class NormalDistributionTests : DistributionTestBase<NormalDistribution>
{
    private const string SuiteName = "distribution-normal";
    protected override string GetSuiteName() => SuiteName;

    protected override ReferenceTestCaseInputBuilder<DistributionInput> GetInputBuilder()
    {
        return new DistributionInputBuilder()
            .Add(new NormalDistribution())
            .Add(new NormalDistribution(1, 2))
            .Add(new NormalDistribution(-1, 0.5))
            .Add(new NormalDistribution(5, 3))
            .Add(new NormalDistribution(0, 0.1))
            .Add(new NormalDistribution(-2, 10));
    }

    public static readonly TheoryData<string> TestCaseNames = ReferenceTestSuiteHelper.GetTheoryData(SuiteName);

    [Theory]
    [MemberData(nameof(TestCaseNames))]
    public void NormalDistributionTest(string testName) => PerformTest(testName);
}