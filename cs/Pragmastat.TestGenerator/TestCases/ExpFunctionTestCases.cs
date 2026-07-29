using Pragmastat.Functions;
using Pragmastat.TestGenerator.Framework;
using Pragmastat.TestGenerator.Framework.SingleDoubleValue;
using Spectre.Console;

namespace Pragmastat.TestGenerator.TestCases;

/// <summary>
/// The shared fixture for the reproducible exponential.
/// </summary>
/// <remarks>
/// <para>
/// Every other suite reaches this function through a margin, which means it is only ever checked
/// where an Edgeworth expansion happens to look. That is thin cover for the one function on the
/// decision path that IEEE 754 leaves free, and it leaves the seven ports free to disagree
/// anywhere the margins do not reach.
/// </para>
/// <para>
/// The gap was real rather than hypothetical. TypeScript carried a handful of expected payloads
/// transcribed by hand from the Go implementation, which is the right check written the wrong way:
/// a copy of another port's output goes stale silently, and this one did, the moment the assembly
/// inside the exponential changed.
/// </para>
/// <para>
/// So the payloads live here instead, generated with everything else and compared by payload
/// rather than by tolerance. Being generated, mise task "tests:check:generator-drift" regenerates
/// them and requires a byte-identical result, so they cannot drift from the code that produces
/// them either.
/// </para>
/// </remarks>
public static class ExpFunctionTestCases
{
  public static void Generate()
  {
    const string suiteName = "exp-function";
    AnsiConsole.MarkupLine($"[yellow]→[/] Generating tests for: [bold]{suiteName}[/]");

    var functions = new Dictionary<string, Func<double, double>> { ["exp_function"] = ExpFunction.Value };
    var inputBuilder = new ReferenceTestCaseInputBuilder<SingleDoubleValueInput>();

    inputBuilder.Add("working-band", new SingleDoubleValueInput("exp_function", Grid(-18.5, 0.0, 401)));
    inputBuilder.Add("edgeworth-band", new SingleDoubleValueInput("exp_function", Grid(-40.0, 0.0, 201)));
    inputBuilder.Add("finite-range", new SingleDoubleValueInput("exp_function", Grid(-745.0, 709.0, 401)));

    // The reduction endpoints, the underflow cutoff and its neighbors, and the first denormals:
    // every place the construction changes behavior rather than merely continues.
    //
    // Arguments above 709.78 are absent because their result is infinite and JSON has no way to
    // write one. The overflow cutoff is checked in each port's own unit tests, where a language
    // can name its infinity; everything JSON can carry is here.
    const double ln2 = 0.69314718055994531;
    inputBuilder.Add("boundaries", new SingleDoubleValueInput("exp_function",
    [
      0.0, -0.0, 1.0, -1.0, 0.5, -0.5, 100.0, -100.0,
      ln2, -ln2, ln2 / 2, -ln2 / 2, 3 * ln2 / 2, -3 * ln2 / 2,
      709.0, 709.5, 709.78,
      -708.0, -744.0, -744.44, -745.0, -745.1, -745.2, -745.3, -746.0,
      1e-300, -1e-300, double.Epsilon, -double.Epsilon,
    ]));

    // eps: 0 makes the generator's own check compare payloads, matching what the seven loaders do.
    var controller = new SingleDoubleValueController(suiteName, functions, eps: 0, shared: true);
    var testData = controller.GenerateData(inputBuilder.Build());
    controller.Save(testData);
    AnsiConsole.MarkupLine($"  [green]✓[/] Generated [bold]{testData.Count}[/] test cases");
  }

  private static double[] Grid(double lo, double hi, int count) =>
    Enumerable.Range(0, count).Select(i => lo + (hi - lo) * i / (count - 1)).ToArray();
}
