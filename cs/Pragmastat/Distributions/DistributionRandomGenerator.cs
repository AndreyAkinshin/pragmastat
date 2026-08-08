using Pragmastat.Metrology;
using Pragmastat.Randomization;

namespace Pragmastat.Distributions;

// =============================================================================
// Helper classes for random generation - NOT part of stable public API.
// Preparation for future version. These classes are not declared in the manual yet.
// Subject to change without notice in future versions.
// =============================================================================

/// <summary>
/// Abstract base class for random generators using deterministic RNG.
/// </summary>
internal abstract class AbstractRandomGenerator
{
  protected readonly Rng Rng;

  protected AbstractRandomGenerator(int? seed = null)
  {
    Rng = seed.HasValue ? new Rng(seed.Value) : new Rng();
  }

  /// <summary>
  /// Returns a random floating-point number from the given distribution.
  /// </summary>
  public abstract double Next();

  /// <summary>
  /// Returns an array of random floating-point numbers from the given distribution.
  /// </summary>
  public double[] Next(int n)
  {
    double[] numbers = new double[n];
    for (int i = 0; i < n; i++)
      numbers[i] = Next();
    return numbers;
  }

  /// <summary>
  /// Returns a sample of random values from the given distribution.
  /// </summary>
  public Sample NextSample(int size, MeasurementUnit? unit = null)
  {
    return new Sample(Next(size), unit);
  }
}

/// <summary>
/// Internal random generator that uses quantile function for sampling.
/// Used for backward compatibility with Simulations and TestGenerator.
/// </summary>
internal class QuantileRandomGenerator : AbstractRandomGenerator
{
  private readonly IDistribution _distribution;

  public QuantileRandomGenerator(IDistribution distribution, int? seed = null)
    : base(seed)
  {
    _distribution = distribution;
  }

  public override double Next()
  {
    return _distribution.QuantileAt(Rng.UniformDouble());
  }
}

/// <summary>
/// Extension methods for distributions to create random generators.
/// </summary>
/// <remarks>
/// There used to be a second generator here that drew through <c>Sample</c>, and a second
/// <c>Random</c> overload returning it. Nothing could reach either: every caller held a family,
/// every family implemented the narrower interface the other overload took, and overload
/// resolution therefore always chose the quantile one. Deleting the interface split made the two
/// overloads identical, which is what surfaced it.
/// </remarks>
internal static class DistributionExtensions
{
  /// <summary>
  /// Creates a random generator for this distribution.
  /// </summary>
  public static AbstractRandomGenerator Random(this IDistribution distribution, int? seed = null)
  {
    return new QuantileRandomGenerator(distribution, seed);
  }
}
