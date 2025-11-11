using JetBrains.Annotations;
using Pragmastat.Distributions;
using Pragmastat.Internal;

namespace Pragmastat.TestGenerator.Framework.Distributions;

public class DistributionInputBuilder : ReferenceTestCaseInputBuilder<DistributionInput>
{
  [PublicAPI]
  public DistributionInputBuilder Add(
    string name,
    Dictionary<string, double> parameters,
    double[] x,
    double[] p)
  {
    Add(name, new DistributionInput { Parameters = parameters, X = x, P = p });
    return this;
  }

  [PublicAPI]
  public DistributionInputBuilder Add(IContinuousDistribution distribution)
  {
    var parameters = CtorArgumentSerializer.SerializeToList(distribution);
    string name = parameters.Select(pair => $"{pair.Name}{pair.Value:R}").JoinToString("_");

    var x = new List<double>();
    var props = new List<double>();
    for (int i = 0; i <= 100; i++)
    {
      double p = i / 100.0;
      double q = distribution.Quantile(p);
      if (double.IsFinite(q))
      {
        props.Add(p);
        if (HasPdf(distribution, q))
          x.Add(q);
      }
    }

    Add(name, new DistributionInput
    {
      Parameters = CtorArgumentSerializer.SerializeToDictionary(distribution),
      X = x.ToArray(),
      P = props.ToArray()
    });
    return this;
  }

  private static bool HasPdf(IContinuousDistribution distribution, double x)
  {
    try
    {
      return double.IsFinite(distribution.Pdf(x));
    }
    catch
    {
      return false;
    }
  }
}
