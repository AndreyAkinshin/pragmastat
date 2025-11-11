using JetBrains.Annotations;

namespace Pragmastat.TestGenerator.Framework.Distributions;

[UsedImplicitly(ImplicitUseTargetFlags.Members)]
public class DistributionInput
{
  public Dictionary<string, double> Parameters { get; set; } = new();
  public double[] X { get; set; } = [];
  public double[] P { get; set; } = [];
}
