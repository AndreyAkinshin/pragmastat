using JetBrains.Annotations;
using Pragmastat.Exceptions;

namespace Pragmastat.TestGenerator.Framework.Compare;

[UsedImplicitly(ImplicitUseTargetFlags.WithMembers)]
public class Compare2Input
{
  public double[] X { get; set; } = [];
  public double[] Y { get; set; } = [];
  public string? Seed { get; set; }
  public ThresholdInput[] Thresholds { get; set; } = [];

  public Compare2Input()
  {
  }

  public Compare2Input(Sample x, Sample y, ThresholdInput[] thresholds, string? seed = null)
  {
    X = x.Values.ToArray();
    Y = y.Values.ToArray();
    Thresholds = thresholds;
    Seed = seed;
  }

  public Sample GetSampleX() => new Sample(X);
  public Sample GetSampleY() => new Sample(Y, validationSubject: Subject.Y);

  public IReadOnlyList<Threshold> GetThresholds()
    => Thresholds.Select(t => t.ToThreshold()).ToList();
}
