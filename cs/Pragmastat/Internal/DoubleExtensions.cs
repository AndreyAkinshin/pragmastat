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

  public static string Format(this double value, string? format = null, IFormatProvider? formatProvider = null)
  {
    return value.ToString(format ?? "G", formatProvider ?? CultureInfo.InvariantCulture);
  }
}
