using JetBrains.Annotations;
using Pragmastat.Core.Internal;

namespace Pragmastat.Core.Metrology.Units;

public readonly struct RatioValue(double ratio)
{
    [PublicAPI] public double Ratio { get; } = ratio;

    public override string ToString() => Ratio.Format() + "x";
    public Measurement ToMeasurement() => new(Ratio, RatioUnit.Instance);
}