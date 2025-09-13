using Pragmastat.Core;
using Pragmastat.Core.Estimators;
using Pragmastat.Core.Internal;
using Pragmastat.Core.Metrology;
using Pragmastat.Core.Metrology.Units;

namespace Pragmastat.Estimators;

public class DisparityEstimator(ITwoSampleEstimator shift, ITwoSampleEstimator spread) : ITwoSampleEstimator
{
    public static readonly DisparityEstimator Instance = new(ShiftEstimator.Instance, AvgSpreadEstimator.Instance);

    public Measurement Estimate(Sample x, Sample y)
    {
        var shiftValue = shift.Estimate(x, y);
        var spreadValue = spread.Estimate(x, y);
        return spreadValue.NominalValue == 0
            ? double.PositiveInfinity.WithUnit(NumberUnit.Instance)
            : (shiftValue / spreadValue).NominalValue.WithUnit(NumberUnit.Instance);
    }
}