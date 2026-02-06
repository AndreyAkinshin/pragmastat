using System.Text.Json;
using Pragmastat.Estimators;

namespace Pragmastat.TestGenerator.Framework.CenterBoundsApprox;

public class CenterBoundsApproxController(string name, double eps = 1e-9)
  : ReferenceTestController<CenterBoundsApproxInput, CenterBoundsApproxOutput>(shared: true)
{
  protected override string SuiteName { get; } = name;

  public override bool Assert(CenterBoundsApproxOutput expected, CenterBoundsApproxOutput actual)
  {
    return Math.Abs(expected.Lower - actual.Lower) < eps &&
           Math.Abs(expected.Upper - actual.Upper) < eps;
  }

  public override CenterBoundsApproxOutput Run(CenterBoundsApproxInput input)
  {
    var bounds = CenterBoundsApproxEstimator.Instance.Estimate(
      input.GetSample(),
      new Probability(input.Misrate),
      input.Seed);
    return new CenterBoundsApproxOutput(bounds);
  }

  public ErrorTestCase<CenterBoundsApproxInput> LoadErrorTestCase(string testName)
  {
    string testSuiteDirectory = ReferenceTestSuiteHelper.GetTestSuiteDirectory(SuiteName, true);
    string filePath = Path.Combine(testSuiteDirectory, testName + ".json");
    string testCaseJson = File.ReadAllText(filePath);
    var options = new JsonSerializerOptions { PropertyNamingPolicy = JsonNamingPolicy.CamelCase };
    return JsonSerializer.Deserialize<ErrorTestCase<CenterBoundsApproxInput>>(testCaseJson, options)
           ?? throw new InvalidOperationException($"Failed to deserialize error test case: {testName}");
  }
}
