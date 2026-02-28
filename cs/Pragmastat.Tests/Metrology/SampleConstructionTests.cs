using System.Text.Json;
using JetBrains.Annotations;
using Pragmastat.Exceptions;
using Pragmastat.TestGenerator.Framework;

namespace Pragmastat.Tests.Metrology;

public class SampleConstructionTests
{
  private const string SuiteName = "sample-construction";

  [UsedImplicitly]
  public static readonly TheoryData<string> TestDataNames = ReferenceTestSuiteHelper.GetTheoryData(SuiteName, true);

  [Theory]
  [MemberData(nameof(TestDataNames))]
  public void SampleConstructionTest(string testName)
  {
    string directory = ReferenceTestSuiteHelper.GetTestSuiteDirectory(SuiteName, true);
    string filePath = Path.Combine(directory, testName + ".json");
    string json = File.ReadAllText(filePath);
    using var doc = JsonDocument.Parse(json);
    var root = doc.RootElement;

    var input = root.GetProperty("input");
    var rawValues = input.GetProperty("values");

    double[] values = ParseValuesWithSpecialFloats(rawValues);
    double[]? weights = input.TryGetProperty("weights", out var rawWeights)
      ? ParseDoubleArray(rawWeights)
      : null;

    if (root.TryGetProperty("expected_error", out _))
    {
      if (weights != null)
        Assert.ThrowsAny<Exception>(() => new Sample(values, weights));
      else
        Assert.ThrowsAny<Exception>(() => new Sample(values));
      return;
    }

    var output = root.GetProperty("output");
    int expectedSize = output.GetProperty("size").GetInt32();
    bool expectedIsWeighted = output.GetProperty("is_weighted").GetBoolean();

    Sample sample = weights != null
      ? new Sample(values, weights)
      : new Sample(values);

    Assert.Equal(expectedSize, sample.Size);
    Assert.Equal(expectedIsWeighted, sample.IsWeighted);
  }

  private static double[] ParseValuesWithSpecialFloats(JsonElement array)
  {
    var values = new double[array.GetArrayLength()];
    int i = 0;
    foreach (var element in array.EnumerateArray())
    {
      values[i++] = element.ValueKind switch
      {
        JsonValueKind.Number => element.GetDouble(),
        JsonValueKind.String => element.GetString() switch
        {
          "NaN" => double.NaN,
          "Infinity" => double.PositiveInfinity,
          "-Infinity" => double.NegativeInfinity,
          var s => throw new JsonException($"Unknown special float value: {s}")
        },
        _ => throw new JsonException($"Unexpected JSON value kind: {element.ValueKind}")
      };
    }
    return values;
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
