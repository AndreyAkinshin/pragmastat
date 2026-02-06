using Pragmastat.Algorithms;
using Pragmastat.Exceptions;
using Pragmastat.Functions;
using Pragmastat.Internal;
using Pragmastat.Randomization;

namespace Pragmastat.Estimators;

/// <summary>
/// Bootstrap-based nominal bounds for Center (Hodges-Lehmann pseudomedian).
/// No symmetry requirement, but provides only nominal (not exact) coverage.
/// </summary>
/// <remarks>
/// WARNING: Bootstrap percentile method has known undercoverage for small samples.
/// When requesting 95% confidence (misrate = 0.05), actual coverage is typically 85-92% for n &lt; 30.
/// Users requiring exact coverage should use CenterBounds (if symmetry holds) or MedianBounds.
/// </remarks>
public class CenterBoundsApproxEstimator : IOneSampleBoundsEstimator
{
  public static readonly CenterBoundsApproxEstimator Instance = new();

  private const int DefaultIterations = 10000;
  private const int MaxSubsampleSize = 5000;
  private const string DefaultSeed = "center-bounds-approx";

  public Bounds Estimate(Sample x, Probability misrate) =>
    Estimate(x, misrate, null, DefaultIterations);

  public Bounds Estimate(Sample x, Probability misrate, string? seed) =>
    Estimate(x, misrate, seed, DefaultIterations);

  internal Bounds Estimate(Sample x, Probability misrate, string? seed, int iterations)
  {
    Assertion.Validity(x, Subject.X);
    Assertion.MoreThan(nameof(iterations), iterations, 0);

    if (double.IsNaN(misrate) || misrate < 0 || misrate > 1)
      throw AssumptionException.Domain(Subject.Misrate);

    int n = x.Size;
    if (n < 2)
      throw AssumptionException.Domain(Subject.X);

    double minMisrate = Math.Max(2.0 / iterations, MinAchievableMisrate.OneSample(n));
    if (misrate < minMisrate)
      throw AssumptionException.Domain(Subject.Misrate);

    // Sort for permutation invariance: same values in any order produce same result
    var sorted = x.SortedValues;

    // Use default seed for cross-language determinism when no seed provided
    var rng = new Rng(seed ?? DefaultSeed);

    // m-out-of-n subsampling: cap at MaxSubsampleSize for performance
    int m = Min(n, MaxSubsampleSize);
    var centers = new double[iterations];

    for (int i = 0; i < iterations; i++)
    {
      var resample = rng.Resample(sorted, m);
      centers[i] = ComputeCenter(resample);
    }

    Array.Sort(centers);

    double alpha = misrate.Value / 2;
    int loIdx = Max(0, (int)Floor(alpha * iterations));
    int hiIdx = Min(iterations - 1, (int)Ceiling((1 - alpha) * iterations) - 1);
    if (loIdx > hiIdx) loIdx = hiIdx;

    double bootstrapLo = centers[loIdx];
    double bootstrapHi = centers[hiIdx];

    // Scale bounds to full n using asymptotic sqrt(n) rate
    if (m < n)
    {
      double center = FastCenter.Estimate(sorted, isSorted: true);
      double scaleFactor = Sqrt((double)m / n);
      double lo = center + (bootstrapLo - center) / scaleFactor;
      double hi = center + (bootstrapHi - center) / scaleFactor;
      return new Bounds(lo, hi, x.Unit);
    }

    return new Bounds(bootstrapLo, bootstrapHi, x.Unit);
  }

  /// <summary>
  /// Compute Hodges-Lehmann center for a list of values.
  /// </summary>
  private static double ComputeCenter(IReadOnlyList<double> values) =>
    FastCenter.Estimate(values);
}
