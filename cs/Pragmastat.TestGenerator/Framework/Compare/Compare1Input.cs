using JetBrains.Annotations;
using Pragmastat.Exceptions;

namespace Pragmastat.TestGenerator.Framework.Compare;

[UsedImplicitly(ImplicitUseTargetFlags.WithMembers)]
public class Compare1Input
{
  public double[] X { get; set; } = [];
  public string? Seed { get; set; }
  public ThresholdInput[] Thresholds { get; set; } = [];

  public Compare1Input()
  {
  }

  public Compare1Input(Sample x, ThresholdInput[] thresholds, string? seed = null)
  {
    X = x.Values.ToArray();
    Thresholds = thresholds;
    Seed = seed;
  }

  public Sample GetSampleX() => new Sample(X);

  public IReadOnlyList<Threshold> GetThresholds()
    => Thresholds.Select(t => t.ToThreshold()).ToList();
}
