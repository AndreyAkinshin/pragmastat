using Pragmastat.Functions;
using Pragmastat.Internal;
using Pragmastat.Randomization;

namespace Pragmastat.Distributions;

/// <summary>
/// Additive (Normal/Gaussian) distribution with given mean and standard deviation.
/// </summary>
/// <remarks>
/// Draws by inverse transform, through the toolkit's own inverse normal CDF.
/// Historically called 'Normal' or 'Gaussian' distribution.
/// </remarks>
public sealed class Additive : IDistribution
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
  /// <exception cref="ArgumentException">
  /// Thrown if mean is not finite, or if stdDev is not finite and positive.
  /// </exception>
  public Additive(double mean, double stdDev)
  {
    if (!mean.IsFinite())
      throw new ArgumentException("mean must be finite");
    if (!(stdDev.IsFinite() && stdDev > 0))
      throw new ArgumentException("stdDev must be finite and positive");
    Mean = mean;
    StdDev = stdDev;
  }

  /// <inheritdoc />
  public double Sample(Rng rng)
  {
    return QuantileAt(SamplingCumulance.Draw(rng));
  }

  /// <inheritdoc />
  public List<double> Samples(Rng rng, int count)
  {
    var result = new List<double>(count);
    for (int i = 0; i < count; i++)
      result.Add(Sample(rng));
    return result;
  }

  /// <inheritdoc />
  public double DensityAt(double x)
  {
    double z = (x - Mean) / StdDev;
    return ExpFunction.Value(-z * z / 2) / (StdDev * Constants.Sqrt2Pi);
  }

  /// <inheritdoc />
  public double CumulativeAt(double x)
  {
    return AdditiveCumulative.Value((x - Mean) / StdDev);
  }

  /// <inheritdoc />
  /// <remarks>
  /// The inverse goes through <see cref="AdditiveQuantile"/>, which is what that function was
  /// built for. It replaces two Newton steps taken against an error function accurate to 1.5e-07,
  /// which had left this quantile eight orders of magnitude coarser than the cumulative beside it.
  /// The infinities at the ends are the ones AdditiveQuantile already returns.
  /// </remarks>
  public double QuantileAt(double p)
  {
    return Mean + StdDev * AdditiveQuantile.Value(p);
  }
}
