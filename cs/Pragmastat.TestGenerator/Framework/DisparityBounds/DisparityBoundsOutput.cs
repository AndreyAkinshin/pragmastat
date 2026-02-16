using JetBrains.Annotations;

namespace Pragmastat.TestGenerator.Framework.DisparityBounds;

[UsedImplicitly(ImplicitUseTargetFlags.WithMembers)]
public class DisparityBoundsOutput
{
  public double Lower { get; set; }
  public double Upper { get; set; }

  public DisparityBoundsOutput()
  {
  }

  public DisparityBoundsOutput(double lower, double upper)
  {
    Lower = lower;
    Upper = upper;
  }

  public DisparityBoundsOutput(Bounds bounds)
  {
    Lower = bounds.Lower;
    Upper = bounds.Upper;
  }
}
