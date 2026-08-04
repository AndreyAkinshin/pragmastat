namespace Pragmastat.Metrology;

public class MeasurementUnit : IEquatable<MeasurementUnit>
{
  public static readonly MeasurementUnit Number = new("number", "Number", "", "Number", 1);
  public static readonly MeasurementUnit Ratio = new("ratio", "Ratio", "", "Ratio", 1);
  public static readonly MeasurementUnit Disparity = new("disparity", "Disparity", "", "Disparity", 1);

  /// <summary>
  /// Creates a measurement unit.
  /// </summary>
  /// <remarks>
  /// An empty abbreviation stays legal: the three dimensionless units have none.
  /// </remarks>
  /// <exception cref="ArgumentException">Thrown if id or family is empty.</exception>
  /// <exception cref="ArgumentOutOfRangeException">Thrown if baseUnits is not positive.</exception>
  public MeasurementUnit(string id, string family, string abbreviation, string fullName, long baseUnits = 1)
  {
    // A non-positive baseUnits poisons every conversion through this unit, and it makes Finer
    // pick it over any well-formed one. An empty family makes unrelated units compatible.
    if (string.IsNullOrEmpty(id))
      throw new ArgumentException("id cannot be empty", nameof(id));
    if (string.IsNullOrEmpty(family))
      throw new ArgumentException("family cannot be empty", nameof(family));
    if (baseUnits <= 0)
      throw new ArgumentOutOfRangeException(nameof(baseUnits), baseUnits, "baseUnits must be positive");
    Id = id;
    Family = family;
    Abbreviation = abbreviation;
    FullName = fullName;
    BaseUnits = baseUnits;
  }

  public string Id { get; }
  public string Family { get; }
  public string Abbreviation { get; }
  public string FullName { get; }
  public long BaseUnits { get; }

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
