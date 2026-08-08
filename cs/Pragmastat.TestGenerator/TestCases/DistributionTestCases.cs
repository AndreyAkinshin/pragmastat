using Pragmastat.Distributions;
using Pragmastat.TestGenerator.Framework.Distributions;
using Spectre.Console;

namespace Pragmastat.TestGenerator.TestCases;

public static class DistributionTestCases
{
  /// <summary>
  /// One shared suite per parametric family, holding the density, the cumulative and the quantile.
  /// </summary>
  /// <remarks>
  /// There used to be two suites here, for Additive and Uniform, written under cs/tests and read
  /// by C# alone. The other three families and the other six ports were unmeasured, which is how
  /// the trio came to exist in one language and nowhere else.
  /// </remarks>
  public static void Generate()
  {
    GenerateTests<Uniform>("dist-uniform",
      new DistributionInputBuilder()
        .Add(new Uniform(0, 1))
        .Add(new Uniform(2, 3))
        .Add(new Uniform(-1, 1))
        .Add(new Uniform(-5, -2))
        .Add(new Uniform(0, 10))
        .Add(new Uniform(-2.5, 7.5)));

    GenerateTests<Exp>("dist-exp",
      new DistributionInputBuilder()
        .Add(new Exp(1))
        .Add(new Exp(0.5))
        .Add(new Exp(2))
        .Add(new Exp(10))
        .Add(new Exp(0.01)));

    GenerateTests<Additive>("dist-additive",
      new DistributionInputBuilder()
        .Add(new Additive(0, 1))
        .Add(new Additive(1, 2))
        .Add(new Additive(-1, 0.5))
        .Add(new Additive(5, 3))
        .Add(new Additive(0, 0.1))
        .Add(new Additive(-2, 10)));

    GenerateTests<Multiplic>("dist-multiplic",
      new DistributionInputBuilder()
        .Add(new Multiplic(0, 1))
        .Add(new Multiplic(1, 0.5))
        .Add(new Multiplic(-1, 2))
        // A family whose median sits at 9.4e-14, which the density's old cutoff at 1e-9 reported
        // as zero across essentially the whole distribution.
        .Add(new Multiplic(-30, 1)));

    GenerateTests<Power>("dist-power",
      new DistributionInputBuilder()
        .Add(new Power(1, 1))
        .Add(new Power(1, 2))
        .Add(new Power(2, 3))
        .Add(new Power(0.5, 0.5))
        .Add(new Power(10, 5)));
  }

  private static void GenerateTests<TDistribution>(string suiteName, DistributionInputBuilder inputBuilder)
    where TDistribution : IDistribution
  {
    AnsiConsole.MarkupLine($"[yellow]→[/] Generating tests for: [bold]{suiteName}[/]");
    var controller = new DistributionController<TDistribution>(suiteName);
    var inputs = inputBuilder.Build();
    var testData = controller.GenerateData(inputs);
    controller.Save(testData);
    AnsiConsole.MarkupLine($"  [green]✓[/] Generated [bold]{testData.Count}[/] test cases");
  }
}
