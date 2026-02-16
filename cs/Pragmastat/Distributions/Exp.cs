using Pragmastat.Internal;
using Pragmastat.Randomization;

namespace Pragmastat.Distributions;

/// <summary>
/// Exponential distribution with given rate parameter.
/// </summary>
/// <remarks>
/// The mean of this distribution is 1/rate.
/// Naturally arises from memoryless processes.
/// </remarks>
public sealed class Exp : IDistribution, IContinuousDistribution
{
  /// <summary>
  /// Standard exponential distribution (rate=1).
  /// </summary>
  internal static readonly Exp Standard = new(1);

  /// <summary>
  /// Rate parameter (lambda).
  /// </summary>
  public double Rate { get; }

  /// <summary>
  /// Create a new exponential distribution with given rate.
  /// </summary>
  /// <param name="rate">Rate parameter (lambda > 0).</param>
  /// <exception cref="ArgumentException">Thrown if rate &lt;= 0.</exception>
  public Exp(double rate)
  {
    if (rate <= 0)
      throw new ArgumentException("rate must be positive");
    Rate = rate;
  }

  /// <inheritdoc />
  public double Sample(Rng rng)
  {
    // Inverse CDF method: -ln(1 - U) / rate
    double u = rng.UniformDouble();
    // Avoid log(0) - use machine epsilon for cross-language consistency
    if (u == 1.0)
      u = 1.0 - Constants.MachineEpsilon;
    return -Math.Log(1.0 - u) / Rate;
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
    if (x < 0) return 0;
    return Rate * Math.Exp(-Rate * x);
  }

  double IContinuousDistribution.Cdf(double x)
  {
    if (x < 0) return 0;
    return 1 - Math.Exp(-Rate * x);
  }

  double IContinuousDistribution.Quantile(Probability p)
  {
    return -Math.Log(1 - p) / Rate;
  }

  // Exponential distribution has infinite spread (heavy tail)
  double? IContinuousDistribution.AsymptoticSpread => null;
}
