using JetBrains.Annotations;
using Pragmastat.ReferenceTests.ReferenceTesting;
using Pragmastat.ReferenceTests.ReferenceTesting.TwoSample;

namespace Pragmastat.ReferenceTests.Estimators;

public class MedRatioTests : TwoSampleEstimatorTestBase
{
    private const string SuiteName = "med-ratio";
    protected override string GetSuiteName() => SuiteName;

    protected override double Estimate(TwoSampleInput input) => input.GetSampleX().MedRatio(input.GetSampleY());

    protected override ReferenceTestCaseInputBuilder<TwoSampleInput> GetInputBuilder() =>
        new TwoSampleInputBuilder()
            .AddNatural([1, 2, 3], [1, 2, 3])
            .AddNormal([5, 10, 30], [5, 10, 30], count: 1)
            .AddUniform([5, 100], [5, 100], count: 1);

    [UsedImplicitly]
    public static readonly TheoryData<string> TestDataNames = ReferenceTestSuiteHelper.GetTheoryData(SuiteName, true);

    [Theory]
    [MemberData(nameof(TestDataNames))]
    public void MedRatioTest(string testName) => PerformTest(testName);
}