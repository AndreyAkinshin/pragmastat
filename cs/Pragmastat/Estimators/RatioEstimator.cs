using Pragmastat.Algorithms;
using Pragmastat.Exceptions;
using Pragmastat.Internal;
using Pragmastat.Metrology;

namespace Pragmastat.Estimators;

public class RatioEstimator : ITwoSampleEstimator
{
  public static readonly RatioEstimator Instance = new();

  public Measurement Estimate(Sample x, Sample y)
  {
    Assertion.NonWeighted("x", x);
    Assertion.NonWeighted("y", y);
    Assertion.CompatibleUnits(x, y);
    (x, y) = Assertion.ConvertToFiner(x, y);
    Assertion.PositivityAssumption(x, Subject.X);
    Assertion.PositivityAssumption(y, Subject.Y);
    return FastRatio
      .Estimate(x.SortedValues, y.SortedValues, [0.5], assumeSorted: true)
      .Single()
      .WithUnit(MeasurementUnit.Ratio);
  }
}
