using JetBrains.Annotations;

namespace Pragmastat.ReferenceTests.Distributions.ReferenceTesting;

[UsedImplicitly(ImplicitUseTargetFlags.Members)]
public class DistributionOutput
{
    public double[] Pdf { get; set; } = [];
    public double[] Cdf { get; set; } = [];
    public double[] Quantiles { get; set; } = [];
}