using Pragmastat.Algorithms;
using Pragmastat.Exceptions;
using Pragmastat.Internal;
using Pragmastat.Metrology;

namespace Pragmastat.Estimators;

public class RelSpreadEstimator : IOneSampleEstimator
{
  public static readonly RelSpreadEstimator Instance = new();

  public Measurement Estimate(Sample x)
  {
    // Check validity (priority 0)
    Assertion.Validity(x, Subject.X, "RelSpread");
    // Check positivity (priority 1)
    Assertion.PositivityAssumption(x, Subject.X, "RelSpread");
    // Calculate center (we know x is valid, center should succeed)
    var centerVal = FastCenter.Estimate(x.SortedValues);
    // Calculate spread (using internal implementation since we already validated)
    var spreadVal = FastSpread.Estimate(x.SortedValues, isSorted: true);
    // center is guaranteed positive because all values are positive
    return (spreadVal / Abs(centerVal)).WithUnit(NumberUnit.Instance);
  }
}
