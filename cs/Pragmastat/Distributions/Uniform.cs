using Pragmastat.Randomization;

namespace Pragmastat.Distributions;

/// <summary>
/// Uniform distribution on [min, max).
/// </summary>
/// <remarks>
/// All values within the interval have equal probability.
/// </remarks>
public sealed class Uniform : IDistribution, IContinuousDistribution
{
  /// <summary>
  /// Standard uniform distribution on [0, 1).
  /// </summary>
  internal static readonly Uniform Standard = new(0, 1);

  /// <summary>
  /// Lower bound (inclusive).
  /// </summary>
  public double Min { get; }

  /// <summary>
  /// Upper bound (exclusive).
  /// </summary>
  public double Max { get; }

  /// <summary>
  /// Create a new uniform distribution on [min, max).
  /// </summary>
  /// <param name="min">Lower bound (inclusive).</param>
  /// <param name="max">Upper bound (exclusive).</param>
  /// <exception cref="ArgumentException">Thrown if min >= max.</exception>
  public Uniform(double min, double max)
  {
    if (min >= max)
      throw new ArgumentException("min must be less than max");
    Min = min;
    Max = max;
  }

  /// <inheritdoc />
  public double Sample(Rng rng)
  {
    return Min + rng.Uniform() * (Max - Min);
  }

  /// <inheritdoc />
  public List<double> Samples(Rng rng, int count)
  {
    var result = new List<double>(count);
    for (int i = 0; i < count; i++)
      result.Add(Sample(rng));
    return result;
  }

  // ===========================================================================
  // Internal statistical functions - NOT part of public API.
  // Preparation for future version, not declared in manual yet.
  // ===========================================================================

  double IContinuousDistribution.Pdf(double x)
  {
    return x < Min || x > Max ? 0 : 1 / (Max - Min);
  }

  double IContinuousDistribution.Cdf(double x)
  {
    if (x < Min) return 0;
    if (x > Max) return 1;
    return (x - Min) / (Max - Min);
  }

  double IContinuousDistribution.Quantile(Probability p)
  {
    return Min + p * (Max - Min);
  }

  double? IContinuousDistribution.AsymptoticSpread => (1 - 1.0 / Math.Sqrt(2.0)) * (Max - Min);
}
