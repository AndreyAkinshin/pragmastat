using JetBrains.Annotations;

namespace Pragmastat.TestGenerator.Framework.SpreadBounds;

[UsedImplicitly(ImplicitUseTargetFlags.WithMembers)]
public class SpreadBoundsInput
{
  public double[] X { get; set; } = [];
  public double Misrate { get; set; }
  public string? Seed { get; set; }

  public SpreadBoundsInput()
  {
  }

  public SpreadBoundsInput(Sample x, double misrate, string? seed = null)
  {
    X = x.Values.ToArray();
    Misrate = misrate;
    Seed = seed;
  }

  public Sample GetSample() => new Sample(X);
}
