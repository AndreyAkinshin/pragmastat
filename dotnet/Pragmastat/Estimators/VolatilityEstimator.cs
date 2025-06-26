using Pragmastat.Core;
using Pragmastat.Core.Estimators;
using Pragmastat.Core.Internal;
using Pragmastat.Core.Metrology;
using Pragmastat.Core.Metrology.Units;

namespace Pragmastat.Estimators;

public class VolatilityEstimator : IOneSampleEstimator
{
    public static readonly VolatilityEstimator Instance = new();

    public Measurement Estimate(Sample x)
    {
        var center = x.Center();
        if (center.NominalValue == 0)
            throw new ArgumentException("Volatility is undefined when Center equals zero", nameof(x));

        return (x.Spread() / Abs(center)).NominalValue.WithUnit(NumberUnit.Instance);
    }
}