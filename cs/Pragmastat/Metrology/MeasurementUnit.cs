namespace Pragmastat.Metrology;

public class MeasurementUnit(string id, string family, string abbreviation, string fullName, long baseUnits = 1)
  : IEquatable<MeasurementUnit>
{
  public static readonly MeasurementUnit Number = new("number", "Number", "", "Number", 1);
  public static readonly MeasurementUnit Ratio = new("ratio", "Ratio", "", "Ratio", 1);
  public static readonly MeasurementUnit Disparity = new("disparity", "Disparity", "", "Disparity", 1);

  public string Id { get; } = id;
  public string Family { get; } = family;
  public string Abbreviation { get; } = abbreviation;
  public string FullName { get; } = fullName;
  public long BaseUnits { get; } = baseUnits;

  public bool IsCompatible(MeasurementUnit other) => Family == other.Family;

  public static MeasurementUnit Finer(MeasurementUnit a, MeasurementUnit b) =>
    a.BaseUnits <= b.BaseUnits ? a : b;

  public static double ConversionFactor(MeasurementUnit from, MeasurementUnit to) =>
    (double)from.BaseUnits / to.BaseUnits;

  public override string ToString() => Abbreviation;

  public bool Equals(MeasurementUnit? other)
  {
    if (ReferenceEquals(null, other)) return false;
    if (ReferenceEquals(this, other)) return true;
    return Id == other.Id && Family == other.Family && BaseUnits == other.BaseUnits;
  }

  public override bool Equals(object? obj)
  {
    if (ReferenceEquals(null, obj)) return false;
    if (ReferenceEquals(this, obj)) return true;
    return obj is MeasurementUnit other && Equals(other);
  }

  public override int GetHashCode()
  {
    unchecked
    {
      var hash = Id.GetHashCode();
      hash = hash * 397 ^ Family.GetHashCode();
      hash = hash * 397 ^ BaseUnits.GetHashCode();
      return hash;
    }
  }
  public static bool operator ==(MeasurementUnit? left, MeasurementUnit? right) => Equals(left, right);
  public static bool operator !=(MeasurementUnit? left, MeasurementUnit? right) => !Equals(left, right);
}
