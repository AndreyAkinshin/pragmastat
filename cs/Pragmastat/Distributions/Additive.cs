using Pragmastat.Functions;
using Pragmastat.Internal;
using Pragmastat.Randomization;

namespace Pragmastat.Distributions;

/// <summary>
/// Additive (Normal/Gaussian) distribution with given mean and standard deviation.
/// </summary>
/// <remarks>
/// Uses the Box-Muller transform to generate samples.
/// Historically called 'Normal' or 'Gaussian' distribution.
/// </remarks>
public sealed class Additive : IDistribution, IContinuousDistribution
{
  /// <summary>
  /// Standard normal distribution (mean=0, stdDev=1).
  /// </summary>
  internal static readonly Additive Standard = new(0, 1);

  /// <summary>
  /// Location parameter (center of the distribution).
  /// </summary>
  public double Mean { get; }

  /// <summary>
  /// Scale parameter (standard deviation).
  /// </summary>
  public double StdDev { get; }

  /// <summary>
  /// Create a new additive (normal) distribution.
  /// </summary>
  /// <param name="mean">Location parameter (center of the distribution).</param>
  /// <param name="stdDev">Scale parameter (standard deviation).</param>
  /// <exception cref="ArgumentException">Thrown if stdDev &lt;= 0.</exception>
  public Additive(double mean, double stdDev)
  {
    if (stdDev <= 0)
      throw new ArgumentException("stdDev must be positive");
    Mean = mean;
    StdDev = stdDev;
  }

  /// <inheritdoc />
  public double Sample(Rng rng)
  {
    // Box-Muller transform
    double u1 = rng.Uniform();
    double u2 = rng.Uniform();

    // Avoid log(0) - use smallest positive subnormal for cross-language consistency
    if (u1 == 0)
      u1 = Constants.SmallestPositiveSubnormal;

    double r = Math.Sqrt(-2.0 * Math.Log(u1));
    double theta = 2.0 * Math.PI * u2;

    // Use the first of the two Box-Muller outputs
    double z = r * Math.Cos(theta);

    return Mean + z * StdDev;
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
    double z = (x - Mean) / StdDev;
    return Math.Exp(-z * z / 2) / (StdDev * Constants.Sqrt2Pi);
  }

  double IContinuousDistribution.Cdf(double x)
  {
    return AcmAlgorithm209.Gauss((x - Mean) / StdDev);
  }

  double IContinuousDistribution.Quantile(Probability p)
  {
    return p.Value switch
    {
      0 => double.NegativeInfinity,
      1 => double.PositiveInfinity,
      _ => Mean + StdDev * Constants.Sqrt2 * ErrorFunction.InverseValue(2 * p - 1)
    };
  }

  // Asymptotic spread constant for normal distribution: c_spr ≈ 0.9538725524
  double? IContinuousDistribution.AsymptoticSpread => 0.9538725524 * StdDev;
}
