using JetBrains.Annotations;
using Pragmastat.Metrology;

namespace Pragmastat.TestGenerator.Framework.Compare;

[UsedImplicitly(ImplicitUseTargetFlags.WithMembers)]
public class ThresholdInput
{
  public string Metric { get; set; } = "";
  public double Value { get; set; }
  public double Misrate { get; set; }

  public ThresholdInput()
  {
  }

  public ThresholdInput(string metric, double value, double misrate)
  {
    Metric = metric;
    Value = value;
    Misrate = misrate;
  }

  public Threshold ToThreshold() => new(
    ParseMetric(Metric),
    new Measurement(Value, MeasurementUnit.Number),
    new Probability(Misrate));

  private static global::Pragmastat.Metric ParseMetric(string s) => s.ToLowerInvariant() switch
  {
    "center" => global::Pragmastat.Metric.Center,
    "spread" => global::Pragmastat.Metric.Spread,
    "shift" => global::Pragmastat.Metric.Shift,
    "ratio" => global::Pragmastat.Metric.Ratio,
    "disparity" => global::Pragmastat.Metric.Disparity,
    _ => throw new ArgumentException($"Unknown metric: {s}")
  };
}
