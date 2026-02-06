namespace Pragmastat.Algorithms;

using System;
using System.Collections.Generic;
using System.Linq;
using Pragmastat.Exceptions;
using Pragmastat.Internal;

/// <summary>
/// Computes quantiles of pairwise ratios via log-transformation and FastShift delegation.
/// Ratio(x, y) = exp(Shift(log(x), log(y)))
/// </summary>
public static class FastRatio
{
  /// <summary>
  /// Computes quantiles of all pairwise ratios { x_i / y_j }.
  /// Time: O((m + n) * log(precision)) per quantile. Space: O(m + n).
  /// </summary>
  /// <remarks>
  /// Log-transformation preserves sort order for positive values.
  /// </remarks>
  /// <param name="x">First sample (must be strictly positive).</param>
  /// <param name="y">Second sample (must be strictly positive).</param>
  /// <param name="p">Probabilities in [0, 1].</param>
  /// <param name="assumeSorted">If true, assumes x and y are already sorted in ascending order.</param>
  /// <returns>Quantile values for each probability in p.</returns>
  public static double[] Estimate(IReadOnlyList<double> x, IReadOnlyList<double> y, double[] p, bool assumeSorted = false)
  {
    // Log-transform both samples (includes positivity check)
    var logX = MathExtensions.Log(x, Subject.X);
    var logY = MathExtensions.Log(y, Subject.Y);

    // Delegate to FastShift in log-space
    var logResult = FastShift.Estimate(logX, logY, p, assumeSorted);

    // Exp-transform back to ratio-space
    return logResult.Select(v => Math.Exp(v)).ToArray();
  }
}
