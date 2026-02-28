using JetBrains.Annotations;
using Pragmastat.Estimators;
using Pragmastat.Exceptions;
using Pragmastat.TestGenerator.Framework;
using Pragmastat.TestGenerator.Framework.TwoSample;

namespace Pragmastat.Tests.Estimators;

public class DisparityTests
{
  private const string SuiteName = "disparity";
  private readonly TwoSampleEstimatorController controller = new(SuiteName, input => DisparityEstimator.Instance.Estimate(input.GetSampleX(), input.GetSampleY()));

  [UsedImplicitly]
  public static readonly TheoryData<string> TestDataNames = ReferenceTestSuiteHelper.GetTheoryData(SuiteName, true);

  [Theory]
  [MemberData(nameof(TestDataNames))]
  public void DisparityTest(string testName)
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
}
