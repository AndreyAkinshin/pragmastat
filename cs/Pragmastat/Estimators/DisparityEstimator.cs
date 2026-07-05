using Pragmastat.Algorithms;
using Pragmastat.Exceptions;
using Pragmastat.Internal;
using Pragmastat.Metrology;

namespace Pragmastat.Estimators;

public class DisparityEstimator : ITwoSampleEstimator
{
  public static readonly DisparityEstimator Instance = new();

  /// <summary>
  /// Raw native-array entry point. Returns a unitless disparity estimate.
  /// </summary>
  /// <param name="x">First sample values.</param>
  /// <param name="y">Second sample values.</param>
  /// <param name="assumeSorted">
  /// When true, both <paramref name="x"/> and <paramref name="y"/> are assumed already sorted
  /// ascending and the internal sort is skipped. This changes the computation path; passing true
  /// on unsorted input is undefined behavior and yields a wrong result. The caller is responsible.
  /// </param>
  public double Estimate(double[] x, double[] y, bool assumeSorted = false) => EstimateRaw(x, y, assumeSorted);

  public Measurement Estimate(Sample x, Sample y)
  {
    Assertion.NonWeighted("x", x);
    Assertion.NonWeighted("y", y);
    Assertion.CompatibleUnits(x, y);
    (x, y) = Assertion.ConvertToFiner(x, y);
    return EstimateRaw(x.SortedValues, y.SortedValues, assumeSorted: true).WithUnit(MeasurementUnit.Disparity);
  }

  /// <summary>
  /// Single shared implementation. Both the raw and Sample entry points call this.
  /// </summary>
  internal static double EstimateRaw(IReadOnlyList<double> x, IReadOnlyList<double> y, bool assumeSorted)
  {
    Assertion.Validity(x, Subject.X);
    Assertion.Validity(y, Subject.Y);

    int n = x.Count;
    int m = y.Count;

    var spreadX = SpreadImpl.Estimate(x, assumeSorted: assumeSorted);
    if (spreadX <= 0)
      throw AssumptionException.Sparity(Subject.X);
    var spreadY = SpreadImpl.Estimate(y, assumeSorted: assumeSorted);
    if (spreadY <= 0)
      throw AssumptionException.Sparity(Subject.Y);

    var shiftVal = ShiftImpl.Estimate(x, y, [0.5], assumeSorted)[0];
    var avgSpreadVal = (n * spreadX + m * spreadY) / (n + m);

    return shiftVal / avgSpreadVal;
  }
}
