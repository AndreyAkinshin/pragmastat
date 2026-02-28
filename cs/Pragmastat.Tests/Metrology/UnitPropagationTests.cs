using System.Text.Json;
using JetBrains.Annotations;
using Pragmastat.Estimators;
using Pragmastat.Exceptions;
using Pragmastat.Metrology;
using Pragmastat.TestGenerator.Framework;

namespace Pragmastat.Tests.Metrology;

public class UnitPropagationTests
{
  private const string SuiteName = "unit-propagation";
  private const double Tolerance = 1e-9;

  private static readonly UnitRegistry Registry = UnitRegistry.Standard();

  [UsedImplicitly]
  public static readonly TheoryData<string> TestDataNames = ReferenceTestSuiteHelper.GetTheoryData(SuiteName, true);

  [Theory]
  [MemberData(nameof(TestDataNames))]
  public void UnitPropagationTest(string testName)
  {
    string directory = ReferenceTestSuiteHelper.GetTestSuiteDirectory(SuiteName, true);
    string filePath = Path.Combine(directory, testName + ".json");
    string json = File.ReadAllText(filePath);
    using var doc = JsonDocument.Parse(json);
    var root = doc.RootElement;
    var input = root.GetProperty("input");

    // Weighted-rejected error case
    if (root.TryGetProperty("expected_error", out _))
    {
      double[] x = ParseDoubleArray(input.GetProperty("x"));
      double[] xWeights = ParseDoubleArray(input.GetProperty("x_weights"));
      var sample = new Sample(x, xWeights);
      Assert.Throws<WeightedSampleNotSupportedException>(() => CenterEstimator.Instance.Estimate(sample));
      return;
    }

    string estimator = input.GetProperty("estimator").GetString()!;
    double[] xValues = ParseDoubleArray(input.GetProperty("x"));
    string xUnitId = input.GetProperty("x_unit").GetString()!;
    MeasurementUnit xUnit = Registry.Resolve(xUnitId);
    var sx = new Sample(xValues, xUnit);

    var output = root.GetProperty("output");
    string expectedUnitId = output.GetProperty("unit").GetString()!;

    switch (estimator)
    {
      case "center":
        {
          Measurement m = CenterEstimator.Instance.Estimate(sx);
          Assert.Equal(expectedUnitId, m.Unit.Id);
          if (output.TryGetProperty("value", out var valueProp))
            Assert.True(Math.Abs(m.NominalValue - valueProp.GetDouble()) < Tolerance,
              $"Value = {m.NominalValue}, want {valueProp.GetDouble()}");
          break;
        }
      case "spread":
        {
          Measurement m = SpreadEstimator.Instance.Estimate(sx);
          Assert.Equal(expectedUnitId, m.Unit.Id);
          break;
        }
      case "shift":
        {
          double[] yValues = ParseDoubleArray(input.GetProperty("y"));
          string yUnitId = input.GetProperty("y_unit").GetString()!;
          MeasurementUnit yUnit = Registry.Resolve(yUnitId);
          var sy = new Sample(yValues, yUnit, Subject.Y);
          Measurement m = Toolkit.Shift(sx, sy);
          Assert.Equal(expectedUnitId, m.Unit.Id);
          break;
        }
      case "ratio":
        {
          double[] yValues = ParseDoubleArray(input.GetProperty("y"));
          string yUnitId = input.GetProperty("y_unit").GetString()!;
          MeasurementUnit yUnit = Registry.Resolve(yUnitId);
          var sy = new Sample(yValues, yUnit, Subject.Y);
          Measurement m = Toolkit.Ratio(sx, sy);
          Assert.Equal(expectedUnitId, m.Unit.Id);
          break;
        }
      case "disparity":
        {
          double[] yValues = ParseDoubleArray(input.GetProperty("y"));
          string yUnitId = input.GetProperty("y_unit").GetString()!;
          MeasurementUnit yUnit = Registry.Resolve(yUnitId);
          var sy = new Sample(yValues, yUnit, Subject.Y);
          Measurement m = Toolkit.Disparity(sx, sy);
          Assert.Equal(expectedUnitId, m.Unit.Id);
          break;
        }
      default:
        throw new InvalidOperationException($"Unknown estimator: {estimator}");
    }
  }

  private static double[] ParseDoubleArray(JsonElement array)
  {
    var values = new double[array.GetArrayLength()];
    int i = 0;
    foreach (var element in array.EnumerateArray())
      values[i++] = element.GetDouble();
    return values;
  }
}
