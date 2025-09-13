using JetBrains.Annotations;
using Pragmastat.ReferenceTests.ReferenceTesting;
using Pragmastat.ReferenceTests.ReferenceTesting.OneSample;

namespace Pragmastat.ReferenceTests.Estimators;

public class RelSpreadTests : OneSampleEstimatorTestBase
{
    private const string SuiteName = "rel-spread";
    protected override string GetSuiteName() => SuiteName;

    protected override double Estimate(OneSampleInput input) => input.ToSample().RelSpread();

    protected override ReferenceTestCaseInputBuilder<OneSampleInput> GetInputBuilder() =>
        new OneSampleInputBuilder()
            .AddNatural([1, 2, 3])
            .AddUniform([5, 10, 20, 30, 100], count: 1);

    [UsedImplicitly]
    public static readonly TheoryData<string> TestDataNames = ReferenceTestSuiteHelper.GetTheoryData(SuiteName, true);

    [Theory]
    [MemberData(nameof(TestDataNames))]
    public void RelSpreadTest(string testName) => PerformTest(testName);
}