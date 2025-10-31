using JetBrains.Annotations;

namespace Pragmastat.ReferenceTests.Generator.Framework.Distributions;

[UsedImplicitly(ImplicitUseTargetFlags.Members)]
public class DistributionOutput
{
  public double[] Pdf { get; set; } = [];
  public double[] Cdf { get; set; } = [];
  public double[] Quantiles { get; set; } = [];
}
