namespace Pragmastat.ReferenceTests.ReferenceTesting.SingleDoubleValue;

public abstract class SingleDoubleValueTestBase : ReferenceTestBase<SingleDoubleValueInput, double[]>
{
  private record TestCase(string Name, Func<double, double> Func, double[] Input);

  private readonly List<TestCase> testCases = [];

  protected void RegisterFunction(string name, Func<double, double> func, double[] input)
  {
    testCases.Add(new TestCase(name, func, input));
  }

  protected override ReferenceTestController<SingleDoubleValueInput, double[]> CreateController() =>
    new SingleDoubleValueController(
      GetSuiteName(),
      testCases.ToDictionary(testCast => testCast.Name, testCast => testCast.Func));

  protected override ReferenceTestCaseInputBuilder<SingleDoubleValueInput> GetInputBuilder()
  {
    var builder = new ReferenceTestCaseInputBuilder<SingleDoubleValueInput>();
    foreach (var testCase in testCases)
      builder.Add(testCase.Name, new SingleDoubleValueInput(testCase.Name, testCase.Input));
    return builder;
  }
}
