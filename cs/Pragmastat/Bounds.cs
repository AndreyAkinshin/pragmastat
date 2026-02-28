using System.Globalization;
using Pragmastat.Metrology;

namespace Pragmastat;

public class Bounds(double lower, double upper, MeasurementUnit unit)
{
  public double Lower { get; } = lower;
  public double Upper { get; } = upper;
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
