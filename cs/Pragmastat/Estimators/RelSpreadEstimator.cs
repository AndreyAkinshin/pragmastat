using Pragmastat.Algorithms;
using Pragmastat.Exceptions;
using Pragmastat.Internal;
using Pragmastat.Metrology;

namespace Pragmastat.Estimators;

[Obsolete("Use Spread(x) / Math.Abs(Center(x)) instead.")]
public class RelSpreadEstimator : IOneSampleEstimator
{
  public static readonly RelSpreadEstimator Instance = new();

  public Measurement Estimate(Sample x)
  {
    Assertion.NonWeighted("x", x);
    Assertion.PositivityAssumption(x, Subject.X);
    var centerVal = FastCenter.Estimate(x.SortedValues);
    var spreadVal = FastSpread.Estimate(x.SortedValues, isSorted: true);
    return (spreadVal / Abs(centerVal)).WithUnit(NumberUnit.Instance);
  }
}
