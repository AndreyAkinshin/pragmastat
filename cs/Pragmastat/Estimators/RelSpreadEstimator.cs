using Pragmastat.Internal;
using Pragmastat.Metrology;

namespace Pragmastat.Estimators;

public class RelSpreadEstimator : IOneSampleEstimator
{
  public static readonly RelSpreadEstimator Instance = new();

  public Measurement Estimate(Sample x)
  {
    var center = x.Center();
    if (center.NominalValue == 0)
      throw new ArgumentException("RelSpread is undefined when Center equals zero", nameof(x));

    return (x.Spread() / Abs(center)).NominalValue.WithUnit(NumberUnit.Instance);
  }
}
