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

    // Natural sequences (32 tests)
    // [n, m] ∈ {1, 2, 3, 4} × {1, 2, 3, 4} × 2 misrates
    int[] naturalSizes = [1, 2, 3, 4];
    double[] naturalMisrates = [1e-1, 1e-2];
    foreach (var misrate in naturalMisrates)
    {
      foreach (var n in naturalSizes)
      {
        foreach (var m in naturalSizes)
        {
          string testName = $"natural-{n}-{m}-mr{FormatMisrate(misrate)}";
          inputBuilder.Add(testName, new PairwiseMarginInput(n, m, misrate));
        }
      }
    }

    // Edge cases (10 tests)
    inputBuilder.Add("boundary-min", new PairwiseMarginInput(1, 1, 0.5));
    inputBuilder.Add("boundary-zero-margin-small", new PairwiseMarginInput(2, 2, 1e-6));
    inputBuilder.Add("boundary-loose", new PairwiseMarginInput(5, 5, 0.9));
    inputBuilder.Add("symmetry-2-5", new PairwiseMarginInput(2, 5, 0.1));
    inputBuilder.Add("symmetry-5-2", new PairwiseMarginInput(5, 2, 0.1));
    inputBuilder.Add("symmetry-3-7", new PairwiseMarginInput(3, 7, 0.05));
    inputBuilder.Add("symmetry-7-3", new PairwiseMarginInput(7, 3, 0.05));
    inputBuilder.Add("asymmetry-extreme-1-100", new PairwiseMarginInput(1, 100, 0.1));
    inputBuilder.Add("asymmetry-extreme-100-1", new PairwiseMarginInput(100, 1, 0.1));
    inputBuilder.Add("asymmetry-extreme-2-50", new PairwiseMarginInput(2, 50, 0.05));

    // Comprehensive grid (300 tests)
    // Misrates to test
    double[] misrates = [1e-1, 1e-2, 1e-3, 1e-4, 1e-5, 1e-6];

    // Small sample sizes: all combinations of 1 <= n, m <= 5 (150 tests)
    int[] smallSizes = [1, 2, 3, 4, 5];

    // Larger sample sizes (150 tests)
    int[] largeSizes = [10, 20, 30, 50, 100];

    foreach (var misrate in misrates)
    {
      // Small samples
      foreach (var n in smallSizes)
      {
        foreach (var m in smallSizes)
        {
          string testName = $"n{n}_m{m}_mr{FormatMisrate(misrate)}";
          inputBuilder.Add(testName, new PairwiseMarginInput(n, m, misrate));
        }
      }

      // Large samples
      foreach (var n in largeSizes)
      {
        foreach (var m in largeSizes)
        {
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
}
