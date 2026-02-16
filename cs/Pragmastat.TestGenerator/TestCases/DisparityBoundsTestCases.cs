using Pragmastat.TestGenerator.Framework.DisparityBounds;
using Spectre.Console;

namespace Pragmastat.TestGenerator.TestCases;

public static class DisparityBoundsTestCases
{
  public static void Generate()
  {
    const string suiteName = "disparity-bounds";
    AnsiConsole.MarkupLine($"[yellow]→[/] Generating tests for: [bold]{suiteName}[/]");

    var inputBuilder = new DisparityBoundsInputBuilder();

    // Demo examples
    inputBuilder.Add("demo-1",
      new Sample(Enumerable.Range(1, 30).Select(x => (double)x).ToArray()),
      new Sample(Enumerable.Range(21, 30).Select(x => (double)x).ToArray()),
      0.02);
    inputBuilder.Add("demo-2",
      new Sample(Enumerable.Range(1, 30).Select(x => (double)x).ToArray()),
      new Sample(Enumerable.Range(21, 30).Select(x => (double)x).ToArray()),
      0.005);
    inputBuilder.Add("demo-3",
      new Sample(Enumerable.Range(1, 20).Select(x => (double)x).ToArray()),
      new Sample(Enumerable.Range(5, 20).Select(x => (double)x).ToArray()),
      0.05);

    // Natural sequences
    inputBuilder.AddNatural([10, 15], [10, 15], 0.2);
    inputBuilder.AddNatural([20], [20], 0.1);

    // Property validation (n = m = 10, misrate = 0.2)
    var baseX = new Sample(0, 2, 4, 6, 8, 10, 12, 14, 16, 18);
    var baseY = new Sample(2, 4, 6, 8, 10, 12, 14, 16, 18, 20);
    inputBuilder.Add("property-identity", baseX, baseX, 0.2);
    inputBuilder.Add("property-location-shift", new Sample(baseX.Values.Select(v => v + 10).ToArray()),
      new Sample(baseY.Values.Select(v => v + 10).ToArray()), 0.2);
    inputBuilder.Add("property-scale-2x", new Sample(baseX.Values.Select(v => v * 2).ToArray()),
      new Sample(baseY.Values.Select(v => v * 2).ToArray()), 0.2);
    inputBuilder.Add("property-scale-neg", new Sample(baseX.Values.Select(v => -v).ToArray()),
      new Sample(baseY.Values.Select(v => -v).ToArray()), 0.2);
    inputBuilder.Add("property-symmetry", new Sample(Enumerable.Range(1, 10).Select(x => (double)x).ToArray()),
      new Sample(Enumerable.Range(6, 10).Select(x => (double)x).ToArray()), 0.2);
    inputBuilder.Add("property-symmetry-swapped", new Sample(Enumerable.Range(6, 10).Select(x => (double)x).ToArray()),
      new Sample(Enumerable.Range(1, 10).Select(x => (double)x).ToArray()), 0.2);

    // Edge cases
    inputBuilder.Add("edge-small", new Sample(1, 2, 3, 4, 5, 6), new Sample(2, 3, 4, 5, 6, 7), 0.6);
    inputBuilder.Add("edge-wide-range",
      new Sample(1, 10, 100, 1000, 10000, 100000, 1000000, 10000000, 100000000, 1000000000),
      new Sample(2, 20, 200, 2000, 20000, 200000, 2000000, 20000000, 200000000, 2000000000),
      0.2);
    inputBuilder.Add("edge-negative",
      new Sample(-10, -9, -8, -7, -6, -5, -4, -3, -2, -1),
      new Sample(-20, -19, -18, -17, -16, -15, -14, -13, -12, -11),
      0.2);
    inputBuilder.Add("edge-mixed-signs",
      new Sample(-9, -7, -5, -3, -1, 1, 3, 5, 7, 9),
      new Sample(-8, -6, -4, -2, 0, 2, 4, 6, 8, 10),
      0.2);
    inputBuilder.Add("edge-asymmetric-10-20",
      new Sample(Enumerable.Range(1, 10).Select(x => (double)x).ToArray()),
      new Sample(Enumerable.Range(1, 20).Select(x => (double)x).ToArray()),
      0.2);

    // Misrate variation (n = m = 20)
    var misrateX = new Sample(Enumerable.Range(1, 20).Select(x => (double)x).ToArray());
    var misrateY = new Sample(Enumerable.Range(5, 20).Select(x => (double)x).ToArray());
    inputBuilder.Add("misrate-2e-1", misrateX, misrateY, 0.2);
    inputBuilder.Add("misrate-1e-1", misrateX, misrateY, 0.1);
    inputBuilder.Add("misrate-5e-2", misrateX, misrateY, 0.05);
    inputBuilder.Add("misrate-2e-2", misrateX, misrateY, 0.02);
    inputBuilder.Add("misrate-1e-2", misrateX, misrateY, 0.01);

    // Additive and uniform distributions (single sample each for stability)
    inputBuilder.AddAdditive([20], [20], 0.05, count: 1);
    inputBuilder.AddUniform([20], [20], 0.05, count: 1);

    // Unsorted tests
    inputBuilder.AddUnsorted("reverse-x", new Sample(10, 9, 8, 7, 6, 5, 4, 3, 2, 1),
      new Sample(Enumerable.Range(1, 10).Select(x => (double)x).ToArray()), 0.2);
    inputBuilder.AddUnsorted("reverse-y", new Sample(Enumerable.Range(1, 10).Select(x => (double)x).ToArray()),
      new Sample(10, 9, 8, 7, 6, 5, 4, 3, 2, 1), 0.2);
    inputBuilder.AddUnsorted("reverse-both", new Sample(10, 9, 8, 7, 6, 5, 4, 3, 2, 1),
      new Sample(10, 9, 8, 7, 6, 5, 4, 3, 2, 1), 0.2);
    inputBuilder.AddUnsorted("shuffle-x", new Sample(5, 1, 3, 2, 4, 10, 6, 9, 7, 8),
      new Sample(Enumerable.Range(1, 10).Select(x => (double)x).ToArray()), 0.2);
    inputBuilder.AddUnsorted("shuffle-y", new Sample(Enumerable.Range(1, 10).Select(x => (double)x).ToArray()),
      new Sample(6, 2, 9, 1, 10, 3, 8, 4, 7, 5), 0.2);
    inputBuilder.AddUnsorted("wide-range",
      new Sample(1000, 1, 100, 10, 10000, 100000, 1000000, 10000000, 100000000, 1000000000),
      new Sample(10000, 100, 1, 1000, 10, 100000, 1000000, 10000000, 100000000, 1000000000),
      0.2);

    var controller = new DisparityBoundsController(suiteName);
    var inputs = inputBuilder.Build();
    var testData = controller.GenerateData(inputs);
    controller.Save(testData);
    AnsiConsole.MarkupLine($"  [green]✓[/] Generated [bold]{testData.Count}[/] test cases");
  }
}
