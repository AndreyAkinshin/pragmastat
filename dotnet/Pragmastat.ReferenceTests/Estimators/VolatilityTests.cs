using JetBrains.Annotations;
using Pragmastat.ReferenceTests.ReferenceTesting;
using Pragmastat.ReferenceTests.ReferenceTesting.OneSample;

namespace Pragmastat.ReferenceTests.Estimators;

public class VolatilityTests : OneSampleEstimatorTestBase
{
    private const string SuiteName = "volatility";
    protected override string GetSuiteName() => SuiteName;

    protected override double Estimate(OneSampleInput input) => input.ToSample().Volatility();

    protected override ReferenceTestCaseInputBuilder<OneSampleInput> GetInputBuilder() =>
        new OneSampleInputBuilder()
            .AddNatural([1, 2, 3])
            .AddUniform([5, 10, 20, 30, 100], count: 1);

    [UsedImplicitly]
    public static readonly TheoryData<string> TestDataNames = ReferenceTestSuiteHelper.GetTheoryData(SuiteName, true);

    [Theory]
    [MemberData(nameof(TestDataNames))]
    public void VolatilityTest(string testName) => PerformTest(testName);
}