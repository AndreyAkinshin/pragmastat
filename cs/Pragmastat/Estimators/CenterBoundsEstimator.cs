using Pragmastat.Algorithms;
using Pragmastat.Exceptions;
using Pragmastat.Functions;
using Pragmastat.Internal;
using Pragmastat.Metrology;

using static Pragmastat.Functions.MinAchievableMisrate;

namespace Pragmastat.Estimators;

/// <summary>
/// Exact distribution-free bounds for Center (Hodges-Lehmann pseudomedian).
/// Requires weak symmetry assumption: distribution symmetric around unknown center.
/// </summary>
public class CenterBoundsEstimator : IOneSampleBoundsEstimator
{
  public static readonly CenterBoundsEstimator Instance = new();

  /// <summary>
  /// Raw native-array entry point. Returns unitless bounds.
  /// </summary>
  /// <param name="x">Input values.</param>
  /// <param name="misrate">Misclassification rate.</param>
  /// <param name="assumeSorted">
  /// When true, <paramref name="x"/> is assumed already sorted ascending and the internal sort is
  /// skipped. This changes the computation path; passing true on unsorted input is undefined
  /// behavior and yields a wrong result. The caller is responsible.
  /// </param>
  public Bounds Estimate(double[] x, double misrate, bool assumeSorted = false) =>
    EstimateRaw(x, misrate, assumeSorted);

  public Bounds Estimate(Sample x, Probability misrate)
  {
    Assertion.NonWeighted("x", x);
    var rb = EstimateRaw(x.SortedValues, misrate, assumeSorted: true);
    return new Bounds(rb.Lower, rb.Upper, x.Unit);
  }

  /// <summary>
  /// Single shared implementation. Both the raw and Sample entry points call this.
  /// Returns unitless bounds (the Sample path re-attaches the unit).
  /// </summary>
  internal static Bounds EstimateRaw(IReadOnlyList<double> x, double misrate, bool assumeSorted)
  {
    Assertion.Validity(x, Subject.X);

    if (double.IsNaN(misrate) || misrate < 0 || misrate > 1)
      throw AssumptionException.Domain(Subject.Misrate);

    int n = x.Count;

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

    IReadOnlyList<double> sorted = assumeSorted ? x : x.CopyToArrayAndSort();
    var (lo, hi) = CenterQuantilesImpl.Bounds(sorted, kLeft, kRight);
    return new Bounds(lo, hi, MeasurementUnit.Number);
  }
}
