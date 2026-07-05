using Pragmastat.Exceptions;
using Pragmastat.Internal;
using Pragmastat.Metrology;

using static Pragmastat.Functions.MinAchievableMisrate;

namespace Pragmastat.Estimators;

/// <summary>
/// Provides bounds on the Ratio estimator via log-transformation and ShiftBounds delegation.
/// RatioBounds(x, y, misrate) = exp(ShiftBounds(log(x), log(y), misrate))
/// </summary>
public class RatioBoundsEstimator : ITwoSampleBoundsEstimator
{
  public static readonly RatioBoundsEstimator Instance = new();

  /// <summary>
  /// Raw native-array entry point. Returns unitless bounds.
  /// </summary>
  /// <param name="x">First sample values (must be strictly positive).</param>
  /// <param name="y">Second sample values (must be strictly positive).</param>
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
    // ratio_bounds is order-independent and log is monotonic, so the cached sorted
    // values stay sorted after the log-transform — reuse them to skip a re-sort.
    var rb = EstimateRaw(x.SortedValues, y.SortedValues, misrate, assumeSorted: true);
    return new Bounds(rb.Lower, rb.Upper, MeasurementUnit.Ratio);
  }

  /// <summary>
  /// Single shared implementation. Both the raw and Sample entry points call this.
  /// Returns unitless bounds (the Sample path re-attaches the ratio unit).
  /// </summary>
  internal static Bounds EstimateRaw(IReadOnlyList<double> x, IReadOnlyList<double> y, double misrate, bool assumeSorted)
  {
    Assertion.Validity(x, Subject.X);
    Assertion.Validity(y, Subject.Y);

    if (double.IsNaN(misrate) || misrate < 0 || misrate > 1)
      throw AssumptionException.Domain(Subject.Misrate);

    double minMisrate = TwoSample(x.Count, y.Count);
    if (misrate < minMisrate)
      throw AssumptionException.Domain(Subject.Misrate);

    // Log-transform both samples (includes the positivity check, reported per subject).
    double[] logX = MathExtensions.Log(x, Subject.X);
    double[] logY = MathExtensions.Log(y, Subject.Y);

    // log is monotonic: sorted positive input -> sorted log output, so assumeSorted carries over.
    var logBounds = ShiftBoundsEstimator.EstimateRaw(logX, logY, misrate, assumeSorted);

    return new Bounds(Math.Exp(logBounds.Lower), Math.Exp(logBounds.Upper), MeasurementUnit.Number);
  }
}
