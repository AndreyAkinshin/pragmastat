using Pragmastat.Exceptions;
using Pragmastat.Internal;
using Pragmastat.Metrology;

using static Pragmastat.Functions.MinAchievableMisrate;

namespace Pragmastat.Estimators;

/// <summary>
/// Distribution-free bounds for disparity using Bonferroni combination.
/// </summary>
public class DisparityBoundsEstimator : ITwoSampleBoundsEstimator
{
  public static readonly DisparityBoundsEstimator Instance = new();

  /// <summary>
  /// Raw native-array entry point. Returns unitless bounds.
  /// </summary>
  /// <param name="x">First sample values, in the order the disjoint-pair shuffle should run on.</param>
  /// <param name="y">Second sample values, in the order the disjoint-pair shuffle should run on.</param>
  /// <param name="misrate">Misclassification rate.</param>
  /// <param name="assumeSorted">
  /// When true, the inputs are treated as already sorted ascending and the embedded
  /// order-independent sparity and shift-bounds sub-computations skip re-sorting. This flag is
  /// inert (never changes the result) ONLY when the inputs are genuinely sorted: the embedded
  /// shift-bounds consumes the inputs as a sorted view, so on UNSORTED input passing true is
  /// undefined behavior and CAN change the result. The caller is responsible.
  /// </param>
  public Bounds Estimate(double[] x, double[] y, double misrate, bool assumeSorted = false) =>
    EstimateRaw(x, assumeSorted ? x : null, y, assumeSorted ? y : null, misrate, null);

  public Bounds Estimate(double[] x, double[] y, double misrate, string? seed, bool assumeSorted = false) =>
    EstimateRaw(x, assumeSorted ? x : null, y, assumeSorted ? y : null, misrate, seed);

  public Bounds Estimate(Sample x, Sample y, Probability misrate)
  {
    return Estimate(x, y, misrate, null);
  }

  public Bounds Estimate(Sample x, Sample y, Probability misrate, string? seed)
  {
    Assertion.NonWeighted("x", x);
    Assertion.NonWeighted("y", y);
    Assertion.CompatibleUnits(x, y);
    (x, y) = Assertion.ConvertToFiner(x, y);
    // The shuffle runs on the original order; the cached sorted views are sparity/shift-only.
    var rb = EstimateRaw(x.Values, x.SortedValues, y.Values, y.SortedValues, misrate, seed);
    return new Bounds(rb.Lower, rb.Upper, MeasurementUnit.Disparity);
  }

  /// <summary>
  /// Single shared implementation. <paramref name="x"/>/<paramref name="y"/> are always in
  /// ORIGINAL order; <paramref name="sortedX"/>/<paramref name="sortedY"/>, when non-null, are
  /// pre-sorted views used only for the order-independent sparity and shift-bounds sub-computations.
  /// Returns unitless bounds (the Sample path re-attaches the disparity unit).
  /// </summary>
  internal static Bounds EstimateRaw(
    IReadOnlyList<double> x, IReadOnlyList<double>? sortedX,
    IReadOnlyList<double> y, IReadOnlyList<double>? sortedY,
    double misrate, string? seed)
  {
    Assertion.Validity(x, Subject.X);
    Assertion.Validity(y, Subject.Y);

    if (double.IsNaN(misrate) || misrate < 0 || misrate > 1)
      throw AssumptionException.Domain(Subject.Misrate);

    int n = x.Count;
    int m = y.Count;
    if (n < 2)
      throw AssumptionException.Domain(Subject.X);
    if (m < 2)
      throw AssumptionException.Domain(Subject.Y);

    double minShift = TwoSample(n, m);
    double minX = OneSample(n / 2);
    double minY = OneSample(m / 2);
    double minAvg = 2.0 * Math.Max(minX, minY);

    if (misrate < minShift + minAvg)
      throw AssumptionException.Domain(Subject.Misrate);

    double extra = misrate - (minShift + minAvg);
    double alphaShift = minShift + extra / 2.0;
    double alphaAvg = minAvg + extra / 2.0;

    // The Spread > 0 sparity check is performed by AvgSpreadBoundsEstimator below (identical
    // predicate, same Subject.X/Y order). ShiftBounds runs first but cannot throw for these inputs
    // (alphaShift >= the two-sample minimum), so it cannot mask that sparity error.
    // ShiftBounds is order-independent given sorted input; use the sorted views when present.
    Bounds shiftBounds = sortedX != null && sortedY != null
      ? ShiftBoundsEstimator.EstimateRaw(sortedX, sortedY, alphaShift, assumeSorted: true)
      : ShiftBoundsEstimator.EstimateRaw(x, y, alphaShift, assumeSorted: false);
    Bounds avgBounds = AvgSpreadBoundsEstimator.EstimateRaw(x, sortedX, y, sortedY, alphaAvg, seed);

    double la = avgBounds.Lower;
    double ua = avgBounds.Upper;
    double ls = shiftBounds.Lower;
    double us = shiftBounds.Upper;

    if (la > 0.0)
    {
      double r1 = ls / la;
      double r2 = ls / ua;
      double r3 = us / la;
      double r4 = us / ua;
      double lower = Math.Min(Math.Min(r1, r2), Math.Min(r3, r4));
      double upper = Math.Max(Math.Max(r1, r2), Math.Max(r3, r4));
      return new Bounds(lower, upper, MeasurementUnit.Number);
    }

    if (ua <= 0.0)
    {
      if (ls == 0.0 && us == 0.0)
        return new Bounds(0.0, 0.0, MeasurementUnit.Number);
      if (ls >= 0.0)
        return new Bounds(0.0, double.PositiveInfinity, MeasurementUnit.Number);
      if (us <= 0.0)
        return new Bounds(double.NegativeInfinity, 0.0, MeasurementUnit.Number);
      return new Bounds(double.NegativeInfinity, double.PositiveInfinity, MeasurementUnit.Number);
    }

    if (ls > 0.0)
      return new Bounds(ls / ua, double.PositiveInfinity, MeasurementUnit.Number);
    if (us < 0.0)
      return new Bounds(double.NegativeInfinity, us / ua, MeasurementUnit.Number);
    if (ls == 0.0 && us == 0.0)
      return new Bounds(0.0, 0.0, MeasurementUnit.Number);
    if (ls == 0.0 && us > 0.0)
      return new Bounds(0.0, double.PositiveInfinity, MeasurementUnit.Number);
    if (ls < 0.0 && us == 0.0)
      return new Bounds(double.NegativeInfinity, 0.0, MeasurementUnit.Number);

    return new Bounds(double.NegativeInfinity, double.PositiveInfinity, MeasurementUnit.Number);
  }
}
