using Pragmastat.Estimators;
using Pragmastat.Exceptions;
using Pragmastat.Metrology;

namespace Pragmastat.Internal;

internal static class CompareEngine
{
  private readonly struct MetricSpec
  {
    public Metric Metric { get; }
    public Func<Threshold, Sample, Sample?, Measurement> ValidateAndNormalize { get; }
    public Func<Sample, Sample?, Measurement> Estimate { get; }
    public Func<Sample, Sample?, Probability, Bounds> Bounds { get; }
    public Func<Sample, Sample?, Probability, string, Bounds>? SeededBounds { get; }

    public MetricSpec(
      Metric metric,
      Func<Threshold, Sample, Sample?, Measurement> validateAndNormalize,
      Func<Sample, Sample?, Measurement> estimate,
      Func<Sample, Sample?, Probability, Bounds> bounds,
      Func<Sample, Sample?, Probability, string, Bounds>? seededBounds = null)
    {
      Metric = metric;
      ValidateAndNormalize = validateAndNormalize;
      Estimate = estimate;
      Bounds = bounds;
      SeededBounds = seededBounds;
    }
  }

  private static readonly MetricSpec[] Compare1Specs =
  [
    new MetricSpec(
      Metric.Center,
      ValidateCenter,
      (x, _) => CenterEstimator.Instance.Estimate(x),
      (x, _, alpha) => CenterBoundsEstimator.Instance.Estimate(x, alpha)),
    new MetricSpec(
      Metric.Spread,
      ValidateSpread,
      (x, _) => SpreadEstimator.Instance.Estimate(x),
      (x, _, alpha) => SpreadBoundsEstimator.Instance.Estimate(x, alpha),
      (x, _, alpha, seed) => SpreadBoundsEstimator.Instance.Estimate(x, alpha, seed)),
  ];

  private static readonly MetricSpec[] Compare2Specs =
  [
    new MetricSpec(
      Metric.Shift,
      ValidateShift,
      (x, y) => ShiftEstimator.Instance.Estimate(x, y!),
      (x, y, alpha) => ShiftBoundsEstimator.Instance.Estimate(x, y!, alpha)),
    new MetricSpec(
      Metric.Ratio,
      ValidateRatio,
      (x, y) => RatioEstimator.Instance.Estimate(x, y!),
      (x, y, alpha) => RatioBoundsEstimator.Instance.Estimate(x, y!, alpha)),
    new MetricSpec(
      Metric.Disparity,
      ValidateDisparity,
      (x, y) => DisparityEstimator.Instance.Estimate(x, y!),
      (x, y, alpha) => DisparityBoundsEstimator.Instance.Estimate(x, y!, alpha),
      (x, y, alpha, seed) => DisparityBoundsEstimator.Instance.Estimate(x, y!, alpha, seed)),
  ];

  private static Measurement ValidateCenter(Threshold threshold, Sample x, Sample? _)
  {
    if (!threshold.Value.Unit.IsCompatible(x.Unit))
      throw new UnitMismatchException(threshold.Value.Unit, x.Unit);
    if (!threshold.Value.NominalValue.IsFinite())
      throw new ArgumentOutOfRangeException(nameof(threshold), "threshold.Value must be finite");
    double factor = MeasurementUnit.ConversionFactor(threshold.Value.Unit, x.Unit);
    return new Measurement(threshold.Value.NominalValue * factor, x.Unit);
  }

  private static Measurement ValidateSpread(Threshold threshold, Sample x, Sample? _) =>
    ValidateCenter(threshold, x, null);

  private static Measurement ValidateShift(Threshold threshold, Sample x, Sample? y)
  {
    if (!threshold.Value.Unit.IsCompatible(x.Unit))
      throw new UnitMismatchException(threshold.Value.Unit, x.Unit);
    if (!threshold.Value.NominalValue.IsFinite())
      throw new ArgumentOutOfRangeException(nameof(threshold), "threshold.Value must be finite");
    var finerUnit = MeasurementUnit.Finer(x.Unit, y!.Unit);
    double factor = MeasurementUnit.ConversionFactor(threshold.Value.Unit, finerUnit);
    return new Measurement(threshold.Value.NominalValue * factor, finerUnit);
  }

  private static Measurement ValidateRatio(Threshold threshold, Sample _, Sample? __)
  {
    var unit = threshold.Value.Unit;
    if (unit != MeasurementUnit.Ratio && unit != MeasurementUnit.Number)
      throw new UnitMismatchException(unit, MeasurementUnit.Ratio);
    double value = threshold.Value.NominalValue;
    if (value <= 0 || !value.IsFinite())
      throw new ArgumentOutOfRangeException(nameof(threshold), "Ratio threshold.Value must be finite and positive");
    return new Measurement(value, MeasurementUnit.Ratio);
  }

