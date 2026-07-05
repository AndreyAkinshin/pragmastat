using Pragmastat.Algorithms;
using Pragmastat.Exceptions;
using Pragmastat.Internal;
using Pragmastat.Metrology;

namespace Pragmastat.Estimators;

public class RatioEstimator : ITwoSampleEstimator
{
  public static readonly RatioEstimator Instance = new();

  /// <summary>
  /// Raw native-array entry point. Returns a unitless ratio estimate.
  /// </summary>
  /// <param name="x">First sample values (must be strictly positive).</param>
  /// <param name="y">Second sample values (must be strictly positive).</param>
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
    return EstimateRaw(x.SortedValues, y.SortedValues, assumeSorted: true).WithUnit(MeasurementUnit.Ratio);
  }

  /// <summary>
  /// Single shared implementation. Both the raw and Sample entry points call this.
  /// </summary>
  internal static double EstimateRaw(IReadOnlyList<double> x, IReadOnlyList<double> y, bool assumeSorted)
  {
    Assertion.Validity(x, Subject.X);
    Assertion.Validity(y, Subject.Y);
    Assertion.PositivityAssumption(x, Subject.X);
    Assertion.PositivityAssumption(y, Subject.Y);
    return RatioImpl.Estimate(x, y, [0.5], assumeSorted).Single();
  }
}
