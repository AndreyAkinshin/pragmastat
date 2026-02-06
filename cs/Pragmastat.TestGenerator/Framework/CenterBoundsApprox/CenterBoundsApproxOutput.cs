using JetBrains.Annotations;

namespace Pragmastat.TestGenerator.Framework.CenterBoundsApprox;

[UsedImplicitly(ImplicitUseTargetFlags.WithMembers)]
public class CenterBoundsApproxOutput
{
  public double Lower { get; set; }
  public double Upper { get; set; }

  public CenterBoundsApproxOutput()
  {
  }

  public CenterBoundsApproxOutput(double lower, double upper)
  {
    Lower = lower;
    Upper = upper;
  }

  public CenterBoundsApproxOutput(Bounds bounds)
  {
    Lower = bounds.Lower;
    Upper = bounds.Upper;
  }
}
