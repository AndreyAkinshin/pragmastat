using Pragmastat.Algorithms;
using Pragmastat.Exceptions;
using Pragmastat.Internal;
using Pragmastat.Metrology;

namespace Pragmastat.Estimators;

public class CenterEstimator : IOneSampleEstimator
{
  public static readonly CenterEstimator Instance = new();

  /// <summary>
  /// Raw native-array entry point. Returns a unitless center estimate.
  /// </summary>
  /// <param name="x">Input values.</param>
  /// <param name="assumeSorted">
  /// When true, <paramref name="x"/> is assumed to be already sorted ascending and the
  /// internal sort is skipped. Passing true on unsorted input is a contract violation
  /// (undefined behavior): the result is unspecified, but termination is guaranteed.
  /// The selection loop is bounded and fails with a deterministic convergence error
  /// (<see cref="InvalidOperationException"/>) on pathological input.
  /// </param>
  public double Estimate(double[] x, bool assumeSorted = false) => EstimateRaw(x, assumeSorted);

  public Measurement Estimate(Sample x)
  {
    Assertion.NonWeighted("x", x);
    // The cached sorted values double as the already-sorted input for the impl.
    return EstimateRaw(x.SortedValues, assumeSorted: true).WithUnitOf(x);
  }

  /// <summary>
  /// Single shared implementation. Both the raw and Sample entry points call this.
  /// </summary>
  internal static double EstimateRaw(IReadOnlyList<double> x, bool assumeSorted)
  {
    Assertion.Validity(x, Subject.X);
    return CenterImpl.Estimate(x, assumeSorted: assumeSorted);
  }
}
