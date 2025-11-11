using Pragmastat.Functions;
using Pragmastat.TestGenerator.Framework;
using Pragmastat.TestGenerator.Framework.SingleDoubleValue;

namespace Pragmastat.Tests;

public class ApproximationTests
{
  private const string SuiteName = "approximations";
  private readonly SingleDoubleValueController controller;

  public ApproximationTests()
  {
    var functions = new Dictionary<string, Func<double, double>>
    {
      ["acm209"] = AcmAlgorithm209.Gauss,
      ["erf"] = AbramowitzStegunErf.Value,
      ["erf_inverse"] = ErfInverse.Value
    };
    controller = new SingleDoubleValueController(SuiteName, functions);
  }

  public static readonly TheoryData<string> TestDataNames = ReferenceTestSuiteHelper.GetTheoryData(SuiteName);

  [Theory]
  [MemberData(nameof(TestDataNames))]
  public void ApproximationTest(string testName)
  {
    var testCase = controller.LoadTestCase(testName);
    var actual = controller.Run(testCase.Input);
    Assert.True(controller.Assert(testCase.Output, actual));
  }
}
