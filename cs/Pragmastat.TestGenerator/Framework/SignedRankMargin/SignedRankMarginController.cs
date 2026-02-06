using System.Text.Json;

namespace Pragmastat.TestGenerator.Framework.SignedRankMargin;

public class SignedRankMarginController(string name)
  : ReferenceTestController<SignedRankMarginInput, int>(shared: true)
{
  protected override string SuiteName { get; } = name;
  public override bool Assert(int expected, int actual) => expected == actual;

  public override int Run(SignedRankMarginInput input) =>
    Functions.SignedRankMargin.Instance.Calc(input.N, input.Misrate);

  public ErrorTestCase<SignedRankMarginInput> LoadErrorTestCase(string testName)
  {
    string testSuiteDirectory = ReferenceTestSuiteHelper.GetTestSuiteDirectory(SuiteName, true);
    string filePath = Path.Combine(testSuiteDirectory, testName + ".json");
    string testCaseJson = File.ReadAllText(filePath);
    var options = new JsonSerializerOptions { PropertyNamingPolicy = JsonNamingPolicy.CamelCase };
    return JsonSerializer.Deserialize<ErrorTestCase<SignedRankMarginInput>>(testCaseJson, options)
           ?? throw new InvalidOperationException($"Failed to deserialize error test case: {testName}");
  }
}
