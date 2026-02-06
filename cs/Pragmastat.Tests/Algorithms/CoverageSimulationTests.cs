namespace Pragmastat.Tests.Algorithms;

using Pragmastat.Estimators;
using Pragmastat.Randomization;

/// <summary>
/// Smoke tests for bounds estimators.
/// These tests verify basic functionality with minimal iterations (50).
/// For deep statistical validation with 1000+ iterations, use Pragmastat.Simulations.
/// </summary>
public class CoverageSimulationTests
{
  // 50 iterations is a smoke test only.
  // Statistical basis: For coverage ~0.90, with n=50, binomial SD ≈ 0.04.
  // Tolerance of 0.10-0.20 gives ~2.5-5 SD margin for smoke testing.
  // For production validation, use 200+ iterations in Pragmastat.Simulations.
  private const int SmokeTestIterations = 50;

  // =========================================================================
  // CenterBounds Coverage (symmetric distributions only)
  // =========================================================================

  [Theory]
  [InlineData(10, 0.10, 0.0)]
  [InlineData(30, 0.05, 0.0)]
  public void CenterBounds_Coverage_AdditiveDistribution(int n, double misrate, double trueCenterOffset)
  {
    var rng = new Rng($"center-bounds-additive-{n}-{misrate}");
    double trueCenter = trueCenterOffset; // Additive(0,1) has center = 0
    int covered = 0;

    for (int i = 0; i < SmokeTestIterations; i++)
    {
      var values = GenerateAdditive(rng, n);
      var sample = new Sample(values);
      var bounds = Toolkit.CenterBounds(sample, new Probability(misrate));

      if (bounds.Lower <= trueCenter && trueCenter <= bounds.Upper)
        covered++;
    }

    double actualCoverage = (double)covered / SmokeTestIterations;
    double expectedCoverage = 1 - misrate;

    // Allow statistical variation: coverage should be within 10% of expected
    Assert.True(
      actualCoverage >= expectedCoverage - 0.10,
      $"CenterBounds coverage {actualCoverage:F3} should be >= {expectedCoverage - 0.10:F3} " +
      $"(n={n}, misrate={misrate}, expected={expectedCoverage})");
  }

  [Theory]
  [InlineData(10, 0.10)]
  [InlineData(30, 0.05)]
  public void CenterBounds_Coverage_UniformSymmetric(int n, double misrate)
  {
    var rng = new Rng($"center-bounds-uniform-{n}-{misrate}");
    double trueCenter = 0.0; // Uniform(-1,1) has center = 0
    int covered = 0;

    for (int i = 0; i < SmokeTestIterations; i++)
    {
      var values = new double[n];
      for (int j = 0; j < n; j++)
        values[j] = rng.Uniform(-1, 1);

      var sample = new Sample(values);
      var bounds = Toolkit.CenterBounds(sample, new Probability(misrate));

      if (bounds.Lower <= trueCenter && trueCenter <= bounds.Upper)
        covered++;
    }

    double actualCoverage = (double)covered / SmokeTestIterations;
    double expectedCoverage = 1 - misrate;

    Assert.True(
      actualCoverage >= expectedCoverage - 0.10,
      $"CenterBounds coverage {actualCoverage:F3} should be >= {expectedCoverage - 0.10:F3} " +
      $"(n={n}, misrate={misrate}, expected={expectedCoverage})");
  }

  // =========================================================================
  // MedianBounds Coverage (all distributions - distribution-free)
  // =========================================================================

