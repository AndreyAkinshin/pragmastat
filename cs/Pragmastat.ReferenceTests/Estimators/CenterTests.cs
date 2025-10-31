using JetBrains.Annotations;
using Pragmastat.ReferenceTests.Generator.Framework;
using Pragmastat.ReferenceTests.Generator.Framework.OneSample;

namespace Pragmastat.ReferenceTests.Estimators;

public class CenterTests
{
  private const string SuiteName = "center";
  private readonly OneSampleEstimatorController controller = new(SuiteName, input => input.ToSample().Center());

  [UsedImplicitly]
  public static readonly TheoryData<string> TestDataNames = ReferenceTestSuiteHelper.GetTheoryData(SuiteName, true);

  [Theory]
  [MemberData(nameof(TestDataNames))]
  public void CenterTest(string testName)
  {
    var testCase = controller.LoadTestCase(testName);
    var actual = controller.Run(testCase.Input);
    Assert.True(controller.Assert(testCase.Output, actual));
  }

  [Fact]
  public void CenterPerformanceTest()
  {
    // Performance test: x = (1, 2, 3, ..., 100000), expected output: 50000.5
    // This test validates the fast O(n log n) algorithm and ensures it completes in under 5 seconds.
    // The test case is not stored in the repository because it generates a large JSON file (~1.5 MB).
    var x = Enumerable.Range(1, 100000).Select(i => (double)i).ToArray();
    var sample = new Sample(x);
    var actual = sample.Center();
    const double expected = 50000.5;
    Assert.True(controller.Assert(expected, actual));
  }
}
