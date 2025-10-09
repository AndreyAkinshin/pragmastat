using Pragmastat.Internal;

namespace Pragmastat.Metrology;

public abstract class MeasurementUnit(string abbreviation, string fullName, long baseUnits = 1)
  : IEquatable<MeasurementUnit>
{
  public string Abbreviation { get; } = abbreviation;
  public string FullName { get; } = fullName;
  public long BaseUnits { get; } = baseUnits;

  public virtual string AbbreviationAscii => Abbreviation;
  public string GetFlavor() => GetType().Name.Replace("Unit", "");

  public override string ToString() => Abbreviation;
  public virtual string? DefaultFormat => null;

  public bool Equals(MeasurementUnit? other)
  {
    if (ReferenceEquals(null, other)) return false;
    if (ReferenceEquals(this, other)) return true;
    return Abbreviation == other.Abbreviation &&
           FullName == other.FullName &&
           BaseUnits == other.BaseUnits;
  }

  public override bool Equals(object? obj)
  {
    if (ReferenceEquals(null, obj)) return false;
    if (ReferenceEquals(this, obj)) return true;
    if (obj.GetType() != GetType()) return false;
    return Equals((MeasurementUnit)obj);
  }

  public override int GetHashCode() => HashCodeHelper.Combine(Abbreviation, FullName, BaseUnits);
  public static bool operator ==(MeasurementUnit? left, MeasurementUnit? right) => Equals(left, right);
  public static bool operator !=(MeasurementUnit? left, MeasurementUnit? right) => !Equals(left, right);
}
