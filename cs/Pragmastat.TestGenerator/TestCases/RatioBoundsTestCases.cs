using Pragmastat.TestGenerator.Framework.RatioBounds;
using Spectre.Console;

namespace Pragmastat.TestGenerator.TestCases;

public static class RatioBoundsTestCases
{
  public static void Generate()
  {
    const string suiteName = "ratio-bounds";
    AnsiConsole.MarkupLine($"[yellow]->[/] Generating tests for: [bold]{suiteName}[/]");

    var inputBuilder = new RatioBoundsInputBuilder();

    // Demo examples (n = m = 5, positive samples) - 3 tests
    inputBuilder.Add("demo-1", new Sample(1, 2, 3, 4, 5), new Sample(2, 3, 4, 5, 6), 0.05);
    inputBuilder.Add("demo-2", new Sample(1, 2, 3, 4, 5), new Sample(2, 3, 4, 5, 6), 0.01);
    inputBuilder.Add("demo-3", new Sample(2, 3, 4, 5, 6), new Sample(2, 3, 4, 5, 6), 0.05);

    // Natural sequences (9 combinations: [5,8,10] x [5,8,10], misrate = 1e-2) - 9 tests
    inputBuilder.AddNatural([5, 8, 10], [5, 8, 10], 1e-2);

    // Property validation (n = m = 10, misrate = 1e-3) - 6 tests
    inputBuilder.Add("property-identity", new Sample(1, 2, 3, 4, 5, 6, 7, 8, 9, 10), new Sample(1, 2, 3, 4, 5, 6, 7, 8, 9, 10), 1e-3);
    inputBuilder.Add("property-scale-2x", new Sample(2, 4, 6, 8, 10, 12, 14, 16, 18, 20), new Sample(1, 2, 3, 4, 5, 6, 7, 8, 9, 10), 1e-3);
    inputBuilder.Add("property-reciprocal", new Sample(1, 2, 3, 4, 5, 6, 7, 8, 9, 10), new Sample(2, 4, 6, 8, 10, 12, 14, 16, 18, 20), 1e-3);
    inputBuilder.Add("property-common-scale", new Sample(10, 20, 30, 40, 50, 60, 70, 80, 90, 100), new Sample(20, 40, 60, 80, 100, 120, 140, 160, 180, 200), 1e-3);
    inputBuilder.Add("property-small-values", new Sample(0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9, 1.0), new Sample(0.2, 0.4, 0.6, 0.8, 1.0, 1.2, 1.4, 1.6, 1.8, 2.0), 1e-3);
    inputBuilder.Add("property-mixed-scales", new Sample(0.01, 0.1, 1, 10, 100, 1000, 0.5, 5, 50, 500), new Sample(0.1, 1, 10, 100, 1000, 10000, 5, 50, 500, 5000), 1e-3);

    // Edge cases - 10 tests
    inputBuilder.Add("edge-min-samples", new Sample(2, 3, 4, 5, 6), new Sample(3, 4, 5, 6, 7), 0.05);
    inputBuilder.Add("edge-permissive-misrate", new Sample(1, 2, 3, 4, 5), new Sample(2, 3, 4, 5, 6), 0.5);
    inputBuilder.Add("edge-strict-misrate", new Sample(Enumerable.Range(1, 20).Select(x => (double)x).ToArray()),
      new Sample(Enumerable.Range(2, 20).Select(x => (double)x).ToArray()), 1e-6);
    inputBuilder.Add("edge-unity-ratio", new Sample(5, 5, 5, 5, 5, 5, 5, 5, 5, 5), new Sample(5, 5, 5, 5, 5, 5, 5, 5, 5, 5), 1e-3);
    inputBuilder.Add("edge-asymmetric-3-100", new Sample(50, 51, 52),
      new Sample(Enumerable.Range(1, 100).Select(x => (double)x).ToArray()), 1e-2);
    inputBuilder.Add("edge-asymmetric-5-50", new Sample(25, 26, 27, 28, 29),
      new Sample(Enumerable.Range(1, 50).Select(x => (double)x).ToArray()), 1e-3);
    inputBuilder.Add("edge-duplicates", new Sample(3, 3, 3, 3, 3), new Sample(5, 5, 5, 5, 5), 1e-2);
    inputBuilder.Add("edge-wide-range", new Sample(0.001, 0.01, 0.1, 1, 10, 100, 1000, 10000, 100000, 1000000),
      new Sample(0.1, 1, 10, 100, 1000, 10000, 100000, 1000000, 10000000, 100000000), 1e-3);
    inputBuilder.Add("edge-tiny-values", new Sample(1e-6, 2e-6, 3e-6, 4e-6, 5e-6, 6e-6, 7e-6, 8e-6, 9e-6, 10e-6),
      new Sample(2e-6, 3e-6, 4e-6, 5e-6, 6e-6, 7e-6, 8e-6, 9e-6, 10e-6, 11e-6), 1e-3);
    inputBuilder.Add("edge-large-values", new Sample(1e8, 2e8, 3e8, 4e8, 5e8, 6e8, 7e8, 8e8, 9e8, 10e8),
      new Sample(2e8, 3e8, 4e8, 5e8, 6e8, 7e8, 8e8, 9e8, 10e8, 11e8), 1e-3);

    // Multiplicative (log-normal) distribution (9 combinations: [10,30,50] x [10,30,50], misrate = 1e-3) - 9 tests
    inputBuilder.AddMultiplic([10, 30, 50], [10, 30, 50], 1e-3, count: 1);

    // Uniform distribution (4 combinations: [10,100] x [10,100], misrate = 1e-4, positive range) - 4 tests
    inputBuilder.AddUniform([10, 100], [10, 100], 1e-4, count: 1);

    // Misrate variation (n = m = 20) - 5 tests
    var misrateX = new Sample(Enumerable.Range(1, 20).Select(x => (double)x).ToArray());
    var misrateY = new Sample(Enumerable.Range(1, 20).Select(x => x * 2.0).ToArray());
    inputBuilder.Add("misrate-1e-2", misrateX, misrateY, 1e-2);
    inputBuilder.Add("misrate-1e-3", misrateX, misrateY, 1e-3);
    inputBuilder.Add("misrate-1e-4", misrateX, misrateY, 1e-4);
    inputBuilder.Add("misrate-1e-5", misrateX, misrateY, 1e-5);
    inputBuilder.Add("misrate-1e-6", misrateX, misrateY, 1e-6);

    // Unsorted tests (positive values only) - 15 tests
    inputBuilder.AddUnsorted("x-natural-5-5", new Sample(5, 3, 1, 4, 2), new Sample(1, 2, 3, 4, 5), 1e-2);
    inputBuilder.AddUnsorted("y-natural-5-5", new Sample(1, 2, 3, 4, 5), new Sample(5, 3, 1, 4, 2), 1e-2);
    inputBuilder.AddUnsorted("both-natural-5-5", new Sample(5, 3, 1, 4, 2), new Sample(5, 3, 1, 4, 2), 1e-2);
    inputBuilder.AddUnsorted("x-shuffle-5-5", new Sample(3, 1, 5, 4, 2), new Sample(1, 2, 3, 4, 5), 1e-2);
    inputBuilder.AddUnsorted("y-shuffle-5-5", new Sample(1, 2, 3, 4, 5), new Sample(4, 2, 5, 1, 3), 1e-2);
    inputBuilder.AddUnsorted("both-shuffle-5-5", new Sample(3, 1, 5, 4, 2), new Sample(2, 4, 1, 5, 3), 1e-2);
    inputBuilder.AddUnsorted("demo-unsorted-x", new Sample(5, 1, 4, 2, 3), new Sample(2, 3, 4, 5, 6), 0.05);
    inputBuilder.AddUnsorted("demo-unsorted-y", new Sample(1, 2, 3, 4, 5), new Sample(6, 2, 5, 3, 4), 0.05);
    inputBuilder.AddUnsorted("demo-both-unsorted", new Sample(4, 1, 5, 2, 3), new Sample(5, 2, 6, 3, 4), 0.05);
    inputBuilder.AddUnsorted("identity-unsorted", new Sample(4, 1, 5, 2, 3), new Sample(5, 1, 4, 3, 2), 1e-2);
    inputBuilder.AddUnsorted("scale-unsorted", new Sample(10, 30, 20), new Sample(15, 5, 10), 0.5);
    inputBuilder.AddUnsorted("asymmetric-5-10", new Sample(2, 5, 1, 3, 4), new Sample(10, 5, 2, 8, 4, 1, 9, 3, 7, 6), 1e-2);
    inputBuilder.AddUnsorted("duplicates", new Sample(3, 3, 3, 3, 3), new Sample(5, 5, 5, 5, 5), 1e-2);
    inputBuilder.AddUnsorted("mixed-duplicates-x", new Sample(2, 1, 3, 2, 1), new Sample(1, 1, 2, 2, 3), 1e-2);
    inputBuilder.AddUnsorted("mixed-duplicates-y", new Sample(1, 1, 2, 2, 3), new Sample(3, 2, 1, 3, 2), 1e-2);

    var controller = new RatioBoundsController(suiteName);
    var inputs = inputBuilder.Build();
    var testData = controller.GenerateData(inputs);
    controller.Save(testData);
    AnsiConsole.MarkupLine($"  [green]+[/] Generated [bold]{testData.Count}[/] test cases");
  }
}
