using Pragmastat.TestGenerator.Framework.MedianBounds;
using Spectre.Console;

namespace Pragmastat.TestGenerator.TestCases;

public static class MedianBoundsTestCases
{
  public static void Generate()
  {
    const string suiteName = "median-bounds";
    AnsiConsole.MarkupLine($"[yellow]→[/] Generating tests for: [bold]{suiteName}[/]");

    var inputBuilder = new MedianBoundsInputBuilder();

    // Demo examples (n = 5)
    inputBuilder.Add("demo-1", new Sample(1, 2, 3, 4, 5), 0.1);
    inputBuilder.Add("demo-2", new Sample(Enumerable.Range(1, 10).Select(x => (double)x).ToArray()), 0.05);
    inputBuilder.Add("demo-3", new Sample(0, 2, 4, 6, 8), 0.1);

    // Natural sequences
    inputBuilder.Add("natural-5", new Sample(1, 2, 3, 4, 5), 0.1);
    inputBuilder.Add("natural-7", new Sample(1, 2, 3, 4, 5, 6, 7), 0.05);
    inputBuilder.Add("natural-10", new Sample(Enumerable.Range(1, 10).Select(x => (double)x).ToArray()), 0.05);
    inputBuilder.Add("natural-20", new Sample(Enumerable.Range(1, 20).Select(x => (double)x).ToArray()), 0.01);

    // Edge cases (n=1 is domain error, min n is 2)
    inputBuilder.Add("edge-two-elements", new Sample(1, 3), 0.5);
    inputBuilder.Add("edge-three-elements", new Sample(1, 2, 3), 0.5);
    inputBuilder.Add("edge-duplicates", new Sample(5, 5, 5, 5, 5, 5, 5, 5, 5, 5), 0.05);
    inputBuilder.Add("edge-loose-misrate", new Sample(1, 2, 3, 4, 5), 0.5);
    inputBuilder.Add("edge-strict-misrate", new Sample(Enumerable.Range(1, 10).Select(x => (double)x).ToArray()), 0.05);
    inputBuilder.Add("edge-wide-range", new Sample(0.001, 1, 100, 1000, 10000), 0.1);
    inputBuilder.Add("edge-negative", new Sample(-5, -4, -3, -2, -1), 0.1);

    // Asymmetric distributions (no symmetry requirement)
    inputBuilder.Add("asymmetric-right-skew", new Sample(1, 2, 3, 4, 10), 0.1);
    inputBuilder.Add("asymmetric-left-skew", new Sample(1, 7, 8, 9, 10), 0.1);
    inputBuilder.Add("asymmetric-bimodal", new Sample(1, 1, 5, 9, 9), 0.1);
    inputBuilder.Add("asymmetric-outlier", new Sample(1, 2, 3, 4, 100), 0.1);

    // Misrate variation (n=10)
    var misrateSample = new Sample(Enumerable.Range(1, 10).Select(x => (double)x).ToArray());
    inputBuilder.Add("misrate-1e-1", misrateSample, 1e-1);
    inputBuilder.Add("misrate-5e-2", misrateSample, 5e-2);
    inputBuilder.Add("misrate-1e-2", misrateSample, 1e-2);

    // Additive distribution
    inputBuilder.AddAdditive([10, 20], 0.05, count: 1);

    // Uniform distribution
    inputBuilder.AddUniform([10, 20], 0.05, count: 1);

    // Unsorted tests
    inputBuilder.AddUnsorted("reverse-5", new Sample(5, 4, 3, 2, 1), 0.1);
    inputBuilder.AddUnsorted("shuffle-5", new Sample(3, 1, 4, 2, 5), 0.1);
    inputBuilder.AddUnsorted("reverse-7", new Sample(7, 6, 5, 4, 3, 2, 1), 0.05);
    inputBuilder.AddUnsorted("shuffle-7", new Sample(4, 7, 2, 5, 1, 6, 3), 0.05);
    inputBuilder.AddUnsorted("negative-5", new Sample(-1, -3, -2, -5, -4), 0.1);
    inputBuilder.AddUnsorted("mixed-signs-5", new Sample(2, -1, 0, -2, 1), 0.1);

    var controller = new MedianBoundsController(suiteName);
    var inputs = inputBuilder.Build();
    var testData = controller.GenerateData(inputs);
    controller.Save(testData);
    AnsiConsole.MarkupLine($"  [green]✓[/] Generated [bold]{testData.Count}[/] test cases");
  }
}
