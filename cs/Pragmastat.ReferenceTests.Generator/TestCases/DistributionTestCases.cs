using Pragmastat.Distributions;
using Pragmastat.ReferenceTests.Generator.Framework.Distributions;
using Spectre.Console;

namespace Pragmastat.ReferenceTests.Generator.TestCases;

public static class DistributionTestCases
{
  public static void Generate()
  {
    GenerateTests<AdditiveDistribution>("distribution-normal",
      new DistributionInputBuilder()
        .Add(new AdditiveDistribution())
        .Add(new AdditiveDistribution(1, 2))
        .Add(new AdditiveDistribution(-1, 0.5))
        .Add(new AdditiveDistribution(5, 3))
        .Add(new AdditiveDistribution(0, 0.1))
        .Add(new AdditiveDistribution(-2, 10)));

    GenerateTests<UniformDistribution>("distribution-uniform",
      new DistributionInputBuilder()
        .Add(new UniformDistribution(0, 1))
        .Add(new UniformDistribution(2, 3))
        .Add(new UniformDistribution(-1, 1))
        .Add(new UniformDistribution(-5, -2))
        .Add(new UniformDistribution(0, 10))
        .Add(new UniformDistribution(-2.5, 7.5)));
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

