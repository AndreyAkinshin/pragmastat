namespace Pragmastat.Tests.Algorithms;

using Pragmastat.Algorithms;
using Pragmastat.Estimators;
using Pragmastat.Exceptions;
using Pragmastat.Functions;
using Pragmastat.Randomization;

/// <summary>
/// Verification tests comparing new algorithms against naive brute-force implementations.
/// </summary>
public class AlgorithmVerificationTests
{
  // =========================================================================
  // FastCenterQuantiles verification
  // =========================================================================

  [Fact]
  public void FastCenterQuantiles_MatchesNaive_SmallInputs()
  {
    var testCases = new[]
    {
      new double[] { 1, 2, 3 },
      new double[] { 1, 2, 3, 4, 5 },
      new double[] { -2, -1, 0, 1, 2 },
    };

    foreach (var data in testCases)
    {
      var sorted = data.OrderBy(x => x).ToList();
      var naiveAverages = ComputeAllPairwiseAverages(sorted);

      // Only test first, middle, and last quantiles for speed
      var testRanks = new[] { 1, naiveAverages.Count / 2, naiveAverages.Count };

      foreach (var k in testRanks)
      {
        double expected = naiveAverages[k - 1];
        double actual = FastCenterQuantiles.Quantile(sorted, k);

        Assert.True(
          Math.Abs(expected - actual) < 1e-10,
          $"Mismatch for data [{string.Join(", ", data)}], k={k}: expected {expected}, got {actual}, diff={Math.Abs(expected - actual)}");
      }
    }
  }

  [Fact]
  public void FastCenterQuantiles_Bounds_MatchesNaive()
  {
    var data = new double[] { 1, 2, 3, 4, 5 };
    var sorted = data.OrderBy(x => x).ToList();
    var naiveAverages = ComputeAllPairwiseAverages(sorted);

    // Test various margin combinations
    var testRanks = new[] { (1L, 15L), (2L, 14L), (3L, 13L), (5L, 11L), (7L, 9L), (1L, 1L), (15L, 15L) };

    foreach (var (lo, hi) in testRanks)
    {
      var (actualLo, actualHi) = FastCenterQuantiles.Bounds(sorted, lo, hi);

      double expectedLo = naiveAverages[(int)Math.Min(lo, hi) - 1];
      double expectedHi = naiveAverages[(int)Math.Max(lo, hi) - 1];

      Assert.True(
        Math.Abs(expectedLo - actualLo) < 1e-10 && Math.Abs(expectedHi - actualHi) < 1e-10,
        $"Bounds mismatch for ranks ({lo}, {hi}): expected ({expectedLo}, {expectedHi}), got ({actualLo}, {actualHi})");
    }
  }

  // Intentionally naive O(n² log n) implementation for verification against optimized algorithm.
  // This brute-force approach materializes all pairwise averages, which is infeasible for large n
  // but provides a ground truth for testing the fast O(n log n) algorithm.
  private static List<double> ComputeAllPairwiseAverages(IReadOnlyList<double> sorted)
  {
    var averages = new List<double>();
    int n = sorted.Count;
    for (int i = 0; i < n; i++)
    {
      for (int j = i; j < n; j++)
      {
        averages.Add((sorted[i] + sorted[j]) / 2.0);
      }
    }
    averages.Sort();
    return averages;
  }

  // =========================================================================
  // SignedRankMargin verification
  // =========================================================================

  [Fact]
  public void SignedRankMargin_ExactDistribution_SmallN()
  {
    // For small n, verify the CDF computation against brute force
    for (int n = 2; n <= 10; n++)
    {
      var exactCdf = ComputeExactSignedRankCdf(n);
      double minMisrate = MinAchievableMisrate.OneSample(n);

      // Test various misrates that are achievable
      var testMisrates = new[] { 0.5, 0.25, 0.1, 0.05 }
        .Where(m => m >= minMisrate)
        .ToList();

      foreach (var misrate in testMisrates)
      {
        int margin = SignedRankMargin.Instance.Calc(n, misrate);

        // Verify: CDF at margin/2 should be approximately misrate/2
        int halfMargin = margin / 2;
        double actualCdf = exactCdf[halfMargin];

        // The margin should give us the smallest w where CDF(w) >= misrate/2
        // So CDF(halfMargin) >= misrate/2, and CDF(halfMargin-1) < misrate/2
        Assert.True(
          actualCdf >= misrate / 2 - 1e-10,
          $"n={n}, misrate={misrate}: CDF({halfMargin})={actualCdf} should be >= {misrate / 2}");

        if (halfMargin > 0)
        {
          double prevCdf = exactCdf[halfMargin - 1];
          Assert.True(
            prevCdf < misrate / 2 + 1e-10,
            $"n={n}, misrate={misrate}: CDF({halfMargin - 1})={prevCdf} should be < {misrate / 2}");
        }
      }
    }
  }

  private static double[] ComputeExactSignedRankCdf(int n)
  {
    long total = 1L << n;
    long maxW = n * (n + 1) / 2;

    var count = new long[maxW + 1];
    count[0] = 1;

    for (int i = 1; i <= n; i++)
    {
      for (int w = Math.Min((int)maxW, i * (i + 1) / 2); w >= i; w--)
        count[w] += count[w - i];
    }

    var cdf = new double[maxW + 1];
    long cumulative = 0;
    for (int w = 0; w <= maxW; w++)
    {
      cumulative += count[w];
      cdf[w] = (double)cumulative / total;
    }
    return cdf;
  }

  // =========================================================================
  // CenterBoundsEstimator verification
  // =========================================================================

  [Fact]
  public void CenterBounds_Coverage_Simulation()
  {
    // Simulate coverage for a symmetric distribution
    var rng = new Rng("center-coverage-test");
    int n = 10;
    double misrate = 0.1;
    int iterations = 100;
    int covered = 0;
    double trueCenter = 0.0; // For Uniform(-1, 1), center is 0

    for (int i = 0; i < iterations; i++)
    {
      var values = new double[n];
      for (int j = 0; j < n; j++)
        values[j] = rng.UniformDouble(-1, 1);

      var sample = new Sample(values);

      try
      {
        var bounds = Toolkit.CenterBounds(sample, new Probability(misrate));

        if (bounds.Lower <= trueCenter && trueCenter <= bounds.Upper)
          covered++;
      }
      catch (AssumptionException ex) when (ex.Violation.Subject == Subject.Misrate)
      {
        // If misrate is too small for the sample size, skip
      }
    }

    double actualCoverage = (double)covered / iterations;

    // Expected coverage = 0.9. With 100 iterations, SD = sqrt(0.9*0.1/100) ≈ 0.030.
    // Threshold 0.80 is ~3.3 SD below expected, so false failures are rare.
    Assert.True(
      actualCoverage >= 0.80,
      $"CenterBounds coverage {actualCoverage} should be >= 0.80 (expected ~0.9)");
  }

  [Fact]
  public void CenterBounds_MatchesCenter_ForSymmetricData()
  {
    // For symmetric data, the center should be within the bounds
    var symmetricData = new double[] { -2, -1, 0, 1, 2 };
    var sample = new Sample(symmetricData);

    var center = Toolkit.Center(sample);
    var bounds = Toolkit.CenterBounds(sample, new Probability(0.5)); // Loose misrate for small n

    Assert.True(
      bounds.Lower <= center.NominalValue && center.NominalValue <= bounds.Upper,
      $"Center {center.NominalValue} should be within bounds [{bounds.Lower}, {bounds.Upper}]");
  }

}
