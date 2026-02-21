using Pragmastat.Algorithms;
using Pragmastat.Exceptions;
using Pragmastat.Functions;
using Pragmastat.Internal;

namespace Pragmastat.Estimators;

/// <summary>
/// Distribution-free bounds for AvgSpread via Bonferroni combination of SpreadBounds.
/// Uses equal split alpha = misrate / 2.
/// </summary>
internal class AvgSpreadBoundsEstimator : ITwoSampleBoundsEstimator
{
  internal static readonly AvgSpreadBoundsEstimator Instance = new();

  public Bounds Estimate(Sample x, Sample y, Probability misrate)
  {
    return Estimate(x, y, misrate, null);
  }

  public Bounds Estimate(Sample x, Sample y, Probability misrate, string? seed)
  {
    Assertion.MatchedUnit(x, y);

    if (double.IsNaN(misrate) || misrate < 0 || misrate > 1)
      throw AssumptionException.Domain(Subject.Misrate);

    int n = x.Size;
    int m = y.Size;
    if (n < 2)
      throw AssumptionException.Domain(Subject.X);
    if (m < 2)
      throw AssumptionException.Domain(Subject.Y);

    double alpha = misrate / 2.0;
    double minX = MinAchievableMisrate.OneSample(n / 2);
    double minY = MinAchievableMisrate.OneSample(m / 2);
    if (alpha < minX || alpha < minY)
      throw AssumptionException.Domain(Subject.Misrate);

    if (FastSpread.Estimate(x.SortedValues, isSorted: true) <= 0)
      throw AssumptionException.Sparity(Subject.X);
    if (FastSpread.Estimate(y.SortedValues, isSorted: true) <= 0)
      throw AssumptionException.Sparity(Subject.Y);

    Bounds boundsX = seed == null
      ? SpreadBoundsEstimator.Instance.Estimate(x, alpha)
      : SpreadBoundsEstimator.Instance.Estimate(x, alpha, seed);
    Bounds boundsY = seed == null
      ? SpreadBoundsEstimator.Instance.Estimate(y, alpha)
      : SpreadBoundsEstimator.Instance.Estimate(y, alpha, seed);

    double weightX = (double)n / (n + m);
    double weightY = (double)m / (n + m);

    double lower = weightX * boundsX.Lower + weightY * boundsY.Lower;
    double upper = weightX * boundsX.Upper + weightY * boundsY.Upper;
    return new Bounds(lower, upper, x.Unit);
  }
}
