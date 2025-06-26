using Pragmastat.Core.Internal;
using Pragmastat.Core.Metrology.Formatting;
using Pragmastat.Core.Metrology.Units;

namespace Pragmastat.Core.Metrology;

public class Measurement(double nominalValue, MeasurementUnit unit) : IComparable<Measurement>, IComparable
{
    public static Measurement Zero(MeasurementUnit? unit = null) => new(0, unit ?? NumberUnit.Instance);

    public double NominalValue { get; } = nominalValue;
    public MeasurementUnit Unit { get; } = unit;

    public static implicit operator double(Measurement measurement) => measurement.NominalValue;

    public static Measurement operator +(Measurement a, Measurement b)
    {
        Assertion.Equal(a.Unit, b.Unit);
        return new Measurement(a.NominalValue + b.NominalValue, a.Unit);
    }

    public static Measurement operator -(Measurement a, Measurement b)
    {
        Assertion.Equal(a.Unit, b.Unit);
        return new Measurement(a.NominalValue - b.NominalValue, a.Unit);
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
        if (Unit.GetFlavor() != other.Unit.GetFlavor())
            throw new InvalidOperationException($"Cannot compare units of different flavors: {this} and {other}");
        double a = NominalValue * Unit.BaseUnits;
        double b = other.NominalValue * Unit.BaseUnits;
        return a.CompareTo(b);
    }

    public int CompareTo(object? obj) => obj switch
    {
        null => 1,
        Measurement other => CompareTo(other),
        _ => throw new ArgumentException($"Object must be of type {nameof(Measurement)}")
    };

    public override string ToString() => MeasurementFormatter.Default.Format(this);
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