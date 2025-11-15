using JetBrains.Annotations;

namespace Pragmastat.TestGenerator.Framework.ShiftBounds;

[UsedImplicitly(ImplicitUseTargetFlags.WithMembers)]
public class ShiftBoundsOutput
{
  public double Lower { get; set; }
  public double Upper { get; set; }

  public ShiftBoundsOutput()
  {
  }

  public ShiftBoundsOutput(double lower, double upper)
  {
    Lower = lower;
    Upper = upper;
  }

  public ShiftBoundsOutput(Bounds bounds)
  {
    Lower = bounds.Lower;
    Upper = bounds.Upper;
  }
}

