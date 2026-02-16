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

  public Bounds Estimate(Sample x, Sample y, Probability misrate)
  {
    return Estimate(x, y, misrate, null);
  }

  public Bounds Estimate(Sample x, Sample y, Probability misrate, string? seed)
  {
    Assertion.MatchedUnit(x, y);
    // Check validity (priority 0)
    Assertion.Validity(x, Subject.X);
    Assertion.Validity(y, Subject.Y);

    if (double.IsNaN(misrate) || misrate < 0 || misrate > 1)
      throw AssumptionException.Domain(Subject.Misrate);

    int n = x.Size;
    int m = y.Size;
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

    // Check sparity (priority 2)
    Assertion.Sparity(x, Subject.X);
    Assertion.Sparity(y, Subject.Y);

    var shiftBounds = ShiftBoundsEstimator.Instance.Estimate(x, y, alphaShift);
    var avgBounds = seed == null
      ? AvgSpreadBoundsEstimator.Instance.Estimate(x, y, alphaAvg)
      : AvgSpreadBoundsEstimator.Instance.Estimate(x, y, alphaAvg, seed);

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
      return new Bounds(lower, upper, DisparityUnit.Instance);
    }

    if (ua <= 0.0)
    {
      if (ls == 0.0 && us == 0.0)
        return new Bounds(0.0, 0.0, DisparityUnit.Instance);
      if (ls >= 0.0)
        return new Bounds(0.0, double.PositiveInfinity, DisparityUnit.Instance);
      if (us <= 0.0)
        return new Bounds(double.NegativeInfinity, 0.0, DisparityUnit.Instance);
      return new Bounds(double.NegativeInfinity, double.PositiveInfinity, DisparityUnit.Instance);
    }

    if (ls > 0.0)
      return new Bounds(ls / ua, double.PositiveInfinity, DisparityUnit.Instance);
    if (us < 0.0)
      return new Bounds(double.NegativeInfinity, us / ua, DisparityUnit.Instance);
    if (ls == 0.0 && us == 0.0)
      return new Bounds(0.0, 0.0, DisparityUnit.Instance);
    if (ls == 0.0 && us > 0.0)
      return new Bounds(0.0, double.PositiveInfinity, DisparityUnit.Instance);
    if (ls < 0.0 && us == 0.0)
      return new Bounds(double.NegativeInfinity, 0.0, DisparityUnit.Instance);

    return new Bounds(double.NegativeInfinity, double.PositiveInfinity, DisparityUnit.Instance);
  }
}
