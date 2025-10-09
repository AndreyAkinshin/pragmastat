using JetBrains.Annotations;
using Pragmastat.Internal;

namespace Pragmastat.Metrology;

public readonly struct RatioValue(double ratio)
{
    [PublicAPI] public double Ratio { get; } = ratio;

    public override string ToString() => Ratio.Format() + "x";
    public Measurement ToMeasurement() => new(Ratio, RatioUnit.Instance);
}