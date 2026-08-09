using Pragmastat.Functions;
using Pragmastat.Internal;
using Pragmastat.Randomization;

namespace Pragmastat.Distributions;

/// <summary>
/// Power (Pareto) distribution with minimum value and shape parameter.
/// </summary>
/// <remarks>
/// Follows a power-law distribution where large values are rare but possible.
/// Historically called 'Pareto' distribution.
/// </remarks>
public sealed class Power : IDistribution
{
  /// <summary>
  /// Standard power distribution (min=1, shape=1).
  /// </summary>
  internal static readonly Power Standard = new(1, 1);

  /// <summary>
  /// Minimum value (scale parameter, x_m).
  /// </summary>
  public double Min { get; }

  /// <summary>
  /// Shape parameter (alpha, controls tail heaviness).
  /// </summary>
  public double Shape { get; }

  /// <summary>
  /// Create a new power (Pareto) distribution.
  /// </summary>
  /// <param name="min">Minimum value (lower bound, > 0).</param>
  /// <param name="shape">Shape parameter (alpha > 0, controls tail heaviness).</param>
  /// <exception cref="ArgumentException">
  /// Thrown if min or shape is not finite and positive.
  /// </exception>
  public Power(double min, double shape)
  {
    if (!(min.IsFinite() && min > 0))
      throw new ArgumentException("min must be finite and positive");
    if (!(shape.IsFinite() && shape > 0))
      throw new ArgumentException("shape must be finite and positive");
    Min = min;
    Shape = shape;
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
  /// <para>
  /// <c>shape * min^shape / x^(shape+1)</c>, rearranged as
  /// <c>(shape / x) * exp(shape * (log min - log x))</c>. The toolkit owns an exponential and a
  /// logarithm and owns no general power, so a real exponent is spelled as the exponential of a
  /// logarithm. This family is the reason the logarithm had to exist before the trio could.
  /// </para>
  /// <para>
  /// The support is <c>[min, infinity)</c>, closed at the left, so the density at <c>min</c> is
  /// <c>shape / min</c> rather than zero, while the cumulative there is zero. Uniform reads the
  /// same way at its own lower endpoint.
  /// </para>
  /// </remarks>
  public double DensityAt(double x)
  {
    if (x < Min)
      return 0.0;
    return (Shape / x) * ExpFunction.Value(Shape * (LogFunction.Value(Min) - LogFunction.Value(x)));
  }

  /// <inheritdoc />
  public double CumulativeAt(double x)
  {
    if (x <= Min)
      return 0.0;
    return 1.0 - ExpFunction.Value(Shape * (LogFunction.Value(Min) - LogFunction.Value(x)));
  }

  /// <inheritdoc />
  public double QuantileAt(double p)
  {
    if (p <= 0.0)
      return Min;
    if (p >= 1.0)
      return double.PositiveInfinity;
    return Min * ExpFunction.Value(-LogFunction.Value(1.0 - p) / Shape);
  }
}
