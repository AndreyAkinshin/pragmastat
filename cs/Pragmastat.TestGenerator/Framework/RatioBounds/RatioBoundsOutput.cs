using JetBrains.Annotations;

namespace Pragmastat.TestGenerator.Framework.RatioBounds;

[UsedImplicitly(ImplicitUseTargetFlags.WithMembers)]
public class RatioBoundsOutput
{
  public double Lower { get; set; }
  public double Upper { get; set; }

  public RatioBoundsOutput()
  {
  }

  public RatioBoundsOutput(double lower, double upper)
  {
    Lower = lower;
    Upper = upper;
  }

  public RatioBoundsOutput(Bounds bounds)
  {
    Lower = bounds.Lower;
    Upper = bounds.Upper;
  }
}
