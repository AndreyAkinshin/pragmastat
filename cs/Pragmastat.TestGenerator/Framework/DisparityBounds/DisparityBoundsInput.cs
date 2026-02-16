using JetBrains.Annotations;

namespace Pragmastat.TestGenerator.Framework.DisparityBounds;

[UsedImplicitly(ImplicitUseTargetFlags.WithMembers)]
public class DisparityBoundsInput
{
  public double[] X { get; set; } = [];
  public double[] Y { get; set; } = [];
  public double Misrate { get; set; }
  public string? Seed { get; set; }

  public DisparityBoundsInput()
  {
  }

  public DisparityBoundsInput(Sample x, Sample y, double misrate, string? seed = null)
  {
    X = x.Values.ToArray();
    Y = y.Values.ToArray();
    Misrate = misrate;
    Seed = seed;
  }

  public Sample GetSampleX() => new Sample(X);
  public Sample GetSampleY() => new Sample(Y);
}
