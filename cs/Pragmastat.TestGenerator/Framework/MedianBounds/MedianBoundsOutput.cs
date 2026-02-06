using JetBrains.Annotations;

namespace Pragmastat.TestGenerator.Framework.MedianBounds;

[UsedImplicitly(ImplicitUseTargetFlags.WithMembers)]
public class MedianBoundsOutput
{
  public double Lower { get; set; }
  public double Upper { get; set; }

  public MedianBoundsOutput()
  {
  }

  public MedianBoundsOutput(double lower, double upper)
  {
    Lower = lower;
    Upper = upper;
  }

  public MedianBoundsOutput(Bounds bounds)
  {
    Lower = bounds.Lower;
    Upper = bounds.Upper;
  }
}
