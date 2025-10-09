using JetBrains.Annotations;
using Pragmastat.Distributions.Randomization;
using Pragmastat.Functions;
using Pragmastat.Internal;

namespace Pragmastat.Distributions;

/// <summary>
/// Multiplic distribution ('Log-normal' or 'Galton' in traditional statistics)
/// </summary>
public class MultiplicDistribution : IContinuousDistribution
{
  public static readonly MultiplicDistribution Standard = new();

  [PublicAPI] public double LogMean { get; }
  [PublicAPI] public double LogStdDev { get; }

  public MultiplicDistribution(double logMean = 0, double logStdDev = 1)
  {
    Assertion.Positive(nameof(logStdDev), logStdDev);

    LogMean = logMean;
    LogStdDev = logStdDev;
  }

  public double Pdf(double x)
  {
    if (x < 1e-9)
      return 0;
    return Exp(-(Log(x) - LogMean).Sqr() / (2 * LogStdDev.Sqr())) / (x * LogStdDev * Constants.Sqrt2Pi);
  }

  public double Cdf(double x)
  {
    if (x < 1e-9)
      return 0;
    return 0.5 * (1 + ErrorFunction.Value((Log(x) - LogMean) / (Constants.Sqrt2 * LogStdDev)));
  }

  public double Quantile(Probability p)
  {
    if (p < 1e-9)
      return 0;
    if (p > 1 - 1e-9)
      return double.PositiveInfinity;
    return Exp(LogMean + Constants.Sqrt2 * LogStdDev * ErrorFunction.InverseValue(2 * p - 1));
  }

  public double? AsymptoticSpread => null;

  private class RandomGenerator(Random? random, MultiplicDistribution distribution) : AbstractRandomGenerator(random)
  {
    public override double Next()
    {
      double additive =
        BoxMullerTransform.Apply(distribution.LogMean, distribution.LogStdDev, () => Random.NextDouble());
      return Exp(additive);
    }
  }

  public AbstractRandomGenerator Random(Random? random = null) => new RandomGenerator(random, this);

  public override string ToString() => $"Multiplic({LogMean.ToStringInvariant()},{LogStdDev.ToStringInvariant()})";
}
