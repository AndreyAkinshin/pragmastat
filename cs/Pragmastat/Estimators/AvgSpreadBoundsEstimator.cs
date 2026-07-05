using Pragmastat.Algorithms;
using Pragmastat.Exceptions;
using Pragmastat.Functions;
using Pragmastat.Internal;
using Pragmastat.Metrology;

namespace Pragmastat.Estimators;

/// <summary>
/// Distribution-free bounds for AvgSpread via Bonferroni combination of SpreadBounds.
/// Uses equal split alpha = misrate / 2.
/// </summary>
internal class AvgSpreadBoundsEstimator : ITwoSampleBoundsEstimator
{
  internal static readonly AvgSpreadBoundsEstimator Instance = new();

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
    // The shuffle runs on the original order; the cached sorted views are sparity-only.
    var rb = EstimateRaw(x.Values, x.SortedValues, y.Values, y.SortedValues, misrate, seed);
    return new Bounds(rb.Lower, rb.Upper, x.Unit);
  }

  /// <summary>
  /// Single shared implementation. <paramref name="x"/>/<paramref name="y"/> are always in
  /// ORIGINAL order (the disjoint-pair shuffle is order-dependent). <paramref name="sortedX"/>/
  /// <paramref name="sortedY"/>, when non-null, are pre-sorted views used only to speed up the
  /// order-independent sparity check. Returns unitless bounds.
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

    double alpha = misrate / 2.0;
    double minX = MinAchievableMisrate.OneSample(n / 2);
    double minY = MinAchievableMisrate.OneSample(m / 2);
    if (alpha < minX || alpha < minY)
      throw AssumptionException.Domain(Subject.Misrate);

    if (SpreadBoundsEstimator.SpreadForSparity(x, sortedX) <= 0)
      throw AssumptionException.Sparity(Subject.X);
    if (SpreadBoundsEstimator.SpreadForSparity(y, sortedY) <= 0)
      throw AssumptionException.Sparity(Subject.Y);

    // Validity/domain/sparity already checked above; reuse the inner shuffle directly to avoid
    // re-running SpreadImpl per sample. The shuffle operates on the ORIGINAL order.
    Bounds boundsX = SpreadBoundsEstimator.EstimateInner(x, alpha, seed);
    Bounds boundsY = SpreadBoundsEstimator.EstimateInner(y, alpha, seed);

    double weightX = (double)n / (n + m);
    double weightY = (double)m / (n + m);

    double lower = weightX * boundsX.Lower + weightY * boundsY.Lower;
    double upper = weightX * boundsX.Upper + weightY * boundsY.Upper;
    return new Bounds(lower, upper, MeasurementUnit.Number);
  }
}
