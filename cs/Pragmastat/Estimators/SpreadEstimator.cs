using Pragmastat.Algorithms;
using Pragmastat.Exceptions;
using Pragmastat.Internal;
using Pragmastat.Metrology;

namespace Pragmastat.Estimators;

public class SpreadEstimator : IOneSampleEstimator
{
  public static readonly SpreadEstimator Instance = new();

  public Measurement Estimate(Sample x)
  {
    // Check validity first (priority 0)
    Assertion.Validity(x, Subject.X);
    // Check sparity (priority 2)
    Assertion.Sparity(x, Subject.X);
    return FastSpread.Estimate(x.SortedValues, isSorted: true).WithUnitOf(x);
  }
}
