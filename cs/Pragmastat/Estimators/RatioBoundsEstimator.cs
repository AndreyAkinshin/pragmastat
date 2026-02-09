using Pragmastat.Exceptions;
using Pragmastat.Internal;
using Pragmastat.Metrology;

using static Pragmastat.Functions.MinAchievableMisrate;

namespace Pragmastat.Estimators;

/// <summary>
/// Provides bounds on the Ratio estimator via log-transformation and ShiftBounds delegation.
/// RatioBounds(x, y, misrate) = exp(ShiftBounds(log(x), log(y), misrate))
/// </summary>
public class RatioBoundsEstimator : ITwoSampleBoundsEstimator
{
  public static readonly RatioBoundsEstimator Instance = new();

  public Bounds Estimate(Sample x, Sample y, Probability misrate)
  {
    Assertion.MatchedUnit(x, y);
    Assertion.Validity(x, Subject.X);
    Assertion.Validity(y, Subject.Y);

    if (double.IsNaN(misrate) || misrate < 0 || misrate > 1)
      throw AssumptionException.Domain(Subject.Misrate);

    double minMisrate = TwoSample(x.Size, y.Size);
    if (misrate < minMisrate)
      throw AssumptionException.Domain(Subject.Misrate);

    Assertion.PositivityAssumption(x, Subject.X);
    Assertion.PositivityAssumption(y, Subject.Y);

    // Log-transform samples
    var logX = x.Log();
    var logY = y.Log();

    // Delegate to ShiftBounds in log-space
    var logBounds = ShiftBoundsEstimator.Instance.Estimate(logX, logY, misrate);

    // Exp-transform back to ratio-space
    return new Bounds(
      Math.Exp(logBounds.Lower),
      Math.Exp(logBounds.Upper),
      RatioUnit.Instance
    );
  }
}
