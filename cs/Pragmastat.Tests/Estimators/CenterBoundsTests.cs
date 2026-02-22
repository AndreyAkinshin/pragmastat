using JetBrains.Annotations;
using Pragmastat.Exceptions;
using Pragmastat.TestGenerator.Framework;
using Pragmastat.TestGenerator.Framework.CenterBounds;

namespace Pragmastat.Tests.Estimators;

public class CenterBoundsTests
{
  private const string SuiteName = "center-bounds";
  private readonly CenterBoundsController controller = new(SuiteName);

  [UsedImplicitly]
  public static readonly TheoryData<string> TestDataNames = ReferenceTestSuiteHelper.GetTheoryData(SuiteName, true);

  [Theory]
  [MemberData(nameof(TestDataNames))]
  public void CenterBoundsTest(string testName)
  {
    // Detect error test cases by checking JSON structure
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
    Assert.True(
      controller.Assert(testCase.Output, actual),
      $"Test: {testName}, Expected: [{testCase.Output.Lower}, {testCase.Output.Upper}], Actual: [{actual.Lower}, {actual.Upper}]");
  }
}
