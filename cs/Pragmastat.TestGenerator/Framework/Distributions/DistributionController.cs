using Pragmastat.Distributions;

namespace Pragmastat.TestGenerator.Framework.Distributions;

public class DistributionController(
  string name,
  Func<Dictionary<string, double>, IDistribution> distributionFromParameters,
  double eps = 0,
  bool shared = true)
  : ReferenceTestController<DistributionInput, DistributionOutput>(shared: shared)
{
  protected override string SuiteName { get; } = name;

  /// <summary>
  /// Compares by payload when eps is zero, and within eps otherwise.
  /// </summary>
  /// <remarks>
  /// The trio reaches no platform transcendental in any of the seven ports, so the seven owe each
  /// other identical bits and a tolerance would pass on exactly the divergence the suite exists to
  /// catch. These suites used to be C#-only and compared at 1e-9, which is both.
  /// </remarks>
  public override bool Assert(DistributionOutput expected, DistributionOutput actual)
  {
    return AreEqual(expected.Cumulative, actual.Cumulative) &&
           AreEqual(expected.Density, actual.Density) &&
           AreEqual(expected.Quantile, actual.Quantile);
  }

  private bool AreEqual(double[]? expected, double[]? actual)
  {
    if (expected == null || actual == null)
      return expected == actual;
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

  public override DistributionOutput Run(DistributionInput input)
  {
    var distribution = distributionFromParameters(input.Parameters);
    return new DistributionOutput
    {
      Density = input.X.Select(distribution.DensityAt).ToArray(),
      Cumulative = input.X.Select(distribution.CumulativeAt).ToArray(),
      Quantile = input.P.Select(p => distribution.QuantileAt(p)).ToArray()
    };
  }
}
