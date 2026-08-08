using JetBrains.Annotations;

namespace Pragmastat.TestGenerator.Framework.Distributions;

/// <summary>
/// The three answers a family gives, under the names this toolkit uses for them.
/// </summary>
/// <remarks>
/// Not Pdf, Cdf and Quantiles. Those are the traditional abbreviations, and the manual mentions
/// them once so a reader arriving from a statistics textbook can find their footing; they are not
/// the names anything here is called by. These fields are serialized into the shared fixtures, so
/// the spelling is part of the format that all seven ports read.
/// </remarks>
[UsedImplicitly(ImplicitUseTargetFlags.Members)]
public class DistributionOutput
{
  public double[] Density { get; set; } = [];
  public double[] Cumulative { get; set; } = [];
  public double[] Quantile { get; set; } = [];
}
