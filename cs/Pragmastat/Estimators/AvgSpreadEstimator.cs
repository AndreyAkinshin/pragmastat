using Pragmastat.Algorithms;
using Pragmastat.Exceptions;
using Pragmastat.Internal;
using Pragmastat.Metrology;

namespace Pragmastat.Estimators;

internal class AvgSpreadEstimator : ITwoSampleEstimator
{
  public static readonly AvgSpreadEstimator Instance = new();

  public Measurement Estimate(Sample x, Sample y)
  {
    Assertion.MatchedUnit(x, y);
    // Check validity for x (priority 0, subject x)
    Assertion.Validity(x, Subject.X);
    // Check validity for y (priority 0, subject y)
    Assertion.Validity(y, Subject.Y);
    // Check sparity for x (priority 2, subject x)
    Assertion.Sparity(x, Subject.X);
    // Check sparity for y (priority 2, subject y)
    Assertion.Sparity(y, Subject.Y);

    // Calculate spreads (using internal implementation since we already validated)
    var spreadX = FastSpread.Estimate(x.SortedValues, isSorted: true);
    var spreadY = FastSpread.Estimate(y.SortedValues, isSorted: true);
    return ((x.Size * spreadX + y.Size * spreadY) / (x.Size + y.Size)).WithUnitOf(x);
  }
}
