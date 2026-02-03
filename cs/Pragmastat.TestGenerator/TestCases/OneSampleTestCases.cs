using Pragmastat.TestGenerator.Framework.OneSample;
using Spectre.Console;

namespace Pragmastat.TestGenerator.TestCases;

public static class OneSampleTestCases
{
  public static void Generate()
  {
    // Center: 38 test cases (24 original + 14 unsorted)
    // Note: Performance tests (perf-100k) are not stored in the repository because they generate
    // large JSON files. Instead, they should be implemented manually in each language's test suite.
    GenerateTests("center", input => input.ToSample().Center(),
      new OneSampleInputBuilder()
        // Demo examples (n = 5) - 3 tests
        .Add("demo-1", new Sample(0, 2, 4, 6, 8))
        .Add("demo-2", new Sample(10, 12, 14, 16, 18))
        .Add("demo-3", new Sample(0, 6, 12, 18, 24))
        // Natural sequences (n = 1, 2, 3, 4) - 4 tests
        .AddNatural([1, 2, 3, 4])
        // Negative values (n = 3) - 1 test
        .Add("negative-3", new Sample(-3, -2, -1))
        // Zero values (n = 1, 2) - 2 tests
        .AddZero([1, 2])
        // Additive distribution (n = 5, 10, 30) - 3 tests
        .AddAdditive([5, 10, 30], count: 1)
        // Uniform distribution (n = 5, 100) - 2 tests
        .AddUniform([5, 100], count: 1)
        // Algorithm stress tests - 6 tests
        .Add("duplicates-5", new Sample(3, 3, 3, 3, 3))
        .Add("duplicates-10", new Sample(1, 1, 1, 2, 2, 2, 3, 3, 3, 3))
        .Add("parity-odd-7", new Sample(1, 2, 3, 4, 5, 6, 7))
        .Add("parity-even-6", new Sample(1, 2, 3, 4, 5, 6))
        .Add("parity-odd-49", new Sample(Enumerable.Range(1, 49).Select(x => (double)x).ToArray()))
        .Add("parity-even-50", new Sample(Enumerable.Range(1, 50).Select(x => (double)x).ToArray()))
        // Extreme values - 3 tests
        .Add("extreme-large-5", new Sample(1e8, 2e8, 3e8, 4e8, 5e8))
        .Add("extreme-small-5", new Sample(1e-8, 2e-8, 3e-8, 4e-8, 5e-8))
        .Add("extreme-wide-5", new Sample(0.001, 1, 100, 1000, 1000000))
        // Unsorted tests - 14 tests (verify sorting works correctly)
        .AddUnsortedReverse([2, 3, 4, 5, 7])  // 5 tests: reverse sorted
        .AddUnsortedShuffle("shuffle-3", 2, 1, 3)  // Middle element first
        .AddUnsortedShuffle("shuffle-4", 3, 1, 4, 2)  // Interleaved
        .AddUnsortedShuffle("shuffle-5", 5, 2, 4, 1, 3)  // Complex shuffle
        .AddUnsortedShuffle("last-first-5", 5, 1, 2, 3, 4)  // Last moved to first
        .AddUnsortedShuffle("first-last-5", 2, 3, 4, 5, 1)  // First moved to last
        .AddUnsortedPattern("duplicates-mixed-5", new Sample(3, 3, 3, 3, 3))  // All same (any order)
        .AddUnsortedPattern("duplicates-unsorted-10", new Sample(3, 1, 2, 3, 1, 3, 2, 1, 3, 2))  // Duplicates mixed
        .AddUnsortedShuffle("extreme-large-unsorted-5", 5e8, 1e8, 4e8, 2e8, 3e8)  // Large values unsorted
        .AddUnsortedShuffle("parity-odd-reverse-7", 7, 6, 5, 4, 3, 2, 1));  // Odd size reverse

    // Spread: 30 test cases (requires sparity: spread > 0, so no constant samples or n=1)
    // Note: Performance tests (perf-100k) are not stored in the repository because they generate
    // large JSON files. Instead, they should be implemented manually in each language's test suite.
    GenerateTests("spread", input => input.ToSample().Spread(),
      new OneSampleInputBuilder()
        // Demo examples (n = 5) - 3 tests
        .Add("demo-1", new Sample(0, 2, 4, 6, 8))
        .Add("demo-2", new Sample(10, 12, 14, 16, 18))
        .Add("demo-3", new Sample(0, 4, 8, 12, 16))
        // Natural sequences (n = 2, 3, 4) - 3 tests (n=1 excluded: spread=0)
        .AddNatural([2, 3, 4])
        // Negative values (n = 3) - 1 test
        .Add("negative-3", new Sample(-3, -2, -1))
        // Note: Zero samples excluded (spread=0 violates sparity)
        // Additive distribution (n = 5, 10, 30) - 3 tests
        .AddAdditive([5, 10, 30], count: 1)
        // Uniform distribution (n = 5, 100) - 2 tests
        .AddUniform([5, 100], count: 1)
        // Algorithm stress tests - 5 tests (duplicates-5 excluded: all same values have spread=0)
        .Add("duplicates-10", new Sample(1, 1, 1, 2, 2, 2, 3, 3, 3, 3))
        .Add("parity-odd-7", new Sample(1, 2, 3, 4, 5, 6, 7))
        .Add("parity-even-6", new Sample(1, 2, 3, 4, 5, 6))
        .Add("parity-odd-49", new Sample(Enumerable.Range(1, 49).Select(x => (double)x).ToArray()))
        .Add("parity-even-50", new Sample(Enumerable.Range(1, 50).Select(x => (double)x).ToArray()))
        // Extreme values - 3 tests
        .Add("extreme-large-5", new Sample(1e8, 2e8, 3e8, 4e8, 5e8))
        .Add("extreme-small-5", new Sample(1e-8, 2e-8, 3e-8, 4e-8, 5e-8))
        .Add("extreme-wide-5", new Sample(0.001, 1, 100, 1000, 1000000))
        // Unsorted tests - 10 tests (excluded: duplicates-mixed-5 with all same values)
        .AddUnsortedReverse([2, 3, 4, 5, 7])  // 5 tests: reverse sorted
        .AddUnsortedShuffle("shuffle-3", 3, 1, 2)  // Rotated
        .AddUnsortedShuffle("shuffle-4", 4, 2, 1, 3)  // Mixed order
        .AddUnsortedShuffle("shuffle-5", 5, 1, 3, 2, 4)  // Partial shuffle
        .AddUnsortedPattern("duplicates-unsorted-10", new Sample(2, 3, 1, 3, 2, 1, 2, 3, 1, 3))  // Duplicates mixed
        .AddUnsortedShuffle("extreme-wide-unsorted-5", 1000, 0.001, 1000000, 100, 1));  // Wide range unsorted

    // RelSpread: 18 test cases (requires positivity: all values > 0)
    GenerateTests("rel-spread", input => input.ToSample().RelSpread(),
      new OneSampleInputBuilder()
        // Demo examples (n = 5) - 2 tests (positive only)
        .Add("demo-1", new Sample(1, 3, 5, 7, 9))
        .Add("demo-2", new Sample(5, 15, 25, 35, 45))
        // Natural sequences (n = 2, 3, 4) - 3 tests (n=1 excluded: spread=0)
        .AddNatural([2, 3, 4])
        // Note: Negative values excluded (violates positivity)
        // Uniform distribution (n = 5, 10, 20, 30, 100) - 5 tests
        .AddUniform([5, 10, 20, 30, 100], count: 1)
        // Composite estimator stress tests - 3 tests
        .Add("composite-small-center", new Sample(0.001, 0.002, 0.003, 0.004, 0.005))
        .Add("composite-large-spread", new Sample(1, 100, 200, 300, 1000))
        .Add("composite-extreme-ratio", new Sample(1, 1.0001, 1.0002, 1.0003, 1.0004))
        // Unsorted tests - 5 tests (excluded: negative and zero values)
        .AddUnsortedReverse([3, 4, 5])  // 3 tests: reverse sorted
        .AddUnsortedPattern("composite-small-unsorted", new Sample(0.005, 0.001, 0.003, 0.002, 0.004))  // Small center unsorted
        .AddUnsortedPattern("composite-large-unsorted", new Sample(1000, 1, 300, 100, 200)));  // Large spread unsorted
  }

  private static void GenerateTests(string suiteName, Func<OneSampleInput, double> estimate,
    OneSampleInputBuilder inputBuilder)
  {
    AnsiConsole.MarkupLine($"[yellow]→[/] Generating tests for: [bold]{suiteName}[/]");
    var controller = new OneSampleEstimatorController(suiteName, estimate);
    var inputs = inputBuilder.Build();
    var testData = controller.GenerateData(inputs);
    controller.Save(testData);
    AnsiConsole.MarkupLine($"  [green]✓[/] Generated [bold]{testData.Count}[/] test cases");
  }
}
