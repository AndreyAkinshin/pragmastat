using Pragmastat.Distributions.Randomization;

namespace Pragmastat.Distributions;

public interface IContinuousDistribution
{
  /// <summary>
  /// Probability density function 
  /// </summary>
  double Pdf(double x);

  /// <summary>
  /// Cumulative distribution function
  /// </summary>
  double Cdf(double x);

  /// <summary>
  /// Quantile function
  /// </summary>
  double Quantile(Probability p);

  AbstractRandomGenerator Random(Random? random = null);

  /// <summary>
  /// Asymptotic value of spread for the given distribution.
  /// Returns null in case of unknown or infinite spread.
  /// </summary>
  double? AsymptoticSpread { get; }
}
