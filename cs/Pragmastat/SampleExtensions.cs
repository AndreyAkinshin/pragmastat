using Pragmastat.Metrology;

namespace Pragmastat;

public static class SampleExtensions
{
  // ── One-sample estimators ────────────────────────────────────────────────

  /// <summary>Estimates the central value using the Hodges-Lehmann estimator.</summary>
  public static Measurement Center(this Sample x) => Toolkit.Center(x);
  /// <summary>Estimates data dispersion using the Shamos estimator.</summary>
  public static Measurement Spread(this Sample x) => Toolkit.Spread(x);

  // ── One-sample bounds ────────────────────────────────────────────────────

  /// <summary>Provides distribution-free bounds for the center estimator.</summary>
  public static Bounds CenterBounds(this Sample x) => Toolkit.CenterBounds(x);
  /// <summary>Provides distribution-free bounds for the center estimator.</summary>
  public static Bounds CenterBounds(this Sample x, Probability misrate) => Toolkit.CenterBounds(x, misrate);

  /// <summary>Provides distribution-free bounds for the spread estimator.</summary>
  public static Bounds SpreadBounds(this Sample x) => Toolkit.SpreadBounds(x);
  /// <summary>Provides distribution-free bounds for the spread estimator.</summary>
  public static Bounds SpreadBounds(this Sample x, Probability misrate) => Toolkit.SpreadBounds(x, misrate);
  /// <summary>Provides distribution-free bounds for the spread estimator.</summary>
  public static Bounds SpreadBounds(this Sample x, Probability misrate, string seed) => Toolkit.SpreadBounds(x, misrate, seed);

  // ── Two-sample estimators ────────────────────────────────────────────────

  /// <summary>Measures the typical difference between two samples using the Hodges-Lehmann shift estimator.</summary>
  public static Measurement Shift(this Sample x, Sample y) => Toolkit.Shift(x, y);
  /// <summary>Measures how many times larger x is compared to y via log-transformed shift.</summary>
  public static Measurement Ratio(this Sample x, Sample y) => Toolkit.Ratio(x, y);
  /// <summary>Measures effect size as shift normalized by average spread.</summary>
  public static Measurement Disparity(this Sample x, Sample y) => Toolkit.Disparity(x, y);

  // ── Two-sample bounds ────────────────────────────────────────────────────

  /// <summary>Provides distribution-free bounds for the shift estimator.</summary>
  public static Bounds ShiftBounds(this Sample x, Sample y) => Toolkit.ShiftBounds(x, y);
  /// <summary>Provides distribution-free bounds for the shift estimator.</summary>
  public static Bounds ShiftBounds(this Sample x, Sample y, Probability misrate) => Toolkit.ShiftBounds(x, y, misrate);

  /// <summary>Provides distribution-free bounds for the ratio estimator.</summary>
  public static Bounds RatioBounds(this Sample x, Sample y) => Toolkit.RatioBounds(x, y);
  /// <summary>Provides distribution-free bounds for the ratio estimator.</summary>
  public static Bounds RatioBounds(this Sample x, Sample y, Probability misrate) => Toolkit.RatioBounds(x, y, misrate);

  /// <summary>Provides distribution-free bounds for the disparity estimator.</summary>
  public static Bounds DisparityBounds(this Sample x, Sample y) => Toolkit.DisparityBounds(x, y);
  /// <summary>Provides distribution-free bounds for the disparity estimator.</summary>
  public static Bounds DisparityBounds(this Sample x, Sample y, Probability misrate) => Toolkit.DisparityBounds(x, y, misrate);
  /// <summary>Provides distribution-free bounds for the disparity estimator.</summary>
  public static Bounds DisparityBounds(this Sample x, Sample y, Probability misrate, string seed) => Toolkit.DisparityBounds(x, y, misrate, seed);

  // ── Compare1 ──────────────────────────────────────────────────────────────

  /// <summary>One-sample confirmatory analysis.</summary>
  public static IReadOnlyList<Projection> Compare1(this Sample x, IReadOnlyList<Threshold> thresholds)
    => Toolkit.Compare1(x, thresholds);
  /// <summary>One-sample confirmatory analysis with seed for reproducibility.</summary>
  public static IReadOnlyList<Projection> Compare1(this Sample x, IReadOnlyList<Threshold> thresholds, string seed)
    => Toolkit.Compare1(x, thresholds, seed);

  // ── Compare2 ──────────────────────────────────────────────────────────────

  /// <summary>Two-sample confirmatory analysis.</summary>
  public static IReadOnlyList<Projection> Compare2(this Sample x, Sample y, IReadOnlyList<Threshold> thresholds)
    => Toolkit.Compare2(x, y, thresholds);
  /// <summary>Two-sample confirmatory analysis with seed for reproducibility.</summary>
  public static IReadOnlyList<Projection> Compare2(this Sample x, Sample y, IReadOnlyList<Threshold> thresholds, string seed)
    => Toolkit.Compare2(x, y, thresholds, seed);
}
