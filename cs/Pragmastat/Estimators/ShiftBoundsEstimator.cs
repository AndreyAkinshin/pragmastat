using Pragmastat.Algorithms;
using Pragmastat.Exceptions;
using Pragmastat.Functions;
using Pragmastat.Internal;
using Pragmastat.Metrology;

using static Pragmastat.Functions.MinAchievableMisrate;

namespace Pragmastat.Estimators;

public class ShiftBoundsEstimator : ITwoSampleBoundsEstimator
{
  public static readonly ShiftBoundsEstimator Instance = new();

  /// <summary>
  /// Raw native-array entry point. Returns unitless bounds.
  /// </summary>
  /// <param name="x">First sample values.</param>
  /// <param name="y">Second sample values.</param>
  /// <param name="misrate">Misclassification rate.</param>
  /// <param name="assumeSorted">
  /// When true, both <paramref name="x"/> and <paramref name="y"/> are assumed already sorted
  /// ascending and the internal sort is skipped. This changes the computation path; passing true
  /// on unsorted input is undefined behavior and yields a wrong result. The caller is responsible.
  /// </param>
  public Bounds Estimate(double[] x, double[] y, double misrate, bool assumeSorted = false) =>
    EstimateRaw(x, y, misrate, assumeSorted);

  public Bounds Estimate(Sample x, Sample y, Probability misrate)
  {
    Assertion.NonWeighted("x", x);
    Assertion.NonWeighted("y", y);
    Assertion.CompatibleUnits(x, y);
    (x, y) = Assertion.ConvertToFiner(x, y);
    var rb = EstimateRaw(x.SortedValues, y.SortedValues, misrate, assumeSorted: true);
    return new Bounds(rb.Lower, rb.Upper, x.Unit);
  }

  /// <summary>
  /// Single shared implementation. Both the raw and Sample entry points call this.
  /// Returns unitless bounds (the Sample path re-attaches the unit).
  /// </summary>
  internal static Bounds EstimateRaw(IReadOnlyList<double> x, IReadOnlyList<double> y, double misrate, bool assumeSorted)
  {
    Assertion.Validity(x, Subject.X);
    Assertion.Validity(y, Subject.Y);

    int n = x.Count, m = y.Count;
    long total = (long)n * m;

    if (double.IsNaN(misrate) || misrate < 0 || misrate > 1)
      throw AssumptionException.Domain(Subject.Misrate);

    double minMisrate = TwoSample(n, m);
    if (misrate < minMisrate)
      throw AssumptionException.Domain(Subject.Misrate);

    // Order-independent: produce sorted views once (or reuse the already-sorted input).
    IReadOnlyList<double> xs = assumeSorted ? x : x.CopyToArrayAndSort();
    IReadOnlyList<double> ys = assumeSorted ? y : y.CopyToArrayAndSort();

    // Special case: when there's only one pairwise difference, bounds collapse to a single value
    if (total == 1)
    {
      double value = xs[0] - ys[0];
      return new Bounds(value, value, MeasurementUnit.Number);
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

    double[] bounds = ShiftImpl.Estimate(xs, ys, p, assumeSorted: true);
    double lower = Math.Min(bounds[0], bounds[1]);
    double upper = Math.Max(bounds[0], bounds[1]);
    return new Bounds(lower, upper, MeasurementUnit.Number);
  }
}
