using System.Text.Json;

namespace Pragmastat.TestGenerator.Framework.MedianBounds;

public class MedianBoundsController(string name, double eps = 1e-9)
  : ReferenceTestController<MedianBoundsInput, MedianBoundsOutput>(shared: true)
{
  protected override string SuiteName { get; } = name;

  public override bool Assert(MedianBoundsOutput expected, MedianBoundsOutput actual)
  {
    return Math.Abs(expected.Lower - actual.Lower) < eps &&
           Math.Abs(expected.Upper - actual.Upper) < eps;
  }

  public override MedianBoundsOutput Run(MedianBoundsInput input)
  {
    var bounds = Toolkit.MedianBounds(input.GetSample(), new Probability(input.Misrate));
    return new MedianBoundsOutput(bounds);
  }

  public ErrorTestCase<MedianBoundsInput> LoadErrorTestCase(string testName)
  {
    string testSuiteDirectory = ReferenceTestSuiteHelper.GetTestSuiteDirectory(SuiteName, true);
    string filePath = Path.Combine(testSuiteDirectory, testName + ".json");
    string testCaseJson = File.ReadAllText(filePath);
    var options = new JsonSerializerOptions { PropertyNamingPolicy = JsonNamingPolicy.CamelCase };
    return JsonSerializer.Deserialize<ErrorTestCase<MedianBoundsInput>>(testCaseJson, options)
           ?? throw new InvalidOperationException($"Failed to deserialize error test case: {testName}");
  }
}
