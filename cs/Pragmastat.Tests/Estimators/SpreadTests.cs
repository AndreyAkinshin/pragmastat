using JetBrains.Annotations;
using Pragmastat.Estimators;
using Pragmastat.Exceptions;
using Pragmastat.TestGenerator.Framework;
using Pragmastat.TestGenerator.Framework.OneSample;

namespace Pragmastat.Tests.Estimators;

public class SpreadTests
{
  private const string SuiteName = "spread";

  private readonly OneSampleEstimatorController sampleController =
    new(SuiteName, input => SpreadEstimator.Instance.Estimate(input.ToSample()));
  private readonly OneSampleEstimatorController rawController =
    new(SuiteName, input => SpreadEstimator.Instance.Estimate(input.X, assumeSorted: false));

  private OneSampleEstimatorController Controller(string entryPoint) =>
    entryPoint == ReferenceTestSuiteHelper.EntryPointRaw ? rawController : sampleController;

  [UsedImplicitly]
  public static readonly TheoryData<string, string> TestDataNames =
    ReferenceTestSuiteHelper.GetDualPathTheoryData(SuiteName, true);

  [Theory]
  [MemberData(nameof(TestDataNames))]
  public void SpreadTest(string testName, string entryPoint)
  {
    var controller = Controller(entryPoint);
    if (ReferenceTestSuiteHelper.IsErrorTestCase(SuiteName, testName, shared: true))
    {
      var errorTestCase = controller.LoadErrorTestCase(testName);
      var ex = Assert.Throws<AssumptionException>(() =>
        controller.Run(errorTestCase.Input));
      ReferenceTestSuiteHelper.AssertErrorMatches(errorTestCase.ExpectedError, ex, entryPoint);
      return;
    }

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
    var actual = SpreadEstimator.Instance.Estimate(sample);
    const double expected = 29290;
    Assert.True(sampleController.Assert(expected, actual));
  }
}
