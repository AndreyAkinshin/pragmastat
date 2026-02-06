using Pragmastat.TestGenerator.Framework.CenterBoundsApprox;
using Spectre.Console;

namespace Pragmastat.TestGenerator.TestCases;

public static class CenterBoundsApproxTestCases
{
  public static void Generate()
  {
    const string suiteName = "center-bounds-approx";
    AnsiConsole.MarkupLine($"[yellow]→[/] Generating tests for: [bold]{suiteName}[/]");

    var inputBuilder = new CenterBoundsApproxInputBuilder();

    // Demo examples with seeds for reproducibility - 3 tests
    inputBuilder.AddSeeded("demo-1", new Sample(1, 2, 3, 4, 5), 0.05, "demo-seed-1");
    inputBuilder.AddSeeded("demo-2", new Sample(1, 2, 3, 4, 5), 0.01, "demo-seed-2");
    inputBuilder.AddSeeded("demo-3", new Sample(0, 2, 4, 6, 8), 0.05, "demo-seed-3");

    // Natural sequences with seeds - 3 tests
    inputBuilder.AddNaturalSeeded([5, 10, 20], 0.05, "natural");

    // Edge cases with seeds (n=1 is domain error, min n is 2) - 5 tests
    inputBuilder.AddSeeded("edge-two-elements", new Sample(1, 3), 0.5, "edge-two");
    inputBuilder.AddSeeded("edge-three-elements", new Sample(1, 2, 3), 0.25, "edge-three");
    inputBuilder.AddSeeded("edge-duplicates", new Sample(5, 5, 5, 5, 5), 0.05, "edge-dup");
    inputBuilder.AddSeeded("edge-loose-misrate", new Sample(1, 2, 3, 4, 5), 0.5, "edge-loose");
    inputBuilder.AddSeeded("edge-negative", new Sample(-5, -4, -3, -2, -1), 0.05, "edge-neg");

    // Asymmetric distributions (bootstrap doesn't require symmetry) - 4 tests
    inputBuilder.AddSeeded("asymmetric-right-skew", new Sample(1, 2, 3, 4, 10), 0.05, "asym-right");
    inputBuilder.AddSeeded("asymmetric-left-skew", new Sample(1, 7, 8, 9, 10), 0.05, "asym-left");
    inputBuilder.AddSeeded("asymmetric-bimodal", new Sample(1, 1, 5, 9, 9), 0.05, "asym-bimodal");
    inputBuilder.AddSeeded("asymmetric-outlier", new Sample(1, 2, 3, 4, 100), 0.05, "asym-outlier");

    // Misrate variation - 4 tests
    var misrateSample = new Sample(0, 2, 4, 6, 8, 10);
    inputBuilder.AddSeeded("misrate-1e-1", misrateSample, 1e-1, "misrate-1");
    inputBuilder.AddSeeded("misrate-5e-2", misrateSample, 5e-2, "misrate-2");
    inputBuilder.AddSeeded("misrate-1e-2", misrateSample, 1e-2, "misrate-3");
    inputBuilder.AddSeeded("misrate-1e-3", misrateSample, 1e-3, "misrate-4");

    // Additive distribution with seeds - 3 tests
    inputBuilder.AddAdditiveSeeded([5, 10, 20], 0.05, "additive", count: 1);

    // Uniform distribution with seeds - 3 tests
    inputBuilder.AddUniformSeeded([5, 10, 20], 0.05, "uniform", count: 1);

    // Property tests - algebraic invariants verified with same seed for comparability
    inputBuilder.AddSeeded("property-determinism", new Sample(1, 2, 3, 4, 5, 6, 7, 8, 9, 10), 0.05, "determinism-test");
    inputBuilder.AddSeeded("property-permutation-invariance", new Sample(7, 3, 10, 1, 5, 9, 2, 6, 4, 8), 0.05, "determinism-test");
    inputBuilder.AddSeeded("property-base", new Sample(1, 2, 3, 4, 5), 0.05, "determinism-test");
    inputBuilder.AddSeeded("property-location-shift", new Sample(11, 12, 13, 14, 15), 0.05, "determinism-test");
    inputBuilder.AddSeeded("property-scale-2x", new Sample(2, 4, 6, 8, 10), 0.05, "determinism-test");

    var controller = new CenterBoundsApproxController(suiteName);
    var inputs = inputBuilder.Build();
    var testData = controller.GenerateData(inputs);
    controller.Save(testData);
    AnsiConsole.MarkupLine($"  [green]✓[/] Generated [bold]{testData.Count}[/] test cases");
  }
}
