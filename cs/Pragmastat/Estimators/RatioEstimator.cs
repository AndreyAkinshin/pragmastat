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
    // Check validity for x (priority 0, subject x)
    Assertion.Validity(x, Subject.X, "Ratio");
    // Check validity for y (priority 0, subject y)
    Assertion.Validity(y, Subject.Y, "Ratio");
    // Check positivity for x (priority 1, subject x)
    Assertion.PositivityAssumption(x, Subject.X, "Ratio");
    // Check positivity for y (priority 1, subject y)
    Assertion.PositivityAssumption(y, Subject.Y, "Ratio");
    return FastRatio
      .Estimate(x.SortedValues, y.SortedValues, [0.5], assumeSorted: true)
      .Single()
      .WithUnit(RatioUnit.Instance);
  }
}
