using System.Globalization;
using Pragmastat.Exceptions;
using Pragmastat.Internal;

namespace Pragmastat.Metrology;

public class Measurement(double nominalValue, MeasurementUnit unit) : IComparable<Measurement>, IComparable
{
  public static Measurement Zero(MeasurementUnit? unit = null) => new(0, unit ?? NumberUnit.Instance);

  public double NominalValue { get; } = nominalValue;
  public MeasurementUnit Unit { get; } = unit;

  public static implicit operator double(Measurement measurement) => measurement.NominalValue;

  public static Measurement operator +(Measurement a, Measurement b)
  {
    if (!a.Unit.IsCompatible(b.Unit))
      throw new UnitMismatchException(a.Unit, b.Unit);
    var target = MeasurementUnit.Finer(a.Unit, b.Unit);
    double av = a.NominalValue * MeasurementUnit.ConversionFactor(a.Unit, target);
    double bv = b.NominalValue * MeasurementUnit.ConversionFactor(b.Unit, target);
    return new Measurement(av + bv, target);
  }

  public static Measurement operator -(Measurement a, Measurement b)
  {
    if (!a.Unit.IsCompatible(b.Unit))
      throw new UnitMismatchException(a.Unit, b.Unit);
    var target = MeasurementUnit.Finer(a.Unit, b.Unit);
    double av = a.NominalValue * MeasurementUnit.ConversionFactor(a.Unit, target);
    double bv = b.NominalValue * MeasurementUnit.ConversionFactor(b.Unit, target);
    return new Measurement(av - bv, target);
  }

  public static Measurement operator +(Measurement a, double b) => new(a.NominalValue + b, a.Unit);
  public static Measurement operator +(double a, Measurement b) => new(a + b.NominalValue, b.Unit);
  public static Measurement operator +(Measurement a, int b) => new(a.NominalValue + b, a.Unit);
  public static Measurement operator +(int a, Measurement b) => new(a + b.NominalValue, b.Unit);

  public static Measurement operator -(Measurement a, double b) => new(a.NominalValue - b, a.Unit);
  public static Measurement operator -(double a, Measurement b) => new(a - b.NominalValue, b.Unit);
  public static Measurement operator -(Measurement a, int b) => new(a.NominalValue - b, a.Unit);
  public static Measurement operator -(int a, Measurement b) => new(a - b.NominalValue, b.Unit);

  public static Measurement operator *(Measurement a, double b) => new(a.NominalValue * b, a.Unit);
  public static Measurement operator *(Measurement a, int b) => new(a.NominalValue * b, a.Unit);
  public static Measurement operator *(double a, Measurement b) => new(a * b.NominalValue, b.Unit);
  public static Measurement operator *(int a, Measurement b) => new(a * b.NominalValue, b.Unit);

  public static Measurement operator /(Measurement a, double b) => new(a.NominalValue / b, a.Unit);
  public static Measurement operator /(Measurement a, int b) => new(a.NominalValue / b, a.Unit);

  public static bool operator <(Measurement? left, Measurement? right) =>
    Comparer<Measurement?>.Default.Compare(left, right) < 0;

  public static bool operator >(Measurement? left, Measurement? right) =>
    Comparer<Measurement?>.Default.Compare(left, right) > 0;

  public static bool operator <=(Measurement? left, Measurement? right) =>
    Comparer<Measurement?>.Default.Compare(left, right) <= 0;

  public static bool operator >=(Measurement? left, Measurement? right) =>
    Comparer<Measurement?>.Default.Compare(left, right) >= 0;

  public int CompareTo(Measurement? other)
  {
    if (ReferenceEquals(this, other))
      return 0;
    if (ReferenceEquals(null, other))
      return 1;
    if (!Unit.IsCompatible(other.Unit))
      throw new InvalidOperationException($"Cannot compare units of different families: {this} and {other}");
    double a = NominalValue * Unit.BaseUnits;
    double b = other.NominalValue * other.Unit.BaseUnits;
    return a.CompareTo(b);
  }

  public int CompareTo(object? obj) => obj switch
  {
    null => 1,
    Measurement other => CompareTo(other),
    _ => throw new ArgumentException($"Object must be of type {nameof(Measurement)}")
  };

  public override string ToString()
  {
    string value = NominalValue.ToString("G", CultureInfo.InvariantCulture);
    return Unit.Abbreviation.Length > 0 ? $"{value} {Unit.Abbreviation}" : value;
  }
}

public static class MeasurementExtensions
{
  public static double[] AsDoubles(this Measurement[] measurements)
  {
    double[] result = new double[measurements.Length];
    for (int i = 0; i < measurements.Length; i++)
      result[i] = measurements[i].NominalValue;
    return result;
  }
}
