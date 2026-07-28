using System.Globalization;
using Pragmastat.Internal;
using Pragmastat.Metrology;

namespace Pragmastat;

public class Bounds(double lower, double upper, MeasurementUnit unit)
{
  // Every bounds estimator reaches its caller through this constructor, so normalizing here is
  // the single place that keeps a negative zero out of every endpoint the library hands back.
  // See DoubleExtensions.NormalizeZero for why the sign of a zero must not survive.
  public double Lower { get; } = lower.NormalizeZero();
  public double Upper { get; } = upper.NormalizeZero();
  public MeasurementUnit Unit { get; } = unit;

  public bool Contains(double x) => Lower <= x && x <= Upper;

  public override string ToString()
  {
    string lo = Lower.ToString("G", CultureInfo.InvariantCulture);
    string hi = Upper.ToString("G", CultureInfo.InvariantCulture);
    string unitSuffix = Unit.Abbreviation.Length > 0 ? " " + Unit.Abbreviation : "";
    return $"[{lo};{hi}]{unitSuffix}";
  }
}
