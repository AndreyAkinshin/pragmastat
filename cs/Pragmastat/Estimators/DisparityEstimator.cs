using Pragmastat.Algorithms;
using Pragmastat.Exceptions;
using Pragmastat.Internal;
using Pragmastat.Metrology;

namespace Pragmastat.Estimators;

public class DisparityEstimator : ITwoSampleEstimator
{
  public static readonly DisparityEstimator Instance = new();

  public Measurement Estimate(Sample x, Sample y)
  {
    Assertion.MatchedUnit(x, y);
    // Check validity for x (priority 0, subject x)
    Assertion.Validity(x, Subject.X);
    // Check validity for y (priority 0, subject y)
    Assertion.Validity(y, Subject.Y);

    var spreadX = FastSpread.Estimate(x.SortedValues, isSorted: true);
    if (spreadX <= 0)
      throw AssumptionException.Sparity(Subject.X);
    var spreadY = FastSpread.Estimate(y.SortedValues, isSorted: true);
    if (spreadY <= 0)
      throw AssumptionException.Sparity(Subject.Y);

    // Calculate shift (we know inputs are valid)
    var shiftVal = FastShift.Estimate(x.SortedValues, y.SortedValues, [0.5], true)[0];
    var avgSpreadVal = (x.Size * spreadX + y.Size * spreadY) / (x.Size + y.Size);

    return (shiftVal / avgSpreadVal).WithUnit(DisparityUnit.Instance);
  }
}
