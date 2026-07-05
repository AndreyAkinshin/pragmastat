using Pragmastat.Estimators;
using Pragmastat.Internal;
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

  /// <summary>Measures the typical difference between two samples using the Hodges-Lehmann shift estimator.</summary>
  public static Measurement Shift(Sample x, Sample y) => ShiftEstimator.Instance.Estimate(x, y);
  /// <summary>Measures how many times larger x is compared to y via log-transformed shift.</summary>
  public static Measurement Ratio(Sample x, Sample y) => RatioEstimator.Instance.Estimate(x, y);
  /// <summary>Measures effect size as shift normalized by average spread.</summary>
  public static Measurement Disparity(Sample x, Sample y) => DisparityEstimator.Instance.Estimate(x, y);

  // ── Raw native-array overloads (unitless results) ─────────────────────────
  // These accept double[] directly, returning plain numbers / unitless Bounds. The assumeSorted
  // flag lets callers with already-sorted data skip the internal sort. See the estimator XML docs
  // for the assumeSorted contract; passing true on unsorted input is undefined behavior.

  /// <summary>Estimates the central value (unitless) from a native array.</summary>
  public static double Center(double[] x, bool assumeSorted = false) => CenterEstimator.Instance.Estimate(x, assumeSorted);
  /// <summary>Estimates data dispersion (unitless) from a native array.</summary>
  public static double Spread(double[] x, bool assumeSorted = false) => SpreadEstimator.Instance.Estimate(x, assumeSorted);

  /// <summary>Measures the typical difference (unitless) from native arrays.</summary>
  public static double Shift(double[] x, double[] y, bool assumeSorted = false) => ShiftEstimator.Instance.Estimate(x, y, assumeSorted);
  /// <summary>Measures how many times larger x is compared to y (unitless) from native arrays.</summary>
  public static double Ratio(double[] x, double[] y, bool assumeSorted = false) => RatioEstimator.Instance.Estimate(x, y, assumeSorted);
  /// <summary>Measures effect size (unitless) from native arrays.</summary>
  public static double Disparity(double[] x, double[] y, bool assumeSorted = false) => DisparityEstimator.Instance.Estimate(x, y, assumeSorted);
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

  // ── Raw native-array bounds overloads (unitless results) ──────────────────

  /// <summary>Distribution-free bounds (unitless) for the shift estimator from native arrays.</summary>
  public static Bounds ShiftBounds(double[] x, double[] y, bool assumeSorted = false) =>
    ShiftBounds(x, y, DefaultMisrate, assumeSorted);
  /// <summary>Distribution-free bounds (unitless) for the shift estimator from native arrays.</summary>
  public static Bounds ShiftBounds(double[] x, double[] y, double misrate, bool assumeSorted = false) =>
    ShiftBoundsEstimator.Instance.Estimate(x, y, misrate, assumeSorted);

  /// <summary>Distribution-free bounds (unitless) for the ratio estimator from native arrays.</summary>
  public static Bounds RatioBounds(double[] x, double[] y, bool assumeSorted = false) =>
    RatioBounds(x, y, DefaultMisrate, assumeSorted);
  /// <summary>Distribution-free bounds (unitless) for the ratio estimator from native arrays.</summary>
  public static Bounds RatioBounds(double[] x, double[] y, double misrate, bool assumeSorted = false) =>
    RatioBoundsEstimator.Instance.Estimate(x, y, misrate, assumeSorted);

  /// <summary>Distribution-free bounds (unitless) for the disparity estimator from native arrays.</summary>
  public static Bounds DisparityBounds(double[] x, double[] y, bool assumeSorted = false) =>
    DisparityBounds(x, y, DefaultMisrate, assumeSorted);
  /// <summary>Distribution-free bounds (unitless) for the disparity estimator from native arrays.</summary>
  public static Bounds DisparityBounds(double[] x, double[] y, double misrate, bool assumeSorted = false) =>
    DisparityBoundsEstimator.Instance.Estimate(x, y, misrate, assumeSorted);
  /// <summary>Distribution-free bounds (unitless) for the disparity estimator from native arrays, with seed.</summary>
  public static Bounds DisparityBounds(double[] x, double[] y, double misrate, string seed, bool assumeSorted = false) =>
    DisparityBoundsEstimator.Instance.Estimate(x, y, misrate, seed, assumeSorted);

  /// <summary>Distribution-free bounds (unitless) for the center estimator from a native array.</summary>
  public static Bounds CenterBounds(double[] x, bool assumeSorted = false) =>
    CenterBounds(x, DefaultMisrate, assumeSorted);
  /// <summary>Distribution-free bounds (unitless) for the center estimator from a native array.</summary>
  public static Bounds CenterBounds(double[] x, double misrate, bool assumeSorted = false) =>
    CenterBoundsEstimator.Instance.Estimate(x, misrate, assumeSorted);

  /// <summary>Distribution-free bounds (unitless) for the spread estimator from a native array.</summary>
  public static Bounds SpreadBounds(double[] x, bool assumeSorted = false) =>
    SpreadBounds(x, DefaultMisrate, assumeSorted);
  /// <summary>Distribution-free bounds (unitless) for the spread estimator from a native array.</summary>
  public static Bounds SpreadBounds(double[] x, double misrate, bool assumeSorted = false) =>
    SpreadBoundsEstimator.Instance.Estimate(x, misrate, assumeSorted);
  /// <summary>Distribution-free bounds (unitless) for the spread estimator from a native array, with seed.</summary>
  public static Bounds SpreadBounds(double[] x, double misrate, string seed, bool assumeSorted = false) =>
    SpreadBoundsEstimator.Instance.Estimate(x, misrate, seed, assumeSorted);

  /// <summary>One-sample confirmatory analysis: compares Center/Spread against practical thresholds.</summary>
  public static IReadOnlyList<Projection> Compare1(Sample x, IReadOnlyList<Threshold> thresholds)
    => CompareEngine.Compare1(x, thresholds, null);
  /// <summary>One-sample confirmatory analysis with seed for reproducibility.</summary>
  public static IReadOnlyList<Projection> Compare1(Sample x, IReadOnlyList<Threshold> thresholds, string seed)
  {
    Assertion.NotNull("seed", seed);
    return CompareEngine.Compare1(x, thresholds, seed);
  }

  /// <summary>Two-sample confirmatory analysis: compares Shift/Ratio/Disparity against practical thresholds.</summary>
  public static IReadOnlyList<Projection> Compare2(Sample x, Sample y, IReadOnlyList<Threshold> thresholds)
    => CompareEngine.Compare2(x, y, thresholds, null);
  /// <summary>Two-sample confirmatory analysis with seed for reproducibility.</summary>
  public static IReadOnlyList<Projection> Compare2(Sample x, Sample y, IReadOnlyList<Threshold> thresholds, string seed)
  {
    Assertion.NotNull("seed", seed);
    return CompareEngine.Compare2(x, y, thresholds, seed);
  }
}
