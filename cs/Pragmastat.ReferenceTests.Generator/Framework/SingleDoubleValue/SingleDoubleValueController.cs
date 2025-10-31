namespace Pragmastat.ReferenceTests.Generator.Framework.SingleDoubleValue;

public class SingleDoubleValueController(
  string name,
  Dictionary<string, Func<double, double>> functions,
  double eps = 1e-9)
  : ReferenceTestController<SingleDoubleValueInput, double[]>
{
  protected override string SuiteName { get; } = name;

  public override bool Assert(double[] expected, double[] actual)
  {
    if (expected.Length != actual.Length)
      return false;
    for (int i = 0; i < expected.Length; i++)
      if (Math.Abs(expected[i] - actual[i]) > eps)
        return false;
    return true;
  }

  public override double[] Run(SingleDoubleValueInput input) => input.Arg.Select(functions[input.Name]).ToArray();
}
