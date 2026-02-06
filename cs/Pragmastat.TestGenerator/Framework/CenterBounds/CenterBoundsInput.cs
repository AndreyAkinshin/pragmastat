using JetBrains.Annotations;

namespace Pragmastat.TestGenerator.Framework.CenterBounds;

[UsedImplicitly(ImplicitUseTargetFlags.WithMembers)]
public class CenterBoundsInput
{
  public double[] X { get; set; } = [];
  public double Misrate { get; set; }

  public CenterBoundsInput()
  {
  }

  public CenterBoundsInput(Sample x, double misrate)
  {
    X = x.Values.ToArray();
    Misrate = misrate;
  }

  public Sample GetSample() => new Sample(X);
}
