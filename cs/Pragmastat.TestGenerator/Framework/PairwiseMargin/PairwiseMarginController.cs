using System.Text.Json;

namespace Pragmastat.TestGenerator.Framework.PairwiseMargin;

public class PairwiseMarginController(string name)
  : ReferenceTestController<PairwiseMarginInput, int>(shared: true)
{
  protected override string SuiteName { get; } = name;
  public override bool Assert(int expected, int actual) => expected == actual;

  public override int Run(PairwiseMarginInput input) =>
    Functions.PairwiseMargin.Instance.Calc(input.N, input.M, input.Misrate);

  public ErrorTestCase<PairwiseMarginInput> LoadErrorTestCase(string testName)
  {
    string testSuiteDirectory = ReferenceTestSuiteHelper.GetTestSuiteDirectory(SuiteName, true);
    string filePath = Path.Combine(testSuiteDirectory, testName + ".json");
    string testCaseJson = File.ReadAllText(filePath);
    var options = new JsonSerializerOptions { PropertyNamingPolicy = JsonNamingPolicy.CamelCase };
    return JsonSerializer.Deserialize<ErrorTestCase<PairwiseMarginInput>>(testCaseJson, options)
           ?? throw new InvalidOperationException($"Failed to deserialize error test case: {testName}");
  }
}

