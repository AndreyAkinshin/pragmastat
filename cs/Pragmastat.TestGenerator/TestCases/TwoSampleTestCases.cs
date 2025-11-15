using Pragmastat.TestGenerator.Framework.TwoSample;
using Spectre.Console;

namespace Pragmastat.TestGenerator.TestCases;

public static class TwoSampleTestCases
{
  public static void Generate()
  {
    // Shift: 60 test cases (42 original + 18 unsorted)
    // Note: Performance tests (perf-100k-100k) are not stored in the repository because they generate
    // large JSON files. Instead, they should be implemented manually in each language's test suite.
    GenerateTests("shift", input => input.GetSampleX().Shift(input.GetSampleY()),
      new TwoSampleInputBuilder()
        // Demo examples (n = m = 5) - 5 tests
        .Add("demo-1", new Sample(0, 2, 4, 6, 8), new Sample(10, 12, 14, 16, 18))
        .Add("demo-2", new Sample(0, 2, 4, 6, 8), new Sample(0, 2, 4, 6, 8))
        .Add("demo-3", new Sample(7, 9, 11, 13, 15), new Sample(13, 15, 17, 19, 21))
        .Add("demo-4", new Sample(0, 4, 8, 12, 16), new Sample(20, 24, 28, 32, 36))
        .Add("demo-5", new Sample(10, 12, 14, 16, 18), new Sample(0, 2, 4, 6, 8))
        // Natural sequences (9 combinations) - 9 tests
        .AddNatural([1, 2, 3], [1, 2, 3])
        // Negative values (n = m = 2) - 1 test
        .Add("negative-2-2", new Sample(-2, -1), new Sample(-2, -1))
        // Mixed-sign values (n = m = 2) - 1 test
        .Add("mixed-2-2", new Sample(-1, 1), new Sample(-1, 1))
        // Zero values (4 combinations) - 4 tests
        .AddZero([1, 2], [1, 2])
        // Additive distribution (9 combinations) - 9 tests
        .AddAdditive([5, 10, 30], [5, 10, 30], count: 1)
        // Uniform distribution (4 combinations) - 4 tests
        .AddUniform([5, 100], [5, 100], count: 1)
        // Algorithm stress tests - 6 tests
        .Add("duplicates-5-5", new Sample(3, 3, 3, 3, 3), new Sample(3, 3, 3, 3, 3))
        .Add("duplicates-10-10", new Sample(1, 1, 2, 2, 3, 3, 4, 4, 5, 5), new Sample(1, 1, 2, 2, 3, 3, 4, 4, 5, 5))
        .Add("parity-odd-7-7", new Sample(1, 2, 3, 4, 5, 6, 7), new Sample(1, 2, 3, 4, 5, 6, 7))
        .Add("parity-even-6-6", new Sample(1, 2, 3, 4, 5, 6), new Sample(1, 2, 3, 4, 5, 6))
        .Add("parity-asymmetric-7-6", new Sample(1, 2, 3, 4, 5, 6, 7), new Sample(1, 2, 3, 4, 5, 6))
        .Add("parity-large-49-50",
          new Sample(Enumerable.Range(1, 49).Select(x => (double)x).ToArray()),
          new Sample(Enumerable.Range(1, 50).Select(x => (double)x).ToArray()))
        // Extreme asymmetry - 3 tests
        .Add("asymmetry-1-100", new Sample(50), new Sample(Enumerable.Range(1, 100).Select(x => (double)x).ToArray()))
        .Add("asymmetry-2-50", new Sample(10, 20), new Sample(Enumerable.Range(1, 50).Select(x => (double)x).ToArray()))
        .Add("asymmetry-constant-varied", new Sample(5, 5, 5, 5, 5), new Sample(1, 2, 3, 4, 5, 6, 7, 8, 9, 10))
        // Unsorted tests - 18 tests (verify independent sorting of each sample)
        .AddUnsortedVariants("natural", 3, 3)  // 3 tests: unsorted-x, unsorted-y, unsorted-both
        .AddUnsortedVariants("natural", 4, 4)  // 3 tests
        .AddUnsortedVariants("natural", 5, 5)  // 3 tests
        .AddUnsorted("reverse-3-3", new Sample(3, 2, 1), new Sample(3, 2, 1))  // Both reversed
        .AddUnsorted("x-shuffle-3-3", new Sample(2, 1, 3), new Sample(1, 2, 3))  // X shuffled, Y sorted
        .AddUnsorted("y-shuffle-3-3", new Sample(1, 2, 3), new Sample(3, 1, 2))  // X sorted, Y shuffled
        .AddUnsorted("both-shuffle-4-4", new Sample(3, 1, 4, 2), new Sample(4, 2, 1, 3))  // Both shuffled
        .AddUnsorted("duplicates-mixed-5-5", new Sample(3, 3, 3, 3, 3), new Sample(3, 3, 3, 3, 3))  // All same
        .AddUnsorted("x-unsorted-duplicates", new Sample(2, 1, 3, 2, 1), new Sample(1, 1, 2, 2, 3))  // X has unsorted duplicates
        .AddUnsorted("y-unsorted-duplicates", new Sample(1, 1, 2, 2, 3), new Sample(3, 2, 1, 3, 2))  // Y has unsorted duplicates
        .AddUnsorted("asymmetric-unsorted-2-5", new Sample(2, 1), new Sample(5, 2, 4, 1, 3))  // Asymmetric sizes, both unsorted
        .AddUnsorted("negative-unsorted-3-3", new Sample(-1, -3, -2), new Sample(-2, -3, -1)));

    // Ratio: 37 test cases (25 original + 12 unsorted)
    GenerateTests("ratio", input => input.GetSampleX().Ratio(input.GetSampleY()),
      new TwoSampleInputBuilder()
        // Demo examples (n = m = 5) - 3 tests
        .Add("demo-1", new Sample(1, 2, 4, 8, 16), new Sample(2, 4, 8, 16, 32))
        .Add("demo-2", new Sample(1, 2, 4, 8, 16), new Sample(1, 2, 4, 8, 16))
        .Add("demo-3", new Sample(2, 4, 8, 16, 32), new Sample(10, 20, 40, 80, 160))
        // Natural sequences (9 combinations) - 9 tests
        .AddNatural([1, 2, 3], [1, 2, 3])
        // Additive distribution (9 combinations) - 9 tests
        .AddAdditive([5, 10, 30], [5, 10, 30], count: 1)
        // Uniform distribution (4 combinations) - 4 tests
        .AddUniform([5, 100], [5, 100], count: 1)
        // Unsorted tests - 12 tests (verify independent sorting for ratio calculation)
        .AddUnsortedVariants("natural", 3, 3)  // 3 tests
        .AddUnsortedVariants("natural", 4, 4)  // 3 tests
        .AddUnsorted("demo-unsorted-x", new Sample(16, 1, 8, 2, 4), new Sample(2, 4, 8, 16, 32))  // Demo-1 with X unsorted
        .AddUnsorted("demo-unsorted-y", new Sample(1, 2, 4, 8, 16), new Sample(32, 2, 16, 4, 8))  // Demo-1 with Y unsorted
        .AddUnsorted("demo-both-unsorted", new Sample(8, 1, 16, 4, 2), new Sample(16, 32, 2, 8, 4))  // Demo-1 both unsorted
        .AddUnsorted("identity-unsorted", new Sample(4, 1, 8, 2, 16), new Sample(16, 1, 8, 4, 2))  // Identity property, both unsorted
        .AddUnsorted("asymmetric-unsorted-2-3", new Sample(2, 1), new Sample(3, 1, 2))  // Asymmetric, both unsorted
        .AddUnsorted("power-unsorted-5", new Sample(16, 2, 8, 1, 4), new Sample(32, 4, 16, 2, 8)));  // Powers of 2 unsorted

    // AvgSpread: 49 test cases (35 original + 14 unsorted)
    // Important: AvgSpread computes Spread(x) and Spread(y) independently, so unsorted tests
    // verify that both samples are sorted independently before computing their spreads
    GenerateTests("avg-spread", input => input.GetSampleX().AvgSpread(input.GetSampleY()),
      new TwoSampleInputBuilder()
        // Demo examples (n = m = 5) - 5 tests
        .Add("demo-1", new Sample(0, 3, 6, 9, 12), new Sample(0, 2, 4, 6, 8))
        .Add("demo-2", new Sample(0, 3, 6, 9, 12), new Sample(0, 3, 6, 9, 12))
        .Add("demo-3", new Sample(0, 6, 12, 18, 24), new Sample(0, 9, 18, 27, 36))
        .Add("demo-4", new Sample(0, 2, 4, 6, 8), new Sample(0, 3, 6, 9, 12))
        .Add("demo-5", new Sample(0, 6, 12, 18, 24), new Sample(0, 4, 8, 12, 16))
        // Natural sequences (9 combinations) - 9 tests
        .AddNatural([1, 2, 3], [1, 2, 3])
        // Negative values (n = m = 2) - 1 test
        .Add("negative-2-2", new Sample(-2, -1), new Sample(-2, -1))
        // Zero values (4 combinations) - 4 tests
        .AddZero([1, 2], [1, 2])
        // Additive distribution (9 combinations) - 9 tests
        .AddAdditive([5, 10, 30], [5, 10, 30], count: 1)
        // Uniform distribution (4 combinations) - 4 tests
        .AddUniform([5, 100], [5, 100], count: 1)
        // Composite estimator stress tests - 3 tests
        .Add("composite-asymmetric-weights", new Sample(1, 2), new Sample(3, 4, 5, 6, 7, 8, 9, 10))
        .Add("composite-zero-spread-one", new Sample(5, 5, 5), new Sample(1, 2, 3, 4, 5))
        .Add("composite-extreme-sizes", new Sample(10), new Sample(1, 2, 3, 4, 5, 6, 7, 8, 9, 10))
        // Unsorted tests - 14 tests (critical for verifying independent sorting)
        .AddUnsortedVariants("natural", 3, 3)  // 3 tests
        .AddUnsortedVariants("natural", 4, 4)  // 3 tests
        .AddUnsorted("demo-unsorted-x", new Sample(12, 0, 6, 3, 9), new Sample(0, 2, 4, 6, 8))  // Demo-1 with X unsorted
        .AddUnsorted("demo-unsorted-y", new Sample(0, 3, 6, 9, 12), new Sample(8, 0, 4, 2, 6))  // Demo-1 with Y unsorted
        .AddUnsorted("demo-both-unsorted", new Sample(9, 0, 12, 3, 6), new Sample(6, 0, 8, 2, 4))  // Demo-1 both unsorted
        .AddUnsorted("identity-unsorted", new Sample(6, 0, 12, 3, 9), new Sample(9, 0, 12, 6, 3))  // Demo-2 unsorted
        .AddUnsorted("negative-unsorted", new Sample(-1, -2), new Sample(-1, -2))  // Negative unsorted
        .AddUnsorted("zero-unsorted-2-2", new Sample(0, 0), new Sample(0, 0))  // Zeros (any order)
        .AddUnsorted("asymmetric-weights-unsorted", new Sample(2, 1), new Sample(8, 3, 6, 4, 10, 5, 9, 7))  // Asymmetric unsorted
        .AddUnsorted("zero-spread-x-unsorted", new Sample(5, 5, 5), new Sample(5, 1, 4, 2, 3)));

    // Disparity: 28 test cases (16 original + 12 unsorted)
    // Important: Disparity = Shift / AvgSpread, so unsorted tests verify both components
    GenerateTests("disparity", input => input.GetSampleX().Disparity(input.GetSampleY()),
      new TwoSampleInputBuilder()
        // Demo examples (n = m = 5) - 4 tests
        .Add("demo-1", new Sample(0, 3, 6, 9, 12), new Sample(0, 2, 4, 6, 8))
        .Add("demo-2", new Sample(5, 8, 11, 14, 17), new Sample(5, 7, 9, 11, 13))
        .Add("demo-3", new Sample(0, 6, 12, 18, 24), new Sample(0, 4, 8, 12, 16))
        .Add("demo-4", new Sample(0, 2, 4, 6, 8), new Sample(0, 3, 6, 9, 12))
        // Natural sequences (4 combinations: [2,3] x [2,3]) - 4 tests
        .AddNatural([2, 3], [2, 3])
        // Negative values (n = m = 2) - 1 test
        .Add("negative-2-2", new Sample(-2, -1), new Sample(-2, -1))
        // Uniform distribution (4 combinations) - 4 tests
        .AddUniform([5, 100], [5, 100], count: 1)
        // Composite estimator stress tests - 3 tests
        .Add("composite-small-avgspread", new Sample(10.001, 10.002, 10.003), new Sample(10.004, 10.005, 10.006))
        .Add("composite-large-avgspread", new Sample(1, 100, 200), new Sample(50, 150, 250))
        .Add("composite-extreme-disparity", new Sample(1, 1.001), new Sample(100, 100.001))
        // Unsorted tests - 12 tests (verify both Shift and AvgSpread handle sorting)
        .AddUnsortedVariants("natural", 3, 3)  // 3 tests
        .AddUnsortedVariants("natural", 4, 4)  // 3 tests
        .AddUnsorted("demo-unsorted-x", new Sample(12, 0, 6, 3, 9), new Sample(0, 2, 4, 6, 8))  // Demo-1 with X unsorted
        .AddUnsorted("demo-unsorted-y", new Sample(0, 3, 6, 9, 12), new Sample(8, 0, 4, 2, 6))  // Demo-1 with Y unsorted
        .AddUnsorted("demo-both-unsorted", new Sample(9, 0, 12, 3, 6), new Sample(6, 0, 8, 2, 4))  // Demo-1 both unsorted
        .AddUnsorted("location-invariance-unsorted", new Sample(17, 5, 11, 8, 14), new Sample(13, 5, 9, 7, 11))  // Demo-2 unsorted
        .AddUnsorted("scale-invariance-unsorted", new Sample(24, 0, 12, 6, 18), new Sample(16, 0, 8, 4, 12))  // Demo-3 unsorted
        .AddUnsorted("anti-symmetry-unsorted", new Sample(8, 0, 4, 2, 6), new Sample(12, 0, 6, 3, 9)));  // Demo-4 reversed and unsorted
  }

  private static void GenerateTests(string suiteName, Func<TwoSampleInput, double> estimate,
    TwoSampleInputBuilder inputBuilder)
  {
    AnsiConsole.MarkupLine($"[yellow]→[/] Generating tests for: [bold]{suiteName}[/]");
    var controller = new TwoSampleEstimatorController(suiteName, estimate);
    var inputs = inputBuilder.Build();
    var testData = controller.GenerateData(inputs);
    controller.Save(testData);
    AnsiConsole.MarkupLine($"  [green]✓[/] Generated [bold]{testData.Count}[/] test cases");
  }
}

