using JetBrains.Annotations;

namespace Pragmastat.TestGenerator.Framework.CenterBoundsApprox;

[UsedImplicitly(ImplicitUseTargetFlags.WithMembers)]
public class CenterBoundsApproxInput
{
  public double[] X { get; set; } = [];
  public double Misrate { get; set; }
  public string? Seed { get; set; }

  public CenterBoundsApproxInput()
  {
  }

  public CenterBoundsApproxInput(Sample x, double misrate, string? seed = null)
  {
    X = x.Values.ToArray();
    Misrate = misrate;
    Seed = seed;
  }

  public Sample GetSample() => new Sample(X);
}
