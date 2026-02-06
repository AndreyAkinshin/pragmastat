using JetBrains.Annotations;

namespace Pragmastat.TestGenerator.Framework.MedianBounds;

[UsedImplicitly(ImplicitUseTargetFlags.WithMembers)]
public class MedianBoundsInput
{
  public double[] X { get; set; } = [];
  public double Misrate { get; set; }

  public MedianBoundsInput()
  {
  }

  public MedianBoundsInput(Sample x, double misrate)
  {
    X = x.Values.ToArray();
    Misrate = misrate;
  }

  public Sample GetSample() => new Sample(X);
}
