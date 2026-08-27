using System.Text.Json;
using JetBrains.Annotations;
using Pragmastat.Distributions;
using Pragmastat.Exceptions;
using Pragmastat.Randomization;
using Pragmastat.TestGenerator.Framework;

namespace Pragmastat.Tests.Distributions;

/// <summary>
/// LinearDistModel and NeighborDistModel against their shared dist-linear/dist-neighbor
/// fixtures.
/// </summary>
/// <remarks>
/// Unlike the five parametric families in <see cref="DistFamilyReferenceTests"/>, this suite's
/// oracle is not one of the seven ports: it is an independent binary64 + exact-Fraction
/// reimplementation (tests/oracles/gen_dist_tests.py), written directly from the normative
/// algorithm so a bug shared by every language cannot pass itself. Every case is compared by
/// binary64 payload, matching the suite's declared exact class in tests/manifest.json. Only the
/// raw vector path is exercised here, never the Sample-wrapped measured path.
/// </remarks>
public class DistReferenceTests
{
  [UsedImplicitly]
  public static readonly TheoryData<string> LinearQuantileCases =
    ReferenceTestSuiteHelper.GetTheoryData("dist-linear/quantile", true);
  [UsedImplicitly]
  public static readonly TheoryData<string> LinearCumulativeCases =
    ReferenceTestSuiteHelper.GetTheoryData("dist-linear/cumulative", true);
  [UsedImplicitly]
  public static readonly TheoryData<string> LinearDensityCases =
    ReferenceTestSuiteHelper.GetTheoryData("dist-linear/density", true);
  [UsedImplicitly]
  public static readonly TheoryData<string> LinearSampleCases =
    ReferenceTestSuiteHelper.GetTheoryData("dist-linear/sample", true);

  [UsedImplicitly]
  public static readonly TheoryData<string> NeighborQuantileCases =
    ReferenceTestSuiteHelper.GetTheoryData("dist-neighbor/quantile", true);
  [UsedImplicitly]
  public static readonly TheoryData<string> NeighborCumulativeCases =
    ReferenceTestSuiteHelper.GetTheoryData("dist-neighbor/cumulative", true);
  [UsedImplicitly]
  public static readonly TheoryData<string> NeighborDensityCases =
    ReferenceTestSuiteHelper.GetTheoryData("dist-neighbor/density", true);
  [UsedImplicitly]
  public static readonly TheoryData<string> NeighborSampleCases =
    ReferenceTestSuiteHelper.GetTheoryData("dist-neighbor/sample", true);
  [UsedImplicitly]
  public static readonly TheoryData<string> NeighborLandscapeCases =
    ReferenceTestSuiteHelper.GetTheoryData("dist-neighbor/landscape", true);

  [Theory]
  [MemberData(nameof(LinearQuantileCases))]
  public void LinearQuantileReference(string testName) =>
    CheckQuery("dist-linear/quantile", testName, BuildLinear, (model, y) => model.Quantile(y));

  [Theory]
  [MemberData(nameof(LinearCumulativeCases))]
  public void LinearCumulativeReference(string testName) =>
    CheckQuery("dist-linear/cumulative", testName, BuildLinear, (model, y) => model.Cumulative(y));

  [Theory]
  [MemberData(nameof(LinearDensityCases))]
  public void LinearDensityReference(string testName) =>
    CheckQuery("dist-linear/density", testName, BuildLinear, (model, y) => model.Density(y));

  [Theory]
  [MemberData(nameof(LinearSampleCases))]
  public void LinearSampleReference(string testName) =>
    CheckSample("dist-linear/sample", testName, x => BuildLinear(x, false));

  [Theory]
  [MemberData(nameof(NeighborQuantileCases))]
  public void NeighborQuantileReference(string testName) =>
    CheckQuery("dist-neighbor/quantile", testName, BuildNeighbor, (model, y) => model.Quantile(y));

  [Theory]
  [MemberData(nameof(NeighborCumulativeCases))]
  public void NeighborCumulativeReference(string testName) =>
    CheckQuery("dist-neighbor/cumulative", testName, BuildNeighbor, (model, y) => model.Cumulative(y));

  [Theory]
  [MemberData(nameof(NeighborDensityCases))]
  public void NeighborDensityReference(string testName) =>
    CheckQuery("dist-neighbor/density", testName, BuildNeighbor, (model, y) => model.Density(y));

  [Theory]
  [MemberData(nameof(NeighborSampleCases))]
  public void NeighborSampleReference(string testName) =>
    CheckSample("dist-neighbor/sample", testName, x => BuildNeighbor(x, false));

