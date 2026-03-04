using Pragmastat.TestGenerator.Framework.Compare;
using Spectre.Console;

namespace Pragmastat.TestGenerator.TestCases;

public static class Compare2TestCases
{
  private static readonly double[] X30 = Enumerable.Range(1, 30).Select(x => (double)x).ToArray();
  private static readonly double[] Y30 = Enumerable.Range(21, 30).Select(x => (double)x).ToArray();
  private static readonly double[] X20 = Enumerable.Range(1, 20).Select(x => (double)x).ToArray();
  private static readonly double[] Y20 = Enumerable.Range(11, 20).Select(x => (double)x).ToArray();

  public static void Generate()
  {
    const string suiteName = "compare2";
    AnsiConsole.MarkupLine($"[yellow]→[/] Generating tests for: [bold]{suiteName}[/]");

    var inputBuilder = new Compare2InputBuilder();
    var x30 = new Sample(X30);
    var y30 = new Sample(Y30);

    // Demo examples — single threshold, clear verdicts
    inputBuilder.Add("demo-shift-less", x30, y30, [new ThresholdInput("shift", 0, 0.02)]);
    inputBuilder.Add("demo-shift-greater", y30, x30, [new ThresholdInput("shift", 0, 0.02)]);
    inputBuilder.Add("demo-shift-inconclusive", x30, x30, [new ThresholdInput("shift", 0, 0.02)]);
    inputBuilder.Add("demo-ratio-less",
      new Sample(Enumerable.Range(1, 20).Select(v => (double)v).ToArray()),
      new Sample(Enumerable.Range(21, 20).Select(v => (double)v).ToArray()),
      [new ThresholdInput("ratio", 1, 0.02)]);
    inputBuilder.Add("demo-disparity-less", x30, y30, [new ThresholdInput("disparity", 0, 0.05)]);

    // Multi-threshold (multiple thresholds per call)
    inputBuilder.Add("multi-shift-ratio", x30, y30,
    [
      new ThresholdInput("shift", 0, 0.05),
      new ThresholdInput("ratio", 1, 0.05)
    ]);
    inputBuilder.Add("multi-shift-disparity", x30, y30,
    [
      new ThresholdInput("shift", 0, 0.05),
      new ThresholdInput("disparity", 0, 0.05)
    ]);
    inputBuilder.Add("multi-all-three", x30, y30,
    [
      new ThresholdInput("shift", 0, 0.05),
      new ThresholdInput("ratio", 1, 0.05),
      new ThresholdInput("disparity", 0, 0.05)
    ]);
    inputBuilder.Add("multi-two-shifts", x30, y30,
    [
      new ThresholdInput("shift", 5, 0.05),
      new ThresholdInput("shift", -5, 0.05)
    ]);

    // Input order preservation (reverse canonical order in input)
    inputBuilder.Add("order-disparity-shift", x30, y30,
    [
      new ThresholdInput("disparity", 0, 0.05),
      new ThresholdInput("shift", 0, 0.05)
    ]);
    inputBuilder.Add("order-ratio-shift", x30, y30,
    [
      new ThresholdInput("ratio", 1, 0.05),
      new ThresholdInput("shift", 0, 0.05)
    ]);

    // Misrate variation (x = [1..20], y = [11..30])
    var x20 = new Sample(X20);
    var y20 = new Sample(Y20);
    inputBuilder.Add("misrate-2e-1", x20, y20, [new ThresholdInput("shift", 0, 0.2)]);
    inputBuilder.Add("misrate-1e-1", x20, y20, [new ThresholdInput("shift", 0, 0.1)]);
    inputBuilder.Add("misrate-5e-2", x20, y20, [new ThresholdInput("shift", 0, 0.05)]);
    inputBuilder.Add("misrate-2e-2", x20, y20, [new ThresholdInput("shift", 0, 0.02)]);
    inputBuilder.Add("misrate-1e-2", x20, y20, [new ThresholdInput("shift", 0, 0.01)]);

    // Natural sequences (various sizes, misrate = 0.2)
    foreach (int nx in new[] { 10, 15 })
      foreach (int ny in new[] { 10, 15 })
      {
        var xN = new Sample(Enumerable.Range(1, nx).Select(v => (double)v).ToArray());
        var yN = new Sample(Enumerable.Range(nx + 1, ny).Select(v => (double)v).ToArray());
        inputBuilder.Add($"natural-{nx}-{ny}", xN, yN, [new ThresholdInput("shift", 0, 0.2)]);
      }

    // Property validation
    var baseX = new Sample(Enumerable.Range(1, 20).Select(v => (double)v).ToArray());
    inputBuilder.Add("property-shift-identity", baseX, baseX,
      [new ThresholdInput("shift", 0, 0.1)]);
    inputBuilder.Add("property-ratio-identity", baseX, baseX,
      [new ThresholdInput("ratio", 1, 0.1)]);

    var controller = new Compare2Controller(suiteName);
    var inputs = inputBuilder.Build();
    var testData = controller.GenerateData(inputs);
    controller.Save(testData);

    // Error test cases (AssumptionException)
    // Use object initializers to bypass Sample constructor validation
    controller.SaveErrorTestCase("error-empty-x",
      new Compare2Input { X = [], Y = X20, Seed = "compare2-tests", Thresholds = [new ThresholdInput("shift", 0, 0.05)] },
      "validity", "x");

    controller.SaveErrorTestCase("error-empty-y",
      new Compare2Input { X = X20, Y = [], Seed = "compare2-tests", Thresholds = [new ThresholdInput("shift", 0, 0.05)] },
      "validity", "y");

    controller.SaveErrorTestCase("error-constant-x-disparity",
      new Compare2Input { X = [5, 5, 5, 5, 5, 5, 5, 5, 5, 5], Y = X20, Seed = "compare2-tests", Thresholds = [new ThresholdInput("disparity", 0, 0.05)] },
      "sparity", "x");

    controller.SaveErrorTestCase("error-constant-y-disparity",
      new Compare2Input { X = X20, Y = [5, 5, 5, 5, 5, 5, 5, 5, 5, 5], Seed = "compare2-tests", Thresholds = [new ThresholdInput("disparity", 0, 0.05)] },
      "sparity", "y");

    AnsiConsole.MarkupLine($"  [green]✓[/] Generated [bold]{testData.Count}[/] test cases + error fixtures");
  }
}
