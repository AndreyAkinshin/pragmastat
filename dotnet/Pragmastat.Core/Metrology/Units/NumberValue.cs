using Pragmastat.Core.Internal;

namespace Pragmastat.Core.Metrology.Units;

public readonly struct NumberValue(double value)
{
    public double Value { get; } = value;

    public static NumberValue Of(double value) => new(value);

    public override string ToString() => Value.ToStringInvariant();
}