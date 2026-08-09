using Pragmastat.Internal;
using Pragmastat.Randomization;

namespace Pragmastat.Distributions;

/// <summary>
/// Uniform distribution on [min, max).
/// </summary>
/// <remarks>
/// All values within the interval have equal probability.
/// </remarks>
public sealed class Uniform : IDistribution
{
  /// <summary>
  /// Standard uniform distribution on [0, 1).
  /// </summary>
  internal static readonly Uniform Standard = new(0, 1);

  /// <summary>
  /// Lower bound (inclusive).
  /// </summary>
  public double Min { get; }

  /// <summary>
  /// Upper bound (exclusive).
  /// </summary>
  public double Max { get; }

  /// <summary>
  /// Create a new uniform distribution on [min, max).
  /// </summary>
  /// <param name="min">Lower bound (inclusive).</param>
  /// <param name="max">Upper bound (exclusive).</param>
  /// <exception cref="ArgumentException">
  /// Thrown if min or max is not finite, or if min is not less than max.
  /// </exception>
  public Uniform(double min, double max)
  {
    if (!min.IsFinite())
      throw new ArgumentException("min must be finite");
    if (!max.IsFinite())
      throw new ArgumentException("max must be finite");
    if (!(min < max))
      throw new ArgumentException("min must be less than max");
    Min = min;
    Max = max;
  }

  /// <inheritdoc />
  public double Sample(Rng rng)
  {
    return QuantileAt(rng.UniformDouble());
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
  /// The support is half-open, so the density is zero at <c>Max</c> and positive at <c>Min</c>.
  /// This is the one family assembled from arithmetic alone: no primitive of the toolkit appears
  /// on any of its three paths.
  /// <para>
  /// NaN is refused before the bounds are compared. This is the only one of the fifteen functions
  /// whose result does not depend on its argument, so a NaN reaching the comparison satisfies
  /// neither half of it and falls through to a perfectly ordinary density. The other four families
  /// carry NaN into arithmetic and return it without being asked to.
  /// </para>
  /// </remarks>
  public double DensityAt(double x)
  {
    if (double.IsNaN(x))
      return x;
    return x < Min || x >= Max ? 0.0 : 1.0 / (Max - Min);
  }

  /// <inheritdoc />
  public double CumulativeAt(double x)
  {
    if (x < Min)
      return 0.0;
    if (x >= Max)
      return 1.0;
    return (x - Min) / (Max - Min);
  }

  /// <inheritdoc />
  public double QuantileAt(double p)
  {
    if (p <= 0.0)
      return Min;
    if (p >= 1.0)
      return Max;
    return Min + p * (Max - Min);
  }
}