  [Theory]
  [MemberData(nameof(NeighborLandscapeCases))]
  public void NeighborLandscapeReference(string testName)
  {
    JsonElement root = LoadCase("dist-neighbor/landscape", testName);
    JsonElement input = root.GetProperty("input");
    double[] x = ParseValuesWithSpecialFloats(input.GetProperty("x"));

    var model = new NeighborDistModel(x);
    NeighborLandscape landscape = model.GetLandscape();

    JsonElement output = root.GetProperty("output");
    string context = $"Suite: dist-neighbor/landscape, test {testName}";

    Assert.Equal(output.GetProperty("k0").GetInt32(), landscape.ScaleIndex);
    Assert.Equal(output.GetProperty("radius").GetInt32(), landscape.Radius);
    BitwiseAssert.Equal(
      ParseValuesWithSpecialFloats(output.GetProperty("boundaries")),
      landscape.Boundaries,
      context + ", boundaries");
    BitwiseAssert.Equal(
      ParseValuesWithSpecialFloats(output.GetProperty("cumulative")),
      landscape.Cumulative,
      context + ", cumulative");
    BitwiseAssert.Equal(
      ParseValuesWithSpecialFloats(output.GetProperty("masses")),
      landscape.Masses,
      context + ", masses");
    BitwiseAssert.Equal(
      ParseValuesWithSpecialFloats(output.GetProperty("densities")),
      landscape.Densities,
      context + ", densities");

    int[] expectedSources = output.GetProperty("sources").EnumerateArray().Select(e => e.GetInt32()).ToArray();
    Assert.Equal(expectedSources, landscape.Sources);
  }

  private static LinearDistModel BuildLinear(double[] x, bool assumeSorted) => new(x, assumeSorted);

  private static NeighborDistModel BuildNeighbor(double[] x, bool assumeSorted) => new(x, assumeSorted);

  /// <summary>
  /// Runs one quantile/cumulative/density fixture. An expected_error case is checked before the
  /// operation ever runs: whether the model refuses to build, or builds and then refuses the
  /// query, is asserted against the fixture's own claim, never inferred from whatever exception
  /// happened to come out.
  /// </summary>
  private static void CheckQuery<TModel>(
    string suiteName,
    string testName,
    Func<double[], bool, TModel> build,
    Func<TModel, double[], double[]> query)
  {
    JsonElement root = LoadCase(suiteName, testName);
    JsonElement input = root.GetProperty("input");
    double[] x = ParseValuesWithSpecialFloats(input.GetProperty("x"));
    double[] y = ParseValuesWithSpecialFloats(input.GetProperty("y"));
    bool assumeSorted = input.TryGetProperty("assume_sorted", out var assumeSortedElement) &&
                         assumeSortedElement.GetBoolean();

    if (root.TryGetProperty("expected_error", out var expectedErrorElement))
    {
      var expectedError = expectedErrorElement.Deserialize<ExpectedError>(JsonOptions)!;
      var ex = Assert.Throws<AssumptionException>(() => query(build(x, assumeSorted), y));
      Assert.Equal(expectedError.Id, ex.Violation.Id.ToId());
      Assert.Equal(expectedError.Subject, ex.Violation.Subject.ToId());
      return;
    }

    TModel model = build(x, assumeSorted);
    double[] actual = query(model, y);
    double[] expected = ParseValuesWithSpecialFloats(root.GetProperty("output"));
    BitwiseAssert.Equal(expected, actual, $"Suite: {suiteName}, test {testName}");
  }

  /// <summary>No case under a sample sub-suite declares an expected_error.</summary>
  private static void CheckSample<TModel>(string suiteName, string testName, Func<double[], TModel> build)
    where TModel : IDistribution
  {
    JsonElement root = LoadCase(suiteName, testName);
    JsonElement input = root.GetProperty("input");
    double[] x = ParseValuesWithSpecialFloats(input.GetProperty("x"));
    string seed = input.GetProperty("seed").GetString()!;
    int count = input.GetProperty("count").GetInt32();

    TModel model = build(x);
    var rng = new Rng(seed);
    List<double> actual = model.Samples(rng, count);

    double[] expected = ParseValuesWithSpecialFloats(root.GetProperty("output"));
    BitwiseAssert.Equal(expected, actual, $"Suite: {suiteName}, test {testName}");
  }

  private static readonly JsonSerializerOptions JsonOptions =
    new() { PropertyNamingPolicy = JsonNamingPolicy.CamelCase };

  private static JsonElement LoadCase(string suiteName, string testName)
  {
    string directory = ReferenceTestSuiteHelper.GetTestSuiteDirectory(suiteName, shared: true);
    string filePath = Path.Combine(directory, testName + ".json");
    string json = File.ReadAllText(filePath);
    using var doc = JsonDocument.Parse(json);
    return doc.RootElement.Clone();
  }

  /// <summary>
  /// JSON has no literal for NaN or an infinity, so the generator spells them as the strings
  /// "NaN"/"Infinity"/"-Infinity" in place of a number -- both on the input side (a handful of
  /// error-nan-*/error-inf-* fixtures) and on the output side (extreme-overflow-* density can
  /// legitimately diverge to Infinity).
  /// </summary>
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
}
