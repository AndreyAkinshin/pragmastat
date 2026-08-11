using Pragmastat.Exceptions;
using Pragmastat.Metrology;

namespace Pragmastat.Internal;

/// <summary>
/// Shared sample preparation for the distribution estimation models.
/// </summary>
/// <remarks>
/// Runs the two assumptions that follow validity in the canonical order: <c>domain(x)</c> (at
/// least two observations and a finite representable range) and <c>distinctness(x)</c> (no
/// repeated value; -0.0 and +0.0 count as repeated). The caller's array is never retained: the
/// model always owns a private copy.
/// </remarks>
internal static class DistSampleGuard
{
  public static double[] Prepare(double[] values, bool assumeSorted)
  {
    double[] copy = new double[values.Length];
    Array.Copy(values, copy, values.Length);
    if (!assumeSorted)
      Array.Sort(copy);

    int n = copy.Length;
    if (n < 2)
      throw AssumptionException.Domain(Subject.X);

    double range = copy[n - 1] - copy[0];
    if (double.IsNaN(range) || double.IsInfinity(range))
      throw AssumptionException.Domain(Subject.X);

    Assertion.DistinctnessAssumption(copy, Subject.X);

    return copy;
  }

  /// <summary>Attaches a unit to every element of a query result.</summary>
  public static Measurement[] ToMeasurements(double[] values, MeasurementUnit unit)
  {
    Measurement[] result = new Measurement[values.Length];
    for (int i = 0; i < values.Length; i++)
      result[i] = new Measurement(values[i], unit);
    return result;
  }

  /// <summary>
  /// Converts a measured query vector into the model's own unit.
  /// </summary>
  /// <remarks>
  /// A query in a compatible but different unit is converted rather than rejected, matching the
  /// two-sample estimators, which reconcile their inputs instead of demanding an exact match. An
  /// incompatible unit is a caller error and is reported as one.
  /// </remarks>
  public static double[] ToRaw(Measurement[] values, MeasurementUnit unit)
  {
    Assertion.NotNull(nameof(values), values);
    double[] raw = new double[values.Length];
    for (int i = 0; i < values.Length; i++)
    {
      MeasurementUnit source = values[i].Unit;
      if (!source.IsCompatible(unit))
        throw new UnitMismatchException(source, unit);
      raw[i] = values[i].Value * MeasurementUnit.ConversionFactor(source, unit);
    }

    return raw;
  }
}
