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
    Assertion.MatchedUnit(x, y);
    // Check positivity for x (priority 1, subject x)
    Assertion.PositivityAssumption(x, Subject.X);
    // Check positivity for y (priority 1, subject y)
    Assertion.PositivityAssumption(y, Subject.Y);
    return FastRatio
      .Estimate(x.SortedValues, y.SortedValues, [0.5], assumeSorted: true)
      .Single()
      .WithUnit(RatioUnit.Instance);
  }
}
