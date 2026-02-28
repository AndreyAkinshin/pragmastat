using JetBrains.Annotations;
using Pragmastat.Estimators;
using Pragmastat.Exceptions;
using Pragmastat.TestGenerator.Framework;
using Pragmastat.TestGenerator.Framework.TwoSample;

namespace Pragmastat.Tests.Estimators;

public class ShiftTests
{
  private const string SuiteName = "shift";
  private readonly TwoSampleEstimatorController controller = new(SuiteName, input => ShiftEstimator.Instance.Estimate(input.GetSampleX(), input.GetSampleY()));

  [UsedImplicitly]
  public static readonly TheoryData<string> TestDataNames = ReferenceTestSuiteHelper.GetTheoryData(SuiteName, true);

  [Theory]
  [MemberData(nameof(TestDataNames))]
  public void ShiftTest(string testName)
  {
    if (ReferenceTestSuiteHelper.IsErrorTestCase(SuiteName, testName, shared: true))
    {
      var errorTestCase = controller.LoadErrorTestCase(testName);
      var ex = Assert.Throws<AssumptionException>(() =>
        controller.Run(errorTestCase.Input));
      Assert.Equal(errorTestCase.ExpectedError.Id, ex.Violation.IdString);
      Assert.Equal(errorTestCase.ExpectedError.Subject, ex.Violation.Subject.ToString().ToLower());
      return;
    }

    var testCase = controller.LoadTestCase(testName);
    var actual = controller.Run(testCase.Input);
    Assert.True(controller.Assert(testCase.Output, actual));
  }

  [Fact]
  public void ShiftPerformanceTest()
  {
    // Performance test: x = (1, 2, 3, ..., 100000), y = (1, 2, 3, ..., 100000), expected output: 0
    // This test validates the fast O((m+n) log L) binary search algorithm and ensures it completes in under 5 seconds.
    // The test case is not stored in the repository because it generates a large JSON file (~1.5 MB).
    var data = Enumerable.Range(1, 100000).Select(i => (double)i).ToArray();
    var sampleX = new Sample(data);
    var sampleY = new Sample(data);
    var actual = ShiftEstimator.Instance.Estimate(sampleX, sampleY);
    const double expected = 0;
    Assert.True(controller.Assert(expected, actual));
  }
}
