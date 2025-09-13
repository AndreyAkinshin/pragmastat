using JetBrains.Annotations;
using Pragmastat.ReferenceTests.ReferenceTesting;
using Pragmastat.ReferenceTests.ReferenceTesting.TwoSample;

namespace Pragmastat.ReferenceTests.Estimators;

public class AvgSpreadTests : TwoSampleEstimatorTestBase
{
    private const string SuiteName = "avg-spread";
    protected override string GetSuiteName() => SuiteName;

    protected override double Estimate(TwoSampleInput input) => input.GetSampleX().AvgSpread(input.GetSampleY());

    protected override ReferenceTestCaseInputBuilder<TwoSampleInput> GetInputBuilder() =>
        new TwoSampleInputBuilder()
            .AddNatural([1, 2, 3], [1, 2, 3])
            .AddZero([1, 2], [1, 2])
            .AddNormal([5, 10, 30], [5, 10, 30], count: 1)
            .AddUniform([5, 100], [5, 100], count: 1);

    [UsedImplicitly]
    public static readonly TheoryData<string> TestDataNames = ReferenceTestSuiteHelper.GetTheoryData(SuiteName, true);

    [Theory]
    [MemberData(nameof(TestDataNames))]
    public void AvgSpreadTest(string testName) => PerformTest(testName);
}