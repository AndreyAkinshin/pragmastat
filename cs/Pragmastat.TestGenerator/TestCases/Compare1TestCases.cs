using Pragmastat.TestGenerator.Framework.Compare;
using Spectre.Console;

namespace Pragmastat.TestGenerator.TestCases;

public static class Compare1TestCases
{
  private static readonly double[] X10 = Enumerable.Range(1, 10).Select(x => (double)x).ToArray();
  private static readonly double[] X15 = Enumerable.Range(1, 15).Select(x => (double)x).ToArray();
  private static readonly double[] X20 = Enumerable.Range(1, 20).Select(x => (double)x).ToArray();

  public static void Generate()
  {
    const string suiteName = "compare1";
    AnsiConsole.MarkupLine($"[yellow]→[/] Generating tests for: [bold]{suiteName}[/]");

    var inputBuilder = new Compare1InputBuilder();

    // Demo examples — single threshold, clear verdicts
    var x = new Sample(X10);
    inputBuilder.Add("demo-center-less", x, [new ThresholdInput("center", 20, 0.05)]);
    inputBuilder.Add("demo-center-greater", x, [new ThresholdInput("center", 1, 0.05)]);
    inputBuilder.Add("demo-center-inconclusive", x, [new ThresholdInput("center", 5.5, 0.05)]);
    inputBuilder.Add("demo-spread-less", x, [new ThresholdInput("spread", 20, 0.2)]);
    inputBuilder.Add("demo-spread-greater", x, [new ThresholdInput("spread", 0.1, 0.2)]);

    // Multi-threshold (multiple thresholds per call)
    inputBuilder.Add("multi-center-spread", x,
    [
      new ThresholdInput("center", 20, 0.05),
      new ThresholdInput("spread", 0.1, 0.2)
    ]);
    inputBuilder.Add("multi-two-centers", x,
    [
      new ThresholdInput("center", 20, 0.05),
      new ThresholdInput("center", 1, 0.05)
    ]);
    inputBuilder.Add("multi-mixed", x,
    [
      new ThresholdInput("center", 3, 0.05),
      new ThresholdInput("spread", 5, 0.2),
      new ThresholdInput("center", 20, 0.05)
    ]);

    // Input order preservation (reverse canonical order in input)
    inputBuilder.Add("order-spread-center", x,
    [
      new ThresholdInput("spread", 5, 0.1),
      new ThresholdInput("center", 20, 0.05)
    ]);

    // Misrate variation (x = [1..20])
    var x20 = new Sample(X20);
    inputBuilder.Add("misrate-1e-1", x20, [new ThresholdInput("center", 10, 0.1)]);
    inputBuilder.Add("misrate-1e-2", x20, [new ThresholdInput("center", 10, 0.01)]);
    inputBuilder.Add("misrate-1e-3", x20, [new ThresholdInput("center", 10, 0.001)]);

    // Natural sequences
    inputBuilder.Add("natural-10", new Sample(X10), [new ThresholdInput("center", 5.5, 0.1)]);
    inputBuilder.Add("natural-15", new Sample(X15), [new ThresholdInput("center", 8, 0.1)]);
    inputBuilder.Add("natural-20", new Sample(X20), [new ThresholdInput("center", 10.5, 0.1)]);

    var controller = new Compare1Controller(suiteName);
    var inputs = inputBuilder.Build();
    var testData = controller.GenerateData(inputs);
    controller.Save(testData);

    // Error test cases (AssumptionException)
    // Use object initializers to bypass Sample constructor validation
    controller.SaveErrorTestCase("error-empty-x",
      new Compare1Input { X = [], Seed = "compare1-tests", Thresholds = [new ThresholdInput("center", 5, 0.05)] },
      "validity", "x");

    controller.SaveErrorTestCase("error-single-x-center",
      new Compare1Input { X = [5], Seed = "compare1-tests", Thresholds = [new ThresholdInput("center", 5, 0.5)] },
      "domain", "x");

    controller.SaveErrorTestCase("error-constant-spread",
      new Compare1Input { X = [5, 5, 5, 5, 5, 5], Seed = "compare1-tests", Thresholds = [new ThresholdInput("spread", 1, 0.2)] },
      "sparity", "x");

    AnsiConsole.MarkupLine($"  [green]✓[/] Generated [bold]{testData.Count}[/] test cases + error fixtures");
  }
}
