using Pragmastat.Functions;
using Pragmastat.Internal;
using Pragmastat.Randomization;

namespace Pragmastat.Distributions;

/// <summary>
/// Multiplicative (Log-Normal) distribution.
/// </summary>
/// <remarks>
/// The logarithm of samples follows an Additive (Normal) distribution.
/// Historically called 'Log-Normal' or 'Galton' distribution.
/// </remarks>
public sealed class Multiplic : IDistribution, IContinuousDistribution
{
  /// <summary>
  /// Standard multiplicative distribution (logMean=0, logStdDev=1).
  /// </summary>
  internal static readonly Multiplic Standard = new(0, 1);

  /// <summary>
  /// Mean of the log values (location parameter).
  /// </summary>
  public double LogMean { get; }

  /// <summary>
  /// Standard deviation of the log values (scale parameter).
  /// </summary>
  public double LogStdDev { get; }

  private readonly Additive _additive;

  /// <summary>
  /// Create a new multiplicative (log-normal) distribution.
  /// </summary>
  /// <param name="logMean">Mean of log values (location parameter).</param>
  /// <param name="logStdDev">Standard deviation of log values (scale parameter).</param>
  /// <exception cref="ArgumentException">Thrown if logStdDev &lt;= 0.</exception>
  public Multiplic(double logMean, double logStdDev)
  {
    if (logStdDev <= 0)
      throw new ArgumentException("logStdDev must be positive");
    LogMean = logMean;
    LogStdDev = logStdDev;
    _additive = new Additive(logMean, logStdDev);
  }

  /// <inheritdoc />
  public double Sample(Rng rng)
  {
    return Math.Exp(_additive.Sample(rng));
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
    if (x < 1e-9) return 0;
    double logX = Math.Log(x);
    double z = (logX - LogMean) / LogStdDev;
    return Math.Exp(-z * z / 2) / (x * LogStdDev * Constants.Sqrt2Pi);
  }

  double IContinuousDistribution.Cdf(double x)
  {
    if (x < 1e-9) return 0;
    return 0.5 * (1 + ErrorFunction.Value((Math.Log(x) - LogMean) / (Constants.Sqrt2 * LogStdDev)));
  }

  double IContinuousDistribution.Quantile(Probability p)
  {
    return p.Value switch
    {
      0 => 0,
      1 => double.PositiveInfinity,
      _ => Math.Exp(LogMean + Constants.Sqrt2 * LogStdDev * ErrorFunction.InverseValue(2 * p - 1))
    };
  }

  // Log-normal distribution has finite spread but complex formula, mark as unknown for now
  double? IContinuousDistribution.AsymptoticSpread => null;
}
