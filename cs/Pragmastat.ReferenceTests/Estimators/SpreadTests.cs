using JetBrains.Annotations;
using Pragmastat.ReferenceTests.Generator.Framework;
using Pragmastat.ReferenceTests.Generator.Framework.OneSample;

namespace Pragmastat.ReferenceTests.Estimators;

public class SpreadTests
{
  private const string SuiteName = "spread";
  private readonly OneSampleEstimatorController controller = new(SuiteName, input => input.ToSample().Spread());

  [UsedImplicitly]
  public static readonly TheoryData<string> TestDataNames = ReferenceTestSuiteHelper.GetTheoryData(SuiteName, true);

  [Theory]
  [MemberData(nameof(TestDataNames))]
  public void SpreadTest(string testName)
  {
    var testCase = controller.LoadTestCase(testName);
    var actual = controller.Run(testCase.Input);
    Assert.True(controller.Assert(testCase.Output, actual));
  }

  [Fact]
  public void SpreadPerformanceTest()
  {
    // Performance test: x = (1, 2, 3, ..., 100000), expected output: 29290
    // This test validates the fast O(n log n) algorithm and ensures it completes in under 5 seconds.
    // The test case is not stored in the repository because it generates a large JSON file (~1.5 MB).
    var x = Enumerable.Range(1, 100000).Select(i => (double)i).ToArray();
    var sample = new Sample(x);
    var actual = sample.Spread();
    const double expected = 29290;
    Assert.True(controller.Assert(expected, actual));
  }
}
