using JetBrains.Annotations;

namespace Pragmastat.TestGenerator.Framework.ShiftBounds;

[UsedImplicitly(ImplicitUseTargetFlags.WithMembers)]
public class ShiftBoundsInput
{
  public double[] X { get; set; } = [];
  public double[] Y { get; set; } = [];
  public double Misrate { get; set; }

  public ShiftBoundsInput()
  {
  }

  public ShiftBoundsInput(Sample x, Sample y, double misrate)
  {
    X = x.Values.ToArray();
    Y = y.Values.ToArray();
    Misrate = misrate;
  }

  public Sample GetSampleX() => new Sample(X);
  public Sample GetSampleY() => new Sample(Y);
}

