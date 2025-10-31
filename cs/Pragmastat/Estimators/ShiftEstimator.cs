using Pragmastat.Algorithms;
using Pragmastat.Internal;
using Pragmastat.Metrology;

namespace Pragmastat.Estimators;

public class ShiftEstimator : ITwoSampleEstimator
{
  public static readonly ShiftEstimator Instance = new();

  public Measurement Estimate(Sample x, Sample y)
  {
    Assertion.MatchedUnit(x, y);
    return FastShift
      .Estimate(x.SortedValues, y.SortedValues, [0.5], true)
      .Single().WithUnitOf(x);
  }
}
