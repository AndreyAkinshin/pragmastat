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
    return Math.Pow(2, 1 - n);
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
    return n + m <= BinomialCoefficientFunction.MaxAcceptableN
      ? 2.0 / BinomialCoefficientFunction.BinomialCoefficient(n + m, n)
      : 2.0 / BinomialCoefficientFunction.BinomialCoefficient(n + m, n * 1.0);
  }
}
