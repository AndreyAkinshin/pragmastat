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
public sealed class Multiplic : IDistribution
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
  /// <exception cref="ArgumentException">
  /// Thrown if logMean is not finite, or if logStdDev is not finite and positive.
  /// </exception>
  public Multiplic(double logMean, double logStdDev)
  {
    if (!logMean.IsFinite())
      throw new ArgumentException("logMean must be finite");
    if (!(logStdDev.IsFinite() && logStdDev > 0))
      throw new ArgumentException("logStdDev must be finite and positive");
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

  /// <inheritdoc />
  /// <remarks>
  /// The support is <c>x &gt; 0</c>, and that is the test. It used to be <c>x &lt; 1e-9</c>, which
  /// is not the edge of the support but an arbitrary point inside it: a Multiplic(-30, 1) has its
  /// median at 9.4e-14, so that threshold reported zero density and zero probability across
  /// essentially the whole distribution.
  /// </remarks>
  public double DensityAt(double x)
  {
    if (x <= 0.0)
      return 0.0;
    double z = (LogFunction.Value(x) - LogMean) / LogStdDev;
    return ExpFunction.Value(-z * z / 2) / (x * LogStdDev * Constants.Sqrt2Pi);
  }

  /// <inheritdoc />
  /// <remarks>
  /// A lognormal variable's distribution function is the normal one applied to log x, so this
  /// goes through AdditiveCumulative rather than spelling the same thing out through erf.
  /// </remarks>
  public double CumulativeAt(double x)
  {
    if (x <= 0.0)
      return 0.0;
    return AdditiveCumulative.Value((LogFunction.Value(x) - LogMean) / LogStdDev);
  }

  /// <inheritdoc />
  public double QuantileAt(double p)
  {
    if (p <= 0.0)
      return 0.0;
    if (p >= 1.0)
      return double.PositiveInfinity;
    return ExpFunction.Value(LogMean + LogStdDev * AdditiveQuantile.Value(p));
  }
}
