using Pragmastat.Functions;
using Pragmastat.TestGenerator.Framework;
using Pragmastat.TestGenerator.Framework.SingleDoubleValue;
using Spectre.Console;

namespace Pragmastat.TestGenerator.TestCases;

/// <summary>
/// The shared fixture for the reproducible inverse normal CDF.
/// </summary>
/// <remarks>
/// <para>
/// Phi^-1 exists in seven copies for the reason the logarithm and the exponential do: IEEE 754
/// fixes nothing about it, and a quantile is a value a caller compares and reports. Nothing calls
/// it yet, so without this suite the seven implementations would meet nowhere at all.
/// </para>
/// <para>
/// The arguments are chosen where the construction changes behavior rather than where it merely
/// continues. The function has three branches and two seams between them, and a seam is where a
/// port that mis-copied a bound sends an argument down the wrong approximant: the two sides of a
/// seam must agree to the last bits even though they arrive by different polynomials. The central
/// band is where most callers land and where the odd form must hold exactly. The far tail runs to
/// the smallest positive double, which is the last argument a caller can pass, and is the only
/// place the second tail approximant is exercised at all.
/// </para>
/// <para>
/// Arguments at or outside the open interval are absent because their result is an infinity and
/// JSON can write neither infinity nor NaN. Those are covered in each port's own unit tests, where
/// a language can name its infinity; everything JSON can carry is here.
/// </para>
/// <para>
/// Being generated, mise task "tests:check:generator-drift" regenerates the suite and requires a
/// byte-identical result, so the payloads cannot drift from the code that produces them.
/// </para>
/// </remarks>
public static class AdditiveQuantileTestCases
{
  public static void Generate()
  {
    const string suiteName = "additive-quantile";
    AnsiConsole.MarkupLine($"[yellow]→[/] Generating tests for: [bold]{suiteName}[/]");

    var functions = new Dictionary<string, Func<double, double>>
    {
      ["additive_quantile"] = AdditiveQuantile.Value
    };
    var inputBuilder = new ReferenceTestCaseInputBuilder<SingleDoubleValueInput>();

    inputBuilder.Add("central", new SingleDoubleValueInput("additive_quantile", Central()));
    inputBuilder.Add("branch-seams", new SingleDoubleValueInput("additive_quantile", BranchSeams()));
    inputBuilder.Add("near-tail", new SingleDoubleValueInput("additive_quantile", NearTail()));
    inputBuilder.Add("far-tail", new SingleDoubleValueInput("additive_quantile", FarTail()));
    inputBuilder.Add("textbook", new SingleDoubleValueInput("additive_quantile", Textbook()));

    // eps: 0 makes the generator's own check compare payloads, matching what the seven loaders do.
    var controller = new SingleDoubleValueController(suiteName, functions, eps: 0, shared: true);
    var testData = controller.GenerateData(inputBuilder.Build());
    controller.Save(testData);
    AnsiConsole.MarkupLine($"  [green]✓[/] Generated [bold]{testData.Count}[/] test cases");
  }

  /// <summary>
  /// The central band, walked densely, plus the center itself.
  /// </summary>
  /// <remarks>
  /// This branch multiplies its approximant by q = p - 1/2, so it is odd by construction and
  /// returns exactly zero at the middle. Walking it densely is what catches a transcription error
  /// in a coefficient: the rational form is smooth, so a wrong digit shows up as a smooth bias
  /// rather than as a spike at one argument.
  /// </remarks>
  private static double[] Central()
  {
    var values = new List<double> { 0.5 };
    for (int i = 1; i <= 200; i++)
    {
      double q = 0.425 * i / 200.0;
      values.Add(0.5 + q);
      values.Add(0.5 - q);
    }
    return values.ToArray();
  }

  /// <summary>
  /// Both seams between the three branches, approached from each side.
  /// </summary>
  /// <remarks>
  /// The first seam is |p - 1/2| = 0.425, where the central form gives way to the tail. The second
  /// is w = 5 in the tail variable, which is p = exp(-25), and it separates the two tail
  /// approximants. A port that copied a bound as a strict comparison where it should be
  /// non-strict, or that transposed the two tail branches, produces its first wrong answer here
  /// and nowhere else.
  /// </remarks>
  private static double[] BranchSeams()
  {
    var values = new List<double>();
    foreach (double p in new[] { 0.075, 0.925, Math.Exp(-25.0), 1.0 - Math.Exp(-25.0) })
    {
      values.Add(p);
      double up = p, down = p;
      for (int i = 0; i < 3; i++)
      {
        up = Math.BitIncrement(up);
        down = Math.BitDecrement(down);
        values.Add(up);
        values.Add(down);
      }
    }
    return values.Where(p => p > 0.0 && p < 1.0).Distinct().ToArray();
  }

  /// <summary>
  /// The near tail, on both sides, spread geometrically over the band it covers.
  /// </summary>
  private static double[] NearTail()
  {
    var values = new List<double>();
    for (int k = 2; k <= 11; k++)
    {
      double p = Math.Pow(10.0, -k * 0.5 - 1.0);
      values.Add(p);
      values.Add(1.0 - p);
    }
    return values.Where(p => p > 0.0 && p < 1.0).Distinct().ToArray();
  }

  /// <summary>
  /// The far tail, by decades, ending at the smallest positive double.
  /// </summary>
  /// <remarks>
  /// Only the lower side runs to the extreme: 1 - p stops resolving long before p does, so an
  /// upper-tail argument past 1 - 2^-53 is simply 1 and carries no information. The lower side is
  /// where the second tail approximant is reached, and the last argument here is the last one a
  /// caller can pass at all.
  /// </remarks>
  private static double[] FarTail()
  {
    var values = new List<double>();
    for (int k = 12; k <= 320; k += 4)
      values.Add(Math.Pow(10.0, -k));
    values.Add(double.Epsilon);
    values.Add(2.2250738585072014e-308);
    return values.Where(p => p > 0.0 && p < 1.0).Distinct().ToArray();
  }

  /// <summary>
  /// The quantiles a reader is most likely to recognize, and their complements.
  /// </summary>
  private static double[] Textbook()
  {
    var values = new List<double>();
    foreach (double p in new[] { 0.75, 0.9, 0.95, 0.975, 0.99, 0.995, 0.999, 0.9995, 0.99995 })
    {
      values.Add(p);
      values.Add(1.0 - p);
    }
    return values.Distinct().ToArray();
  }
}
