using System.Text.Json;
using System.Text.Json.Serialization;
using Pragmastat.Exceptions;
using Pragmastat.Estimators;

namespace Pragmastat.Tests.Estimators;

/// <summary>
/// Assumption violation conformance tests.
/// These tests verify that assumption violations are reported correctly and
/// consistently across all languages.
/// </summary>
public class AssumptionTests
{
  public record ExpectedViolation(
    [property: JsonPropertyName("id")] string Id);

  public record TestInputs(
    [property: JsonPropertyName("x")] JsonElement[]? X,
    [property: JsonPropertyName("y")] JsonElement[]? Y);

  public record AssumptionTestCase(
    [property: JsonPropertyName("name")] string Name,
    [property: JsonPropertyName("function")] string Function,
    [property: JsonPropertyName("inputs")] TestInputs Inputs,
    [property: JsonPropertyName("expected_violation")] ExpectedViolation ExpectedViolation);

  private record AssumptionTestSuite(
    [property: JsonPropertyName("suite")] string Suite,
    [property: JsonPropertyName("description")] string Description,
    [property: JsonPropertyName("cases")] AssumptionTestCase[] Cases);

  private record SuiteEntry(
    [property: JsonPropertyName("name")] string Name,
    [property: JsonPropertyName("file")] string File,
    [property: JsonPropertyName("description")] string Description);

  private record Manifest(
    [property: JsonPropertyName("name")] string Name,
    [property: JsonPropertyName("description")] string Description,
    [property: JsonPropertyName("suites")] SuiteEntry[] Suites);

  private static string FindRepoRoot()
  {
    var current = new DirectoryInfo(AppContext.BaseDirectory);
    while (current != null)
    {
      if (File.Exists(Path.Combine(current.FullName, "CITATION.cff")))
        return current.FullName;
      current = current.Parent;
    }
    throw new InvalidOperationException("Could not find repository root (CITATION.cff not found)");
  }

  private static double ParseValue(JsonElement element)
  {
    if (element.ValueKind == JsonValueKind.Number)
      return element.GetDouble();
    if (element.ValueKind == JsonValueKind.String)
    {
      var str = element.GetString();
      return str switch
      {
        "NaN" => double.NaN,
        "Infinity" => double.PositiveInfinity,
        "-Infinity" => double.NegativeInfinity,
        _ => throw new ArgumentException($"Unknown string value: {str}")
      };
    }
    throw new ArgumentException($"Unexpected value kind: {element.ValueKind}");
  }

  private static double[] ParseArray(JsonElement[]? arr)
  {
    if (arr == null)
      return [];
    return arr.Select(ParseValue).ToArray();
  }

  private static double CallFunction(string funcName, double[] x, double[] y)
  {
    var sampleX = new Sample(x);
    return funcName switch
    {
      "Center" => sampleX.Center(),
      "Ratio" => sampleX.Ratio(new Sample(y)),
      "RelSpread" => sampleX.RelSpread(),
      "Spread" => sampleX.Spread(),
      "Shift" => sampleX.Shift(new Sample(y)),
      "AvgSpread" => sampleX.AvgSpread(new Sample(y)),
      "Disparity" => sampleX.Disparity(new Sample(y)),
      _ => throw new ArgumentException($"Unknown function: {funcName}")
    };
  }

  public static IEnumerable<object[]> GetTestCases()
  {
    var repoRoot = FindRepoRoot();
    var assumptionsDir = Path.Combine(repoRoot, "tests", "assumptions");
    var manifestPath = Path.Combine(assumptionsDir, "manifest.json");
    var manifestJson = File.ReadAllText(manifestPath);
    var manifest = JsonSerializer.Deserialize<Manifest>(manifestJson)!;

    foreach (var suiteEntry in manifest.Suites)
    {
      var suitePath = Path.Combine(assumptionsDir, suiteEntry.File);
      var suiteJson = File.ReadAllText(suitePath);
      var suite = JsonSerializer.Deserialize<AssumptionTestSuite>(suiteJson)!;

      foreach (var testCase in suite.Cases)
      {
        yield return [suite.Suite, testCase];
      }
    }
  }

  public static TheoryData<string, AssumptionTestCase> TestData
  {
    get
    {
      var data = new TheoryData<string, AssumptionTestCase>();
      foreach (var item in GetTestCases())
      {
        data.Add((string)item[0], (AssumptionTestCase)item[1]);
      }
      return data;
    }
  }

  [Theory]
  [MemberData(nameof(TestData))]
  public void AssumptionViolationTest(string suiteName, AssumptionTestCase testCase)
  {
    _ = suiteName; // Used for test display name

    var x = ParseArray(testCase.Inputs.X);
    var y = ParseArray(testCase.Inputs.Y);

    var expectedId = testCase.ExpectedViolation.Id;

    var ex = Assert.Throws<AssumptionException>(() => CallFunction(testCase.Function, x, y));

    Assert.Equal(expectedId, ex.Violation.IdString);
  }
}
