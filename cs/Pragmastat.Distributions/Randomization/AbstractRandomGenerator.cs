using Pragmastat.Internal;
using Pragmastat.Metrology;

namespace Pragmastat.Distributions.Randomization;

public abstract class AbstractRandomGenerator(Random? random = null)
{
  protected readonly Random Random = random ?? new Random();

  /// <summary>
  /// Returns a random floating-point number from the given distribution.
  /// </summary>
  /// <returns>A random double-precision floating-point number from the given distribution.</returns>
  public abstract double Next();

  /// <summary>
  /// Returns an array of random floating-point numbers from the given distribution.
  /// </summary>
  /// <param name="n">The size of the returned array.</param>
  /// <returns>An array of random double-precision floating-point numbers from the given distribution.</returns>
  public double[] Next(int n)
  {
    double[] numbers = new double[n];
    for (int i = 0; i < n; i++)
      numbers[i] = Next();
    return numbers;
  }

  public Sample NextSample(int size, MeasurementUnit? unit = null) => Next(size).ToSample(unit);
}
