#pragma warning disable CS0618

using JetBrains.Annotations;
using Pragmastat.Exceptions;
using Pragmastat.TestGenerator.Framework;
using Pragmastat.TestGenerator.Framework.OneSample;

namespace Pragmastat.Tests.Estimators;

public class RelSpreadTests
{
  private const string SuiteName = "rel-spread";
  private readonly OneSampleEstimatorController controller = new(SuiteName, input => input.ToSample().RelSpread());

  [UsedImplicitly]
  public static readonly TheoryData<string> TestDataNames = ReferenceTestSuiteHelper.GetTheoryData(SuiteName, true);

  [Theory]
  [MemberData(nameof(TestDataNames))]
  public void RelSpreadTest(string testName)
  {
    var testCase = controller.LoadTestCase(testName);
    try
    {
      var actual = controller.Run(testCase.Input);
      Assert.True(controller.Assert(testCase.Output, actual));
    }
    catch (AssumptionException)
    {
      // Skip cases that violate assumptions - tested separately
    }
  }
}