  private static Measurement ValidateDisparity(Threshold threshold, Sample _, Sample? __)
  {
    var unit = threshold.Value.Unit;
    if (unit != MeasurementUnit.Disparity && unit != MeasurementUnit.Number)
      throw new UnitMismatchException(unit, MeasurementUnit.Disparity);
    double value = threshold.Value.NominalValue;
    if (!value.IsFinite())
      throw new ArgumentOutOfRangeException(nameof(threshold), "Disparity threshold.Value must be finite");
    return new Measurement(value, MeasurementUnit.Disparity);
  }

  public static IReadOnlyList<Projection> Compare1(Sample x, IReadOnlyList<Threshold> thresholds, string? seed)
  {
    Assertion.NonWeighted("x", x);
    Assertion.NotNullOrEmpty("thresholds", thresholds);
    Assertion.ItemNotNull("thresholds", thresholds);

    foreach (var threshold in thresholds)
    {
      if (threshold.Metric is Metric.Shift or Metric.Ratio or Metric.Disparity)
        throw new ArgumentException(
          $"Metric {threshold.Metric} is not supported by Compare1. Use Compare2 instead.",
          nameof(thresholds));
    }

    foreach (var threshold in thresholds)
    {
      if (!threshold.Value.NominalValue.IsFinite())
        throw new ArgumentOutOfRangeException(nameof(thresholds), "threshold.Value must be finite");
    }

    var normalizedValues = new Measurement[thresholds.Count];
    for (int i = 0; i < thresholds.Count; i++)
    {
      var spec = GetSpec(Compare1Specs, thresholds[i].Metric);
      normalizedValues[i] = spec.ValidateAndNormalize(thresholds[i], x, null);
    }

    return Execute(Compare1Specs, x, null, thresholds, normalizedValues, seed);
  }

  public static IReadOnlyList<Projection> Compare2(
    Sample x, Sample y, IReadOnlyList<Threshold> thresholds, string? seed)
  {
    Assertion.NonWeighted("x", x);
    Assertion.NonWeighted("y", y);
    Assertion.CompatibleUnits(x, y);
    Assertion.NotNullOrEmpty("thresholds", thresholds);
    Assertion.ItemNotNull("thresholds", thresholds);

    foreach (var threshold in thresholds)
    {
      if (threshold.Metric is Metric.Center or Metric.Spread)
        throw new ArgumentException(
          $"Metric {threshold.Metric} is not supported by Compare2. Use Compare1 instead.",
          nameof(thresholds));
    }

    foreach (var threshold in thresholds)
    {
      if (!threshold.Value.NominalValue.IsFinite())
        throw new ArgumentOutOfRangeException(nameof(thresholds), "threshold.Value must be finite");
    }

    var normalizedValues = new Measurement[thresholds.Count];
    for (int i = 0; i < thresholds.Count; i++)
    {
      var spec = GetSpec(Compare2Specs, thresholds[i].Metric);
      normalizedValues[i] = spec.ValidateAndNormalize(thresholds[i], x, y);
    }

    return Execute(Compare2Specs, x, y, thresholds, normalizedValues, seed);
  }

  private static MetricSpec GetSpec(MetricSpec[] specs, Metric metric)
  {
    foreach (var spec in specs)
      if (spec.Metric == metric) return spec;
    throw new ArgumentException($"No spec found for metric {metric}");
  }

  private static IReadOnlyList<Projection> Execute(
    MetricSpec[] canonicalSpecs,
    Sample x,
    Sample? y,
    IReadOnlyList<Threshold> thresholds,
    Measurement[] normalizedValues,
    string? seed)
  {
    var results = new Projection[thresholds.Count];

    var byMetric = thresholds
      .Select((t, i) => (t, i, normalizedValues[i]))
      .GroupBy(item => item.t.Metric)
      .ToDictionary(g => g.Key, g => g.ToList());

    foreach (var spec in canonicalSpecs)
    {
      if (!byMetric.TryGetValue(spec.Metric, out var entries)) continue;
      var estimate = spec.Estimate(x, y);
      foreach (var (threshold, inputIndex, normalizedValue) in entries)
      {
        var bounds = (seed != null && spec.SeededBounds != null)
          ? spec.SeededBounds(x, y, threshold.Misrate, seed)
          : spec.Bounds(x, y, threshold.Misrate);
        var verdict = ComputeVerdict(bounds, normalizedValue);
        results[inputIndex] = new Projection(threshold, estimate, bounds, verdict);
      }
    }

    return results;
  }

  private static ComparisonVerdict ComputeVerdict(Bounds bounds, Measurement normalizedThreshold)
  {
    double t = normalizedThreshold.NominalValue;
    if (bounds.Lower > t) return ComparisonVerdict.Greater;
    if (bounds.Upper < t) return ComparisonVerdict.Less;
    return ComparisonVerdict.Inconclusive;
  }
}
