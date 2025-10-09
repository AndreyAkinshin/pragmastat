namespace Pragmastat.Functions;

/// <summary>
/// Gauss error function erf
///
/// <remarks>
/// erf(z) = 2 / \sqrt{\pi} \int_0^z e^{-t^2} dt
/// </remarks>
/// </summary>
internal static class ErrorFunction
{
  /// <summary>
  /// The value of the error function
  /// </summary>
  public static double Value(double x) => AbramowitzStegunErf.Value(x);

  /// <summary>
  /// The value of the inverse error function
  /// </summary>
  public static double InverseValue(double p) => ErfInverse.Value(p);
}
