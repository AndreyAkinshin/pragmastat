using Pragmastat.TestGenerator.Framework;
using Pragmastat.TestGenerator.Framework.SignedRankMargin;
using Spectre.Console;

namespace Pragmastat.TestGenerator.TestCases;

public static class SignedRankMarginTestCases
{
  public static void Generate()
  {
    const string suiteName = "signed-rank-margin";
    AnsiConsole.MarkupLine($"[yellow]→[/] Generating tests for: [bold]{suiteName}[/]");

    var inputBuilder = new ReferenceTestCaseInputBuilder<SignedRankMarginInput>();

    // Demo examples - 4 tests
    inputBuilder.Add("demo-1", new SignedRankMarginInput(30, 1e-6));
    inputBuilder.Add("demo-2", new SignedRankMarginInput(30, 1e-5));
    inputBuilder.Add("demo-3", new SignedRankMarginInput(30, 1e-4));
    inputBuilder.Add("demo-4", new SignedRankMarginInput(30, 1e-3));

    // Small sample sizes (exact computation) - only use achievable misrates
    // Min achievable misrate = 2^(1-n), so:
    // n=2: min=0.5, n=3: min=0.25, n=4: min=0.125, n=5: min~0.0625, n=6: min~0.031, n=10: min~0.002
    AddWithValidMisrates(inputBuilder, "exact", [2, 3, 4, 5, 6, 10], [1e-1, 5e-2, 1e-2, 5e-3, 1e-3]);

    // Edge cases - 6 tests
    inputBuilder.Add("boundary-n2-min", new SignedRankMarginInput(2, 0.5));
    inputBuilder.Add("boundary-n3-min", new SignedRankMarginInput(3, 0.25));
    inputBuilder.Add("boundary-n4-min", new SignedRankMarginInput(4, 0.125));
    inputBuilder.Add("boundary-loose", new SignedRankMarginInput(5, 0.9));
    inputBuilder.Add("boundary-tight", new SignedRankMarginInput(10, 0.01));
    inputBuilder.Add("boundary-very-tight", new SignedRankMarginInput(20, 0.001));

    // Larger sample sizes (still exact for n <= 250) - all misrates achievable for these sizes
    int[] mediumSizes = [15, 20, 30, 50, 100];
    double[] mediumMisrates = [1e-1, 1e-2, 1e-3, 1e-4];

    foreach (var misrate in mediumMisrates)
    {
      foreach (var n in mediumSizes)
      {
        string testName = $"medium-n{n}-mr{FormatMisrate(misrate)}";
        inputBuilder.Add(testName, new SignedRankMarginInput(n, misrate));
      }
    }

    var controller = new SignedRankMarginController(suiteName);
    var inputs = inputBuilder.Build();
    var testData = controller.GenerateData(inputs);
    controller.Save(testData);
    AnsiConsole.MarkupLine($"  [green]✓[/] Generated [bold]{testData.Count}[/] test cases");
  }

  private static void AddWithValidMisrates(
    ReferenceTestCaseInputBuilder<SignedRankMarginInput> inputBuilder,
    string prefix,
    int[] sizes,
    double[] misrates)
  {
    foreach (var n in sizes)
    {
      double minMisrate = Math.Pow(2, 1 - n);
      foreach (var misrate in misrates)
      {
        if (misrate >= minMisrate)
        {
          string testName = $"{prefix}-n{n}-mr{FormatMisrate(misrate)}";
          inputBuilder.Add(testName, new SignedRankMarginInput(n, misrate));
        }
      }
    }
  }

  private static string FormatMisrate(double misrate)
  {
    if (Math.Abs(misrate - 0.1) < 1e-9)
      return "1e1";
    if (Math.Abs(misrate - 0.05) < 1e-9)
      return "5e2";
    if (Math.Abs(misrate - 0.01) < 1e-9)
      return "1e2";
    if (Math.Abs(misrate - 0.005) < 1e-9)
      return "5e3";
    if (Math.Abs(misrate - 0.001) < 1e-9)
      return "1e3";
    if (Math.Abs(misrate - 0.0001) < 1e-9)
      return "1e4";
    if (Math.Abs(misrate - 0.00001) < 1e-9)
      return "1e5";
    if (Math.Abs(misrate - 0.000001) < 1e-9)
      return "1e6";

    int exponent = -(int)Math.Round(Math.Log10(misrate));
    return $"e{exponent}";
  }
}
