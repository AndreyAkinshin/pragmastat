using Pragmastat.Functions;
using Pragmastat.Randomization;
using Pragmastat.TestGenerator.Framework;
using Pragmastat.TestGenerator.Framework.SingleDoubleValue;
using Spectre.Console;

namespace Pragmastat.TestGenerator.TestCases;

/// <summary>
/// The shared fixture for the reproducible logarithm.
/// </summary>
/// <remarks>
/// <para>
/// This exists for the reason the exponential's does, one step sharper. Ratio is
/// exp(median(log x - log y)): the logarithm feeds a median, and a median selects an element out
/// of the pairwise set. A last-bit difference in log does not move the answer by a last bit, it
/// moves which element is selected, and the gap between two neighboring pairwise elements is
/// whatever the data says it is.
/// </para>
/// <para>
/// Without this suite the seven ports meet only through the ratio fixtures, which compare with a
/// tolerance precisely because the platform logarithm used to decide the last bits. Reaching a
/// function through a tolerant comparison is not covering it: the divergence this fixture exists
/// to catch is the one that comparison is set up to forgive.
/// </para>
/// <para>
/// Being generated, mise task "tests:check:generator-drift" regenerates these and requires a
/// byte-identical result, so they cannot drift from the code that produces them.
/// </para>
/// </remarks>
public static class LogFunctionTestCases
{
  public static void Generate()
  {
    const string suiteName = "log-function";
    AnsiConsole.MarkupLine($"[yellow]→[/] Generating tests for: [bold]{suiteName}[/]");

    var functions = new Dictionary<string, Func<double, double>> { ["log_function"] = LogFunction.Value };
    var inputBuilder = new ReferenceTestCaseInputBuilder<SingleDoubleValueInput>();

    // The band the estimators actually reach: ratio log-transforms sample values, and samples are
    // measurements. Geometric rather than linear, because that is how the reduction sees them:
    // a linear grid over 1e-6..1e6 would put almost every point in the same few exponents.
    inputBuilder.Add("working-band", new SingleDoubleValueInput("log_function", GeomGrid(1e-6, 1e6, 401)));

    // The full finite range, down to the smallest denormal and up to the largest finite double.
    // This is where the exponent extraction has to be right rather than merely plausible: the
    // denormal end has no exponent field to read and needs the pre-scaling.
    inputBuilder.Add("finite-range", new SingleDoubleValueInput("log_function", GeomGrid(5e-324, 1.5e308, 401)));

    // Around 1, where the result crosses zero. Every implementation's error is largest in relative
    // terms here, and it is the one place a wrong sign or a lost low bit is invisible in absolute
    // terms.
    inputBuilder.Add("near-one", new SingleDoubleValueInput("log_function", GeomGrid(0.5, 2.0, 401)));

    // Every place the construction changes behavior rather than merely continues: the two ends of
    // the reduced range and their immediate neighbors, where the recentering branch switches; the
    // powers of two, where the exponent increments; the denormal boundary, where the pre-scaling
    // turns on.
    //
    // Zero and negative arguments are absent because their results are -Infinity and NaN, and JSON
    // can write neither. Each port's own unit tests cover them, where a language can name both.
    const double sqrt2 = 1.4142135623730951;
    const double sqrt2Over2 = 0.7071067811865476;
    inputBuilder.Add("boundaries", new SingleDoubleValueInput("log_function",
    [
      1.0, 2.0, 0.5, 4.0, 0.25, 8.0, 1024.0, 1.0 / 1024.0,
      sqrt2, Math.BitIncrement(sqrt2), Math.BitDecrement(sqrt2),
      sqrt2Over2, Math.BitIncrement(sqrt2Over2), Math.BitDecrement(sqrt2Over2),
      Math.BitIncrement(1.0), Math.BitDecrement(1.0),
      Math.BitIncrement(2.0), Math.BitDecrement(2.0),
      Math.E, 10.0, 3.0, 7.0,
      2.2250738585072014e-308, Math.BitDecrement(2.2250738585072014e-308),
      double.Epsilon, 1e-320, 1e-300, 1e300,
      double.MaxValue, Math.BitDecrement(double.MaxValue),
    ]));

    // Uniformly random binary64 payloads, which the grids above cannot imitate. A geometric grid
    // walks the exponent smoothly and leaves the significand nearly fixed, so a fault in the
    // mantissa mask or in the exponent-field replacement can sit between two grid points and never
    // be evaluated. Drawing the whole 64-bit pattern spreads the significand uniformly instead.
    //
    // The draw is the toolkit's own Rng at a fixed seed, so the file regenerates byte for byte and
    // mise task "tests:check:generator-drift" holds it to that.
    inputBuilder.Add("random-payloads", new SingleDoubleValueInput("log_function", RandomPayloads(512)));

    // eps: 0 makes the generator's own check compare payloads, matching what the seven loaders do.
    var controller = new SingleDoubleValueController(suiteName, functions, eps: 0, shared: true);
    var testData = controller.GenerateData(inputBuilder.Build());
    controller.Save(testData);
    AnsiConsole.MarkupLine($"  [green]✓[/] Generated [bold]{testData.Count}[/] test cases");
  }

  /// <summary>
  /// <paramref name="count"/> positive finite doubles drawn as raw 64-bit patterns.
  /// </summary>
  /// <remarks>
  /// Rejects the patterns the suite cannot carry rather than mapping them onto something else: a
  /// negative, an infinity and a NaN have no JSON literal, and 1 takes the early return, which the
  /// boundaries file already states. Rejection keeps every value that does appear an untouched
  /// draw.
  /// </remarks>
  private static double[] RandomPayloads(int count)
  {
    var rng = new Rng(20260806);
    var values = new List<double>(count);
    while (values.Count < count)
    {
      // Assembled from four 16-bit draws. Rng.UniformInt64 refuses a range wider than 2^52,
      // because it draws through a double and could not deliver every value of a wider one, so
      // asking it for the full 64-bit range throws rather than returning a biased pattern.
      long bits = 0;
      for (int part = 0; part < 4; part++)
        bits = (bits << 16) | rng.UniformInt64(0, 0xffff);

      double candidate = BitConverter.Int64BitsToDouble(bits);
      if (candidate > 0 && !double.IsInfinity(candidate) && !double.IsNaN(candidate) && candidate != 1.0)
        values.Add(candidate);
    }
    return values.ToArray();
  }

  /// <summary>
  /// A grid geometric in the argument, so the points spread evenly over the exponents rather than
  /// bunching into the top one.
  /// </summary>
  private static double[] GeomGrid(double lo, double hi, int count)
  {
    double logLo = Math.Log(lo), logHi = Math.Log(hi);
    return Enumerable.Range(0, count)
      .Select(i => Math.Exp(logLo + (logHi - logLo) * i / (count - 1)))
      .Where(v => v > 0 && double.IsFinite(v))
      .ToArray();
  }
}
