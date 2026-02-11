using Pragmastat.TestGenerator.Framework.SpreadBounds;
using Spectre.Console;

namespace Pragmastat.TestGenerator.TestCases;

public static class SpreadBoundsTestCases
{
  public static void Generate()
  {
    const string suiteName = "spread-bounds";
    AnsiConsole.MarkupLine($"[yellow]→[/] Generating tests for: [bold]{suiteName}[/]");

    var inputBuilder = new SpreadBoundsInputBuilder();

    // Demo examples
    inputBuilder.Add("demo-1", new Sample(Enumerable.Range(1, 30).Select(x => (double)x).ToArray()), 0.01);
    inputBuilder.Add("demo-2", new Sample(Enumerable.Range(1, 30).Select(x => (double)x).ToArray()), 0.002);
    inputBuilder.Add("demo-3", new Sample(Enumerable.Range(1, 15).Select(x => (double)x).ToArray()), 0.07);

    // Natural sequences (misrate varies by size to satisfy minimum constraints)
    inputBuilder.AddNatural([10], 0.15);
    inputBuilder.AddNatural([15], 0.05);
    inputBuilder.AddNatural([20, 30], 0.05);

    // Property validation (n=10, misrate=0.2)
    inputBuilder.Add("property-identity", new Sample(1, 2, 3, 4, 5, 6, 7, 8, 9, 10), 0.2);
    inputBuilder.Add("property-location-shift", new Sample(11, 12, 13, 14, 15, 16, 17, 18, 19, 20), 0.2);
    inputBuilder.Add("property-scale-2x", new Sample(2, 4, 6, 8, 10, 12, 14, 16, 18, 20), 0.2);
    inputBuilder.Add("property-scale-neg", new Sample(-10, -9, -8, -7, -6, -5, -4, -3, -2, -1), 0.2);

    // Edge cases
    inputBuilder.Add("edge-small-non-trivial", new Sample(1, 2, 3, 4, 5), 0.8);
    inputBuilder.Add("edge-large-misrate", new Sample(Enumerable.Range(1, 10).Select(x => (double)x).ToArray()), 0.5);
    inputBuilder.Add("edge-duplicates-mixed", new Sample(1, 1, 1, 2, 3, 4, 5), 0.5);
    inputBuilder.Add("edge-wide-range", new Sample(1, 10, 100, 1000, 10000), 0.8);
    inputBuilder.Add("edge-negative", new Sample(-5, -4, -3, -2, -1), 0.8);
    inputBuilder.Add("edge-large-n", new Sample(Enumerable.Range(1, 100).Select(x => (double)x).ToArray()), 0.01);
    inputBuilder.Add("edge-n2", new Sample(1, 3), 1.0);

    // Additive distribution (misrate varies by size)
    inputBuilder.AddAdditive([20], 0.02, count: 1);
    inputBuilder.AddAdditive([30, 50], 0.01, count: 1);

    // Uniform distribution (misrate varies by size)
    inputBuilder.AddUniform([20], 0.02, count: 1);
    inputBuilder.AddUniform([50], 0.01, count: 1);

    // Misrate variation (x = [1..30])
    var misrateSample = new Sample(Enumerable.Range(1, 25).Select(x => (double)x).ToArray());
    inputBuilder.Add("misrate-5e-1", misrateSample, 0.5);
    inputBuilder.Add("misrate-1e-1", misrateSample, 0.1);
    inputBuilder.Add("misrate-5e-2", misrateSample, 0.05);
    inputBuilder.Add("misrate-1e-2", misrateSample, 0.01);
    inputBuilder.Add("misrate-2e-3", misrateSample, 0.002);

    // Conservatism tests (misrate = 0.1)
    inputBuilder.Add("conservatism-12", new Sample(Enumerable.Range(1, 12).Select(x => (double)x).ToArray()), 0.1);
    inputBuilder.Add("conservatism-15", new Sample(Enumerable.Range(1, 15).Select(x => (double)x).ToArray()), 0.1);
    inputBuilder.Add("conservatism-20", new Sample(Enumerable.Range(1, 20).Select(x => (double)x).ToArray()), 0.1);
    inputBuilder.Add("conservatism-30", new Sample(Enumerable.Range(1, 30).Select(x => (double)x).ToArray()), 0.1);
    inputBuilder.Add("conservatism-50", new Sample(Enumerable.Range(1, 50).Select(x => (double)x).ToArray()), 0.1);

    // Unsorted tests
    inputBuilder.AddUnsorted("reverse-10", new Sample(10, 9, 8, 7, 6, 5, 4, 3, 2, 1), 0.2);
    inputBuilder.AddUnsorted("reverse-15", new Sample(Enumerable.Range(1, 15).Reverse().Select(x => (double)x).ToArray()), 0.07);
    inputBuilder.AddUnsorted("shuffle-10", new Sample(5, 8, 2, 10, 3, 7, 1, 9, 4, 6), 0.2);
    inputBuilder.AddUnsorted("shuffle-15", new Sample(8, 3, 12, 1, 15, 6, 10, 4, 14, 2, 11, 7, 13, 5, 9), 0.07);
    inputBuilder.AddUnsorted("negative-5", new Sample(-1, -3, -2, -5, -4), 0.8);
    inputBuilder.AddUnsorted("mixed-signs-5", new Sample(2, -1, 0, -2, 1), 0.8);
    inputBuilder.AddUnsorted("duplicates", new Sample(1, 3, 1, 3, 2), 0.8);
    inputBuilder.AddUnsorted("wide-range", new Sample(1000, 1, 100, 10, 10000), 0.8);

    var controller = new SpreadBoundsController(suiteName);
    var inputs = inputBuilder.Build();
    var testData = controller.GenerateData(inputs);
    controller.Save(testData);
    AnsiConsole.MarkupLine($"  [green]✓[/] Generated [bold]{testData.Count}[/] test cases");
  }
}
