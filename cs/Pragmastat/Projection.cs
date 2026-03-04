using Pragmastat.Metrology;

namespace Pragmastat;

public sealed class Projection(
  Threshold threshold,
  Measurement estimate,
  Bounds bounds,
  ComparisonVerdict verdict)
{
  public Threshold Threshold { get; } = threshold;
  public Measurement Estimate { get; } = estimate;
  public Bounds Bounds { get; } = bounds;
  public ComparisonVerdict Verdict { get; } = verdict;
}
