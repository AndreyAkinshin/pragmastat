using JetBrains.Annotations;

namespace Pragmastat.TestGenerator.Framework.AvgSpreadBounds;

[UsedImplicitly(ImplicitUseTargetFlags.WithMembers)]
public class AvgSpreadBoundsOutput
{
  public double Lower { get; set; }
  public double Upper { get; set; }

  public AvgSpreadBoundsOutput()
  {
  }

  public AvgSpreadBoundsOutput(double lower, double upper)
  {
    Lower = lower;
    Upper = upper;
  }

  public AvgSpreadBoundsOutput(Bounds bounds)
  {
    Lower = bounds.Lower;
    Upper = bounds.Upper;
  }
}
