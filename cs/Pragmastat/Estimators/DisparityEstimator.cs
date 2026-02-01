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
    Assertion.Validity(x, Subject.X, "Disparity");
    // Check validity for y (priority 0, subject y)
    Assertion.Validity(y, Subject.Y, "Disparity");
    // Check sparity for x (priority 2, subject x)
    Assertion.Sparity(x, Subject.X, "Disparity");
    // Check sparity for y (priority 2, subject y)
    Assertion.Sparity(y, Subject.Y, "Disparity");

    // Calculate shift (we know inputs are valid)
    var shiftVal = FastShift.Estimate(x.SortedValues, y.SortedValues, [0.5], true)[0];
    // Calculate avg_spread (using internal implementation since we already validated)
    var spreadX = FastSpread.Estimate(x.SortedValues, isSorted: true);
    var spreadY = FastSpread.Estimate(y.SortedValues, isSorted: true);
    var avgSpreadVal = (x.Size * spreadX + y.Size * spreadY) / (x.Size + y.Size);

    return (shiftVal / avgSpreadVal).WithUnit(DisparityUnit.Instance);
  }
}
