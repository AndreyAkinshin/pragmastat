using Pragmastat.Algorithms;
using Pragmastat.Exceptions;
using Pragmastat.Functions;
using Pragmastat.Internal;

using static Pragmastat.Functions.MinAchievableMisrate;

namespace Pragmastat.Estimators;

/// <summary>
/// Exact distribution-free bounds for Center (Hodges-Lehmann pseudomedian).
/// Requires weak symmetry assumption: distribution symmetric around unknown center.
/// </summary>
public class CenterBoundsEstimator : IOneSampleBoundsEstimator
{
  public static readonly CenterBoundsEstimator Instance = new();

  public Bounds Estimate(Sample x, Probability misrate)
  {
    if (double.IsNaN(misrate) || misrate < 0 || misrate > 1)
      throw AssumptionException.Domain(Subject.Misrate);

    int n = x.Size;

    if (n < 2)
      throw AssumptionException.Domain(Subject.X);

    double minMisrate = OneSample(n);
    if (misrate < minMisrate)
      throw AssumptionException.Domain(Subject.Misrate);

    int margin = SignedRankMargin.Instance.Calc(n, misrate);
    long totalPairs = (long)n * (n + 1) / 2;

    int halfMargin = margin / 2;
    long maxHalfMargin = (totalPairs - 1) / 2;
    if (halfMargin > maxHalfMargin)
    {
      if (maxHalfMargin > int.MaxValue)
        throw new OverflowException("halfMargin exceeds int range");
      halfMargin = (int)maxHalfMargin;
    }

    long kLeft = halfMargin + 1;
    long kRight = totalPairs - halfMargin;

    var (lo, hi) = FastCenterQuantiles.Bounds(x.SortedValues, kLeft, kRight);
    return new Bounds(lo, hi, x.Unit);
  }
}
