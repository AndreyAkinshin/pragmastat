using JetBrains.Annotations;
using Pragmastat.ReferenceTests.ReferenceTesting;
using Pragmastat.ReferenceTests.ReferenceTesting.TwoSample;

namespace Pragmastat.ReferenceTests.Estimators;

public class MedDisparityTests : TwoSampleEstimatorTestBase
{
    private const string SuiteName = "med-disparity";
    protected override string GetSuiteName() => SuiteName;

    protected override double Estimate(TwoSampleInput input) => input.GetSampleX().MedDisparity(input.GetSampleY());

    protected override ReferenceTestCaseInputBuilder<TwoSampleInput> GetInputBuilder() =>
        new TwoSampleInputBuilder()
            .AddNatural([2, 3], [2, 3])
            .AddUniform([5, 100], [5, 100], count: 1);

    [UsedImplicitly]
    public static readonly TheoryData<string> TestDataNames = ReferenceTestSuiteHelper.GetTheoryData(SuiteName, true);

    [Theory]
    [MemberData(nameof(TestDataNames))]
    public void MedDisparityTest(string testName) => PerformTest(testName);
}