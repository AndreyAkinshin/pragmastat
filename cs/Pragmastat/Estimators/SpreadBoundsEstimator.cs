using Pragmastat.Algorithms;
using Pragmastat.Functions;
using Pragmastat.Exceptions;
using Pragmastat.Internal;
using Pragmastat.Metrology;
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

  /// <summary>
  /// Raw native-array entry point. Returns unitless bounds.
  /// </summary>
  /// <param name="x">Input values, in the order the disjoint-pair shuffle should run on.</param>
  /// <param name="misrate">Misclassification rate.</param>
  /// <param name="assumeSorted">
  /// The disjoint-pair shuffle ALWAYS runs on the order of <paramref name="x"/>, so this flag
  /// never changes the result. When true, the order-independent sparity (Spread &gt; 0) check
  /// skips re-sorting by treating <paramref name="x"/> as already sorted ascending. Passing true
  /// on unsorted input is undefined behavior for the sparity check. The caller is responsible.
  /// </param>
  public Bounds Estimate(double[] x, double misrate, bool assumeSorted = false) =>
    EstimateRaw(x, assumeSorted ? x : null, misrate, null);

  public Bounds Estimate(double[] x, double misrate, string? seed, bool assumeSorted = false) =>
    EstimateRaw(x, assumeSorted ? x : null, misrate, seed);

  public Bounds Estimate(Sample x, Probability misrate)
  {
    return Estimate(x, misrate, null);
  }

  public Bounds Estimate(Sample x, Probability misrate, string? seed)
  {
    Assertion.NonWeighted("x", x);
    // The shuffle runs on the original order; the cached sorted view is sparity-only.
    var rb = EstimateRaw(x.Values, x.SortedValues, misrate, seed);
    return new Bounds(rb.Lower, rb.Upper, x.Unit);
  }

  /// <summary>
  /// Single shared implementation. <paramref name="x"/> is always in ORIGINAL order (the
  /// disjoint-pair shuffle is order-dependent). <paramref name="sortedX"/>, when non-null, is a
  /// pre-sorted view used only to speed up the order-independent sparity check. Returns unitless
  /// bounds (the Sample path re-attaches the unit).
  /// </summary>
  internal static Bounds EstimateRaw(IReadOnlyList<double> x, IReadOnlyList<double>? sortedX, double misrate, string? seed)
  {
    Assertion.Validity(x, Subject.X);

    if (double.IsNaN(misrate) || misrate < 0 || misrate > 1)
      throw AssumptionException.Domain(Subject.Misrate);

    int n = x.Count;
    if (n < 2)
      throw AssumptionException.Sparity(Subject.X);

    int m = n / 2;
    double minMisrate = MinAchievableMisrate.OneSample(m);
    if (misrate < minMisrate)
      throw AssumptionException.Domain(Subject.Misrate);
    if (SpreadForSparity(x, sortedX) <= 0)
      throw AssumptionException.Sparity(Subject.X);

    return EstimateInner(x, misrate, seed);
  }

  /// <summary>
  /// Shared spread value for the sparity (Spread &gt; 0) check, reused by AvgSpreadBounds. The
  /// result is order-independent, so a pre-sorted view (when available) is reused to skip
  /// re-sorting; otherwise the original order is sorted internally by SpreadImpl.
  /// </summary>
  internal static double SpreadForSparity(IReadOnlyList<double> original, IReadOnlyList<double>? sorted) =>
    sorted != null
      ? SpreadImpl.Estimate(sorted, assumeSorted: true)
      : SpreadImpl.Estimate(original, assumeSorted: false);

  /// <summary>
  /// Shuffles the original order into disjoint pairs and returns order-statistic bounds. The
  /// caller is responsible for validity/domain/sparity checks, so AvgSpreadBounds can reuse it
  /// without re-running SpreadImpl. Returns unitless bounds.
  /// </summary>
  internal static Bounds EstimateInner(IReadOnlyList<double> x, double misrate, string? seed)
  {
    int n = x.Count;
    int m = n / 2;

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
      diffs[i] = Math.Abs(x[a] - x[b]);
    }
    Array.Sort(diffs);

    double lower = diffs[kLeft - 1];
    double upper = diffs[kRight - 1];

    return new Bounds(lower, upper, MeasurementUnit.Number);
  }
}
