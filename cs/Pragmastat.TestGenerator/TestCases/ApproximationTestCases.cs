using Pragmastat.Distributions;
using Pragmastat.Functions;
using Pragmastat.TestGenerator.Framework;
using Pragmastat.TestGenerator.Framework.SingleDoubleValue;
using Spectre.Console;

namespace Pragmastat.TestGenerator.TestCases;

public static class ApproximationTestCases
{
  public static void Generate()
  {
    const string suiteName = "approximations";
    AnsiConsole.MarkupLine($"[yellow]→[/] Generating tests for: [bold]{suiteName}[/]");

    var functions = new Dictionary<string, Func<double, double>>();
    var inputBuilder = new ReferenceTestCaseInputBuilder<SingleDoubleValueInput>();

    // Register approximation functions
    double[] milliles = Uniform(0, 1, 1001, 1);
    double[] normalMilliles = milliles.Select(p => AdditiveDistribution.Standard.Quantile(p)).ToArray();

    functions["acm209"] = AcmAlgorithm209.Gauss;
    inputBuilder.Add("acm209", new SingleDoubleValueInput("acm209", normalMilliles));

    functions["erf"] = AbramowitzStegunErf.Value;
    inputBuilder.Add("erf", new SingleDoubleValueInput("erf", normalMilliles));

    functions["erf_inverse"] = ErfInverse.Value;
    inputBuilder.Add("erf_inverse", new SingleDoubleValueInput("erf_inverse", Uniform(-1, 1, 1001, 1)));

    var controller = new SingleDoubleValueController(suiteName, functions);
    var inputs = inputBuilder.Build();
    var testData = controller.GenerateData(inputs);
    controller.Save(testData);
    AnsiConsole.MarkupLine($"  [green]✓[/] Generated [bold]{testData.Count}[/] test cases");
  }

  private static double[] Uniform(double l, double r, int count, int trim = 0)
  {
    return Enumerable.Range(0, count)
      .Select(i => 1.0 * (l * (count - 1 - i) + r * i) / (count - 1))
      .Skip(trim)
      .SkipLast(trim)
      .ToArray();
  }
}