  [Theory]
  [InlineData(10, 0.10)]
  [InlineData(30, 0.05)]
  public void MedianBounds_Coverage_AdditiveDistribution(int n, double misrate)
  {
    var rng = new Rng($"median-bounds-additive-{n}-{misrate}");
    double trueMedian = 0.0; // Additive(0,1) has median = 0
    int covered = 0;

    for (int i = 0; i < SmokeTestIterations; i++)
    {
      var values = GenerateAdditive(rng, n);
      var sample = new Sample(values);
      var bounds = Toolkit.MedianBounds(sample, new Probability(misrate));

      if (bounds.Lower <= trueMedian && trueMedian <= bounds.Upper)
        covered++;
    }

    double actualCoverage = (double)covered / SmokeTestIterations;
    double expectedCoverage = 1 - misrate;

    Assert.True(
      actualCoverage >= expectedCoverage - 0.10,
      $"MedianBounds coverage {actualCoverage:F3} should be >= {expectedCoverage - 0.10:F3} " +
      $"(n={n}, misrate={misrate}, expected={expectedCoverage})");
  }

  [Theory]
  [InlineData(10, 0.10)]
  [InlineData(30, 0.05)]
  public void MedianBounds_Coverage_UniformDistribution(int n, double misrate)
  {
    var rng = new Rng($"median-bounds-uniform-{n}-{misrate}");
    double trueMedian = 0.5; // Uniform(0,1) has median = 0.5
    int covered = 0;

    for (int i = 0; i < SmokeTestIterations; i++)
    {
      var values = new double[n];
      for (int j = 0; j < n; j++)
        values[j] = rng.Uniform();

      var sample = new Sample(values);
      var bounds = Toolkit.MedianBounds(sample, new Probability(misrate));

      if (bounds.Lower <= trueMedian && trueMedian <= bounds.Upper)
        covered++;
    }

    double actualCoverage = (double)covered / SmokeTestIterations;
    double expectedCoverage = 1 - misrate;

    Assert.True(
      actualCoverage >= expectedCoverage - 0.10,
      $"MedianBounds coverage {actualCoverage:F3} should be >= {expectedCoverage - 0.10:F3} " +
      $"(n={n}, misrate={misrate}, expected={expectedCoverage})");
  }

  [Theory]
  [InlineData(10, 0.10)]
  [InlineData(30, 0.05)]
  public void MedianBounds_Coverage_ExponentialDistribution(int n, double misrate)
  {
    var rng = new Rng($"median-bounds-exp-{n}-{misrate}");
    double trueMedian = Math.Log(2); // Exp(1) has median = ln(2)
    int covered = 0;

    for (int i = 0; i < SmokeTestIterations; i++)
    {
      var values = GenerateExponential(rng, n);
      var sample = new Sample(values);
      var bounds = Toolkit.MedianBounds(sample, new Probability(misrate));

      if (bounds.Lower <= trueMedian && trueMedian <= bounds.Upper)
        covered++;
    }

    double actualCoverage = (double)covered / SmokeTestIterations;
    double expectedCoverage = 1 - misrate;

    // MedianBounds is distribution-free, should work for Exponential too
    Assert.True(
      actualCoverage >= expectedCoverage - 0.10,
      $"MedianBounds coverage {actualCoverage:F3} should be >= {expectedCoverage - 0.10:F3} " +
      $"(n={n}, misrate={misrate}, expected={expectedCoverage})");
  }

  // =========================================================================
  // Helper methods for generating distributions
  // =========================================================================

  /// <summary>
  /// Generate samples from Additive(0, 1) distribution (sum of Uniform(-0.5, 0.5)).
  /// This is symmetric around 0.
  /// </summary>
  private static double[] GenerateAdditive(Rng rng, int n, int components = 12)
  {
    var values = new double[n];
    for (int i = 0; i < n; i++)
    {
      double sum = 0;
      for (int j = 0; j < components; j++)
        sum += rng.Uniform(-0.5, 0.5);
      values[i] = sum / Math.Sqrt(components / 12.0); // Scale to unit variance
    }
    return values;
  }

  /// <summary>
  /// Generate samples from Exp(1) distribution using inverse transform.
  /// This is right-skewed (asymmetric).
  /// </summary>
  private static double[] GenerateExponential(Rng rng, int n)
  {
    var values = new double[n];
    for (int i = 0; i < n; i++)
      values[i] = -Math.Log(1 - rng.Uniform()); // Inverse CDF of Exp(1)
    return values;
  }
}
