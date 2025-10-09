using JetBrains.Annotations;

namespace Pragmastat.ReferenceTests.ReferenceTesting.TwoSample;

[UsedImplicitly(ImplicitUseTargetFlags.WithMembers)]
public class TwoSampleInput
{
  public Dictionary<string, double>? Parameters { get; set; }
  public double[] X { get; set; } = [];
  public double[] Y { get; set; } = [];

  public TwoSampleInput()
  {
  }

  public TwoSampleInput(Dictionary<string, double>? parameters, double[] x, double[] y)
  {
    Parameters = parameters;
    X = x;
    Y = y;
  }

  public TwoSampleInput(Sample x, Sample y, Dictionary<string, double>? parameters = null)
  {
    Parameters = parameters;
    X = x.Values.ToArray();
    Y = y.Values.ToArray();
  }

  public Sample GetSampleX() => new Sample(X);
  public Sample GetSampleY() => new Sample(Y);
}
