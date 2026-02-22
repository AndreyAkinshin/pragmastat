using Pragmastat.Estimators;
using Pragmastat.Metrology;

namespace Pragmastat;

public static class Toolkit
{
  /// <summary>Default misclassification rate for bounds estimators.</summary>
  public static readonly Probability DefaultMisrate = 1e-3;
  /// <summary>Estimates the central value using the Hodges-Lehmann estimator.</summary>
  public static Measurement Center(Sample x) => CenterEstimator.Instance.Estimate(x);
  /// <summary>Estimates data dispersion using the Shamos estimator.</summary>
  public static Measurement Spread(Sample x) => SpreadEstimator.Instance.Estimate(x);
  /// <summary>Measures relative dispersion as spread divided by absolute center.</summary>
  [Obsolete("Use Spread(x) / Math.Abs(Center(x)) instead.")]
  public static Measurement RelSpread(Sample x) => RelSpreadEstimator.Instance.Estimate(x);

  /// <summary>Measures the typical difference between two samples using the Hodges-Lehmann shift estimator.</summary>
  public static Measurement Shift(Sample x, Sample y) => ShiftEstimator.Instance.Estimate(x, y);
  /// <summary>Measures how many times larger x is compared to y via log-transformed shift.</summary>
  public static Measurement Ratio(Sample x, Sample y) => RatioEstimator.Instance.Estimate(x, y);
  /// <summary>Measures effect size as shift normalized by average spread.</summary>
  public static Measurement Disparity(Sample x, Sample y) => DisparityEstimator.Instance.Estimate(x, y);
  internal static Measurement AvgSpread(Sample x, Sample y) => AvgSpreadEstimator.Instance.Estimate(x, y);

  internal static Bounds AvgSpreadBounds(Sample x, Sample y) => AvgSpreadBounds(x, y, DefaultMisrate);
  internal static Bounds AvgSpreadBounds(Sample x, Sample y, Probability misrate) =>
    AvgSpreadBoundsEstimator.Instance.Estimate(x, y, misrate);
  internal static Bounds AvgSpreadBounds(Sample x, Sample y, Probability misrate, string seed) =>
    AvgSpreadBoundsEstimator.Instance.Estimate(x, y, misrate, seed);

  /// <summary>Provides distribution-free bounds for the shift estimator.</summary>
  public static Bounds ShiftBounds(Sample x, Sample y) => ShiftBounds(x, y, DefaultMisrate);
  /// <summary>Provides distribution-free bounds for the shift estimator.</summary>
  public static Bounds ShiftBounds(Sample x, Sample y, Probability misrate) =>
    ShiftBoundsEstimator.Instance.Estimate(x, y, misrate);

  /// <summary>Provides distribution-free bounds for the ratio estimator.</summary>
  public static Bounds RatioBounds(Sample x, Sample y) => RatioBounds(x, y, DefaultMisrate);
  /// <summary>Provides distribution-free bounds for the ratio estimator.</summary>
  public static Bounds RatioBounds(Sample x, Sample y, Probability misrate) =>
    RatioBoundsEstimator.Instance.Estimate(x, y, misrate);

  /// <summary>Provides distribution-free bounds for the disparity estimator.</summary>
  public static Bounds DisparityBounds(Sample x, Sample y) => DisparityBounds(x, y, DefaultMisrate);
  /// <summary>Provides distribution-free bounds for the disparity estimator.</summary>
  public static Bounds DisparityBounds(Sample x, Sample y, Probability misrate) =>
    DisparityBoundsEstimator.Instance.Estimate(x, y, misrate);
  /// <summary>Provides distribution-free bounds for the disparity estimator.</summary>
  public static Bounds DisparityBounds(Sample x, Sample y, Probability misrate, string seed) =>
    DisparityBoundsEstimator.Instance.Estimate(x, y, misrate, seed);

  /// <summary>Provides distribution-free bounds for the center estimator.</summary>
  public static Bounds CenterBounds(Sample x) => CenterBounds(x, DefaultMisrate);
  /// <summary>Provides distribution-free bounds for the center estimator.</summary>
  public static Bounds CenterBounds(Sample x, Probability misrate) =>
    CenterBoundsEstimator.Instance.Estimate(x, misrate);

  /// <summary>Provides distribution-free bounds for the spread estimator.</summary>
  public static Bounds SpreadBounds(Sample x) => SpreadBounds(x, DefaultMisrate);
  /// <summary>Provides distribution-free bounds for the spread estimator.</summary>
  public static Bounds SpreadBounds(Sample x, Probability misrate) =>
    SpreadBoundsEstimator.Instance.Estimate(x, misrate);
  /// <summary>Provides distribution-free bounds for the spread estimator.</summary>
  public static Bounds SpreadBounds(Sample x, Probability misrate, string seed) =>
    SpreadBoundsEstimator.Instance.Estimate(x, misrate, seed);
}
