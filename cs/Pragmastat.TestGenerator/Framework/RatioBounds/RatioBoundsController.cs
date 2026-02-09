using System.Text.Json;

namespace Pragmastat.TestGenerator.Framework.RatioBounds;

public class RatioBoundsController(string name, double eps = 1e-9)
  : ReferenceTestController<RatioBoundsInput, RatioBoundsOutput>(shared: true)
{
  protected override string SuiteName { get; } = name;

  public override bool Assert(RatioBoundsOutput expected, RatioBoundsOutput actual)
  {
    return Math.Abs(expected.Lower - actual.Lower) < eps &&
           Math.Abs(expected.Upper - actual.Upper) < eps;
  }

  public override RatioBoundsOutput Run(RatioBoundsInput input)
  {
    var bounds = Toolkit.RatioBounds(input.GetSampleX(), input.GetSampleY(), new Probability(input.Misrate));
    return new RatioBoundsOutput(bounds);
  }

  public ErrorTestCase<RatioBoundsInput> LoadErrorTestCase(string testName)
  {
    string testSuiteDirectory = ReferenceTestSuiteHelper.GetTestSuiteDirectory(SuiteName, true);
    string filePath = Path.Combine(testSuiteDirectory, testName + ".json");
    string testCaseJson = File.ReadAllText(filePath);
    var options = new JsonSerializerOptions { PropertyNamingPolicy = JsonNamingPolicy.CamelCase };
    return JsonSerializer.Deserialize<ErrorTestCase<RatioBoundsInput>>(testCaseJson, options)
           ?? throw new InvalidOperationException($"Failed to deserialize error test case: {testName}");
  }
}
