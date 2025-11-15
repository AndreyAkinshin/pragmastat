using JetBrains.Annotations;
using Pragmastat.TestGenerator.Framework;
using Pragmastat.TestGenerator.Framework.ShiftBounds;

namespace Pragmastat.Tests.Estimators;

public class ShiftBoundsTests
{
  private const string SuiteName = "shift-bounds";
  private readonly ShiftBoundsController controller = new(SuiteName);

  [UsedImplicitly]
  public static readonly TheoryData<string> TestDataNames = ReferenceTestSuiteHelper.GetTheoryData(SuiteName, true);

  [Theory]
  [MemberData(nameof(TestDataNames))]
  public void ShiftBoundsTest(string testName)
  {
    var testCase = controller.LoadTestCase(testName);
    var actual = controller.Run(testCase.Input);
    Assert.True(controller.Assert(testCase.Output, actual));
  }
}
