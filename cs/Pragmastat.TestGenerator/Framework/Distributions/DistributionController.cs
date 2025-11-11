using Pragmastat.Distributions;

namespace Pragmastat.TestGenerator.Framework.Distributions;

public class DistributionController(
  string name,
  Func<Dictionary<string, double>, IContinuousDistribution> distributionFromParameters,
  double eps = 1e-9)
  : ReferenceTestController<DistributionInput, DistributionOutput>
{
  protected override string SuiteName { get; } = name;

  public override bool Assert(DistributionOutput expected, DistributionOutput actual)
  {
    return AreEqual(expected.Cdf, actual.Cdf, eps) &&
           AreEqual(expected.Pdf, actual.Pdf, eps) &&
           AreEqual(expected.Quantiles, actual.Quantiles, eps);
  }

  public override DistributionOutput Run(DistributionInput input)
  {
    var distribution = distributionFromParameters(input.Parameters);
    return new DistributionOutput
    {
      Pdf = input.X.Select(distribution.Pdf).ToArray(),
      Cdf = input.X.Select(distribution.Cdf).ToArray(),
      Quantiles = input.P.Select(p => distribution.Quantile(p)).ToArray()
    };
  }
}
