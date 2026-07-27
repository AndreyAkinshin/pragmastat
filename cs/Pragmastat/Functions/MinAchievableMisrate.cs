using Pragmastat.Exceptions;

namespace Pragmastat.Functions;

/// <summary>
/// Computes minimum achievable misrate for distribution-free bounds.
/// </summary>
internal static class MinAchievableMisrate
{
  /// <summary>
  /// Minimum achievable misrate for one-sample signed-rank based bounds.
  /// </summary>
  /// <param name="n">Sample size (must be positive).</param>
  /// <returns>Minimum achievable misrate: 2^(1-n)</returns>
  /// <exception cref="AssumptionException">Thrown when n is not positive.</exception>
  public static double OneSample(int n)
  {
    if (n <= 0)
      throw AssumptionException.Domain(Subject.X);
    // Repeated halving rather than Pow: this is a power of two, and halving is exact in
    // binary64. A general power function returns the same value in every implementation
    // anyone ships, but the specification does not require it to, and this value is a
    // domain boundary: it decides which misrates the toolkit accepts at all.
    // Math.ScaleB would say it more directly but does not exist in netstandard2.0.
    double result = 1.0;
    for (int i = 1; i < n; i++)
      result /= 2.0;
    return result;
  }

  /// <summary>
  /// Minimum achievable misrate for two-sample Mann-Whitney based bounds.
  /// </summary>
  /// <param name="n">Size of first sample.</param>
  /// <param name="m">Size of second sample.</param>
  /// <returns>Minimum achievable misrate.</returns>
  public static double TwoSample(int n, int m)
  {
    if (n <= 0)
      throw AssumptionException.Domain(Subject.X);
    if (m <= 0)
      throw AssumptionException.Domain(Subject.Y);
    return 2.0 / BinomialCoefficientFunction.BinomialCoefficient(n + m, n);
  }
}
