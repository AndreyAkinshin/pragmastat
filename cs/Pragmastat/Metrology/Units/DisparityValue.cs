using Pragmastat.Internal;

namespace Pragmastat.Metrology;

public readonly struct DisparityValue(double value)
{
  public double Value { get; } = value;

  public static DisparityValue Of(double value) => new(value);

  public override string ToString() => Value.ToStringInvariant();
}

