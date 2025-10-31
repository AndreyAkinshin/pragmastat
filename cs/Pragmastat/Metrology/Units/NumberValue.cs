using Pragmastat.Internal;

namespace Pragmastat.Metrology;

public readonly struct NumberValue(double value)
{
  public double Value { get; } = value;

  public static NumberValue Of(double value) => new(value);

  public override string ToString() => Value.ToStringInvariant();
  public Measurement ToMeasurement() => new(Value, NumberUnit.Instance);
}
