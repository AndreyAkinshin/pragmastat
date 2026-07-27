using Pragmastat.Functions;
using Pragmastat.TestGenerator.Framework;
using Pragmastat.TestGenerator.Framework.PairwiseMargin;
using Spectre.Console;

namespace Pragmastat.TestGenerator.TestCases;

public static class PairwiseMarginTestCases
{
  public static void Generate()
  {
    const string suiteName = "pairwise-margin";
    AnsiConsole.MarkupLine($"[yellow]→[/] Generating tests for: [bold]{suiteName}[/]");

    var inputBuilder = new ReferenceTestCaseInputBuilder<PairwiseMarginInput>();

    // Demo examples (4 tests)
    inputBuilder.Add("demo-1", new PairwiseMarginInput(30, 30, 1e-6));
    inputBuilder.Add("demo-2", new PairwiseMarginInput(30, 30, 1e-5));
    inputBuilder.Add("demo-3", new PairwiseMarginInput(30, 30, 1e-4));
    inputBuilder.Add("demo-4", new PairwiseMarginInput(30, 30, 1e-3));

    // Natural sequences
    // [n, m] ∈ {1, 2, 3, 4} × {1, 2, 3, 4} × 2 misrates, filtered by min_misrate
    int[] naturalSizes = [1, 2, 3, 4];
    double[] naturalMisrates = [1e-1, 1e-2];
    foreach (var misrate in naturalMisrates)
    {
      foreach (var n in naturalSizes)
      {
        foreach (var m in naturalSizes)
        {
          if (misrate < MinAchievableMisrate.TwoSample(n, m)) continue;
          string testName = $"natural-{n}-{m}-mr{FormatMisrate(misrate)}";
          inputBuilder.Add(testName, new PairwiseMarginInput(n, m, misrate));
        }
      }
    }

    // Edge cases (10 tests)
    inputBuilder.Add("boundary-min", new PairwiseMarginInput(1, 1, 1.0));
    inputBuilder.Add("boundary-zero-margin-small", new PairwiseMarginInput(20, 20, 1e-6));
    inputBuilder.Add("boundary-loose", new PairwiseMarginInput(5, 5, 0.9));
    inputBuilder.Add("symmetry-2-5", new PairwiseMarginInput(2, 5, 0.1));
    inputBuilder.Add("symmetry-5-2", new PairwiseMarginInput(5, 2, 0.1));
    inputBuilder.Add("symmetry-3-7", new PairwiseMarginInput(3, 7, 0.05));
    inputBuilder.Add("symmetry-7-3", new PairwiseMarginInput(7, 3, 0.05));
    inputBuilder.Add("asymmetry-extreme-1-100", new PairwiseMarginInput(1, 100, 0.1));
    inputBuilder.Add("asymmetry-extreme-100-1", new PairwiseMarginInput(100, 1, 0.1));
    inputBuilder.Add("asymmetry-extreme-2-50", new PairwiseMarginInput(2, 50, 0.05));

    // Overflow boundary: n+m in [60, 66], symmetric and asymmetric splits
    int[] boundarySizes = [60, 61, 62, 63, 64, 65, 66];
    double[] boundaryMisrates = [1e-1, 1e-3];
    foreach (var total in boundarySizes)
    {
      foreach (var misrate in boundaryMisrates)
      {
        int n1 = total / 2;
        int m1 = total - n1;
        if (misrate >= MinAchievableMisrate.TwoSample(n1, m1))
          inputBuilder.Add($"boundary-overflow-n{n1}_m{m1}_mr{FormatMisrate(misrate)}",
            new PairwiseMarginInput(n1, m1, misrate));

        int n2 = total / 3;
        int m2 = total - n2;
        if (n2 >= 1 && m2 >= 1 && misrate >= MinAchievableMisrate.TwoSample(n2, m2))
          inputBuilder.Add($"boundary-overflow-n{n2}_m{m2}_mr{FormatMisrate(misrate)}",
            new PairwiseMarginInput(n2, m2, misrate));
      }
    }

    // At the misrate floor.
    //
    // The floor is `2 / C(n+m, n)`, so `cdf >= misrate/2` there is `1/C >= 1/C`: an exact
    // equality in exact arithmetic, decided entirely by how the two sides rounded. Nothing else
    // in this suite samples it. The boundary-overflow cases above are named for the same
    // n+m range but ask at 0.1 and 0.001, which for n+m = 62 is fifteen orders of magnitude
    // away from where the answer is actually in doubt.
    //
    // The range spans the switch from the exact integer binomial to the multiplicative
    // recurrence, and covers both symmetric and lopsided splits, because the two call sites ask
    // for C(n+m, n) and C(n+m, m) and those must agree bit for bit.
    int[] floorTotals = [55, 58, 60, 61, 62, 63, 64, 66, 70, 90, 128, 200, 340];
    foreach (var total in floorTotals)
    {
      int[] splits = [total / 2, total / 3, total / 7, 4, 1];
      foreach (var n in splits)
      {
        int m = total - n;
        if (n < 1 || m < 1) continue;
        double floor = MinAchievableMisrate.TwoSample(n, m);
        if (double.IsNaN(floor) || floor <= 0 || floor >= 1) continue;
        inputBuilder.Add($"floor-n{n}_m{m}", new PairwiseMarginInput(n, m, floor));
      }
    }

    // At the Edgeworth crossover. Above the exact threshold the margin comes from an
    // Edgeworth expansion compared against the misrate, and that comparison decides an
    // integer index which selects an order statistic. Round misrates never sit near it, so a
    // port evaluating a different normal approximation agreed on every fixture while
    // disagreeing in general; R did exactly that until it was unified.
    foreach (var (n, m) in new[] { (201, 200), (150, 150), (100, 300), (250, 60), (400, 400) })
    {
      double crossover = CrossoverMisrate(mr => PairwiseMargin.Instance.Calc(n, m, mr), 1e-4, 0.2);
      if (double.IsNaN(crossover)) continue;
      inputBuilder.Add($"edgeworth-n{n}_m{m}", new PairwiseMarginInput(n, m, crossover));
      double below = BitDecrement(crossover);
      if (below > 0) inputBuilder.Add($"edgeworth-n{n}_m{m}-below", new PairwiseMarginInput(n, m, below));
    }

    // Comprehensive grid, filtered by min_misrate
    // Misrates to test
    double[] misrates = [1e-1, 1e-2, 1e-3, 1e-4, 1e-5, 1e-6];

    // Small sample sizes: all valid combinations of 1 <= n, m <= 5
    int[] smallSizes = [1, 2, 3, 4, 5];

    // Larger sample sizes
    int[] largeSizes = [10, 20, 30, 50, 100];

    foreach (var misrate in misrates)
    {
      // Small samples — skip if misrate below floor
      foreach (var n in smallSizes)
      {
        foreach (var m in smallSizes)
        {
          if (misrate < MinAchievableMisrate.TwoSample(n, m)) continue;
          string testName = $"n{n}_m{m}_mr{FormatMisrate(misrate)}";
          inputBuilder.Add(testName, new PairwiseMarginInput(n, m, misrate));
        }
      }

      // Large samples — skip if misrate below floor
      foreach (var n in largeSizes)
      {
        foreach (var m in largeSizes)
        {
          if (misrate < MinAchievableMisrate.TwoSample(n, m)) continue;
          string testName = $"n{n}_m{m}_r{FormatMisrate(misrate)}";
          inputBuilder.Add(testName, new PairwiseMarginInput(n, m, misrate));
        }
      }
    }

    var controller = new PairwiseMarginController("pairwise-margin");
    var inputs = inputBuilder.Build();
    var testData = controller.GenerateData(inputs);
    controller.Save(testData);
    AnsiConsole.MarkupLine($"  [green]✓[/] Generated [bold]{testData.Count}[/] test cases");
  }

  private static string FormatMisrate(double misrate)
  {
    // Convert 1e-1 to "1", 1e-2 to "2", etc.
    int exponent = -(int)Math.Round(Math.Log10(misrate));
    return exponent.ToString();
  }

  // Bisects for a misrate where the returned margin changes between one representable value
  // and the next, so the fixture sits exactly where the comparison is in doubt.
  static double CrossoverMisrate(Func<double, int> margin, double lo, double hi)
  {
    int marginLo = margin(lo);
    if (marginLo == margin(hi)) return double.NaN;
    for (int i = 0; i < 200; i++)
    {
      double mid = (lo + hi) / 2;
      if (mid <= lo || mid >= hi) break;
      if (margin(mid) == marginLo) lo = mid;
      else hi = mid;
    }
    return hi;
  }

  static double BitDecrement(double v) => BitConverter.Int64BitsToDouble(BitConverter.DoubleToInt64Bits(v) - 1);
}
