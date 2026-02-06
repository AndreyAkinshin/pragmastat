using JetBrains.Annotations;

namespace Pragmastat.TestGenerator.Framework.CenterBounds;

[UsedImplicitly(ImplicitUseTargetFlags.WithMembers)]
public class CenterBoundsOutput
{
  public double Lower { get; set; }
  public double Upper { get; set; }

  public CenterBoundsOutput()
  {
  }

  public CenterBoundsOutput(double lower, double upper)
  {
    Lower = lower;
    Upper = upper;
  }

  public CenterBoundsOutput(Bounds bounds)
  {
    Lower = bounds.Lower;
    Upper = bounds.Upper;
  }
}
