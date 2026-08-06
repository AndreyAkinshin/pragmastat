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
      ReferenceTestSuiteHelper.AssertErrorMatches(errorTestCase.ExpectedError, ex, ReferenceTestSuiteHelper.EntryPointSample);
      return;
    }

    var testCase = controller.LoadTestCase(testName);
    var actual = controller.Run(testCase.Input);

    // Projection i answers threshold i: the two lists carry the same length and the same order,
    // and the order-* fixtures pin that. So each projection is checked against the conformance
    // class of its own metric instead of the weakest class present in the suite.
    var thresholds = testCase.Input.GetThresholds();
    var expected = testCase.Output.Projections;
    var produced = actual.Projections;
    Assert.Equal(thresholds.Count, expected.Length);
    Assert.Equal(expected.Length, produced.Length);
    for (int i = 0; i < expected.Length; i++)
    {
      string context = $"Suite: {SuiteName}, test {testName}, projection {i}";
      BitwiseAssert.Equal(expected[i].Estimate, produced[i].Estimate, context + ", estimate");
      BitwiseAssert.Equal(expected[i].Lower, produced[i].Lower, context + ", lower");
      BitwiseAssert.Equal(expected[i].Upper, produced[i].Upper, context + ", upper");
      Assert.Equal(expected[i].Verdict, produced[i].Verdict);
    }
  }

  // Every projection compares by payload now. The suite used to grade per metric, because ratio
  // evaluated exp(median(log x - log y)) through the platform's log and exp and two conforming
  // libm implementations could place the result a few ULP apart. Both functions are the toolkit's
  // own, so ratio selects its answer out of a finite set built from the input and maps it back
  // through reproducible arithmetic, exactly like shift and disparity beside it.

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
