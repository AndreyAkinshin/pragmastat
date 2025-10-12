namespace Pragmastat.Algorithms;

using System;
using System.Collections.Generic;
using System.Linq;

public static class FastShift
{
  /// <summary>
  /// Computes quantiles of all pairwise differences { x_i - y_j }.
  /// Time: O((m + n) * log(precision)) per quantile. Space: O(1).
  /// </summary>
  /// <param name="p">Probabilities in [0, 1].</param>
  /// <param name="assumeSorted">If false, collections will be sorted.</param>
  public static double[] Estimate(IReadOnlyList<double> x, IReadOnlyList<double> y, double[] p, bool assumeSorted = false)
  {
    if (x == null || y == null || p == null)
      throw new ArgumentNullException();
    if (x.Count == 0 || y.Count == 0)
      throw new ArgumentException("x and y must be non-empty.");

    foreach (double pk in p)
      if (double.IsNaN(pk) || pk < 0.0 || pk > 1.0)
        throw new ArgumentOutOfRangeException(nameof(p), "Probabilities must be within [0, 1].");

    double[] xs, ys;
    if (assumeSorted)
    {
      xs = x as double[] ?? x.ToArray();
      ys = y as double[] ?? y.ToArray();
    }
    else
    {
      xs = x.OrderBy(v => v).ToArray();
      ys = y.OrderBy(v => v).ToArray();
    }

    int m = xs.Length;
    int n = ys.Length;
    long total = (long)m * n;

    // Type-7 quantile: h = 1 + (n-1)*p, then interpolate between floor(h) and ceil(h)
    var requiredRanks = new SortedSet<long>();
    var interpolationParams = new (long lowerRank, long upperRank, double weight)[p.Length];

    for (int i = 0; i < p.Length; i++)
    {
      double h = 1.0 + (total - 1) * p[i];
      long lowerRank = (long)Math.Floor(h);
      long upperRank = (long)Math.Ceiling(h);
      double weight = h - lowerRank;
      if (lowerRank < 1) lowerRank = 1;
      if (upperRank > total) upperRank = total;
      interpolationParams[i] = (lowerRank, upperRank, weight);
      requiredRanks.Add(lowerRank);
      requiredRanks.Add(upperRank);
    }

    var rankValues = new Dictionary<long, double>();
    foreach (long rank in requiredRanks)
      rankValues[rank] = SelectKthPairwiseDiff(xs, ys, rank);

    var result = new double[p.Length];
    for (int i = 0; i < p.Length; i++)
    {
      var (lowerRank, upperRank, weight) = interpolationParams[i];
      double lower = rankValues[lowerRank];
      double upper = rankValues[upperRank];
      result[i] = weight == 0.0 ? lower : (1.0 - weight) * lower + weight * upper;
    }

    return result;
  }

  // Binary search in [min_diff, max_diff] that snaps to actual discrete values.
  // Avoids materializing all m*n differences.
  private static double SelectKthPairwiseDiff(double[] x, double[] y, long k)
  {
    int m = x.Length;
    int n = y.Length;
    long total = (long)m * n;

    if (k < 1 || k > total)
      throw new ArgumentOutOfRangeException(nameof(k));

    double searchMin = x[0] - y[n - 1];
    double searchMax = x[m - 1] - y[0];

    if (double.IsNaN(searchMin) || double.IsNaN(searchMax))
      throw new InvalidOperationException("NaN in input values.");

    const int maxIterations = 128; // Sufficient for double precision convergence
    double prevMin = double.NegativeInfinity;
    double prevMax = double.PositiveInfinity;

    for (int iter = 0; iter < maxIterations && searchMin != searchMax; iter++)
    {
      double mid = Midpoint(searchMin, searchMax);
      CountAndNeighbors(x, y, mid, out long countLessOrEqual, out double closestBelow, out double closestAbove);

      if (closestBelow == closestAbove)
        return closestBelow;

      // No progress means we're stuck between two discrete values
      if (searchMin == prevMin && searchMax == prevMax)
        return countLessOrEqual >= k ? closestBelow : closestAbove;

      prevMin = searchMin;
      prevMax = searchMax;

      if (countLessOrEqual >= k)
        searchMax = closestBelow;
      else
        searchMin = closestAbove;
    }

    if (searchMin != searchMax)
      throw new InvalidOperationException("Convergence failure (pathological input).");

    return searchMin;
  }

  // Two-pointer algorithm: counts pairs where x[i] - y[j] <= threshold, and tracks
  // the closest actual differences on either side of threshold.
  private static void CountAndNeighbors(
    double[] x, double[] y, double threshold,
    out long countLessOrEqual, out double closestBelow, out double closestAbove)
  {
    int m = x.Length, n = y.Length;
    long count = 0;
    double maxBelow = double.NegativeInfinity;
    double minAbove = double.PositiveInfinity;

    int j = 0;
    for (int i = 0; i < m; i++)
    {
      while (j < n && x[i] - y[j] > threshold)
        j++;

      count += (n - j);

      if (j < n)
      {
        double diff = x[i] - y[j];
        if (diff > maxBelow) maxBelow = diff;
      }

      if (j > 0)
      {
        double diff = x[i] - y[j - 1];
        if (diff < minAbove) minAbove = diff;
      }
    }

    // Fallback to actual min/max if no boundaries found (shouldn't happen in normal operation)
    if (double.IsNegativeInfinity(maxBelow))
      maxBelow = x[0] - y[n - 1];
    if (double.IsPositiveInfinity(minAbove))
      minAbove = x[m - 1] - y[0];

    countLessOrEqual = count;
    closestBelow = maxBelow;
    closestAbove = minAbove;
  }

  private static double Midpoint(double a, double b) => a + (b - a) * 0.5;
}
