using JetBrains.Annotations;
using Pragmastat.Exceptions;
using Pragmastat.Metrology;
using Pragmastat.TestGenerator.Framework;
using Pragmastat.TestGenerator.Framework.Compare;

namespace Pragmastat.Tests.Estimators;

public class Compare1Tests
{
  private const string SuiteName = "compare1";
  private readonly Compare1Controller controller = new(SuiteName);

  [UsedImplicitly]
  public static readonly TheoryData<string> TestDataNames = ReferenceTestSuiteHelper.GetTheoryData(SuiteName, true);

  [Theory]
  [MemberData(nameof(TestDataNames))]
  public void Compare1Test(string testName)
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

  private static readonly Sample ValidSample = new(1, 2, 3, 4, 5, 6, 7, 8, 9, 10);
  private static readonly Threshold ValidThreshold = new(Metric.Center, new Measurement(5, MeasurementUnit.Number), 0.1);

  [Fact]
  public void NullThresholds_Throws()
    => Assert.Throws<ArgumentNullException>(() => Toolkit.Compare1(ValidSample, null!));

  [Fact]
  public void EmptyThresholds_Throws()
    => Assert.Throws<ArgumentException>(() => Toolkit.Compare1(ValidSample, []));

  [Fact]
  public void NullThresholdItem_Throws()
    => Assert.Throws<ArgumentNullException>(() => Toolkit.Compare1(ValidSample, [null!]));

  [Fact]
  public void WrongArityShift_Throws()
    => Assert.Throws<ArgumentException>(() => Toolkit.Compare1(ValidSample,
      [new Threshold(Metric.Shift, new Measurement(0, MeasurementUnit.Number), 0.1)]));

  [Fact]
  public void WrongArityRatio_Throws()
    => Assert.Throws<ArgumentException>(() => Toolkit.Compare1(ValidSample,
      [new Threshold(Metric.Ratio, new Measurement(1, MeasurementUnit.Number), 0.1)]));

  [Fact]
  public void WrongArityDisparity_Throws()
    => Assert.Throws<ArgumentException>(() => Toolkit.Compare1(ValidSample,
      [new Threshold(Metric.Disparity, new Measurement(0, MeasurementUnit.Number), 0.1)]));

  [Fact]
  public void NonFiniteThresholdValue_Throws()
    => Assert.Throws<ArgumentOutOfRangeException>(() => Toolkit.Compare1(ValidSample,
      [new Threshold(Metric.Center, new Measurement(double.PositiveInfinity, MeasurementUnit.Number), 0.1)]));

  [Fact]
  public void NullSeed_SeededOverload_Throws()
    => Assert.Throws<ArgumentNullException>(() => Toolkit.Compare1(ValidSample, [ValidThreshold], null!));
}
