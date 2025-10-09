namespace Pragmastat.Functions;

/// <summary>
/// Gauss error function erf
///
/// <remarks>
/// erf(z) = 2 / \sqrt{\pi} \int_0^z e^{-t^2} dt
/// </remarks>
/// </summary>
internal static class AbramowitzStegunErf
{
  /// <summary>
  /// The value of the error function
  ///
  /// <remarks>
  /// Numerical approximation with maximum error: 1.5 * 10^(-7)
  /// See Abramowitz and Stegun, equation 7.1.26
  /// </remarks>
  /// </summary>
  public static double Value(double x)
  {
    const double p = 0.3275911;
    const double a1 = 0.254829592;
    const double a2 = -0.284496736;
    const double a3 = 1.421413741;
    const double a4 = -1.453152027;
    const double a5 = 1.061405429;

    if (x < 0)
      return -Value(-x);

    double t = 1 / (1 + p * x);
    return 1 - (a1 * t + a2 * t * t + a3 * t * t * t + a4 * t * t * t * t + a5 * t * t * t * t * t) * Exp(-x * x);
  }
}
