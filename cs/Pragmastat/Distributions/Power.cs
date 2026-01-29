using Pragmastat.Internal;
using Pragmastat.Randomization;

namespace Pragmastat.Distributions;

/// <summary>
/// Power (Pareto) distribution with minimum value and shape parameter.
/// </summary>
/// <remarks>
/// Follows a power-law distribution where large values are rare but possible.
/// Historically called 'Pareto' distribution.
/// </remarks>
public sealed class Power : IDistribution, IContinuousDistribution
{
  /// <summary>
  /// Standard power distribution (min=1, shape=1).
  /// </summary>
  internal static readonly Power Standard = new(1, 1);

  /// <summary>
  /// Minimum value (scale parameter, x_m).
  /// </summary>
  public double Min { get; }

  /// <summary>
  /// Shape parameter (alpha, controls tail heaviness).
  /// </summary>
  public double Shape { get; }

  /// <summary>
  /// Create a new power (Pareto) distribution.
  /// </summary>
  /// <param name="min">Minimum value (lower bound, > 0).</param>
  /// <param name="shape">Shape parameter (alpha > 0, controls tail heaviness).</param>
  /// <exception cref="ArgumentException">Thrown if min &lt;= 0 or shape &lt;= 0.</exception>
  public Power(double min, double shape)
  {
    if (min <= 0)
      throw new ArgumentException("min must be positive");
    if (shape <= 0)
      throw new ArgumentException("shape must be positive");
    Min = min;
    Shape = shape;
  }

  /// <inheritdoc />
  public double Sample(Rng rng)
  {
    // Inverse CDF method: min / (1 - U)^(1/shape)
    double u = rng.Uniform();
    // Avoid division by zero - use machine epsilon for cross-language consistency
    if (u == 1.0)
      u = 1.0 - Constants.MachineEpsilon;
    return Min / Math.Pow(1.0 - u, 1.0 / Shape);
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
    if (x <= Min) return 0;
    return Shape * Math.Pow(Min, Shape) / Math.Pow(x, Shape + 1);
  }

  double IContinuousDistribution.Cdf(double x)
  {
    if (x <= Min) return 0;
    return 1 - Math.Pow(Min / x, Shape);
  }

  double IContinuousDistribution.Quantile(Probability p)
  {
    return Min * Math.Pow(1 - p, -1 / Shape);
  }

  // Power distribution has infinite spread for shape <= 2
  double? IContinuousDistribution.AsymptoticSpread => null;
}
