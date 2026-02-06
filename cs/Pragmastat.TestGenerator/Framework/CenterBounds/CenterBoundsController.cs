using System.Text.Json;

namespace Pragmastat.TestGenerator.Framework.CenterBounds;

public class CenterBoundsController(string name, double eps = 1e-9)
  : ReferenceTestController<CenterBoundsInput, CenterBoundsOutput>(shared: true)
{
  protected override string SuiteName { get; } = name;

  public override bool Assert(CenterBoundsOutput expected, CenterBoundsOutput actual)
  {
    return Math.Abs(expected.Lower - actual.Lower) < eps &&
           Math.Abs(expected.Upper - actual.Upper) < eps;
  }

  public override CenterBoundsOutput Run(CenterBoundsInput input)
  {
    var bounds = Toolkit.CenterBounds(input.GetSample(), new Probability(input.Misrate));
    return new CenterBoundsOutput(bounds);
  }

  public ErrorTestCase<CenterBoundsInput> LoadErrorTestCase(string testName)
  {
    string testSuiteDirectory = ReferenceTestSuiteHelper.GetTestSuiteDirectory(SuiteName, true);
    string filePath = Path.Combine(testSuiteDirectory, testName + ".json");
    string testCaseJson = File.ReadAllText(filePath);
    var options = new JsonSerializerOptions { PropertyNamingPolicy = JsonNamingPolicy.CamelCase };
    return JsonSerializer.Deserialize<ErrorTestCase<CenterBoundsInput>>(testCaseJson, options)
           ?? throw new InvalidOperationException($"Failed to deserialize error test case: {testName}");
  }
}
