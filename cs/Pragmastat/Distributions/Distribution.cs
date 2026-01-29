using Pragmastat.Randomization;

namespace Pragmastat.Distributions;

/// <summary>
/// Base interface for distributions that can generate samples.
/// </summary>
public interface IDistribution
{
  /// <summary>
  /// Generate a single sample from this distribution.
  /// </summary>
  /// <param name="rng">The random number generator to use.</param>
  /// <returns>A sample from the distribution.</returns>
  double Sample(Rng rng);

  /// <summary>
  /// Generate multiple samples from this distribution.
  /// </summary>
  /// <param name="rng">The random number generator to use.</param>
  /// <param name="count">Number of samples to generate.</param>
  /// <returns>List of samples.</returns>
  List<double> Samples(Rng rng, int count);
}

// =============================================================================
// Extended distribution interface - NOT part of stable public API.
// Preparation for future version. These methods are not declared in the manual yet.
// Subject to change without notice in future versions.
// =============================================================================

/// <summary>
/// Interface for continuous distributions with full statistical functions.
/// </summary>
/// <remarks>
/// NOT part of stable public API - preparation for future version.
/// These methods are not declared in the manual yet.
/// </remarks>
[System.ComponentModel.EditorBrowsable(System.ComponentModel.EditorBrowsableState.Never)]
public interface IContinuousDistribution : IDistribution
{
  /// <summary>
  /// Probability density function.
  /// </summary>
  double Pdf(double x);

  /// <summary>
  /// Cumulative distribution function.
  /// </summary>
  double Cdf(double x);

  /// <summary>
  /// Quantile function (inverse CDF).
  /// </summary>
  double Quantile(Probability p);

  /// <summary>
  /// Asymptotic value of spread for the given distribution.
  /// Returns null in case of unknown or infinite spread.
  /// </summary>
  double? AsymptoticSpread { get; }
}
