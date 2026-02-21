using Pragmastat.Algorithms;
using Pragmastat.Exceptions;
using Pragmastat.Functions;
using Pragmastat.Internal;

using static Pragmastat.Functions.MinAchievableMisrate;

namespace Pragmastat.Estimators;

public class ShiftBoundsEstimator : ITwoSampleBoundsEstimator
{
  public static readonly ShiftBoundsEstimator Instance = new();

  public Bounds Estimate(Sample x, Sample y, Probability misrate)
  {
    Assertion.MatchedUnit(x, y);

    int n = x.Size, m = y.Size;
    long total = (long)n * m;

    if (double.IsNaN(misrate) || misrate < 0 || misrate > 1)
      throw AssumptionException.Domain(Subject.Misrate);

    double minMisrate = TwoSample(n, m);
    if (misrate < minMisrate)
      throw AssumptionException.Domain(Subject.Misrate);

    // Special case: when there's only one pairwise difference, bounds collapse to a single value
    if (total == 1)
    {
      double value = x.Values[0] - y.Values[0];
      return new Bounds(value, value, x.Unit);
    }

    int margin = PairwiseMargin.Instance.Calc(n, m, misrate);
    long halfMargin = margin / 2L;
    long maxHalfMargin = (total - 1) / 2;
    if (halfMargin > maxHalfMargin)
      halfMargin = maxHalfMargin;

    long kLeft = halfMargin;
    long kRight = total - 1 - halfMargin;

    double denominator = total - 1;
    if (denominator <= 0)
      denominator = 1;
    double[] p =
    [
      kLeft / denominator,
      kRight / denominator
    ];

    double[] bounds = FastShift.Estimate(x.SortedValues, y.SortedValues, p, assumeSorted: true);
    double lower = Math.Min(bounds[0], bounds[1]);
    double upper = Math.Max(bounds[0], bounds[1]);
    return new Bounds(lower, upper, x.Unit);
  }
}
