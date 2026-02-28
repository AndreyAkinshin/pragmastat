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
    Assertion.NonWeighted("x", x);
    Assertion.NonWeighted("y", y);
    Assertion.CompatibleUnits(x, y);
    (x, y) = Assertion.ConvertToFiner(x, y);

    var spreadX = FastSpread.Estimate(x.SortedValues, isSorted: true);
    if (spreadX <= 0)
      throw AssumptionException.Sparity(Subject.X);
    var spreadY = FastSpread.Estimate(y.SortedValues, isSorted: true);
    if (spreadY <= 0)
      throw AssumptionException.Sparity(Subject.Y);

    return ((x.Size * spreadX + y.Size * spreadY) / (x.Size + y.Size)).WithUnitOf(x);
  }
}
