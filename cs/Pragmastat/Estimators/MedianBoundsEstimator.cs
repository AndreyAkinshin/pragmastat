using Pragmastat.Exceptions;
using Pragmastat.Functions;
using Pragmastat.Internal;

using static Pragmastat.Functions.MinAchievableMisrate;

namespace Pragmastat.Estimators;

/// <summary>
/// Exact distribution-free bounds for population median.
/// No symmetry requirement.
/// Uses order statistics with binomial distribution for exact coverage.
/// </summary>
public class MedianBoundsEstimator : IOneSampleBoundsEstimator
{
  public static readonly MedianBoundsEstimator Instance = new();

  public Bounds Estimate(Sample x, Probability misrate)
  {
    Assertion.Validity(x, Subject.X);

    if (double.IsNaN(misrate) || misrate < 0 || misrate > 1)
      throw AssumptionException.Domain(Subject.Misrate);

    int n = x.Size;

    if (n < 2)
      throw AssumptionException.Domain(Subject.X);

    double minMisrate = OneSample(n);
    if (misrate < minMisrate)
      throw AssumptionException.Domain(Subject.Misrate);

    var sorted = x.SortedValues;
    (int lo, int hi) = ComputeOrderStatisticIndices(n, misrate);

    double lower = sorted[lo];
    double upper = sorted[hi];

    return new Bounds(lower, upper, x.Unit);
  }

  /// <summary>
  /// Find order statistic indices that achieve the specified coverage.
  /// Uses binomial distribution: the interval [X_{(lo+1)}, X_{(hi+1)}] (1-based)
  /// has coverage 1 - 2*P(Bin(n,0.5) ≤ lo).
  /// </summary>
  private static (int lo, int hi) ComputeOrderStatisticIndices(int n, Probability misrate)
  {
    double alpha = misrate.Value / 2;

    // Find the largest k where P(Bin(n,0.5) <= k) <= alpha
    // This gives us the tightest confidence interval with coverage >= 1-misrate
    int lo = 0;
    for (int k = 0; k < (n + 1) / 2; k++)
    {
      double tailProb = BinomialTailProbability(n, k);
      if (tailProb <= alpha)
        lo = k; // k is valid, update to largest valid k
      else
        break; // Once we exceed alpha, all subsequent k will too
    }

    // Symmetric interval: hi = n - 1 - lo
    int hi = n - 1 - lo;

    // Ensure valid bounds
    if (hi < lo)
      hi = lo;
    if (hi >= n)
      hi = n - 1;

    return (lo, hi);
  }

  /// <summary>
  /// Compute P(X ≤ k) for X ~ Binomial(n, 0.5).
  /// Note: 2^n overflows double for n > 1024.
  /// </summary>
  private static double BinomialTailProbability(int n, int k)
  {
    if (k < 0)
      return 0;
    if (k >= n)
      return 1;

    // Normal approximation with continuity correction for large n
    // (2^n overflows double for n > 1024)
    if (n > 1023)
    {
      double mean = n / 2.0;
      double std = Sqrt(n / 4.0);
      double z = (k + 0.5 - mean) / std;
      return AcmAlgorithm209.Gauss(z);
    }

    double total = Pow(2, n);
    double sum = 0;
    double coef = 1.0;

    for (int i = 0; i <= k; i++)
    {
      sum += coef;
      coef = coef * (n - i) / (i + 1);
    }

    return sum / total;
  }
}
