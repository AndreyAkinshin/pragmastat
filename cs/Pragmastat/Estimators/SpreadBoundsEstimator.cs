using Pragmastat.Algorithms;
using Pragmastat.Functions;
using Pragmastat.Exceptions;
using Pragmastat.Internal;
using Pragmastat.Randomization;

namespace Pragmastat.Estimators;

/// <summary>
/// Distribution-free bounds for Spread using disjoint pairs with sign-test inversion.
/// Randomizes the cutoff between adjacent ranks to match the requested misrate.
/// Requires misrate >= 2^(1 - floor(n/2)).
/// </summary>
public class SpreadBoundsEstimator : IOneSampleBoundsEstimator
{
  public static readonly SpreadBoundsEstimator Instance = new();

  public Bounds Estimate(Sample x, Probability misrate)
  {
    return Estimate(x, misrate, null);
  }

  public Bounds Estimate(Sample x, Probability misrate, string? seed)
  {
    if (double.IsNaN(misrate) || misrate < 0 || misrate > 1)
      throw AssumptionException.Domain(Subject.Misrate);

    int n = x.Size;
    int m = n / 2;
    double minMisrate = MinAchievableMisrate.OneSample(m);
    if (misrate < minMisrate)
      throw AssumptionException.Domain(Subject.Misrate);

    if (x.Size < 2)
      throw AssumptionException.Sparity(Subject.X);
    if (FastSpread.Estimate(x.SortedValues, isSorted: true) <= 0)
      throw AssumptionException.Sparity(Subject.X);

    var rng = seed == null ? new Rng() : new Rng(seed);

    int margin = SignMargin.Instance.CalcRandomized(m, misrate, rng);
    int halfMargin = margin / 2;
    int maxHalfMargin = (m - 1) / 2;
    if (halfMargin > maxHalfMargin)
      halfMargin = maxHalfMargin;

    int kLeft = halfMargin + 1;
    int kRight = m - halfMargin;

    int[] indices = Enumerable.Range(0, n).ToArray();
    var shuffled = rng.Shuffle(indices);
    var diffs = new double[m];
    for (int i = 0; i < m; i++)
    {
      int a = shuffled[2 * i];
      int b = shuffled[2 * i + 1];
      diffs[i] = Math.Abs(x.Values[a] - x.Values[b]);
    }
    Array.Sort(diffs);

    double lower = diffs[kLeft - 1];
    double upper = diffs[kRight - 1];

    return new Bounds(lower, upper, x.Unit);
  }
}
