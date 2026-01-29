using Pragmastat.Distributions;
using Pragmastat.TestGenerator.Framework.Distributions;
using Spectre.Console;

namespace Pragmastat.TestGenerator.TestCases;

public static class DistributionTestCases
{
  public static void Generate()
  {
    GenerateTests<Additive>("distribution-normal",
      new DistributionInputBuilder()
        .Add(new Additive(0, 1))
        .Add(new Additive(1, 2))
        .Add(new Additive(-1, 0.5))
        .Add(new Additive(5, 3))
        .Add(new Additive(0, 0.1))
        .Add(new Additive(-2, 10)));

    GenerateTests<Uniform>("distribution-uniform",
      new DistributionInputBuilder()
        .Add(new Uniform(0, 1))
        .Add(new Uniform(2, 3))
        .Add(new Uniform(-1, 1))
        .Add(new Uniform(-5, -2))
        .Add(new Uniform(0, 10))
        .Add(new Uniform(-2.5, 7.5)));
  }

  private static void GenerateTests<TDistribution>(string suiteName, DistributionInputBuilder inputBuilder)
    where TDistribution : IContinuousDistribution
  {
    AnsiConsole.MarkupLine($"[yellow]→[/] Generating tests for: [bold]{suiteName}[/]");
    var controller = new DistributionController<TDistribution>(suiteName);
    var inputs = inputBuilder.Build();
    var testData = controller.GenerateData(inputs);
    controller.Save(testData);
    AnsiConsole.MarkupLine($"  [green]✓[/] Generated [bold]{testData.Count}[/] test cases");
  }
}
