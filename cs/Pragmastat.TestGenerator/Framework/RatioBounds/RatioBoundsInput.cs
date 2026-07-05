using JetBrains.Annotations;

namespace Pragmastat.TestGenerator.Framework.RatioBounds;

[UsedImplicitly(ImplicitUseTargetFlags.WithMembers)]
public class RatioBoundsInput
{
  public double[] X { get; set; } = [];
  public double[] Y { get; set; } = [];
  public double Misrate { get; set; }

  public RatioBoundsInput()
  {
  }

  public RatioBoundsInput(Sample x, Sample y, double misrate)
  {
    X = x.Values.ToArray();
    Y = y.Values.ToArray();
    Misrate = misrate;
  }

  public Sample GetSampleX() => new Sample(X);
  public Sample GetSampleY() => new Sample(Y);
}
