using Pragmastat.TestGenerator.Framework.CenterBounds;
using Spectre.Console;

namespace Pragmastat.TestGenerator.TestCases;

public static class CenterBoundsTestCases
{
  public static void Generate()
  {
    const string suiteName = "center-bounds";
    AnsiConsole.MarkupLine($"[yellow]→[/] Generating tests for: [bold]{suiteName}[/]");

    var inputBuilder = new CenterBoundsInputBuilder();

    // Demo examples (n = 5, min misrate ~0.0625)
    inputBuilder.Add("demo-1", new Sample(1, 2, 3, 4, 5), 0.1);
    inputBuilder.Add("demo-2", new Sample(1, 2, 3, 4, 5, 6, 7, 8, 9, 10), 0.01);
    inputBuilder.Add("demo-3", new Sample(0, 2, 4, 6, 8), 0.1);

    // Natural sequences with achievable misrates
    // n=5: min=0.0625, n=7: min~0.016, n=10: min~0.002, n=20: min~2e-6
    inputBuilder.Add("natural-5", new Sample(1, 2, 3, 4, 5), 0.1);
    inputBuilder.Add("natural-7", new Sample(1, 2, 3, 4, 5, 6, 7), 0.05);
    inputBuilder.Add("natural-10", new Sample(Enumerable.Range(1, 10).Select(x => (double)x).ToArray()), 0.01);
    inputBuilder.Add("natural-20", new Sample(Enumerable.Range(1, 20).Select(x => (double)x).ToArray()), 0.01);

    // Symmetric distributions with achievable misrates
    inputBuilder.Add("symmetric-5", new Sample(-2, -1, 0, 1, 2), 0.1);
    inputBuilder.Add("symmetric-7", new Sample(-3, -2, -1, 0, 1, 2, 3), 0.05);
    inputBuilder.Add("symmetric-10", new Sample(-4.5, -3.5, -2.5, -1.5, -0.5, 0.5, 1.5, 2.5, 3.5, 4.5), 0.01);
    inputBuilder.Add("symmetric-15", new Sample(Enumerable.Range(-7, 15).Select(x => (double)x).ToArray()), 0.01);

    // Edge cases (n=1 is domain error, min n is 2)
    inputBuilder.Add("edge-two-elements", new Sample(1, 3), 0.5);
    inputBuilder.Add("edge-three-elements", new Sample(1, 2, 3), 0.25);
    inputBuilder.Add("edge-duplicates-10", new Sample(5, 5, 5, 5, 5, 5, 5, 5, 5, 5), 0.01);
    inputBuilder.Add("edge-loose-misrate", new Sample(1, 2, 3, 4, 5), 0.5);
    inputBuilder.Add("edge-strict-misrate", new Sample(Enumerable.Range(1, 10).Select(x => (double)x).ToArray()), 0.01);
    inputBuilder.Add("edge-wide-range", new Sample(0.001, 1, 100, 1000, 10000), 0.1);
    inputBuilder.Add("edge-negative", new Sample(-5, -4, -3, -2, -1), 0.1);

    // Property validation (n=5, use 0.1 misrate which is > 0.0625)
    inputBuilder.Add("property-identity", new Sample(0, 2, 4, 6, 8), 0.1);
    inputBuilder.Add("property-location-shift", new Sample(10, 12, 14, 16, 18), 0.1);
    inputBuilder.Add("property-scale-2x", new Sample(2, 4, 6, 8, 10), 0.1);
    inputBuilder.Add("property-mixed-signs", new Sample(-2, -1, 0, 1, 2), 0.1);
    inputBuilder.Add("property-centered", new Sample(-3, -1, 0, 1, 3), 0.1);

    // Misrate variation (n=10, min misrate ~0.002)
    var misrateSample = new Sample(Enumerable.Range(1, 10).Select(x => (double)x).ToArray());
    inputBuilder.Add("misrate-1e-1", misrateSample, 1e-1);
    inputBuilder.Add("misrate-5e-2", misrateSample, 5e-2);
    inputBuilder.Add("misrate-1e-2", misrateSample, 1e-2);
    inputBuilder.Add("misrate-5e-3", misrateSample, 5e-3);

    // Additive distribution (use larger sizes for tighter misrates)
    inputBuilder.AddAdditive([10, 20], 0.05, count: 1);

    // Uniform distribution
    inputBuilder.AddUniform([10, 20], 0.05, count: 1);

    // Unsorted tests (n=5 needs misrate >= 0.0625, n=7 needs misrate >= 0.016)
    inputBuilder.AddUnsorted("reverse-5", new Sample(5, 4, 3, 2, 1), 0.1);
    inputBuilder.AddUnsorted("shuffle-5", new Sample(3, 1, 4, 2, 5), 0.1);
    inputBuilder.AddUnsorted("reverse-7", new Sample(7, 6, 5, 4, 3, 2, 1), 0.05);
    inputBuilder.AddUnsorted("shuffle-7", new Sample(4, 7, 2, 5, 1, 6, 3), 0.05);
    inputBuilder.AddUnsorted("negative-5", new Sample(-1, -3, -2, -5, -4), 0.1);
    inputBuilder.AddUnsorted("mixed-signs-5", new Sample(2, -1, 0, -2, 1), 0.1);

    var controller = new CenterBoundsController(suiteName);
    var inputs = inputBuilder.Build();
    var testData = controller.GenerateData(inputs);
    controller.Save(testData);
    AnsiConsole.MarkupLine($"  [green]✓[/] Generated [bold]{testData.Count}[/] test cases");
  }
}
