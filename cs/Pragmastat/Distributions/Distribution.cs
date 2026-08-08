using Pragmastat.Randomization;

namespace Pragmastat.Distributions;

/// <summary>
/// A distribution: three views of one object, plus the ability to draw from it.
/// </summary>
/// <remarks>
/// <para>
/// <c>DensityAt</c> is how tightly probability is packed near a value, <c>CumulativeAt</c> is the
/// probability of not exceeding it, and <c>QuantileAt</c> inverts that. Each family supplies all
/// three out of the toolkit's own primitives, so seven implementations return identical bits.
/// </para>
/// <para>
/// <c>Sample</c> remains separate rather than being defined as
/// <c>QuantileAt(UniformDouble())</c>. Two of the families draw by a route that is not inverse
/// transform sampling, and changing that would change every seeded stream in every language, which
/// is a decision of its own and not this one.
/// </para>
/// <para>
/// Why the names carry <c>At</c>: a reconstruction is also a distribution, and it publishes
/// <c>Density</c>, <c>Cumulative</c> and <c>Quantile</c> as vector queries that validate their
/// input. One type would then carry both spellings of each name. C# can express that as overloads
/// and Rust would quietly resolve it to the inherent method, but four of the seven ports have no
/// such rule, and a method whose meaning depends on which of two identically named things the
/// compiler picks is a trap in any language. <c>At</c> reads as what it is: the value at a point.
/// </para>
/// <para>
/// The parameter of <c>QuantileAt</c> is a plain <c>double</c> rather than a
/// <see cref="Probability"/>: this trio is the mathematics, defined outside <c>(0, 1)</c> as well,
/// where it returns the endpoints of the support.
/// </para>
/// </remarks>
public interface IDistribution
{
  /// <summary>
  /// Generate a single sample from this distribution.
  /// </summary>
  /// <param name="rng">The random number generator to use.</param>
  /// <returns>A sample from the distribution.</returns>
  double Sample(Rng rng);

  /// <summary>
  /// Probability density at <paramref name="x"/>. Zero outside the support.
  /// </summary>
  double DensityAt(double x);

  /// <summary>
  /// Probability of not exceeding <paramref name="x"/>. Zero below the support, one above it.
  /// </summary>
  double CumulativeAt(double x);

  /// <summary>
  /// The value below which a fraction <paramref name="p"/> of the distribution lies.
  /// </summary>
  /// <remarks>
  /// A <paramref name="p"/> outside <c>(0, 1)</c> returns the endpoints of the support, which are
  /// infinities for the unbounded families.
  /// </remarks>
  double QuantileAt(double p);

  /// <summary>
  /// Generate multiple samples from this distribution.
  /// </summary>
  /// <param name="rng">The random number generator to use.</param>
  /// <param name="count">Number of samples to generate.</param>
  /// <returns>List of samples.</returns>
  List<double> Samples(Rng rng, int count);
}
