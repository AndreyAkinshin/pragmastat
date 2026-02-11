using JetBrains.Annotations;

namespace Pragmastat.TestGenerator.Framework.SpreadBounds;

[UsedImplicitly(ImplicitUseTargetFlags.WithMembers)]
public class SpreadBoundsOutput
{
  public double Lower { get; set; }
  public double Upper { get; set; }

  public SpreadBoundsOutput()
  {
  }

  public SpreadBoundsOutput(double lower, double upper)
  {
    Lower = lower;
    Upper = upper;
  }

  public SpreadBoundsOutput(Bounds bounds)
  {
    Lower = bounds.Lower;
    Upper = bounds.Upper;
  }
}
