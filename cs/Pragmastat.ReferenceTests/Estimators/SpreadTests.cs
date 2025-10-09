using JetBrains.Annotations;
using Pragmastat.ReferenceTests.ReferenceTesting;
using Pragmastat.ReferenceTests.ReferenceTesting.OneSample;

namespace Pragmastat.ReferenceTests.Estimators;

public class SpreadTests : OneSampleEstimatorTestBase
{
  private const string SuiteName = "spread";
  protected override string GetSuiteName() => SuiteName;

  protected override double Estimate(OneSampleInput input) => input.ToSample().Spread();

  protected override ReferenceTestCaseInputBuilder<OneSampleInput> GetInputBuilder() =>
    new OneSampleInputBuilder()
      .AddNatural([1, 2, 3])
      .AddZero([1, 2])
      .AddNormal([5, 10, 30], count: 1)
      .AddUniform([5, 100], count: 1);

  [UsedImplicitly]
  public static readonly TheoryData<string> TestDataNames = ReferenceTestSuiteHelper.GetTheoryData(SuiteName, true);

  [Theory]
  [MemberData(nameof(TestDataNames))]
  public void SpreadTest(string testName) => PerformTest(testName);
}
