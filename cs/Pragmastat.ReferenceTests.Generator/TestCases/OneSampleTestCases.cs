using Pragmastat.ReferenceTests.Generator.Framework.OneSample;
using Spectre.Console;

namespace Pragmastat.ReferenceTests.Generator.TestCases;

public static class OneSampleTestCases
{
  public static void Generate()
  {
    GenerateTests("center", input => input.ToSample().Center(),
      new OneSampleInputBuilder()
        .AddNatural([1, 2, 3])
        .AddZero([1, 2])
        .AddNormal([5, 10, 30], count: 1)
        .AddUniform([5, 100], count: 1));

    GenerateTests("spread", input => input.ToSample().Spread(),
      new OneSampleInputBuilder()
        .AddNatural([1, 2, 3])
        .AddZero([1, 2])
        .AddNormal([5, 10, 30], count: 1)
        .AddUniform([5, 100], count: 1));

    GenerateTests("rel-spread", input => input.ToSample().RelSpread(),
      new OneSampleInputBuilder()
        .AddNatural([1, 2, 3])
        .AddUniform([5, 10, 20, 30, 100], count: 1));
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

