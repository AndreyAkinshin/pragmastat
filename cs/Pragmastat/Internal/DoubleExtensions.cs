using System.Globalization;
using Pragmastat.Metrology;

namespace Pragmastat.Internal;

internal static class DoubleExtensions
{
  public static string ToStringInvariant(this double value)
  {
    return value.ToString(CultureInfo.InvariantCulture);
  }

  public static string ToStringInvariant(this double value, string format)
  {
    return value.ToString(format, CultureInfo.InvariantCulture);
  }

  public static Measurement WithUnit(this double value, MeasurementUnit unit) => new(value, unit);
  public static Measurement WithUnitOf(this double value, Sample sample) => new(value, sample.Unit);

  public static bool IsFinite(this double value) => !double.IsNaN(value) && !double.IsInfinity(value);

  /// <summary>
  /// Replaces a negative zero with a positive one. Every other value passes through untouched,
  /// infinities and NaN included.
  /// </summary>
  /// <remarks>
  /// <para>
  /// A sample holding both <c>+0.0</c> and <c>-0.0</c> makes the reported value depend on which of
  /// the two the sort happened to place in the selected position. Comparison cannot tell them
  /// apart, so that position is decided by the sorting algorithm rather than by the data, and each
  /// language port brings its own sort: <c>Center([0, -0, 0, -0, 1])</c> answered <c>-0.0</c> in
  /// one port and <c>+0.0</c> in the others. That contradicts the exact conformance class the
  /// toolkit publishes, which promises identical bits from identical inputs.
  /// </para>
  /// <para>
  /// The sign of a zero carries no statistical meaning here: <c>-0.0 == 0.0</c>, and no estimate
  /// becomes less accurate once the sign is dropped. Every estimator therefore normalizes it away
  /// on the way out, which closes the whole class of divergence rather than the one sample that
  /// exposed it. Only outputs are normalized: a sample may still contain <c>-0.0</c>.
  /// </para>
  /// </remarks>
  public static double NormalizeZero(this double value) => value == 0 ? 0 : value;

  public static string Format(this double value, string? format = null, IFormatProvider? formatProvider = null)
  {
    return value.ToString(format ?? "G", formatProvider ?? CultureInfo.InvariantCulture);
  }
}
