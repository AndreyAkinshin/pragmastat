using Pragmastat.ReferenceTests.Generator.Framework.TwoSample;
using Spectre.Console;

namespace Pragmastat.ReferenceTests.Generator.TestCases;

public static class TwoSampleTestCases
{
  public static void Generate()
  {
    GenerateTests("shift", input => input.GetSampleX().Shift(input.GetSampleY()),
      new TwoSampleInputBuilder()
        .AddNatural([1, 2, 3], [1, 2, 3])
        .AddZero([1, 2], [1, 2])
        .AddNormal([5, 10, 30], [5, 10, 30], count: 1)
        .AddUniform([5, 100], [5, 100], count: 1));

    GenerateTests("ratio", input => input.GetSampleX().Ratio(input.GetSampleY()),
      new TwoSampleInputBuilder()
        .AddNatural([1, 2, 3], [1, 2, 3])
        .AddNormal([5, 10, 30], [5, 10, 30], count: 1)
        .AddUniform([5, 100], [5, 100], count: 1));

    GenerateTests("disparity", input => input.GetSampleX().Disparity(input.GetSampleY()),
      new TwoSampleInputBuilder()
        .AddNatural([2, 3], [2, 3])
        .AddUniform([5, 100], [5, 100], count: 1));

    GenerateTests("avg-spread", input => input.GetSampleX().AvgSpread(input.GetSampleY()),
      new TwoSampleInputBuilder()
        .AddNatural([1, 2, 3], [1, 2, 3])
        .AddZero([1, 2], [1, 2])
        .AddNormal([5, 10, 30], [5, 10, 30], count: 1)
        .AddUniform([5, 100], [5, 100], count: 1));
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

