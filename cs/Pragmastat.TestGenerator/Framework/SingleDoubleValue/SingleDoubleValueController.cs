namespace Pragmastat.TestGenerator.Framework.SingleDoubleValue;

public class SingleDoubleValueController(
  string name,
  Dictionary<string, Func<double, double>> functions,
  double eps = 1e-9,
  bool shared = false)
  : ReferenceTestController<SingleDoubleValueInput, double[]>(shared: shared)
{
  protected override string SuiteName { get; } = name;

  /// <summary>
  /// Compares by payload when eps is zero, and within eps otherwise.
  /// </summary>
  /// <remarks>
  /// A suite whose point is that seven implementations agree bit for bit cannot be checked with a
  /// tolerance, since a tolerance passes on exactly the divergence it exists to catch.
  /// </remarks>
  public override bool Assert(double[] expected, double[] actual)
  {
    if (expected.Length != actual.Length)
      return false;
    for (int i = 0; i < expected.Length; i++)
    {
      bool ok = eps == 0
        ? BitConverter.DoubleToInt64Bits(expected[i]) == BitConverter.DoubleToInt64Bits(actual[i])
        : Math.Abs(expected[i] - actual[i]) <= eps;
      if (!ok)
        return false;
    }
    return true;
  }

  public override double[] Run(SingleDoubleValueInput input) => input.Arg.Select(functions[input.Name]).ToArray();
}
