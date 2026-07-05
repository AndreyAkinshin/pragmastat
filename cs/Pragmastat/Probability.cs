using System.Globalization;
using Pragmastat.Internal;

namespace Pragmastat;

public readonly struct Probability : IComparable, IComparable<Probability>, IEquatable<Probability>, IFormattable
{
  public readonly double Value;

  public Probability(double value)
  {
    // InRangeInclusive uses (value < min || value > max), which is false for NaN; reject NaN
    // explicitly so C# matches Kotlin (which rejects NaN).
    if (double.IsNaN(value))
      throw new ArgumentOutOfRangeException(nameof(value), value, $"{nameof(value)} must not be NaN");
    Assertion.InRangeInclusive(nameof(value), value, 0, 1);
    Value = value;
  }

  public static Probability Of(double value) => new(value);

  public static implicit operator double(Probability probability) => probability.Value;
  public static implicit operator Probability(double value) => new(value);

  public override string ToString()
  {
    return Value.ToString(CultureInfo.InvariantCulture);
  }

  public int CompareTo(object? obj)
  {
    return Value.CompareTo(obj);
  }

  public string ToString(IFormatProvider formatProvider)
  {
    return Value.ToString(formatProvider);
  }

  public string ToString(string? format, IFormatProvider? formatProvider = null)
  {
    return Value.ToString(format, formatProvider ?? CultureInfo.InvariantCulture);
  }

  public bool Equals(Probability other)
  {
    return Value.Equals(other.Value);
  }

  public override bool Equals(object? obj)
  {
    return obj is Probability other && Equals(other);
  }

  public override int GetHashCode()
  {
    return Value.GetHashCode();
  }

  public int CompareTo(Probability other)
  {
    return Value.CompareTo(other.Value);
  }

  public static Probability[] ToProbabilities(double[] values)
  {
    var probabilities = new Probability[values.Length];
    for (int i = 0; i < values.Length; i++)
      probabilities[i] = values[i];
    return probabilities;
  }
}
