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
    var spreadVal = FastSpread.Estimate(x.SortedValues, isSorted: true);
    if (spreadVal <= 0)
      throw AssumptionException.Sparity(Subject.X);
    return spreadVal.WithUnitOf(x);
  }
}
