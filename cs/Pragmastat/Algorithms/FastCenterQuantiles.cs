using System.Diagnostics;

namespace Pragmastat.Algorithms;

/// <summary>
/// Efficiently computes quantiles from all pairwise averages (x[i] + x[j]) / 2 for i ≤ j.
/// Uses binary search with counting function to avoid materializing all N(N+1)/2 pairs.
/// </summary>
internal static class FastCenterQuantiles
{
  private static bool IsSorted(IReadOnlyList<double> list)
  {
    for (int i = 1; i < list.Count; i++)
      if (list[i] < list[i - 1])
        return false;
    return true;
  }
  /// <summary>
  /// Relative epsilon for floating-point comparisons in binary search convergence.
  /// </summary>
  private const double RelativeEpsilon = 1e-14;
  /// <summary>
  /// Compute specified quantile from pairwise averages.
  /// </summary>
  /// <param name="sorted">Sorted input array.</param>
  /// <param name="k">1-based rank of the desired quantile.</param>
  /// <returns>The k-th smallest pairwise average.</returns>
  public static double Quantile(IReadOnlyList<double> sorted, long k)
  {
    Debug.Assert(
      IsSorted(sorted),
      "FastCenterQuantiles.Quantile: input must be sorted");
    int n = sorted.Count;
    if (n == 0)
      throw new ArgumentException("Input cannot be empty", nameof(sorted));

    long totalPairs = (long)n * (n + 1) / 2;
    if (k < 1 || k > totalPairs)
      throw new ArgumentOutOfRangeException(nameof(k), $"k must be in range [1, {totalPairs}]");

    if (n == 1)
      return sorted[0];

    return FindExactQuantile(sorted, k);
  }

  /// <summary>
  /// Compute both lower and upper bounds from pairwise averages.
  /// </summary>
  /// <param name="sorted">Sorted input array.</param>
  /// <param name="marginLo">Rank of lower bound (1-based).</param>
  /// <param name="marginHi">Rank of upper bound (1-based).</param>
  /// <returns>Lower and upper quantiles.</returns>
  public static (double Lo, double Hi) Bounds(IReadOnlyList<double> sorted, long marginLo, long marginHi)
  {
    Debug.Assert(
      IsSorted(sorted),
      "FastCenterQuantiles.Bounds: input must be sorted");
    int n = sorted.Count;
    if (n == 0)
      throw new ArgumentException("Input cannot be empty", nameof(sorted));

    long totalPairs = (long)n * (n + 1) / 2;

    marginLo = Max(1, Min(marginLo, totalPairs));
    marginHi = Max(1, Min(marginHi, totalPairs));

    double lo = FindExactQuantile(sorted, marginLo);
    double hi = FindExactQuantile(sorted, marginHi);

    return (Min(lo, hi), Max(lo, hi));
  }

  /// <summary>
  /// Count pairwise averages ≤ target value.
  /// </summary>
  private static long CountPairsLessOrEqual(IReadOnlyList<double> sorted, double target)
  {
    int n = sorted.Count;
    long count = 0;
    // j is not reset: as i increases, threshold decreases monotonically
    int j = n - 1;

    for (int i = 0; i < n; i++)
    {
      double threshold = 2 * target - sorted[i];

      while (j >= 0 && sorted[j] > threshold)
        j--;

      if (j >= i)
        count += j - i + 1;
    }

    return count;
  }

  /// <summary>
  /// Find exact k-th pairwise average using selection algorithm.
  /// </summary>
  private static double FindExactQuantile(IReadOnlyList<double> sorted, long k)
  {
    int n = sorted.Count;
    long totalPairs = (long)n * (n + 1) / 2;

    if (n == 1)
      return sorted[0];

    if (k == 1)
      return sorted[0];

    if (k == totalPairs)
      return sorted[n - 1];

    double lo = sorted[0];
    double hi = sorted[n - 1];
    const double eps = RelativeEpsilon;

    var candidates = new List<double>(n);

    while (hi - lo > eps * Max(1.0, Max(Abs(lo), Abs(hi))))
    {
      double mid = (lo + hi) / 2;
      long countLessOrEqual = CountPairsLessOrEqual(sorted, mid);

      if (countLessOrEqual >= k)
        hi = mid;
      else
        lo = mid;
    }

    double target = (lo + hi) / 2;

    for (int i = 0; i < n; i++)
    {
      double threshold = 2 * target - sorted[i];

      int left = i;
      int right = n;

      while (left < right)
      {
        int m = (left + right) / 2;
        if (sorted[m] < threshold - eps)
          left = m + 1;
        else
          right = m;
      }

      if (left < n && left >= i && Abs(sorted[left] - threshold) < eps * Max(1.0, Abs(threshold)))
        candidates.Add((sorted[i] + sorted[left]) / 2);

      if (left > i)
      {
        double avgBefore = (sorted[i] + sorted[left - 1]) / 2;
        if (avgBefore <= target + eps)
          candidates.Add(avgBefore);
      }
    }

    if (candidates.Count == 0)
      return target;

    candidates.Sort();

    foreach (double candidate in candidates)
    {
      long countAtCandidate = CountPairsLessOrEqual(sorted, candidate);
      if (countAtCandidate >= k)
        return candidate;
    }

    return target;
  }
}
