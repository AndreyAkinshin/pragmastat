using JetBrains.Annotations;
using Pragmastat.Exceptions;
using Pragmastat.Metrology;
using Pragmastat.TestGenerator.Framework;
using Pragmastat.TestGenerator.Framework.Compare;

namespace Pragmastat.Tests.Estimators;

public class Compare2Tests
{
  private const string SuiteName = "compare2";
  private readonly Compare2Controller controller = new(SuiteName);

  [UsedImplicitly]
  public static readonly TheoryData<string> TestDataNames = ReferenceTestSuiteHelper.GetTheoryData(SuiteName, true);

  [Theory]
  [MemberData(nameof(TestDataNames))]
  public void Compare2Test(string testName)
  {
    if (ReferenceTestSuiteHelper.IsErrorTestCase(SuiteName, testName, shared: true))
    {
      var errorTestCase = controller.LoadErrorTestCase(testName);
      var ex = Assert.Throws<AssumptionException>(() => controller.Run(errorTestCase.Input));
      Assert.Equal(errorTestCase.ExpectedError.Id, ex.Violation.IdString);
      Assert.Equal(errorTestCase.ExpectedError.Subject, ex.Violation.Subject.ToString().ToLower());
      return;
    }

    var testCase = controller.LoadTestCase(testName);
    var actual = controller.Run(testCase.Input);
    Assert.True(controller.Assert(testCase.Output, actual),
      $"Test: {testName}, Projections mismatch");
  }

  // ── API-validation unit tests ─────────────────────────────────────────────

  private static readonly Sample ValidX = new(1, 2, 3, 4, 5, 6, 7, 8, 9, 10);
  private static readonly Sample ValidY = new(6, 7, 8, 9, 10, 11, 12, 13, 14, 15);
  private static readonly Threshold ValidShiftThreshold = new(Metric.Shift, new Measurement(0, MeasurementUnit.Number), 0.1);

  [Fact]
  public void NullThresholds_Throws()
    => Assert.Throws<ArgumentNullException>(() => Toolkit.Compare2(ValidX, ValidY, null!));

  [Fact]
  public void EmptyThresholds_Throws()
    => Assert.Throws<ArgumentException>(() => Toolkit.Compare2(ValidX, ValidY, []));

  [Fact]
  public void NullThresholdItem_Throws()
    => Assert.Throws<ArgumentNullException>(() => Toolkit.Compare2(ValidX, ValidY, [null!]));

  [Fact]
  public void WrongArityCenter_Throws()
    => Assert.Throws<ArgumentException>(() => Toolkit.Compare2(ValidX, ValidY,
      [new Threshold(Metric.Center, new Measurement(5, MeasurementUnit.Number), 0.1)]));

  [Fact]
  public void WrongAritySpread_Throws()
    => Assert.Throws<ArgumentException>(() => Toolkit.Compare2(ValidX, ValidY,
      [new Threshold(Metric.Spread, new Measurement(1, MeasurementUnit.Number), 0.1)]));

  [Fact]
  public void NonFiniteThresholdValue_Throws()
    => Assert.Throws<ArgumentOutOfRangeException>(() => Toolkit.Compare2(ValidX, ValidY,
      [new Threshold(Metric.Shift, new Measurement(double.NaN, MeasurementUnit.Number), 0.1)]));

  [Fact]
  public void RatioThresholdNotPositive_Throws()
    => Assert.Throws<ArgumentOutOfRangeException>(() => Toolkit.Compare2(ValidX, ValidY,
      [new Threshold(Metric.Ratio, new Measurement(0, MeasurementUnit.Number), 0.1)]));

  [Fact]
  public void NullSeed_SeededOverload_Throws()
    => Assert.Throws<ArgumentNullException>(() => Toolkit.Compare2(ValidX, ValidY, [ValidShiftThreshold], null!));
}
